/**
 * The GraphSolve client.
 *
 * Hand-written rather than generated. The behaviour here — which status codes
 * are retried, which failures are charged, and the idempotency key — is the
 * same in every client in this repo and is documented in CONTRIBUTING.md.
 */

import { TokenProvider } from "./auth.js";
import { GeneratedMethods } from "./generated.js";
import {
  AuthenticationError,
  GraphSolveError,
  InsufficientCredits,
  InvalidPayload,
  OutcomeUnknown,
  RateLimited,
  ServerError,
  SolverDidNotConverge,
  ToolNotPermitted,
} from "./errors.js";
import type {
  CancelResponse,
  Job,
  Me,
  QuoteResponse,
  SubmitResponse,
  ToolArguments,
  ToolResponse,
  ToolSummary,
} from "./types.js";

export const DEFAULT_BASE_URL = "https://api-prod.graphsolve.ai";

/**
 * The platform endpoint that issues access tokens. Tokens are issued centrally
 * for every deployment, so this is not derived from the base URL.
 */
export const DEFAULT_TOKEN_URL = "https://prod-user-api.graphsolve.ai/oauth/token";

/**
 * Solves can take minutes. The server's load balancer allows 900 s; a shorter
 * client timeout would abandon work that is still running and still charged.
 */
export const DEFAULT_TIMEOUT_MS = 900_000;

/**
 * Retried whatever the method: neither status reached the engine, so repeating
 * the request cannot duplicate a charge. Any other 4xx fails the same way
 * every time.
 */
const RETRYABLE_STATUS = new Set([429, 503]);

/**
 * Retried on a GET only. A 502 or 504 comes from the load balancer, which
 * cannot say whether the engine already ran the tool — and a tool that ran is
 * metered whether or not its response arrived. The engine does not read
 * `Idempotency-Key`, so repeating a POST would be charged twice.
 */
const RETRYABLE_IF_REPEATABLE = new Set([502, 504]);

/**
 * Said alongside `OutcomeUnknown`. The type is what a caller branches on; this
 * is what a human reads in a log.
 */
const MAY_HAVE_RUN =
  "This request was not retried: it may have reached the engine and run, " +
  "in which case it was charged.";

/** Whether `status` may be retried for a request using `method`. */
function retryable(status: number, method: string): boolean {
  if (RETRYABLE_STATUS.has(status)) {
    return true;
  }
  return RETRYABLE_IF_REPEATABLE.has(status) && method === "GET";
}

/** The error for a call abandoned without knowing whether it ran. */
function outcomeUnknown(message: string, envelope: Partial<ToolResponse> = {}): OutcomeUnknown {
  return new OutcomeUnknown(
    `${message.trimEnd().replace(/\.$/, "")}. ${MAY_HAVE_RUN}`,
    envelope,
  );
}

export type FetchLike = (
  input: string,
  init?: RequestInit,
) => Promise<Response>;

export interface GraphSolveOptions {
  /** Falls back to `GRAPHSOLVE_API_KEY`. Server-side only. */
  apiKey?: string;
  /** Falls back to `GRAPHSOLVE_BASE_URL`, then the production API. */
  baseUrl?: string;
  /** Falls back to `GRAPHSOLVE_TOKEN_URL`, then the production platform. */
  tokenUrl?: string;
  timeoutMs?: number;
  maxRetries?: number;
  /** Injectable for tests, proxies, or a runtime without a global `fetch`. */
  fetch?: FetchLike;
  /** @internal Replaced in tests so the suite does not wait out a backoff. */
  sleep?: (ms: number) => Promise<void>;
}

export interface RunOptions {
  pollIntervalMs?: number;
  maxWaitMs?: number;
}

const wait = (ms: number): Promise<void> =>
  new Promise((resolve) => setTimeout(resolve, ms));

/**
 * A client for the GraphSolve engine API.
 *
 * ```ts
 * const gs = new GraphSolve();      // reads GRAPHSOLVE_API_KEY
 * await gs.convert_units({ value: 1, from_unit: "MPa", to_unit: "psi" });
 * ```
 *
 * One typed method per tool, generated from the API spec, so an editor offers
 * the valid correlation names rather than accepting any string. Use `call()`
 * for a tool newer than the installed version.
 *
 * Method and argument names are snake_case, matching the wire format.
 *
 * The API key is exchanged for an access token on the first call and the token
 * is refreshed before it expires.
 */
export class GraphSolve extends GeneratedMethods {
  private readonly tokens: TokenProvider;
  private readonly baseUrl: string;
  private readonly timeoutMs: number;
  private readonly maxRetries: number;
  private readonly fetchImpl: FetchLike;
  private readonly sleep: (ms: number) => Promise<void>;

