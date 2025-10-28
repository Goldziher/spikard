"""Type definitions for Spikard."""

from collections.abc import Callable
from dataclasses import dataclass
from typing import Any


@dataclass
class Route:
    """Route definition."""

    method: str
    path: str
    handler: Callable[..., Any]
    handler_name: str
    request_schema: dict[str, Any] | None
    response_schema: dict[str, Any] | None
    parameter_schema: dict[str, Any] | None = None
    is_async: bool = False
