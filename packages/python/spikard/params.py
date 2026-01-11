"""Parameter types for dependency injection.

These types are used to extract values from request headers, cookies, etc.
and to specify default values and factories for query/body/path parameters.
"""

import re
from typing import TYPE_CHECKING, Any, TypeVar

if TYPE_CHECKING:
    from collections.abc import Callable

T = TypeVar("T")


class ParamBase:
    """Base class for all parameter wrappers.

    Provides common functionality for default values and default factories.

    When used as a default parameter value, Python will invoke __call__
    when the parameter is not provided, allowing us to lazily generate defaults.
    """

    __slots__ = ("default", "default_factory", "schema")

    def __init__(
        self,
        default: Any = ...,
        *,
        default_factory: Callable[[], Any] | None = None,
        schema: dict[str, Any] | None = None,
    ) -> None:
        if default is not ... and default_factory is not None:
            raise ValueError("Cannot specify both 'default' and 'default_factory'")

        self.default = default
        self.default_factory = default_factory
        self.schema = schema

    def __call__(self) -> Any:
        """Make the wrapper callable so Python can invoke it as a default.

        When a parameter with this wrapper is not provided, Python will
        call this method to get the actual default value.
        """
        return self.get_default()

    def get_default(self) -> Any:
        """Get the default value, invoking factory if needed."""
        if self.default_factory is not None:
            return self.default_factory()
        return self.default

    def has_default(self) -> bool:
        """Check if this parameter has a default value."""
        return self.default is not ... or self.default_factory is not None


class Query[T](ParamBase):
    """Query parameter with optional default or default_factory.

    Use this to specify defaults for query string parameters, similar to FastAPI.

    Examples:
        >>> from spikard import get
        >>>
        >>> @get("/items/")
        >>> def get_items(tags: Query[list[str]] = Query(default_factory=list)):
        ...     return {"tags": tags}
        >>>
        >>> @get("/items/")
        >>> def get_items(limit: Query[int] = Query(default=10)):
        ...     return {"limit": limit}
        >>>
        >>> @get("/items/")
        >>> def get_items(date: Query[datetime.date] = Query(default_factory=datetime.date.today)):
        ...     return {"date": date}
        >>>
        >>> # With custom JSON schema for validation
        >>> @get("/items/")
        >>> def get_items(limit: Query[int] = Query(default=10, schema={"minimum": 1, "maximum": 100})):
        ...     return {"limit": limit}

    Args:
        default: Static default value (if no default_factory provided)
        default_factory: Callable that generates default value when invoked
        schema: Optional JSON schema dict for custom validation (passed to Rust)

    Note:
        Only one of default or default_factory should be provided.
        If both are provided, default_factory takes precedence.
    """


class Body[T](ParamBase):
    """Request body parameter with optional default or default_factory.

    Use this to specify defaults for request body parameters.

    Examples:
        >>> from spikard import post
        >>>
        >>> @post("/items/")
        >>> def create_item(data: Body[dict] = Body(default_factory=dict)):
        ...     return data
        >>>
        >>> # With custom JSON schema for validation
        >>> @post("/items/")
        >>> def create_item(data: Body[dict] = Body(schema={"required": ["name", "price"]})):
        ...     return data

    Args:
        default: Static default value (if no default_factory provided)
        default_factory: Callable that generates default value when invoked
        schema: Optional JSON schema dict for custom validation (passed to Rust)
    """


class Path[T](ParamBase):
    """Path parameter metadata.

    Note: Path parameters are typically required and don't use defaults,
    but this class is provided for API consistency.

    Args:
        default: Static default value (rarely used for path params)
        default_factory: Callable that generates default value (rarely used)
        schema: Optional JSON schema dict for custom validation (passed to Rust)
    """


class Header(ParamBase):
    """Extract a value from request headers.

    Use this as a default parameter value to inject header values into route handlers.

    Examples:
        >>> from spikard import get
        >>>
        >>> @get("/items/")
        >>> def get_items(user_agent: str = Header(default="unknown")):
        ...     return {"user_agent": user_agent}
        >>>
        >>> @get("/users/me")
        >>> def get_user(authorization: str | None = Header(default=None)):
        ...     if authorization:
        ...         return {"authenticated": True}
        ...     return {"authenticated": False}
        >>>
        >>> # With custom JSON schema
        >>> @get("/items/")
        >>> def get_items(api_key: str = Header(schema={"minLength": 32})):
        ...     return {"authenticated": True}

    Args:
        default: Default value if header is not present
        default_factory: Callable that generates default value when invoked
        alias: Alternative header name (e.g., "X-API-Key")
        convert_underscores: Convert underscores to hyphens in header name
        schema: Optional JSON schema dict for custom validation (passed to Rust)
    """

    __slots__ = ("alias", "convert_underscores")

    def __init__(
        self,
        default: Any = ...,
        *,
        default_factory: Callable[[], Any] | None = None,
        alias: str | None = None,
        convert_underscores: bool = True,
        schema: dict[str, Any] | None = None,
    ) -> None:
        super().__init__(default=default, default_factory=default_factory, schema=schema)
        self.alias = alias
        self.convert_underscores = convert_underscores


class Cookie(ParamBase):
    """Extract a value from request cookies.

    Use this as a default parameter value to inject cookie values into route handlers.

    Examples:
        >>> from spikard import get
        >>>
        >>> @get("/items/")
        >>> def get_items(session_id: str | None = Cookie(default=None)):
        ...     return {"session_id": session_id}
        >>>
        >>> @get("/users/me")
        >>> def get_user(key: str = Cookie(schema={"minLength": 10})):
        ...     if key == "secret":
        ...         return {"username": "secret"}
        ...     return {"error": "Invalid key"}
        >>>
        >>> # With default factory
        >>> @get("/items/")
        >>> def get_items(session_data: dict = Cookie(default_factory=dict)):
        ...     return session_data

    Args:
        default: Default value if cookie is not present (use ... for required)
        default_factory: Callable that generates default value when invoked
        min_length: Minimum string length for validation (DEPRECATED: use schema instead)
        max_length: Maximum string length for validation (DEPRECATED: use schema instead)
        pattern: Regex pattern for validation (DEPRECATED: use schema instead)
        schema: Optional JSON schema dict for custom validation (passed to Rust)
    """

    __slots__ = ("max_length", "min_length", "pattern")

    def __init__(
        self,
        default: Any = ...,
        *,
        default_factory: Callable[[], Any] | None = None,
        min_length: int | None = None,
        max_length: int | None = None,
        pattern: str | None = None,
        schema: dict[str, Any] | None = None,
    ) -> None:
        super().__init__(default=default, default_factory=default_factory, schema=schema)
        self.min_length = min_length
        self.max_length = max_length
        self.pattern = re.compile(pattern) if pattern else None


class _Required:
    def __repr__(self) -> str:
        return "..."


REQUIRED = _Required()
