/**
 * Errors thrown by the client.
 *
 * One class per failure mode the API distinguishes, because the right response
 * differs in each case, and so does whether the call was charged.
 */

import type { ToolResponse } from "./types.js";

export class GraphSolveError extends Error {
  /** The response envelope, when there was one. */
  readonly envelope: Partial<ToolResponse>;

  constructor(message: string, envelope: Partial<ToolResponse> = {}) {
    super(message);
    this.name = new.target.name;
    this.envelope = envelope;
  }

  get warnings(): string[] {
    return this.envelope.warnings ?? [];
  }
}

/**
 * 401 — the access token was missing, invalid or expired, or the API key was
 * rejected at the token endpoint.
 */
export class AuthenticationError extends GraphSolveError {}

/**
 * 403 — the token lacks the scope this tool needs: `engine:read` for free
 * lookups, `engine:solve` for anything that runs a calculation.
 */
export class ToolNotPermitted extends GraphSolveError {}

/**
 * 402 — nothing left to spend.
 *
 * Not retried: unlike a rate limit, waiting does not help.
 */
export class InsufficientCredits extends GraphSolveError {}

/** 429 — too many requests, or the compute pool is saturated. */
export class RateLimited extends GraphSolveError {
  /** Seconds to wait, from `Retry-After`, when the server said. */
  readonly retryAfter?: number;

  constructor(
    message: string,
    retryAfter?: number,
    envelope: Partial<ToolResponse> = {},
  ) {
    super(message, envelope);
    this.retryAfter = retryAfter;
  }
}

/** 400 — the request was wrong. Nothing was computed and nothing was charged. */
export class InvalidPayload extends GraphSolveError {}

/**
 * The calculation ran and produced no answer.
 *
 * Not a rejected request. It arrives as a 200 and IS charged, because it
 * consumed compute. Thrown rather than returned because the envelope has no
 * `result` to unpack.
 *
 * `creditsCharged` is on the error so the cost is visible here.
 */
export class SolverDidNotConverge extends GraphSolveError {
  readonly creditsCharged: number;

  constructor(
    message: string,
    creditsCharged = 0,
    envelope: Partial<ToolResponse> = {},
  ) {
    super(message, envelope);
    this.creditsCharged = creditsCharged;
  }
}

/**
 * The call was abandoned without knowing whether it ran.
 *
 * A 502, a 504 or a dropped connection on a request that is not safe to
 * repeat. The engine may already have run the tool, and a tool that ran is
 * charged, so the client does not send it again. Reconcile before repeating it
 * by hand.
 */
export class OutcomeUnknown extends GraphSolveError {}

/**
 * 5xx — our fault. Not charged.
 *
 * A 502 or 504 on a request that cannot be repeated is `OutcomeUnknown`
 * instead: there, whether it was charged is exactly what is not known.
 */
export class ServerError extends GraphSolveError {}
