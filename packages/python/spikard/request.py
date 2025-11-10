"""Request type for Spikard handlers.

At runtime, the Request object is provided by the Rust/Axum backend.
This module provides type stubs for static type checking.
"""

from typing import Any


class Request:
    """HTTP Request object provided to route handlers.

    This is a stub for type checking. At runtime, the actual Request
    object is provided by the Rust backend through Axum.

    Attributes:
        method: HTTP method (GET, POST, etc.)
        path: Request path
        query_string: Query string
        headers: Request headers
        body: Request body (if any)
    """

    method: str
    path: str
    query_string: str
    headers: dict[str, str]
    body: bytes | None

    def __init__(self) -> None:
        """Initialize a Request stub.

        Note: In practice, Request objects are created by the Rust backend,
        not by user code.
        """

    def json(self) -> Any:
        """Parse request body as JSON.

        Returns:
            Parsed JSON data
        """
        raise NotImplementedError("Request.json() is provided at runtime by Rust backend")

    def form(self) -> dict[str, str]:
        """Parse request body as form data.

        Returns:
            Form data as a dictionary
        """
        raise NotImplementedError("Request.form() is provided at runtime by Rust backend")
