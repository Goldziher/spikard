"""Dependency injection module for Spikard.

This module provides dependency injection capabilities similar to Litestar's
Provide pattern, allowing for automatic injection of dependencies into handlers
based on parameter names and type annotations.

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

Generator pattern for cleanup::

    async def create_session(db):
        session = await db.create_session()
        yield session
        await session.close()


    app.provide("session", Provide(create_session, depends_on=["db"]))
"""

import asyncio
import inspect
from collections.abc import AsyncGenerator, Callable, Generator
from typing import Any, Generic, TypeVar

T = TypeVar("T")


class Provide(Generic[T]):
    """Wrapper for dependency factories.

    This class wraps a factory function that will be called to create a dependency
    value when needed. The factory can depend on other dependencies, which will be
    resolved first.

    Parameters
    ----------
    dependency : Callable[..., T] | Callable[..., AsyncGenerator[T, None]] | Callable[..., Generator[T, None, None]]
        The factory function to create the dependency. Can be sync or async,
        and can be a generator for cleanup support.
    depends_on : list[str] | None, optional
        List of dependency keys this factory depends on, by default None.
        Dependencies will be passed as keyword arguments to the factory.
    use_cache : bool, optional
        Whether to cache the resolved value within a single request, by default False.
    singleton : bool, optional
        Whether to cache the resolved value globally across all requests, by default False.
        Takes precedence over use_cache.

    Attributes:
    ----------
    dependency : Callable
        The factory function
    depends_on : list[str]
        List of dependency names this factory needs
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
    Async factory with dependencies::

        async def create_db_pool(config: dict) -> DatabasePool:
            return await connect_to_db(config["db_url"])


        app.provide("db", Provide(create_db_pool, depends_on=["config"], singleton=True))

    Generator for cleanup::

        async def create_session(db: DatabasePool):
            session = await db.create_session()
            yield session
            await session.close()


        app.provide("session", Provide(create_session, depends_on=["db"]))
    """

    def __init__(
        self,
        dependency: Callable[..., T] | Callable[..., AsyncGenerator[T, None]] | Callable[..., Generator[T, None, None]],
        *,
        depends_on: list[str] | None = None,
        use_cache: bool = False,
        cacheable: bool | None = None,  # Alias for use_cache
        singleton: bool = False,
    ) -> None:
        """Initialize a dependency factory.

        Parameters
        ----------
        dependency : Callable
            The factory function to create the dependency
        depends_on : list[str] | None
            List of dependency keys this factory depends on
        use_cache : bool
            Whether to cache within a request
        cacheable : bool | None
            Alias for use_cache (for compatibility with fixtures)
        singleton : bool
            Whether to cache globally
        """
        self.dependency = dependency
        self.depends_on = depends_on or []
        # If cacheable is provided, use it; otherwise use use_cache
        self.use_cache = cacheable if cacheable is not None else use_cache
        self.singleton = singleton
        self.is_async = asyncio.iscoroutinefunction(dependency)
        self.is_generator = inspect.isgeneratorfunction(dependency)
        self.is_async_generator = inspect.isasyncgenfunction(dependency)

        # Extract parameter names from function signature if depends_on not provided
        if not self.depends_on:
            sig = inspect.signature(dependency)
            self.depends_on = [
                param_name
                for param_name, param in sig.parameters.items()
                if param_name not in ("self", "cls", "request", "response")
            ]

    def __repr__(self) -> str:
        """Return a string representation of the Provide instance.

        Returns:
        -------
        str
            String representation showing factory name and attributes
        """
        factory_name = getattr(self.dependency, "__name__", repr(self.dependency))
        return (
            f"Provide({factory_name}, "
            f"depends_on={self.depends_on}, "
            f"singleton={self.singleton}, "
            f"use_cache={self.use_cache})"
        )


def inject_dependencies(
    handler: Callable[..., Any],
    resolved: dict[str, Any],
) -> dict[str, Any]:
    """Match dependencies to handler parameters by name and type.

    This function introspects a handler's parameters and matches them against
    resolved dependencies. It tries to match by parameter name first, then by
    type annotation.

    Parameters
    ----------
    handler : Callable
        The handler function to introspect
    resolved : dict[str, Any]
        Dictionary of resolved dependencies

    Returns:
    -------
    dict[str, Any]
        Keyword arguments to pass to the handler

    Examples:
    --------
    ::

        @app.get("/users")
        async def get_users(db: Database) -> list[User]:
            # 'db' is auto-injected by matching parameter name
            return await db.query(User).all()
    """
    sig = inspect.signature(handler)
    kwargs: dict[str, Any] = {}

    for param_name, param in sig.parameters.items():
        # Skip special parameters
        if param_name in ("self", "cls", "request", "response"):
            continue

        # Try name match first
        if param_name in resolved:
            kwargs[param_name] = resolved[param_name]
        # Try type match
        elif param.annotation != inspect.Parameter.empty:
            for dep_value in resolved.values():
                if isinstance(dep_value, param.annotation):
                    kwargs[param_name] = dep_value
                    break

    return kwargs


__all__ = [
    "Provide",
    "inject_dependencies",
]
