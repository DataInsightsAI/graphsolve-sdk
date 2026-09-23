/**
 * API key exchange and token caching.
 *
 * The client holds an API key and trades it at the platform's token endpoint
 * for a short-lived access token. The token is refreshed before it expires
 * rather than after a 401, which keeps authentication off the retry path
 * entirely.
 */

import { AuthenticationError } from "./errors.js";
import type { FetchLike } from "./client.js";

/**
 * The most of a token's life to give up to the refresh margin. Wide enough
 * that a request started just before the check cannot arrive after expiry.
 */
export const MAX_REFRESH_MARGIN_MS = 300_000;

/**
 * How long before expiry to exchange a token that lives `expiresInMs`.
 *
 * Capped at half the token's life, because the platform caps a token's
 * lifetime at the API key's own remaining life: a key in its last five minutes
 * mints tokens shorter than the flat margin, which would make every cached
 * token stale on arrival and every call exchange again.
 */
export function refreshMarginMs(expiresInMs: number): number {
  return Math.min(MAX_REFRESH_MARGIN_MS, expiresInMs / 2);
}

/** The exchange is a small POST, not a solve, so it gets its own timeout. */
const TOKEN_TIMEOUT_MS = 10_000;

/**
 * Wider than the client's own set: the exchange runs no tool and is charged
 * nothing, so repeating it after a 502 or 504 cannot cost anything.
 */
const RETRYABLE_STATUS = new Set([429, 502, 503, 504]);

export interface TokenProviderOptions {
  apiKey: string;
  tokenUrl: string;
  fetch: FetchLike;
  maxRetries: number;
  sleep: (ms: number) => Promise<void>;
}

/**
 * Fetches, caches and refreshes the access token.
 *
 * Concurrent callers share one exchange by awaiting the same promise. Caching
 * only the result would let a burst of first calls each start their own.
 *
 * The fields use `#` rather than TypeScript's `private`, which is erased at
 * compile time: a `private` field still shows up in `JSON.stringify`, a
 * `console.log` and any error reporter that serialises the client, and one of
 * them holds the API key.
 */
export class TokenProvider {
  readonly #options: TokenProviderOptions;
  #token: string | null = null;
  #refreshAt = 0;
  #inFlight: Promise<string> | null = null;

  constructor(options: TokenProviderOptions) {
    this.#options = options;
  }

  /** The current access token, exchanging for a new one if needed. */
  async get(): Promise<string> {
    const cached = this.#cached();
    if (cached !== null) {
      return cached;
    }
    this.#inFlight ??= this.#exchange().finally(() => {
      this.#inFlight = null;
    });
    return this.#inFlight;
  }

  #cached(): string | null {
    if (this.#token !== null && Date.now() < this.#refreshAt) {
      return this.#token;
    }
    return null;
  }

  async #exchange(): Promise<string> {
    const body = new URLSearchParams({ grant_type: "client_credentials" }).toString();

    let lastError: unknown;
    for (let attempt = 0; attempt <= this.#options.maxRetries; attempt++) {
      let response: Response;
      try {
        response = await this.#options.fetch(this.#options.tokenUrl, {
          method: "POST",
          headers: {
            authorization: `Bearer ${this.#options.apiKey}`,
            "content-type": "application/x-www-form-urlencoded",
          },
          body,
          signal: AbortSignal.timeout(TOKEN_TIMEOUT_MS),
        });
      } catch (error) {
        lastError = error;
        if (attempt === this.#options.maxRetries) {
          throw new AuthenticationError(
            `Could not reach the authentication service: ${String(error)}`,
          );
        }
        await this.#options.sleep(backoffMs(attempt));
        continue;
      }

      if (RETRYABLE_STATUS.has(response.status) && attempt < this.#options.maxRetries) {
        await this.#options.sleep(backoffMs(attempt, response.headers.get("retry-after")));
        continue;
      }

      return this.#store(response);
    }

    throw new AuthenticationError(`Token exchange failed after retries: ${String(lastError)}`);
  }

  async #store(response: Response): Promise<string> {
    let body: Record<string, unknown> = {};
    try {
      body = (await response.json()) as Record<string, unknown>;
    } catch {
      // A gateway can answer with HTML, or nothing at all.
    }

    if (response.status >= 400) {
      throw exchangeError(response.status, body);
    }

    const token = body.access_token;
    const expiresIn = body.expires_in;
    if (typeof token !== "string" || typeof expiresIn !== "number") {
      throw new AuthenticationError(
        "The authentication service returned a token response we could not read.",
      );
    }

    // Refusing here beats caching a token that is stale on arrival, which
    // would mean one exchange per call for as long as the key lasts.
    if (expiresIn <= 0) {
      throw new AuthenticationError(
        "The authentication service issued a token that has already expired. " +
          "This API key is at or past its expiry; issue a new one.",
      );
    }

    const expiresInMs = expiresIn * 1000;
    this.#token = token;
    this.#refreshAt = Date.now() + expiresInMs - refreshMarginMs(expiresInMs);
    return token;
  }
}

/** Separate "the service is down" from "your API key is wrong". */
function exchangeError(status: number, body: Record<string, unknown>): AuthenticationError {
  if (status >= 500) {
    return new AuthenticationError(
      `The authentication service is unavailable (HTTP ${status}).`,
    );
  }
  const detail = body.error_description ?? body.error;
  const suffix = typeof detail === "string" && detail ? `: ${detail}` : ".";
  return new AuthenticationError(`This API key was rejected${suffix}`);
}

function backoffMs(attempt: number, retryAfter?: string | null): number {
  if (retryAfter) {
    const seconds = Number(retryAfter);
    if (Number.isFinite(seconds)) {
      return seconds * 1000;
    }
  }
  return 2 ** attempt * (0.5 + Math.random()) * 1000;
}
