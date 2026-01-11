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
Value dependency::

    from spikard import Spikard

    app = Spikard()
    app.provide("app_name", "MyApp")


    @app.get("/info")
    async def get_info(app_name: str):
        return {"app": app_name}

Factory dependency::

    from spikard import Spikard
    from spikard.di import Provide

    app = Spikard()


    async def create_db_pool(config: dict):
        pool = await connect_to_db(config["db_url"])
        return pool


    app.provide("config", {"db_url": "postgresql://localhost/mydb"})
    app.provide("db", Provide(create_db_pool, depends_on=["config"], singleton=True))

Async generator cleanup::

    async def create_session(db):
        session = await db.create_session()
        yield session
        await session.close()


    app.provide("session", Provide(create_session, depends_on=["db"]))
"""

import asyncio
import inspect
from typing import TYPE_CHECKING, TypeVar

if TYPE_CHECKING:
    from collections.abc import AsyncGenerator, Callable, Generator

T = TypeVar("T")


class Provide[T]:
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

    Parameters
    ----------
    dependency : Callable
        The factory function. Can be sync, async, or async generator.
    depends_on : list[str] | None
        Dependency keys this factory needs. If None, auto-detected from function
        signature by excluding 'self', 'cls', 'request', 'response'.
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
        List of dependency keys this factory needs
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
        depends_on: list[str] | None = None,
        use_cache: bool = False,
        cacheable: bool | None = None,
        singleton: bool = False,
    ) -> None:
        self.dependency = dependency
        self.depends_on = depends_on or []
        self.use_cache = cacheable if cacheable is not None else use_cache
        self.singleton = singleton
        self.is_async = asyncio.iscoroutinefunction(dependency)
        self.is_generator = inspect.isgeneratorfunction(dependency)
        self.is_async_generator = inspect.isasyncgenfunction(dependency)

        if not self.depends_on:
            sig = inspect.signature(dependency)
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
