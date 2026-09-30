/**
 * The behavioural contract from CONTRIBUTING.md, as executable tests.
 *
 * Each of these is a rule every client in this repo must follow.
 */

import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { afterEach, describe, expect, it } from "vitest";

import {
  AuthenticationError,
  DEFAULT_TOKEN_URL,
  GraphSolve,
  GeneratedMethods,
  InsufficientCredits,
  InvalidPayload,
  OutcomeUnknown,
  RateLimited,
  SolverDidNotConverge,
} from "../src/index.js";
import { refreshMarginMs } from "../src/index.js";
import type { GraphSolveOptions, ToolResponse } from "../src/index.js";

const KEY = "gs_test_0123456789abcdef0123456789abcdef";
const TOKEN_URL = "https://auth.test/oauth/token";

type Handler = (url: string, init: RequestInit) => Response;

/** Every backoff this client would have waited out, in milliseconds. */
let slept: number[] = [];

afterEach(() => {
  slept = [];
  delete process.env.GRAPHSOLVE_API_KEY;
  delete process.env.GRAPHSOLVE_BASE_URL;
  delete process.env.GRAPHSOLVE_TOKEN_URL;
});

const tokenBody = (token = "tok-1", expiresIn = 3600) => ({
  access_token: token,
  token_type: "Bearer",
  expires_in: expiresIn,
});

interface Exchange {
  url: string;
  authorization: string | undefined;
  grantType: string | null;
}

/**
 * Wrap a handler so the token endpoint is served alongside it, and record every
 * exchange so a test can assert how many the client performed, where it sent
 * them and what it sent.
 */
function withToken(
  handler: Handler,
  { expiresIn = 3600, tokens = [] as string[] } = {},
): { handler: Handler; exchanges: Exchange[] } {
  const exchanges: Exchange[] = [];
  const wrapped: Handler = (url, init) => {
    if (new URL(url).pathname === "/oauth/token") {
      exchanges.push({
        url,
        authorization: headerOf(init, "authorization"),
        grantType: new URLSearchParams(init.body as string).get("grant_type"),
      });
      const token = tokens[exchanges.length - 1] ?? "tok-1";
      return json(200, tokenBody(token, expiresIn));
    }
    return handler(url, init);
  };
  return { handler: wrapped, exchanges };
}

function client(handler: Handler, options: GraphSolveOptions = {}): GraphSolve {
  return new GraphSolve({
    apiKey: KEY,
    baseUrl: "https://api.test",
    tokenUrl: TOKEN_URL,
    fetch: async (url, init) => handler(url, init ?? {}),
    sleep: async (ms) => {
      slept.push(ms);
    },
    ...options,
  });
}

/** The common case: a working token endpoint plus the handler under test. */
function authed(handler: Handler, options: GraphSolveOptions = {}): GraphSolve {
  return client(withToken(handler).handler, options);
}

function json(status: number, body: unknown, headers: Record<string, string> = {}): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "content-type": "application/json", ...headers },
  });
}

function envelope(
  overrides: Partial<ToolResponse> & { charged?: number } = {},
): ToolResponse {
  const { charged = 0, ...rest } = overrides;
  const status = rest.status ?? "success";
  return {
    status,
    result: null,
    warnings: [],
    errors: status === "success" ? [] : ["it did not converge"],
    metadata: { calculation_time_ms: 1, billing: { tool: "t", credits_charged: charged } },
    ...rest,
  };
}

const bodyOf = (init: RequestInit): Record<string, unknown> =>
  JSON.parse(init.body as string) as Record<string, unknown>;

const headerOf = (init: RequestInit, name: string): string | undefined =>
  (init.headers as Record<string, string>)[name];

const ok: Handler = () => json(200, envelope({ result: { ok: true } }));

// ---------------------------------------------------------------------------
// Credentials and tokens
// ---------------------------------------------------------------------------

describe("credentials", () => {
  it("exchanges the API key from the environment for a bearer token", async () => {
    process.env.GRAPHSOLVE_API_KEY = KEY;
    process.env.GRAPHSOLVE_TOKEN_URL = TOKEN_URL;
    let auth: string | undefined;

    const { handler, exchanges } = withToken((_url, init) => {
      auth = headerOf(init, "authorization");
      return json(200, envelope({ result: { ok: true } }));
    });
    const gs = new GraphSolve({
      baseUrl: "https://api.test",
      fetch: async (url, init) => handler(url, init ?? {}),
    });
    await gs.call("convert_units", {});

    expect(exchanges).toEqual([
      { url: TOKEN_URL, authorization: `Bearer ${KEY}`, grantType: "client_credentials" },
    ]);
    expect(auth).toBe("Bearer tok-1");
  });

  it("refuses to construct without an API key", () => {
    // A missing key is a programming error, not a runtime surprise.
    expect(() => new GraphSolve({ baseUrl: "https://api.test" })).toThrow(
      /GRAPHSOLVE_API_KEY/,
    );
  });

  it("does not derive the token URL from the base URL", async () => {
    // Tokens are issued by the platform for every deployment, not by the engine.
    const { handler, exchanges } = withToken(ok);
    const gs = new GraphSolve({
      apiKey: KEY,
      baseUrl: "https://api.test",
      fetch: async (url, init) => handler(url, init ?? {}),
    });
    await gs.call("convert_units", {});
    expect(exchanges[0]?.url).toBe(DEFAULT_TOKEN_URL);
  });
});

