"""Dependency injection module for Spikard.

This module provides a thin wrapper for dependency injection, delegating all
complex resolution logic to the Rust-based DI engine in spikard-core.

The `Provide` class is a simple metadata wrapper that captures:
- The factory function
- Its dependencies
- Caching strategy (singleton vs per-request)

All actual DI graph resolution, cycle detection, and parallel resolution happens
in Rust via the FFI bridge in crates/spikard-py/src/di.rs.

Examples:
--------
Type-based dependency (recommended)::

    from spikard import Spikard
    from spikard.di import Provide

    app = Spikard()


    class DatabasePool:
        async def connect(self, url: str) -> None: ...
        async def close(self) -> None: ...


    async def create_db_pool(config: dict) -> DatabasePool:
        pool = DatabasePool()
        await pool.connect(config["db_url"])
        return pool


    app.provide("config", {"db_url": "postgresql://localhost/mydb"})
    app.provide(DatabasePool, Provide(create_db_pool, depends_on=["config"], singleton=True))


    @app.get("/data")
    async def get_data(pool: DatabasePool):
        # pool is automatically resolved from type
        return {"status": "ok"}

String-based dependency (legacy, still supported)::

    from spikard import Spikard
    from spikard.di import Provide

    app = Spikard()


    async def create_db_pool(config: dict):
        pool = await connect_to_db(config["db_url"])
        return pool


    app.provide("config", {"db_url": "postgresql://localhost/mydb"})
    app.provide("db", Provide(create_db_pool, depends_on=["config"], singleton=True))

Async generator cleanup::

    from spikard import Spikard
    from spikard.di import Provide


    class SessionManager:
        async def create(self) -> None: ...
        async def close(self) -> None: ...


    async def create_session(db: DatabasePool):
        session = SessionManager()
        await session.create()
        yield session
        await session.close()


    app = Spikard()
    app.provide(SessionManager, Provide(create_session, depends_on=[DatabasePool]))
"""

from __future__ import annotations

import asyncio
import inspect
from typing import TYPE_CHECKING, Generic, TypeVar

if TYPE_CHECKING:
    from collections.abc import AsyncGenerator, Callable, Generator

T = TypeVar("T")


def _normalize_key(key: type | str) -> str:
    """Convert a type or string to the normalized dependency key format.

    When a type is passed, it's converted to "__type__module.qualname" format.
    String keys are returned as-is for backward compatibility.

    Parameters
    ----------
    key : type | str
        The type or string to normalize

    Returns:
    -------
    str
        The normalized key in "__type__module.qualname" format or the original string
    """
    if isinstance(key, str):
        return key
    # key is a type
    module = key.__module__
    qualname = key.__qualname__
    return f"__type__{module}.{qualname}"


class Provide(Generic[T]):
    """Wrapper for dependency factories.

    This class wraps a factory function that will be called to create a dependency
    value when needed. The factory can depend on other dependencies, which are passed
    as keyword arguments to the factory.

    The Rust DI engine handles:
    - Topological sorting of dependencies
    - Parallel resolution of independent dependencies
    - Singleton and per-request caching
    - Cycle detection
    - Cleanup task registration for generators

    Supports both type-based (recommended) and string-based dependency keys.
    Types are automatically converted to "__type__module.qualname" format for consistency
    with the Rust DI engine's key normalization.

    Parameters
    ----------
    dependency : Callable
        The factory function. Can be sync, async, or async generator.
    depends_on : list[type | str] | None
        Dependency keys (as types or strings) this factory needs. Types are converted to
        "__type__module.qualname" format. If None, auto-detected from function signature
        by excluding 'self', 'cls', 'request', 'response'.
    use_cache : bool
        Whether to cache within a request. Overridden by singleton=True.
    cacheable : bool | None
        Alias for use_cache (for backwards compatibility).
    singleton : bool
        Cache globally across all requests. Takes precedence over use_cache.

    Attributes:
    ----------
    dependency : Callable
        The factory function
    depends_on : list[str]
        List of normalized dependency keys this factory needs
    use_cache : bool
        Whether to cache per-request
    singleton : bool
        Whether to cache globally
    is_async : bool
        Whether the factory is an async function
    is_generator : bool
        Whether the factory is a sync generator
    is_async_generator : bool
        Whether the factory is an async generator

    Examples:
    --------
    Type-based dependency (recommended)::

        class DatabasePool:
            pass


        async def create_pool() -> DatabasePool:
            return DatabasePool()


        app.provide(DatabasePool, Provide(create_pool, singleton=True))

    String-based dependency (legacy)::

        async def create_pool() -> dict:
            return {}


        app.provide("db", Provide(create_pool, singleton=True))

    Explicit type dependencies::

        class Config:
            pass


        class DatabasePool:
            pass


        async def create_pool(config: Config) -> DatabasePool:
            return DatabasePool()


        # Explicit: app.provide(DatabasePool, Provide(create_pool, depends_on=[Config]))
        # Or auto-detected from signature: app.provide(DatabasePool, Provide(create_pool))
    """

    __slots__ = (
        "dependency",
        "depends_on",
        "is_async",
        "is_async_generator",
        "is_generator",
        "singleton",
        "use_cache",
    )

    def __init__(
        self,
        dependency: Callable[..., T] | Callable[..., AsyncGenerator[T]] | Callable[..., Generator[T]],
        *,
        depends_on: list[type | str] | None = None,
        use_cache: bool = False,
        cacheable: bool | None = None,
        singleton: bool = False,
    ) -> None:
        self.dependency = dependency
        # Normalize all dependency keys: convert types to "__type__module.qualname" format
        self.depends_on = [_normalize_key(dep) for dep in (depends_on or [])]
        self.use_cache = cacheable if cacheable is not None else use_cache
        self.singleton = singleton
        self.is_async = asyncio.iscoroutinefunction(dependency)
        self.is_generator = inspect.isgeneratorfunction(dependency)
        self.is_async_generator = inspect.isasyncgenfunction(dependency)

        if not self.depends_on:
            try:
                sig = inspect.signature(dependency)
            except (TypeError, ValueError):
                # Built-in callables (for example `str`) may not expose signatures.
                self.depends_on = []
            else:
                self.depends_on = [
                    param_name
                    for param_name, param in sig.parameters.items()
                    if param_name not in ("self", "cls", "request", "response")
                ]

    def __repr__(self) -> str:
        """Return a string representation of the Provide instance."""
        factory_name = getattr(self.dependency, "__name__", repr(self.dependency))
        return (
            f"Provide({factory_name}, "
            f"depends_on={self.depends_on}, "
            f"singleton={self.singleton}, "
            f"use_cache={self.use_cache})"
        )


__all__ = ["Provide"]
