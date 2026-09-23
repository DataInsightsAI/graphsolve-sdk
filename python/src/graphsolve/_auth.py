"""API key exchange and token caching.

The client holds an API key and trades it at the platform's token endpoint for a
short-lived access token. The token is refreshed before it expires rather than
after a 401, which keeps authentication off the retry path entirely.
"""

from __future__ import annotations

import random
import threading
import time
from typing import Any

import httpx

from .errors import AuthenticationError, GraphSolveError

#: The most of a token's life to give up to the refresh margin. Wide enough
#: that a request started just before the check cannot arrive after expiry.
MAX_REFRESH_MARGIN = 300.0

#: The exchange is a small POST, not a solve, so it gets its own timeout rather
#: than the client's 900 s.
TOKEN_TIMEOUT = 10.0

#: Wider than the client's own set: the exchange runs no tool and is charged
#: nothing, so repeating it after a 502 or 504 cannot cost anything.
RETRYABLE_STATUS = frozenset({429, 502, 503, 504})


def refresh_margin(expires_in: float) -> float:
    """How long before expiry to exchange a token that lives `expires_in`.

    Capped at half the token's life, because the platform caps a token's
    lifetime at the API key's own remaining life: a key in its last five
    minutes mints tokens shorter than the flat margin, which would make every
    cached token stale on arrival and every call exchange again.
    """
    return min(MAX_REFRESH_MARGIN, expires_in / 2)


class TokenProvider:
    """Fetches, caches and refreshes the access token.

    Safe to share across threads: the client is synchronous and one instance is
    commonly used from several, so concurrent first calls must still produce
    exactly one exchange.
    """

    def __init__(
        self,
        api_key: str,
        token_url: str,
        *,
        http: httpx.Client,
        max_retries: int = 3,
    ) -> None:
        self._api_key = api_key
        self._token_url = token_url
        self._http = http
        self._max_retries = max_retries
        self._token: str | None = None
        self._refresh_at = 0.0
        self._lock = threading.Lock()

    def token(self) -> str:
        """The current access token, exchanging for a new one if needed."""
        cached = self._cached()
        if cached is not None:
            return cached
        with self._lock:
            # Checked again under the lock: several threads can arrive here at
            # once, and only the first should spend a round trip.
            cached = self._cached()
            if cached is not None:
                return cached
            return self._exchange()

    def _cached(self) -> str | None:
        if self._token and time.time() < self._refresh_at:
            return self._token
        return None

    def _exchange(self) -> str:
        last_error: Exception | None = None

        for attempt in range(self._max_retries + 1):
            try:
                response = self._http.post(
                    self._token_url,
                    data={"grant_type": "client_credentials"},
                    headers={"authorization": f"Bearer {self._api_key}"},
                    timeout=TOKEN_TIMEOUT,
                )
            except httpx.TransportError as exc:
                last_error = exc
                if attempt == self._max_retries:
                    raise AuthenticationError(
                        f"Could not reach the authentication service: {exc}"
                    ) from exc
                time.sleep(_backoff(attempt))
                continue

            if response.status_code in RETRYABLE_STATUS and attempt < self._max_retries:
                time.sleep(_backoff(attempt, response.headers.get("retry-after")))
                continue

            return self._store(response)

        raise AuthenticationError(f"Token exchange failed after retries: {last_error}")

    def _store(self, response: httpx.Response) -> str:
        if response.status_code >= 400:
            raise _exchange_error(response)

        try:
            body: dict[str, Any] = response.json()
            token = body["access_token"]
            expires_in = float(body["expires_in"])
        except (ValueError, KeyError, TypeError) as exc:
            raise AuthenticationError(
                "The authentication service returned a token response we could not read."
            ) from exc

        # Refusing here beats caching a token that is stale on arrival, which
        # would mean one exchange per call for as long as the key lasts.
        if expires_in <= 0:
            raise AuthenticationError(
                "The authentication service issued a token that has already expired. "
                "This API key is at or past its expiry; issue a new one."
            )

        self._token = token
        self._refresh_at = time.time() + expires_in - refresh_margin(expires_in)
        return token


class BearerAuth(httpx.Auth):
    """Attaches the token to each request as httpx sends it.

    Doing it here rather than in the caller means every attempt of a retried
    request reads the token afresh, without the retry loop knowing about auth.
    """

    def __init__(self, tokens: TokenProvider) -> None:
        self._tokens = tokens

    def auth_flow(self, request: httpx.Request) -> Any:
        request.headers["authorization"] = f"Bearer {self._tokens.token()}"
        yield request


def _exchange_error(response: httpx.Response) -> GraphSolveError:
    """Separate "the service is down" from "your API key is wrong"."""
    try:
        body: Any = response.json()
    except ValueError:
        body = {}
    detail = ""
    if isinstance(body, dict):
        detail = body.get("error_description") or body.get("error") or ""

    if response.status_code >= 500:
        return AuthenticationError(
            f"The authentication service is unavailable (HTTP {response.status_code})."
        )
    suffix = f": {detail}" if detail else "."
    return AuthenticationError(f"This API key was rejected{suffix}")


def _backoff(attempt: int, retry_after: str | None = None) -> float:
    if retry_after:
        try:
            return float(retry_after)
        except ValueError:
            pass
    return (2**attempt) * (0.5 + random.random())