describe("tokens", () => {
  it("exchanges once and reuses the token", async () => {
    const { handler, exchanges } = withToken(ok);
    const gs = client(handler);
    await gs.call("convert_units", {});
    await gs.call("convert_units", {});
    expect(exchanges).toHaveLength(1);
  });

  it("refreshes before the token expires", () => {
    // Asserted on the pure function: a token is never issued already stale, so
    // there is nothing to observe from the outside.
    expect(refreshMarginMs(3_600_000)).toBe(300_000);
    for (const expiresInMs of [1_000, 30_000, 200_000, 600_000, 3_600_000, 86_400_000]) {
      const margin = refreshMarginMs(expiresInMs);
      expect(margin).toBeGreaterThan(0);
      expect(margin).toBeLessThan(expiresInMs);
    }
  });

  it("does not exchange on every call for a key near its expiry", async () => {
    // The platform caps a token's life at the key's own remaining life, so a
    // key in its last minutes mints tokens shorter than the flat 300 s margin.
    // Comparing against that flat value made every cached token stale on
    // arrival, and the token endpoint is rate limited.
    const { handler, exchanges } = withToken(ok, { expiresIn: 200 });
    const gs = client(handler);
    for (let i = 0; i < 20; i++) {
      await gs.call("convert_units", {});
    }
    expect(exchanges).toHaveLength(1);
  });

  it("refuses an already-expired token rather than caching it", async () => {
    const gs = client((url) =>
      new URL(url).pathname === "/oauth/token"
        ? json(200, tokenBody("tok-1", 0))
        : ok(url, {}),
    );

    await expect(gs.call("convert_units", {})).rejects.toMatchObject({
      name: "AuthenticationError",
      message: expect.stringContaining("already expired"),
    });
  });

  it("shares one exchange between concurrent callers", async () => {
    // Caching only the result would let a burst of first calls each start
    // their own exchange.
    const { handler, exchanges } = withToken(ok);
    const gs = client(handler);
    await Promise.all(Array.from({ length: 8 }, () => gs.call("convert_units", {})));
    expect(exchanges).toHaveLength(1);
  });

  it("does not retry a rejected API key", async () => {
    let attempts = 0;
    const gs = client(() => {
      attempts++;
      return json(401, { error_description: "unknown key" });
    });

    await expect(gs.call("convert_units", {})).rejects.toThrow(/rejected/);
    expect(attempts).toBe(1);
  });

  it("retries an unavailable auth service, honouring Retry-After", async () => {
    let attempts = 0;
    const gs = client(
      () => {
        attempts++;
        return json(503, {}, { "retry-after": "2" });
      },
      { maxRetries: 2 },
    );

    await expect(gs.call("convert_units", {})).rejects.toThrow(/unavailable/);
    expect(attempts).toBe(3);
    expect(slept).toEqual([2000, 2000]);
  });

  it("never puts the API key in an error", async () => {
    const gs = client(() => json(403, { error: "forbidden" }));
    const error = await gs.call("convert_units", {}).catch((e: unknown) => e);

    expect(error).toBeInstanceOf(AuthenticationError);
    expect(String(error)).not.toContain(KEY);
    expect((error as Error).stack ?? "").not.toContain(KEY);
    // A console.log of the client must not leak it either.
    expect(JSON.stringify(gs)).not.toContain(KEY);
  });

  it("reports identity and scopes from me()", async () => {
    let path = "";
    const gs = authed((url) => {
      path = new URL(url).pathname;
      return json(200, {
        user_id: "u_1",
        tenant_id: "acme",
        org_id: null,
        key_id: "key_1",
        scopes: ["engine:read", "engine:solve"],
        billable: true,
      });
    });

    const me = await gs.me();
    expect(path).toBe("/v1/me");
    expect(me.scopes).toContain("engine:solve");
  });
});

// ---------------------------------------------------------------------------
// Retries
// ---------------------------------------------------------------------------

