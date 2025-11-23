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
    file_params: dict[str, Any] | None = None
    is_async: bool = False
    body_param_name: str | None = None  # Name of the body parameter (default: "body")
    handler_dependencies: list[str] | None = None  # List of dependency keys for DI
