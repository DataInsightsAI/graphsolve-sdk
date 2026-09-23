"""The GraphSolve client.

Hand-written rather than generated. The behaviour here — which status codes are
retried, which failures are charged, and the idempotency key — is the same in
every client in this repo and is documented in CONTRIBUTING.md.
"""

from __future__ import annotations

import os
import random
import time
import uuid
from typing import Any

import httpx

from ._auth import BearerAuth, TokenProvider
from ._generated import GeneratedMethods
from .errors import (
    AuthenticationError,
    GraphSolveError,
    InsufficientCredits,
    InvalidPayload,
    OutcomeUnknown,
    RateLimited,
    ServerError,
    SolverDidNotConverge,
    ToolNotPermitted,
)

DEFAULT_BASE_URL = "https://api-prod.graphsolve.ai"

#: The platform endpoint that issues access tokens. Tokens are issued centrally
#: for every deployment, so this is not derived from the base URL.
DEFAULT_TOKEN_URL = "https://prod-user-api.graphsolve.ai/oauth/token"

#: Solves can take minutes. The server's load balancer allows 900 s; a shorter
#: client timeout would abandon work that is still running and still charged.
DEFAULT_TIMEOUT = 900.0

#: Retried whatever the method: neither status reached the engine, so repeating
#: the request cannot duplicate a charge. Any other 4xx fails the same way
#: every time.
RETRYABLE_STATUS = frozenset({429, 503})

#: Retried on a GET only. A 502 or 504 comes from the load balancer, which
#: cannot say whether the engine already ran the tool — and a tool that ran is
#: metered whether or not its response arrived. The engine does not read
#: `Idempotency-Key`, so repeating a POST would be charged twice.
RETRYABLE_IF_REPEATABLE = frozenset({502, 504})


#: Said alongside OutcomeUnknown. The type is what a caller branches on; this
#: is what a human reads in a log.
_MAY_HAVE_RUN = (
    "This request was not retried: it may have reached the engine and run, "
    "in which case it was charged."
)


def _outcome_unknown(message: str, envelope: dict[str, Any] | None = None) -> OutcomeUnknown:
    """The error for a call abandoned without knowing whether it ran."""
    text = f"{message.rstrip().rstrip('.')}. {_MAY_HAVE_RUN}"
    return OutcomeUnknown(text, envelope=envelope)


def _retryable(status: int, method: str) -> bool:
    """Whether `status` may be retried for a request using `method`."""
    if status in RETRYABLE_STATUS:
        return True
    return status in RETRYABLE_IF_REPEATABLE and method == "GET"


