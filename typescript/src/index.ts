/**
 * Client for the GraphSolve engine API.
 *
 * ```ts
 * import { GraphSolve } from "@graphsolve/sdk";
 *
 * const gs = new GraphSolve();      // reads GRAPHSOLVE_API_KEY
 * const result = await gs.solve_network({ network_json: model });
 * ```
 *
 * Units are SI throughout: pressure MPa, temperature K, rates kSm3/day, lengths
 * m. Permeability in millidarcy is the single exception.
 */

export {
  GraphSolve,
  DEFAULT_BASE_URL,
  DEFAULT_TIMEOUT_MS,
  DEFAULT_TOKEN_URL,
} from "./client.js";
export type { FetchLike, GraphSolveOptions, RunOptions } from "./client.js";
export { MAX_REFRESH_MARGIN_MS, refreshMarginMs } from "./auth.js";
export {
  AuthenticationError,
  GraphSolveError,
  InsufficientCredits,
  OutcomeUnknown,
  InvalidPayload,
  RateLimited,
  ServerError,
  SolverDidNotConverge,
  ToolNotPermitted,
} from "./errors.js";
export * from "./types.js";
export * from "./generated.js";
