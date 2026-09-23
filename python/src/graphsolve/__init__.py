"""Client for the GraphSolve engine API.

    from graphsolve import GraphSolve

    gs = GraphSolve()          # reads GRAPHSOLVE_API_KEY
    gs.call("solve_network", {"network_json": model})

Units are SI throughout: pressure MPa, temperature K, rates kSm3/day, lengths m.
Permeability in millidarcy is the single exception.
"""

from ._client import DEFAULT_BASE_URL, DEFAULT_TOKEN_URL, GraphSolve
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

__all__ = [
    "DEFAULT_BASE_URL",
    "DEFAULT_TOKEN_URL",
    "AuthenticationError",
    "GraphSolve",
    "GraphSolveError",
    "InsufficientCredits",
    "InvalidPayload",
    "OutcomeUnknown",
    "RateLimited",
    "ServerError",
    "SolverDidNotConverge",
    "ToolNotPermitted",
]

__version__ = "1.0.0"
