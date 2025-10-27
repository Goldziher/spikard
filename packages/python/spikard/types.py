"""Type definitions for Spikard."""

from dataclasses import dataclass
from typing import Any, Callable, Optional


@dataclass
class Route:
    """Route definition."""

    method: str
    path: str
    handler: Callable
    handler_name: str
    request_schema: Optional[dict[str, Any]]
    response_schema: Optional[dict[str, Any]]
    is_async: bool
