"""Type conversion utilities using msgspec.

This module handles converting validated JSON data from Rust into Python types
based on handler signatures. It uses msgspec for fast, type-aware conversion
and supports custom decoder registration.
"""

from __future__ import annotations

import inspect
from collections.abc import Callable
from contextlib import suppress
from datetime import date, datetime, time, timedelta
from typing import Any, get_type_hints

import msgspec
from pydantic.fields import FieldInfo

__all__ = ("clear_decoders", "convert_params", "register_decoder")


# Type alias for decoder functions
DecoderFunc = Callable[[type, Any], Any]


# Global registry of custom decoders
_CUSTOM_DECODERS: list[DecoderFunc] = []


def register_decoder(decoder: DecoderFunc) -> None:
    """Register a custom decoder function.

    The decoder function should have the signature:
        def decoder(type_: Type, obj: Any) -> Any

    It should raise NotImplementedError if it cannot handle the type.

    Example:
        ```python
        from spikard import register_decoder


        def my_custom_decoder(type_: type, obj: Any) -> Any:
            if isinstance(obj, MyCustomType):
                return MyCustomType.from_dict(obj)
            raise NotImplementedError


        register_decoder(my_custom_decoder)
        ```
    """
    _CUSTOM_DECODERS.append(decoder)


def clear_decoders() -> None:
    """Clear all registered custom decoders.

    Useful for testing or when you want to reset the decoder registry.
    """
    _CUSTOM_DECODERS.clear()


def _pydantic_decoder(type_: type, obj: Any) -> Any:
    """Decoder for Pydantic models.

    Uses model_validate which trusts the data has already been validated
    by Rust, so it won't re-validate.
    """
    # Check if it's a Pydantic model
    if hasattr(type_, "model_validate"):
        return type_.model_validate(obj)
    raise NotImplementedError


def _default_dec_hook(type_: type, obj: Any) -> Any:
    """Default decoder hook that tries custom decoders, then Pydantic.

    This is called by msgspec when it encounters a type it doesn't know
    how to convert. We try:
    1. Custom user-registered decoders
    2. Pydantic model_validate
    3. Raise NotImplementedError to let msgspec handle it
    """
    # Try custom decoders first
    for decoder in _CUSTOM_DECODERS:
        with suppress(NotImplementedError):
            return decoder(type_, obj)

    # Try Pydantic decoder
    try:
        return _pydantic_decoder(type_, obj)
    except NotImplementedError:
        pass

    # Let msgspec handle it or error
    raise NotImplementedError


def convert_params(
    params: dict[str, Any],
    handler_func: Callable[..., Any],
    *,
    strict: bool = False,
) -> dict[str, Any]:
    """Convert validated parameter dict to typed Python objects.

    This function takes a dictionary of validated parameters from Rust
    and converts them to the appropriate Python types based on the
    handler function's type annotations.

    Args:
        params: Dictionary of validated parameters (already validated by Rust)
        handler_func: The handler function whose signature we'll use for conversion
        strict: If True, raise errors for type mismatches. If False, be lenient.

    Returns:
        Dictionary with the same keys but values converted to proper Python types

    Example:
        ```python
        from datetime import date


        def my_handler(date_param: date, count: int): ...


        # Rust passes: {"date_param": "2023-07-15", "count": 42}
        converted = convert_params({"date_param": "2023-07-15", "count": 42}, my_handler)
        # Result: {"date_param": date(2023, 7, 15), "count": 42}
        ```
    """
    # Get type hints from handler function
    try:
        type_hints = get_type_hints(handler_func)
    except (AttributeError, NameError, TypeError, ValueError):
        # If we can't get type hints, just return params as-is
        return params

    # Get function signature to handle default values
    try:
        sig = inspect.signature(handler_func)
    except (ValueError, TypeError):
        sig = None

    # Convert each parameter based on its type hint
    converted = {}
    for key, value in params.items():
        if key not in type_hints:
            # No type hint, keep as-is
            converted[key] = value
            continue

        target_type = type_hints[key]

        try:
            # Use msgspec.convert for type conversion
            # builtin_types tells msgspec that these types can be constructed from strings
            converted[key] = msgspec.convert(
                value,
                type=target_type,
                strict=strict,
                builtin_types=(datetime, date, time, timedelta),
                dec_hook=_default_dec_hook,
            )
        except (msgspec.DecodeError, msgspec.ValidationError, TypeError, ValueError) as err:
            # If conversion fails and we're not strict, keep the original value
            if strict:
                raise ValueError(f"Failed to convert parameter '{key}' to type {target_type}: {err}") from err
            converted[key] = value

    # Handle parameters with Field() defaults that weren't in params
    # This is critical because Field() creates a FieldInfo object as the default,
    # and if we don't explicitly pass None, Python will use the FieldInfo object!
    if sig:
        for param_name, param in sig.parameters.items():
            if param_name not in converted and isinstance(param.default, FieldInfo):
                # Extract the actual default value from the FieldInfo object
                # FieldInfo.default is the real default value (e.g., None)
                converted[param_name] = param.default.default

    return converted
