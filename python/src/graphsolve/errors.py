"""Errors raised by the client.

One type per failure mode the API distinguishes, because the right response
differs in each case, and so does whether the call was charged.
"""

from __future__ import annotations

from typing import Any


class GraphSolveError(Exception):
    """Base for everything this client raises."""

    def __init__(self, message: str, *, envelope: dict[str, Any] | None = None) -> None:
        super().__init__(message)
        self.envelope = envelope or {}

    @property
    def warnings(self) -> list[str]:
        return self.envelope.get("warnings", [])


class AuthenticationError(GraphSolveError):
    """401 — the access token was missing, invalid or expired, or the API key
    was rejected at the token endpoint."""


class ToolNotPermitted(GraphSolveError):
    """403 — the token lacks the scope this tool needs: `engine:read` for free
    lookups, `engine:solve` for anything that runs a calculation."""


class InsufficientCredits(GraphSolveError):
    """402 — nothing left to spend.

    Not retried: unlike a rate limit, waiting does not help.
    """


class RateLimited(GraphSolveError):
    """429 — too many requests, or the compute pool is saturated."""

    def __init__(self, message: str, *, retry_after: float | None = None, **kw: Any) -> None:
        super().__init__(message, **kw)
        self.retry_after = retry_after


class InvalidPayload(GraphSolveError):
    """400 — the request was wrong. Nothing was computed and nothing was charged."""


class SolverDidNotConverge(GraphSolveError):
    """The calculation ran and produced no answer.

    Not a rejected request. It arrives as a 200 and IS charged, because it
    consumed compute. Raised rather than returned because the envelope has no
    `result` to unpack.

    `credits_charged` is on the exception so the cost is visible here.
    """

    def __init__(self, message: str, *, credits_charged: int = 0, **kw: Any) -> None:
        super().__init__(message, **kw)
        self.credits_charged = credits_charged


class OutcomeUnknown(GraphSolveError):
    """The call was abandoned without knowing whether it ran.

    A 502, a 504 or a dropped connection on a request that is not safe to
    repeat. The engine may already have run the tool, and a tool that ran is
    charged, so the client does not send it again. Reconcile before repeating
    it by hand.
    """


class ServerError(GraphSolveError):
    """5xx — our fault. Not charged.

    A 502 or 504 on a request that cannot be repeated is `OutcomeUnknown`
    instead: there, whether it was charged is exactly what is not known.
    """