describe("retries", () => {
  it("carries an idempotency key on every call", async () => {
    // A retry after a network timeout must not be charged twice.
    const keys: (string | undefined)[] = [];
    const gs = authed((_url, init) => {
      keys.push(headerOf(init, "idempotency-key"));
      return json(200, envelope({ result: { ok: true } }));
    });

    await gs.call("convert_units", {});
    await gs.call("convert_units", {});
    expect(keys.every(Boolean)).toBe(true);
    expect(keys[0]).not.toBe(keys[1]);
  });

  it("never retries running out of credits", async () => {
    // Unlike a rate limit, waiting does not help.
    let calls = 0;
    const gs = authed(() => {
      calls++;
      return json(402, { status: "error", errors: ["Insufficient credits"] });
    }, { maxRetries: 3 });

    await expect(gs.call("solve_network", {})).rejects.toBeInstanceOf(InsufficientCredits);
    expect(calls).toBe(1);
  });

  it("never retries a bad payload", async () => {
    let calls = 0;
    const gs = authed(() => {
      calls++;
      return json(400, { status: "error", errors: ["bad field"] });
    }, { maxRetries: 3 });

    await expect(gs.call("solve_network", {})).rejects.toBeInstanceOf(InvalidPayload);
    expect(calls).toBe(1);
  });

  it("retries a rate limit and honours Retry-After", async () => {
    let calls = 0;
    const gs = authed(() => {
      calls++;
      if (calls === 1) {
        return json(429, { status: "error", errors: ["slow down"] }, { "retry-after": "7" });
      }
      return json(200, envelope({ result: { ok: true } }));
    });

    const response = await gs.call("solve_network", {});
    expect(response.status).toBe("success");
    expect(calls).toBe(2);
    expect(slept).toEqual([7000]);
  });

  it("gives up on a rate limit carrying the wait it was told", async () => {
    const gs = authed(
      () => json(429, { status: "error", errors: ["slow down"] }, { "retry-after": "30" }),
      { maxRetries: 2 },
    );

    await expect(gs.call("solve_network", {})).rejects.toMatchObject({
      name: "RateLimited",
      retryAfter: 30,
    });
    await expect(gs.call("solve_network", {})).rejects.toBeInstanceOf(RateLimited);
  });

  it("never retries a gateway failure on a billed call", async () => {
    // The load balancer cannot say whether the engine already ran the tool, and
    // the engine does not read Idempotency-Key, so a second POST is a second
    // charge.
    let calls = 0;
    const gs = authed(() => {
      calls++;
      return json(504, { errors: ["gateway timeout"] });
    }, { maxRetries: 3 });

    const failure = gs.call("solve_network", {});
    await expect(failure).rejects.toBeInstanceOf(OutcomeUnknown);
    await expect(failure).rejects.toMatchObject({
      message: expect.stringContaining("was charged"),
    });
    expect(calls).toBe(1);
  });

  it("retries a gateway failure on a read", async () => {
    let calls = 0;
    const gs = authed(() => {
      calls++;
      return calls === 1
        ? json(502, {})
        : json(200, { user_id: "u", scopes: ["engine:read"] });
    }, { maxRetries: 3 });

    await expect(gs.me()).resolves.toMatchObject({ user_id: "u" });
    expect(calls).toBe(2);
  });

  it("never retries a transport failure on a billed call", async () => {
    // A POST that got no answer may still have been received and run.
    let calls = 0;
    const gs = authed(() => {
      calls++;
      throw new TypeError("network error");
    }, { maxRetries: 3 });

    await expect(gs.call("solve_network", {})).rejects.toBeInstanceOf(OutcomeUnknown);
    expect(calls).toBe(1);
  });
});

// ---------------------------------------------------------------------------
// Charged versus free
// ---------------------------------------------------------------------------

describe("a failed calculation is not a rejected request", () => {
  it("is a 200, is thrown, and is charged", async () => {
    // A non-converged solve consumed compute and IS charged; a bad payload is
    // a 400 and is free. The client must not collapse the two.
    const gs = authed(() =>
      json(200, envelope({ status: "error", failure_kind: "computation_failed", charged: 25 })),
    );

    await expect(gs.call("solve_network", {})).rejects.toMatchObject({
      name: "SolverDidNotConverge",
      creditsCharged: 25,
    });
  });

  it("keeps the envelope so the failure kind is readable", async () => {
    const gs = authed(() =>
      json(200, envelope({ status: "error", failure_kind: "computation_failed", charged: 25 })),
    );

    await gs.call("solve_network", {}).then(
      () => expect.unreachable("should have thrown"),
      (error: SolverDidNotConverge) => {
        expect(error.envelope.failure_kind).toBe("computation_failed");
      },
    );
  });
});

