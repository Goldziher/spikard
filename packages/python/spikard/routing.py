"""Standalone routing decorators."""

from typing import Callable


def get(path: str) -> Callable:
    """Standalone GET route decorator.

    Args:
        path: URL path pattern

    Returns:
        Decorator function

    Example:
        @get("/users/{user_id}")
        async def get_user(user_id: int):
            return {"id": user_id}
    """
    raise NotImplementedError(
        "Standalone decorators require an app instance. "
        "Use app.get() instead, or create an app = Spikard() instance first."
    )


def post(path: str) -> Callable:
    """Standalone POST route decorator.

    Args:
        path: URL path pattern

    Returns:
        Decorator function
    """
    raise NotImplementedError(
        "Standalone decorators require an app instance. "
        "Use app.post() instead, or create an app = Spikard() instance first."
    )


def put(path: str) -> Callable:
    """Standalone PUT route decorator.

    Args:
        path: URL path pattern

    Returns:
        Decorator function
    """
    raise NotImplementedError(
        "Standalone decorators require an app instance. "
        "Use app.put() instead, or create an app = Spikard() instance first."
    )


def patch(path: str) -> Callable:
    """Standalone PATCH route decorator.

    Args:
        path: URL path pattern

    Returns:
        Decorator function
    """
    raise NotImplementedError(
        "Standalone decorators require an app instance. "
        "Use app.patch() instead, or create an app = Spikard() instance first."
    )


def delete(path: str) -> Callable:
    """Standalone DELETE route decorator.

    Args:
        path: URL path pattern

    Returns:
        Decorator function
    """
    raise NotImplementedError(
        "Standalone decorators require an app instance. "
        "Use app.delete() instead, or create an app = Spikard() instance first."
    )
