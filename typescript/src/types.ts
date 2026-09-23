/**
 * The wire types: what the API sends back.
 *
 * Field names are snake_case to match the wire format. Names the client defines
 * itself (baseUrl, maxRetries, creditsCharged) are camelCase.
 */

/** Arguments for a tool call. */
export type ToolArguments = Record<string, unknown>;

/**
 * Why a call failed.
 *
 * `invalid_input` is the caller's to fix and is not charged; `computation_failed`
 * means the calculation ran and produced no answer, which consumed compute and
 * IS charged; `internal` is ours and is not charged.
 */
export type FailureKind = "invalid_input" | "computation_failed" | "internal";

/** What a call cost. Balances are held by the platform, not reported here. */
export interface Billing {
  tool: string;
  credits_charged: number;
}

export interface ToolMetadata {
  calculation_time_ms: number;
  converged?: boolean | null;
  iterations?: number | null;
  billing?: Billing;
}

/**
 * The standard response envelope.
 *
 * `result` is the tool's own payload and differs per tool, so it is `unknown`
 * unless the caller narrows it.
 */
export interface ToolResponse<T = unknown> {
  status: "success" | "error";
  result?: T;
  warnings: string[];
  errors: string[];
  failure_kind?: FailureKind | null;
  metadata?: ToolMetadata | null;
}

/** One entry from `tools()`. */
export interface ToolSummary {
  name: string;
  domain: string;
  summary: string;
  cost: { base_credits: number; free_ms: number; ms_per_credit: number };
}

/** The identity and scopes an access token carries, from `me()`. */
export interface Me {
  user_id: string;
  tenant_id: string;
  org_id: string | null;
  key_id: string | null;
  /** `null` for the platform's own unscoped traffic. */
  scopes: string[] | null;
  billable: boolean;
}

export type JobState = "queued" | "running" | "succeeded" | "failed" | "cancelled";

export interface Job {
  id: string;
  user_id: string;
  tenant_id: string;
  tool: string;
  state: JobState;
  /** The quote the job was submitted against. */
  estimated_credits: number;
  /** What the run actually cost. Zero until it finishes. */
  charged_credits: number;
  submitted_at: number;
  finished_at?: number | null;
  result?: ToolResponse | null;
  error?: string | null;
}

/** What a job would cost, and why. */
export interface Quote {
  tool: string;
  estimated_credits: number;
  /** How the number was arrived at, so a surprising quote can be argued with. */
  basis: string;
  estimated_runtime_s?: number;
  details?: unknown;
}

export interface QuoteResponse {
  quote: Quote;
}

export interface SubmitResponse {
  job: Job;
  quote: Quote;
}

export interface CancelResponse {
  job_id: string;
  state: JobState;
  note?: string;
}