// ---------------------------------------------------------------------------
// Long-running work
// ---------------------------------------------------------------------------

describe("jobs", () => {
  it("quotes without spending anything", async () => {
    let seen = "";
    const gs = authed((url) => {
      seen = url;
      return json(200, {
        quote: { tool: "run_transient_wave", estimated_credits: 41200, basis: "cells x steps" },
      });
    });

    const quote = await gs.quote("run_transient_wave", {});
    expect(seen).toContain("dry_run=true");
    expect(quote.quote.estimated_credits).toBe(41200);
  });

  it("run() resolves only once the job finishes", async () => {
    const states = ["queued", "running", "succeeded"];
    let poll = 0;
    const gs = authed((_url, init) => {
      if (init.method === "POST") {
        return json(202, { job: { id: "job_1", state: "queued" } });
      }
      return json(200, {
        job: {
          id: "job_1",
          state: states[poll++],
          result: envelope({ result: { pressure: 1.0 } }),
          charged_credits: 3,
        },
      });
    });

    const result = await gs.run("run_transient_wave", {});
    expect((result.result as { pressure: number }).pressure).toBe(1.0);
    expect(poll).toBe(3);
  });

  it("cancels the job rather than abandoning a charge that keeps running", async () => {
    const cancelled: string[] = [];
    const gs = authed((url, init) => {
      if (url.endsWith("/cancel")) {
        cancelled.push(url);
        return json(200, { job_id: "job_1", state: "cancelled", note: "may run to completion" });
      }
      if (init.method === "POST") {
        return json(202, { job: { id: "job_1", state: "queued" } });
      }
      return json(200, { job: { id: "job_1", state: "running" } });
    });

    await expect(
      gs.run("run_transient_wave", {}, { pollIntervalMs: 1, maxWaitMs: -1 }),
    ).rejects.toThrow(/cancelled/);
    expect(cancelled).toHaveLength(1);
  });
});

// ---------------------------------------------------------------------------
// The generated surface
// ---------------------------------------------------------------------------

describe("the generated surface", () => {
  it("names every tool the spec advertises", () => {
    // Every tool in the spec must have a generated method.
    const specPath = fileURLToPath(new URL("../../spec/graphsolve-v1.json", import.meta.url));
    const spec = JSON.parse(readFileSync(specPath, "utf8")) as {
      paths: Record<string, { post?: { operationId: string } }>;
    };
    const tools = Object.entries(spec.paths)
      .filter(([path, item]) => path.startsWith("/v1/tools/") && item.post)
      .map(([, item]) => item.post!.operationId);

    const methods = GeneratedMethods.prototype as unknown as Record<string, unknown>;
    const missing = tools.filter((tool) => typeof methods[tool] !== "function");
    expect(missing).toEqual([]);
    expect(tools).toHaveLength(98);
  });

  it("forwards to the named tool", async () => {
    let path = "";
    let body: Record<string, unknown> = {};
    const gs = authed((url, init) => {
      path = new URL(url).pathname;
      body = bodyOf(init);
      return json(200, envelope({ result: { outlet_pressure: 10.0 } }));
    });

    await gs.calculate_compressor({
      inlet_pressure: 5.0,
      inlet_temperature: 350.0,
      pressure_ratio: 2.0,
      fluid: { gas_rate: 500000, gas_mw: 18.5 },
    });
    expect(path).toBe("/v1/tools/calculate_compressor");
    expect(body.inlet_pressure).toBe(5.0);
  });

  it("leaves an omitted optional out of the payload rather than sending null", async () => {
    // An unset optional must be absent from the payload, not sent as null.
    // Several tools treat "not given" and null differently.
    let body: Record<string, unknown> = {};
    const gs = authed((_url, init) => {
      body = bodyOf(init);
      return json(200, envelope({ result: {} }));
    });

    await gs.calculate_compressor({
      inlet_pressure: 5.0,
      inlet_temperature: 350.0,
      pressure_ratio: 2.0,
      fluid: {},
      mechanical_efficiency: undefined,
    });
    expect(body).not.toHaveProperty("mechanical_efficiency");
    expect(body).not.toHaveProperty("polytropic_efficiency");
  });

  it("takes an object for a blob parameter", async () => {
    // The *_json parameters are anyOf[object, array, string].
    let body: Record<string, unknown> = {};
    const gs = authed((_url, init) => {
      body = bodyOf(init);
      return json(200, envelope({ result: {} }));
    });

    await gs.solve_network({ network_json: { graph_data: { nodes: [] } } });
    expect(typeof body.network_json).toBe("object");
  });
});
