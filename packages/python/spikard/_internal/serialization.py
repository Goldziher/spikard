"""Serialization utilities for converting Python objects to JSON-serializable formats.

This module provides efficient serialization using msgspec for optimal performance.
When handlers return dataclasses, msgspec.Struct instances, or other complex types,
these utilities convert them to basic Python types (dict, list, str, int, etc.) that
can be serialized to JSON.
"""

from typing import Any

import msgspec


def to_builtins(obj: Any) -> Any:
    """Convert Python objects to JSON-serializable builtins using msgspec.

    This function handles conversion of complex Python types (dataclasses, msgspec.Struct,
    Pydantic models, etc.) to basic Python types (dict, list, str, int, float, bool, None)
    that can be serialized to JSON without additional processing.

    Uses msgspec's optimized conversion path for maximum performance and compatibility
    with the Spikard type system.

    Args:
        obj: Any Python object to convert. Can be a dataclass, msgspec.Struct,
            Pydantic model, or any type msgspec can encode.

    Returns:
        A JSON-serializable representation of the object. Complex types become dicts,
        sequences become lists, and scalar types are returned as-is.

    Raises:
        TypeError: If the object cannot be converted to a msgspec-compatible format.

    Examples:
        >>> from dataclasses import dataclass
        >>> @dataclass
        ... class User:
        ...     name: str
        ...     age: int
        >>> to_builtins(User("Alice", 30))
        {'name': 'Alice', 'age': 30}

        >>> import msgspec
        >>> class Point(msgspec.Struct):
        ...     x: int
        ...     y: int
        >>> to_builtins(Point(1, 2))
        {'x': 1, 'y': 2}

        >>> to_builtins([1, 2, 3])
        [1, 2, 3]

        >>> to_builtins("hello")
        'hello'
    """
    return msgspec.to_builtins(obj)
