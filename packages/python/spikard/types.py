"""Type definitions for Spikard."""

from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from collections.abc import Callable

    from spikard.jsonrpc import JsonRpcMethodInfo


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
    body_param_name: str | None = None
    handler_dependencies: list[str] | None = None
    jsonrpc_method: JsonRpcMethodInfo | None = None
