"""Type conversion utilities using msgspec.

This module handles converting validated JSON data from Rust into Python types
based on handler signatures. It uses msgspec for fast, type-aware conversion
and supports multiple type systems:

- Plain dict: No conversion (fastest)
- TypedDict: No runtime conversion, just type hints (fastest)
- dataclass: Direct construction via **kwargs (fast, Python 3.14 compatible)
- NamedTuple: Direct construction via **kwargs (fast)
- msgspec.Struct: Native msgspec support (fastest typed)
- Pydantic: Custom decoder via model_validate (slower)
"""

from __future__ import annotations

import inspect
from collections.abc import Callable
from contextlib import suppress
from dataclasses import is_dataclass
from datetime import date, datetime, time, timedelta
from typing import Any, get_origin, get_type_hints

import msgspec
from pydantic.fields import FieldInfo

__all__ = ("clear_decoders", "convert_params", "register_decoder", "needs_conversion")


DecoderFunc = Callable[[type, Any], Any]


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
    if hasattr(type_, "model_validate"):
        return type_.model_validate(obj)
    raise NotImplementedError


def _is_typed_dict(type_: type) -> bool:
    """Check if a type is a TypedDict.

    TypedDict is special - it's just type hints at runtime, the actual value is a dict.
    """
    return hasattr(type_, "__annotations__") and hasattr(type_, "__total__") and hasattr(type_, "__required_keys__")


def _default_dec_hook(type_: type, obj: Any) -> Any:
    """Default decoder hook that tries custom decoders, then Pydantic.

    This is called by msgspec when it encounters a type it doesn't know
    how to convert. We try:
    1. Custom user-registered decoders
    2. Pydantic model_validate
    3. Raise NotImplementedError to let msgspec handle it

    Note: msgspec natively handles dataclass, NamedTuple, and msgspec.Struct,
    so those types won't reach this hook.
    """
    for decoder in _CUSTOM_DECODERS:
        with suppress(NotImplementedError):
            return decoder(type_, obj)

    try:
        return _pydantic_decoder(type_, obj)
    except NotImplementedError:
        pass

    raise NotImplementedError


def needs_conversion(handler_func: Callable[..., Any]) -> bool:
    """Check if a handler needs parameter type conversion.

    Returns False for handlers with no parameters or only dict/Any parameters,
    avoiding unnecessary conversion overhead.

    Args:
        handler_func: The handler function to check

    Returns:
        True if the handler needs type conversion, False to skip it
    """
    try:
        sig = inspect.signature(handler_func)
        type_hints = get_type_hints(handler_func)
    except (AttributeError, NameError, TypeError, ValueError):
        # Can't inspect - be conservative and convert
        return True

    # If no parameters, no conversion needed
    if not sig.parameters:
        return False

    # Check if any parameter needs conversion (not dict/Any)
    for param_name, param in sig.parameters.items():
        if param_name not in type_hints:
            continue
        target_type = type_hints[param_name]
        origin = get_origin(target_type)

        # dict and Any don't need conversion
        if target_type in (dict, Any) or origin is dict:
            continue

        # TypedDict doesn't need conversion (runtime is dict)
        if _is_typed_dict(target_type):
            continue

        # If we get here, parameter needs conversion
        return True

    # All parameters are dict/Any or no parameters
    return False


def convert_params(  # noqa: C901, PLR0912, PLR0915
    params: dict[str, Any],
    handler_func: Callable[..., Any],
    *,
    strict: bool = False,
) -> dict[str, Any]:
    """Convert validated parameter dict to typed Python objects.

    This function takes a dictionary of validated parameters from Rust
    and converts them to the appropriate Python types based on the
    handler function's type annotations.

    Performance optimization: When body is passed as raw bytes from Rust,
    this function parses JSON in Python using msgspec for maximum performance.

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
    try:
        type_hints = get_type_hints(handler_func)
    except (AttributeError, NameError, TypeError, ValueError):
        return params

    try:
        sig = inspect.signature(handler_func)
    except (ValueError, TypeError, AttributeError):
        sig = None

    handler_params = set()
    if sig:
        handler_params = set(sig.parameters.keys())

    converted = {}
    for key, value in params.items():
        if sig and key not in handler_params:
            continue

        if key not in type_hints:
            converted[key] = value
            continue

        target_type = type_hints[key]
        origin = get_origin(target_type)

        if key == "body" and isinstance(value, bytes):
            if not value:
                converted[key] = None if target_type in (type(None), None) else {}
                continue

            if target_type is bytes:
                converted[key] = value
                continue

            try:
                parsed_json = msgspec.json.decode(value)
                value = parsed_json
            except (msgspec.DecodeError, ValueError):
                if strict:
                    raise
                converted[key] = value
                continue

        if dict in (target_type, origin):
            converted[key] = value
            continue

        if _is_typed_dict(target_type):
            converted[key] = value
            continue

        if is_dataclass(target_type) and isinstance(value, dict):
            try:
                converted[key] = target_type(**value)  # type: ignore[operator]
                continue
            except (TypeError, ValueError) as err:
                if strict:
                    raise ValueError(f"Failed to convert parameter '{key}' to dataclass {target_type}: {err}") from err
                converted[key] = value
                continue

        if isinstance(target_type, type) and hasattr(target_type, "_fields") and isinstance(value, dict):
            try:
                converted[key] = target_type(**value)
                continue
            except (TypeError, ValueError) as err:
                if strict:
                    raise ValueError(f"Failed to convert parameter '{key}' to NamedTuple {target_type}: {err}") from err
                converted[key] = value
                continue

        try:
            converted[key] = msgspec.convert(
                value,
                type=target_type,
                strict=strict,
                builtin_types=(datetime, date, time, timedelta),
                dec_hook=_default_dec_hook,
            )
        except (msgspec.DecodeError, msgspec.ValidationError, TypeError, ValueError) as err:
            if strict:
                raise ValueError(f"Failed to convert parameter '{key}' to type {target_type}: {err}") from err
            converted[key] = value

    if sig:
        for param_name, param in sig.parameters.items():
            if param_name not in converted and isinstance(param.default, FieldInfo):
                converted[param_name] = param.default.default

    return converted