  constructor(options: GraphSolveOptions = {}) {
    super();
    // `process` is absent in a browser and on some edge runtimes. An API key
    // should not be shipped to a browser in any case.
    const env = typeof process !== "undefined" ? process.env : undefined;
    const apiKey = options.apiKey ?? env?.GRAPHSOLVE_API_KEY;
    if (!apiKey) {
      throw new AuthenticationError(
        "No API key. Pass { apiKey } or set GRAPHSOLVE_API_KEY.",
      );
    }
    const impl = options.fetch ?? globalThis.fetch;
    if (!impl) {
      throw new GraphSolveError(
        "No global fetch. Use Node 18 or newer, or pass { fetch }.",
      );
    }

    // `||` for the environment so an empty variable falls through to the default.
    const baseUrl = options.baseUrl ?? (env?.GRAPHSOLVE_BASE_URL || DEFAULT_BASE_URL);
    const tokenUrl = options.tokenUrl ?? (env?.GRAPHSOLVE_TOKEN_URL || DEFAULT_TOKEN_URL);

    this.baseUrl = baseUrl.replace(/\/+$/, "");
    this.timeoutMs = options.timeoutMs ?? DEFAULT_TIMEOUT_MS;
    this.maxRetries = options.maxRetries ?? 3;
    this.fetchImpl = impl;
    this.sleep = options.sleep ?? wait;
    this.tokens = new TokenProvider({
      apiKey,
      tokenUrl,
      fetch: this.fetchImpl,
      maxRetries: this.maxRetries,
      sleep: this.sleep,
    });
  }

  /** The current access token, for a caller wiring their own HTTP. */
  getAccessToken(): Promise<string> {
    return this.tokens.get();
  }

  // ------------------------------------------------------------------ tools

  /**
   * Run a tool and return its response envelope.
   *
   * Every generated method calls this. Use it directly for a tool newer than
   * this client.
   */
  async call<T = unknown>(
    tool: string,
    args: ToolArguments = {},
  ): Promise<ToolResponse<T>> {
    const envelope = await this.request<ToolResponse<T>>(
      "POST",
      `/v1/tools/${tool}`,
      { body: args },
    );
    assertEnvelopeOk(envelope, tool);
    return envelope;
  }

  /** Every tool, with its domain, summary and price. */
  async tools(): Promise<ToolSummary[]> {
    const body = await this.request<{ tools: ToolSummary[] }>(
      "GET",
      "/v1/tools",
    );
    return body.tools;
  }

  /** One tool's full definition: schema, examples, price. */
  describe(tool: string): Promise<Record<string, unknown>> {
    return this.request("GET", `/v1/tools/${tool}`);
  }

  /**
   * The identity and scopes the API key's token carries.
   *
   * Balances are held by the platform, not the engine, so they are not
   * reported here.
   */
  me(): Promise<Me> {
    return this.request("GET", "/v1/me");
  }

  // ------------------------------------------------------------------- jobs

  /**
   * Price a long-running call without running it.
   *
   * Spends nothing: no compute, no job. Worth calling before a transient run,
   * where one field can change the cost by orders of magnitude.
   */
  quote(tool: string, args: ToolArguments): Promise<QuoteResponse> {
    return this.request("POST", `/v1/jobs/${tool}`, {
      body: args,
      query: { dry_run: "true" },
    });
  }

  /** Start a job. Returns immediately with the job and its quote. */
  submit(tool: string, args: ToolArguments): Promise<SubmitResponse> {
    return this.request("POST", `/v1/jobs/${tool}`, { body: args });
  }

  /** Poll a job. */
  async job(jobId: string): Promise<Job> {
    const body = await this.request<{ job: Job }>(
      "GET",
      `/v1/jobs/id/${jobId}`,
    );
    return body.job;
  }

  /**
   * Cancel a job. Compute already under way may run to completion and is
   * charged.
   */
  cancel(jobId: string): Promise<CancelResponse> {
    return this.request("POST", `/v1/jobs/id/${jobId}/cancel`);
  }

  /**
   * Submit a job and resolve when it finishes.
   *
   * Cancels the job if `maxWaitMs` is exceeded, so giving up on the wait also
   * stops the charge.
   */
  async run(
    tool: string,
    args: ToolArguments,
    { pollIntervalMs = 5_000, maxWaitMs }: RunOptions = {},
  ): Promise<ToolResponse> {
    const submitted = await this.submit(tool, args);
    const jobId = submitted.job.id;
    const started = Date.now();

    let job: Job;
    for (;;) {
      job = await this.job(jobId);
      if (job.state === "succeeded" || job.state === "failed" || job.state === "cancelled") {
        break;
      }
      if (maxWaitMs !== undefined && Date.now() - started > maxWaitMs) {
        await this.cancel(jobId);
        throw new GraphSolveError(
          `Job ${jobId} exceeded maxWaitMs of ${maxWaitMs} and was cancelled.`,
        );
      }
      await this.sleep(pollIntervalMs);
    }

    if (job.state !== "succeeded" || !job.result) {
      throw new SolverDidNotConverge(
        `Job ${jobId} finished as ${job.state}: ${job.error ?? "no result"}`,
        job.charged_credits ?? 0,
        job.result ?? {},
      );
    }
    return job.result;
  }

