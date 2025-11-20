"""Utility functions for type introspection.

Adapted from Litestar's typing utilities.
Original source: https://github.com/litestar-org/litestar
License: MIT (see ATTRIBUTIONS.md in project root)
"""

from collections.abc import Collection, Iterable, Mapping, Sequence
from typing import Annotated, Any, get_args, get_origin


def unwrap_annotation(annotation: Any) -> tuple[Any, tuple[Any, ...], tuple[type, ...]]:
    """Unwrap an annotation, extracting the inner type, metadata, and wrappers.

    Args:
        annotation: A type annotation

    Returns:
        Tuple of (inner_type, metadata, wrappers)
    """
    metadata: list[Any] = []
    wrappers: list[Any] = []

    while True:
        origin = get_origin(annotation)

        if origin is Annotated:
            wrappers.append(Annotated)
            args = get_args(annotation)
            if args:
                annotation = args[0]
                metadata.extend(args[1:])
            continue

        break

    return annotation, tuple(metadata), tuple(wrappers)


def get_instantiable_origin(origin: Any, _annotation: Any) -> Any:
    """Get an instantiable version of the origin type.

    For example, Sequence -> list, Mapping -> dict, etc.

    Args:
        origin: The origin type from get_origin()
        annotation: The full annotation

    Returns:
        An instantiable type
    """
    if origin is None:
        return None

    if origin is Sequence or (hasattr(origin, "__origin__") and origin.__origin__ is Sequence):
        return list
    if origin is Mapping or (hasattr(origin, "__origin__") and origin.__origin__ is Mapping):
        return dict
    if origin is Collection or (hasattr(origin, "__origin__") and origin.__origin__ is Collection):
        return list

    return origin


def get_safe_generic_origin(origin: Any, _annotation: Any) -> Any:
    """Get a safe generic origin for the type.

    This is used for safely rebuilding generic types with different args.

    Args:
        origin: The origin type from get_origin()
        annotation: The full annotation

    Returns:
        A safe generic origin
    """
    return origin


def is_class_and_subclass(obj: Any, types: type | tuple[type, ...]) -> bool:
    """Check if obj is a class and subclass of any of the given types.

    Args:
        obj: Object to check
        types: Types to check against

    Returns:
        True if obj is a class and subclass of any of the types
    """
    try:
        return isinstance(obj, type) and issubclass(obj, types)
    except TypeError:
        return False


def is_non_string_sequence(annotation: Any) -> bool:
    """Check if annotation is a sequence but not a string.

    Args:
        annotation: Type annotation to check

    Returns:
        True if it's a non-string sequence
    """
    origin = get_origin(annotation) or annotation

    try:
        return is_class_and_subclass(origin, (list, tuple, set, frozenset, Sequence)) and not is_class_and_subclass(
            origin, (str, bytes)
        )
    except TypeError:
        return False


def is_non_string_iterable(annotation: Any) -> bool:
    """Check if annotation is iterable but not a string.

    Args:
        annotation: Type annotation to check

    Returns:
        True if it's a non-string iterable
    """
    origin = get_origin(annotation) or annotation

    try:
        return is_class_and_subclass(origin, Iterable) and not is_class_and_subclass(origin, (str, bytes))
    except TypeError:
        return False