class GraphSolve(GeneratedMethods):
    """A client for the GraphSolve engine API.

    >>> gs = GraphSolve()          # reads GRAPHSOLVE_API_KEY
    >>> gs.convert_units(value=1.0, from_unit="MPa", to_unit="psi")

    One typed method per tool, generated from the API spec, so an editor offers
    the valid correlation names rather than accepting any string. Use `call()`
    for a tool newer than the installed version.

    The API key is exchanged for an access token on the first call and the token
    is refreshed before it expires. Safe to share across threads.
    """

    def __init__(
        self,
        api_key: str | None = None,
        *,
        base_url: str | None = None,
        token_url: str | None = None,
        timeout: float = DEFAULT_TIMEOUT,
        max_retries: int = 3,
        client: httpx.Client | None = None,
    ) -> None:
        key = api_key or os.environ.get("GRAPHSOLVE_API_KEY")
        if not key:
            raise AuthenticationError(
                "No API key. Pass api_key=... or set GRAPHSOLVE_API_KEY."
            )
        base = base_url or os.environ.get("GRAPHSOLVE_BASE_URL") or DEFAULT_BASE_URL
        token = token_url or os.environ.get("GRAPHSOLVE_TOKEN_URL") or DEFAULT_TOKEN_URL

        self._max_retries = max_retries
        self._http = client or httpx.Client(
            base_url=base.rstrip("/"),
            timeout=timeout,
            headers={"user-agent": "graphsolve-python"},
        )
        self._tokens = TokenProvider(key, token, http=self._http, max_retries=max_retries)
        self._auth = BearerAuth(self._tokens)

    def get_access_token(self) -> str:
        """The current access token, for a caller wiring their own HTTP."""
        return self._tokens.token()

    # ---------------------------------------------------------------- tools

    def call(self, tool: str, arguments: dict[str, Any] | None = None) -> dict[str, Any]:
        """Run a tool and return its response envelope.

        Every generated method calls this. Use it directly for a tool newer
        than this client.
        """
        envelope = self._request("POST", f"/v1/tools/{tool}", json=arguments or {})
        self._raise_for_envelope(envelope, tool)
        return envelope

    def tools(self) -> list[dict[str, Any]]:
        """Every tool, with its domain, summary and price."""
        return self._request("GET", "/v1/tools")["tools"]

    def describe(self, tool: str) -> dict[str, Any]:
        """One tool's full definition: schema, examples, price."""
        return self._request("GET", f"/v1/tools/{tool}")

    def me(self) -> dict[str, Any]:
        """The identity and scopes the API key's token carries.

        Balances are held by the platform, not the engine, so they are not
        reported here.
        """
        return self._request("GET", "/v1/me")

    # ----------------------------------------------------------------- jobs

    def quote(self, tool: str, arguments: dict[str, Any]) -> dict[str, Any]:
        """Price a long-running call without running it.

        Spends nothing: no compute, no job. Worth calling before a transient
        run, where one field can change the cost by orders of magnitude.
        """
        return self._request(
            "POST", f"/v1/jobs/{tool}", json=arguments, params={"dry_run": "true"}
        )

    def submit(self, tool: str, arguments: dict[str, Any]) -> dict[str, Any]:
        """Start a job. Returns immediately with the job and its quote."""
        return self._request("POST", f"/v1/jobs/{tool}", json=arguments)

    def job(self, job_id: str) -> dict[str, Any]:
        """Poll a job."""
        return self._request("GET", f"/v1/jobs/id/{job_id}")["job"]

    def cancel(self, job_id: str) -> dict[str, Any]:
        """Cancel a job.

        Compute already under way may run to completion and is charged.
        """
        return self._request("POST", f"/v1/jobs/id/{job_id}/cancel")

    def run(
        self,
        tool: str,
        arguments: dict[str, Any],
        *,
        poll_interval: float = 5.0,
        max_wait: float | None = None,
    ) -> dict[str, Any]:
        """Submit a job and block until it finishes.

        Cancels the job if `max_wait` is exceeded, so giving up on the wait
        also stops the charge.
        """
        job = self.submit(tool, arguments)["job"]
        job_id, started = job["id"], time.monotonic()

        while True:
            job = self.job(job_id)
            if job["state"] in ("succeeded", "failed", "cancelled"):
                break
            if max_wait is not None and time.monotonic() - started > max_wait:
                self.cancel(job_id)
                raise GraphSolveError(
                    f"Job {job_id} exceeded max_wait of {max_wait}s and was cancelled."
                )
            time.sleep(poll_interval)

        if job["state"] != "succeeded":
            raise SolverDidNotConverge(
                f"Job {job_id} finished as {job['state']}: {job.get('error') or 'no result'}",
                credits_charged=job.get("charged_credits", 0),
                envelope=job.get("result") or {},
            )
        return job["result"]

    # ------------------------------------------------------------- internal

    def _request(
        self,
        method: str,
        path: str,
        *,
        json: Any = None,
        params: dict[str, str] | None = None,
    ) -> dict[str, Any]:
        # One key per logical call, reused across its retries. The engine does
        # not read it yet; send it so that a server that does needs no change
        # here. Until then, not retrying a POST is what stops a double charge.
        headers = {"idempotency-key": str(uuid.uuid4())}
        repeatable = method == "GET"

        last_error: Exception | None = None
        for attempt in range(self._max_retries + 1):
            try:
                response = self._http.request(
                    method,
                    path,
                    json=json,
                    params=params,
                    headers=headers,
                    auth=self._auth,
                )
            except httpx.TransportError as exc:
                last_error = exc
                # A POST that got no answer may still have been received and
                # run, so it is reported rather than repeated.
                message = f"Could not reach GraphSolve: {exc}"
                if not repeatable:
                    raise _outcome_unknown(message) from exc
                if attempt == self._max_retries:
                    raise GraphSolveError(message) from exc
                time.sleep(self._backoff(attempt))
                continue

            if _retryable(response.status_code, method) and attempt < self._max_retries:
                time.sleep(self._backoff(attempt, response.headers.get("retry-after")))
                continue

            return self._decode(response, repeatable=repeatable)

        raise GraphSolveError(f"Request failed after retries: {last_error}")

    @staticmethod
    def _backoff(attempt: int, retry_after: str | None = None) -> float:
        """Exponential backoff with jitter, deferring to Retry-After."""
        if retry_after:
            try:
                return float(retry_after)
            except ValueError:
                pass
        # Jitter, so clients throttled at the same time do not all retry at
        # the same time.
        return (2**attempt) * (0.5 + random.random())

    def _decode(
        self, response: httpx.Response, *, repeatable: bool = True
    ) -> dict[str, Any]:
        try:
            body = response.json()
        except ValueError:
            body = {}
        errors = body.get("errors") or [response.text or response.reason_phrase]
        message = errors[0] if errors else "Unknown error"
        status = response.status_code

        if status < 400:
            return body
        if status == 400:
            raise InvalidPayload(message, envelope=body)
        if status == 401:
            raise AuthenticationError(message, envelope=body)
        if status == 402:
            raise InsufficientCredits(message, envelope=body)
        if status == 403:
            raise ToolNotPermitted(message, envelope=body)
        if status == 429:
            retry_after = response.headers.get("retry-after")
            raise RateLimited(
                message,
                retry_after=float(retry_after) if retry_after else None,
                envelope=body,
            )
        if status >= 500:
            # 502 and 504 are the load balancer's, not the engine's: the tool
            # may have run and been metered. Only a POST reaches here unretried.
            if status in RETRYABLE_IF_REPEATABLE and not repeatable:
                raise _outcome_unknown(message, body)
            raise ServerError(message, envelope=body)
        raise GraphSolveError(f"HTTP {status}: {message}", envelope=body)

    @staticmethod
    def _raise_for_envelope(envelope: dict[str, Any], tool: str) -> None:
        """Raise if a 200 response reports a failed calculation.

        The call ran and is charged, but there is no `result` to unpack, so
        raise here rather than let the caller fail further downstream.
        """
        if envelope.get("status") == "success":
            return
        billing = (envelope.get("metadata") or {}).get("billing") or {}
        errors = envelope.get("errors") or ["no result"]
        raise SolverDidNotConverge(
            f"{tool}: {errors[0]}",
            credits_charged=billing.get("credits_charged", 0),
            envelope=envelope,
        )