  // --------------------------------------------------------------- internal

  private async request<T>(
    method: string,
    path: string,
    options: { body?: unknown; query?: Record<string, string> } = {},
  ): Promise<T> {
    const query = options.query
      ? `?${new URLSearchParams(options.query)}`
      : "";
    const url = `${this.baseUrl}${path}${query}`;
    const headers: Record<string, string> = {
      "user-agent": "graphsolve-typescript",
      // One key per logical call, reused across its retries. The engine does
      // not read it yet; send it so that a server that does needs no change
      // here. Until then, not retrying a POST is what stops a double charge.
      "idempotency-key": crypto.randomUUID(),
    };
    let body: string | undefined;
    if (options.body !== undefined) {
      headers["content-type"] = "application/json";
      // JSON.stringify drops undefined properties, so an omitted optional is
      // absent from the payload rather than null. Several tools treat "not
      // given" and null differently.
      body = JSON.stringify(options.body);
    }

    const repeatable = method === "GET";

    let lastError: unknown;
    for (let attempt = 0; attempt <= this.maxRetries; attempt++) {
      // Read per attempt, so a long retry sequence cannot outlive the token,
      // and outside the try so a credential failure is not mistaken for a
      // network one and retried.
      headers.authorization = `Bearer ${await this.tokens.get()}`;

      let response: Response;
      try {
        response = await this.fetchImpl(url, {
          method,
          headers,
          body,
          signal: AbortSignal.timeout(this.timeoutMs),
        });
      } catch (error) {
        lastError = error;
        // A POST that got no answer may still have been received and run, so
        // it is reported rather than repeated.
        const message = `Could not reach GraphSolve: ${String(error)}`;
        if (!repeatable) {
          throw outcomeUnknown(message);
        }
        if (attempt === this.maxRetries) {
          throw new GraphSolveError(message);
        }
        await this.sleep(backoffMs(attempt));
        continue;
      }

      if (retryable(response.status, method) && attempt < this.maxRetries) {
        await this.sleep(
          backoffMs(attempt, response.headers.get("retry-after")),
        );
        continue;
      }

      return decode<T>(response, repeatable);
    }

    throw new GraphSolveError(`Request failed after retries: ${String(lastError)}`);
  }
}

/** Exponential backoff with jitter, deferring to `Retry-After`. */
function backoffMs(attempt: number, retryAfter?: string | null): number {
  if (retryAfter) {
    const seconds = Number(retryAfter);
    if (Number.isFinite(seconds)) {
      return seconds * 1000;
    }
  }
  // Jitter, so clients throttled at the same time do not all retry at the
  // same time.
  return 2 ** attempt * (0.5 + Math.random()) * 1000;
}

async function decode<T>(response: Response, repeatable = true): Promise<T> {
  let body: Record<string, unknown> = {};
  try {
    body = (await response.json()) as Record<string, unknown>;
  } catch {
    // A gateway can answer with HTML, or nothing at all.
  }
  const errors = (body.errors as string[] | undefined) ?? [];
  const message = errors[0] ?? response.statusText ?? "Unknown error";
  const envelope = body as Partial<ToolResponse>;
  const status = response.status;

  if (status < 400) {
    return body as T;
  }
  switch (status) {
    case 400:
      throw new InvalidPayload(message, envelope);
    case 401:
      throw new AuthenticationError(message, envelope);
    case 402:
      throw new InsufficientCredits(message, envelope);
    case 403:
      throw new ToolNotPermitted(message, envelope);
    case 429: {
      const header = response.headers.get("retry-after");
      const seconds = header === null ? undefined : Number(header);
      throw new RateLimited(
        message,
        seconds !== undefined && Number.isFinite(seconds) ? seconds : undefined,
        envelope,
      );
    }
    default:
      if (status >= 500) {
        // 502 and 504 are the load balancer's, not the engine's: the tool may
        // have run and been metered. Only a POST reaches here unretried.
        if (RETRYABLE_IF_REPEATABLE.has(status) && !repeatable) {
          throw outcomeUnknown(message, envelope);
        }
        throw new ServerError(message, envelope);
      }
      throw new GraphSolveError(`HTTP ${status}: ${message}`, envelope);
  }
}

/**
 * Throw if a 200 response reports a failed calculation.
 *
 * The call ran and is charged, but there is no `result` to unpack, so throw
 * here rather than let the caller fail further downstream.
 */
function assertEnvelopeOk(envelope: ToolResponse, tool: string): void {
  if (envelope.status === "success") {
    return;
  }
  const charged = envelope.metadata?.billing?.credits_charged ?? 0;
  const message = envelope.errors?.[0] ?? "no result";
  throw new SolverDidNotConverge(`${tool}: ${message}`, charged, envelope);
}
