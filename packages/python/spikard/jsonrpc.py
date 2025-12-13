"""JSON-RPC method metadata support for Spikard."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any


@dataclass(frozen=True, slots=True)
class JsonRpcMethodInfo:
    """JSON-RPC method metadata for route registration.

    This dataclass allows you to attach JSON-RPC method metadata to route handlers
    for automatic documentation and method registry integration. It follows the
    JSON-RPC 2.0 specification and supports OpenAPI/OpenRPC documentation generation.

    Attributes:
        method_name: The JSON-RPC method name (e.g., "user.create", "math.add")
        description: Optional human-readable description of what the method does
        params_schema: Optional JSON Schema defining the parameters accepted by this method
        result_schema: Optional JSON Schema defining the structure of the successful result
        deprecated: Whether this method is deprecated (default: False)
        tags: List of tags for categorizing/organizing methods (e.g., ["users", "admin"])

    Example:
        ```python
        from spikard import Spikard
        from spikard.jsonrpc import JsonRpcMethodInfo
        import json

        app = Spikard()

        # Define JSON-RPC method metadata
        add_info = JsonRpcMethodInfo(
            method_name="math.add",
            description="Add two numbers and return the result",
            params_schema={
                "type": "object",
                "properties": {"a": {"type": "number"}, "b": {"type": "number"}},
                "required": ["a", "b"],
            },
            result_schema={"type": "number"},
            tags=["math", "arithmetic"],
        )


        # Attach metadata to a route handler
        @app.post("/rpc", jsonrpc_method=add_info)
        async def add_numbers(a: float, b: float) -> float:
            return a + b
        ```

    Notes:
        - The jsonrpc_method parameter is optional for HTTP routes
        - When present, the route becomes available as a JSON-RPC method
        - Schemas must be valid JSON Schema (Draft 7 or later)
        - Tags help organize methods in documentation (e.g., Swagger/OpenRPC)
        - Deprecated methods can still be called but should warn clients
    """

    method_name: str
    description: str | None = None
    params_schema: dict[str, Any] | None = None
    result_schema: dict[str, Any] | None = None
    deprecated: bool = False
    tags: list[str] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary for serialization.

        Returns a dictionary representation suitable for JSON serialization,
        excluding None values.

        Returns:
            Dictionary with JSON-RPC method metadata
        """
        result: dict[str, Any] = {
            "method_name": self.method_name,
        }

        if self.description is not None:
            result["description"] = self.description

        if self.params_schema is not None:
            result["params_schema"] = self.params_schema

        if self.result_schema is not None:
            result["result_schema"] = self.result_schema

        if self.deprecated:
            result["deprecated"] = True

        if self.tags:
            result["tags"] = self.tags

        return result

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> JsonRpcMethodInfo:
        """Create from dictionary.

        Creates a JsonRpcMethodInfo instance from a dictionary representation,
        useful for deserializing metadata.

        Args:
            data: Dictionary with method metadata

        Returns:
            JsonRpcMethodInfo instance

        Raises:
            KeyError: If required 'method_name' field is missing
            TypeError: If field types don't match expectations
        """
        return cls(
            method_name=data["method_name"],
            description=data.get("description"),
            params_schema=data.get("params_schema"),
            result_schema=data.get("result_schema"),
            deprecated=data.get("deprecated", False),
            tags=data.get("tags", []),
        )
