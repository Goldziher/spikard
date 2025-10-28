"""Parameter types for dependency injection.

These types are used to extract values from request headers, cookies, etc.
"""

import re
from typing import Any


class Header:
    """Extract a value from request headers.

    Use this as a default parameter value to inject header values into route handlers.

    Examples:
        >>> @app.get("/items/")
        >>> def get_items(user_agent: str = Header("User-Agent")):
        ...     return {"user_agent": user_agent}
        >>>
        >>> @app.get("/users/me")
        >>> def get_user(authorization: Optional[str] = Header(None)):
        ...     if authorization:
        ...         return {"authenticated": True}
        ...     return {"authenticated": False}

    Args:
        default: Default value if header is not present
        alias: Alternative header name (e.g., "X-API-Key")
        convert_underscores: Convert underscores to hyphens in header name
    """

    def __init__(
        self,
        default: Any = ...,
        *,
        alias: str | None = None,
        convert_underscores: bool = True,
    ) -> None:
        self.default = default
        self.alias = alias
        self.convert_underscores = convert_underscores


class Cookie:
    """Extract a value from request cookies.

    Use this as a default parameter value to inject cookie values into route handlers.

    Examples:
        >>> @app.get("/items/")
        >>> def get_items(session_id: Optional[str] = Cookie(None)):
        ...     return {"session_id": session_id}
        >>>
        >>> @app.get("/users/me")
        >>> def get_user(key: str = Cookie(..., min_length=10)):
        ...     if key == "secret":
        ...         return {"username": "secret"}
        ...     return {"error": "Invalid key"}

    Args:
        default: Default value if cookie is not present (use ... for required)
        min_length: Minimum string length for validation
        max_length: Maximum string length for validation
        pattern: Regex pattern for validation
    """

    def __init__(
        self,
        default: Any = ...,
        *,
        min_length: int | None = None,
        max_length: int | None = None,
        pattern: str | None = None,
    ) -> None:
        self.default = default
        self.min_length = min_length
        self.max_length = max_length
        self.pattern = re.compile(pattern) if pattern else None


# Sentinel value for required parameters
class _Required:
    def __repr__(self) -> str:
        return "..."


Ellipsis = _Required()
