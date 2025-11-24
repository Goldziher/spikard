"""Type conversion utilities using msgspec.

This module handles converting validated JSON data from Rust into Python types
based on handler signatures. It uses msgspec for fast, type-aware conversion
and supports multiple type systems:

- Plain dict: No conversion (fastest)
- TypedDict: No runtime conversion, just type hints (fastest)
- dataclass: Direct construction via **kwargs (fast, Python 3.14 compatible)
- NamedTuple: Direct construction via **kwargs (fast)
- msgspec.Struct: Native msgspec support (fastest typed)
"""

from __future__ import annotations

import base64
import dataclasses
import inspect
import types
import typing
from collections.abc import Callable
from contextlib import suppress
from dataclasses import is_dataclass
from datetime import date, datetime, time, timedelta
from typing import TYPE_CHECKING, Any, Union, get_args, get_origin, get_type_hints

import msgspec

from spikard.datastructures import UploadFile

if TYPE_CHECKING:
    from pydantic import BaseModel as PydanticBaseModel
else:  # pragma: no cover - runtime optional import
    try:
        from pydantic import BaseModel as PydanticBaseModel
    except ImportError:
        PydanticBaseModel = None  # type: ignore[assignment]

BaseModel = typing.cast("type[PydanticBaseModel] | None", PydanticBaseModel)

__all__ = ("clear_decoders", "convert_params", "needs_conversion", "register_decoder")


DecoderFunc = Callable[[type, Any], Any]


_CUSTOM_DECODERS: list[DecoderFunc] = []

# Cache of msgspec.json.Decoder instances per type
# Performance optimization: reuse decoders instead of creating new ones per request
_MSGSPEC_DECODER_CACHE: dict[type, msgspec.json.Decoder[typing.Any]] = {}


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


def _is_typed_dict(type_: type) -> bool:
    """Check if a type is a TypedDict.

    TypedDict is special - it's just type hints at runtime, the actual value is a dict.
    """
    return hasattr(type_, "__annotations__") and hasattr(type_, "__total__") and hasattr(type_, "__required_keys__")


def _get_or_create_decoder(target_type: type) -> msgspec.json.Decoder[typing.Any]:
    """Get or create a cached msgspec.json.Decoder for the given type.

    Performance optimization: reusing decoders avoids repeated internal state allocation.
    See: https://jcristharif.com/msgspec/perf-tips.html
    """
    if target_type not in _MSGSPEC_DECODER_CACHE:
        _MSGSPEC_DECODER_CACHE[target_type] = msgspec.json.Decoder(
            type=target_type,
            dec_hook=_default_dec_hook,
        )
    return _MSGSPEC_DECODER_CACHE[target_type]


def _default_dec_hook(type_: type, obj: Any) -> Any:
    """Default decoder hook that tries custom decoders.

    This is called by msgspec when it encounters a type it doesn't know
    how to convert. We try:
    1. Custom user-registered decoders
    2. Raise NotImplementedError to let msgspec handle it

    Note: msgspec natively handles dataclass, NamedTuple, and msgspec.Struct,
    so those types won't reach this hook.
    """
    for decoder in _CUSTOM_DECODERS:
        with suppress(NotImplementedError):
            return decoder(type_, obj)

    raise NotImplementedError


def _is_upload_file_type(type_hint: type) -> bool:
    """Check if a type annotation is UploadFile or Optional[UploadFile] or list[UploadFile].

    Args:
        type_hint: The type to check

    Returns:
        True if it's an UploadFile-related type
    """
    if type_hint is UploadFile:
        return True

    origin = get_origin(type_hint)

    if origin is list:
        args = get_args(type_hint)
        if args and args[0] is UploadFile:
            return True

    if origin is Union or origin is types.UnionType:
        args = get_args(type_hint)
        return any(arg is UploadFile for arg in args if arg is not type(None))

    return False


def _get_upload_file_fields(target_type: type) -> dict[str, bool]:
    """Detect which fields in a dataclass/Pydantic model are UploadFile fields.

    Args:
        target_type: The type to analyze (dataclass, Pydantic model, msgspec.Struct, etc.)

    Returns:
        Dict mapping field names to whether they're UploadFile types
        Example: {"avatar": True, "username": False}
    """
    # Only analyze structured types
    if not (is_dataclass(target_type) or hasattr(target_type, "model_fields") or isinstance(target_type, type)):
        return {}

    try:
        type_hints = get_type_hints(target_type)
    except (AttributeError, NameError, TypeError):
        return {}

    return {field_name: _is_upload_file_type(field_type) for field_name, field_type in type_hints.items()}


def _convert_file_json_to_upload_file(file_data: dict[str, Any]) -> Any:
    """Convert JSON file representation to UploadFile instance.

    Args:
        file_data: Dict with keys: filename, content, size, content_type, content_encoding

    Returns:
        UploadFile instance
    """
    content = file_data.get("content", b"")

    if isinstance(content, str):
        content_encoding = file_data.get("content_encoding", "text")
        content = base64.b64decode(content) if content_encoding == "base64" else content.encode("utf-8")
    elif not isinstance(content, bytes):
        content = str(content).encode("utf-8")

    return UploadFile(
        filename=file_data.get("filename", "unknown"),
        content=content,
        content_type=file_data.get("content_type"),
        size=file_data.get("size"),
    )


def _process_upload_file_fields(value: Any, file_fields: dict[str, bool]) -> Any:
    """Convert JSON file representations to UploadFile instances in a structured object.

    Args:
        value: The dict or object containing file fields
        file_fields: Map of field names to whether they're UploadFile types

    Returns:
        Modified value with UploadFile instances
    """
    if not isinstance(value, dict):
        return value

    result = value.copy()

    for field_name, is_file_field in file_fields.items():
        if not is_file_field or field_name not in result:
            continue

        field_value = result[field_name]

        # Skip None values
        if field_value is None:
            continue

        # Handle list[UploadFile]
        if isinstance(field_value, list):
            result[field_name] = [
                _convert_file_json_to_upload_file(f) if isinstance(f, dict) else f for f in field_value
            ]
        # Handle single UploadFile
        elif isinstance(field_value, dict) and "filename" in field_value:
            result[field_name] = _convert_file_json_to_upload_file(field_value)

    return result


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
    for param_name in sig.parameters:
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
    first_param_name = None
    if sig:
        handler_params = set(sig.parameters.keys())
        # Find the first non-self/non-cls parameter (this is the body parameter)
        for param_name in sig.parameters:
            if param_name not in ("self", "cls"):
                first_param_name = param_name
                break

    # If Rust passed "body" but handler uses a different parameter name,
    # rename it to match the handler's first parameter
    if "body" in params and first_param_name and first_param_name != "body" and first_param_name not in params:
        params = params.copy()
        params[first_param_name] = params.pop("body")

    converted = {}
    for key, raw_value in params.items():
        if sig and key not in handler_params:
            continue

        if key not in type_hints:
            converted[key] = raw_value
            continue

        target_type = type_hints[key]
        origin = get_origin(target_type)
        args = get_args(target_type)
        value = raw_value

        # For body parameter with bytes, decode JSON first
        # Body parameter is the first non-self/non-cls parameter
        is_body_param = key == first_param_name and isinstance(value, bytes)
        if is_body_param:
            if not value:
                converted[key] = None if target_type in (type(None), None) else {}
                continue

            if target_type is bytes:
                converted[key] = value
                continue

            # Decode JSON bytes to dict/list for further processing
            try:
                decoded_value = msgspec.json.decode(value)

                # For raw UploadFile/list[UploadFile], unwrap from {"param_name": value} wrapper
                # Rust wraps multipart form fields in an object: {"files": [...]}
                if isinstance(decoded_value, dict) and key in decoded_value and _is_upload_file_type(target_type):
                    value = decoded_value[key]
                else:
                    value = decoded_value

                # Don't continue - fall through to UploadFile/dataclass processing
            except (msgspec.DecodeError, ValueError):
                if strict:
                    raise
                converted[key] = value
                continue

        # Handle direct UploadFile or list[UploadFile] parameters
        if _is_upload_file_type(target_type):
            if origin is Union and any(arg is UploadFile for arg in args) and isinstance(value, dict):
                converted[key] = _convert_file_json_to_upload_file(value)
                continue
            if target_type is UploadFile and isinstance(value, dict):
                converted[key] = _convert_file_json_to_upload_file(value)
                continue
            if origin is list and args and args[0] is UploadFile and isinstance(value, list):
                converted[key] = [_convert_file_json_to_upload_file(f) if isinstance(f, dict) else f for f in value]
                continue

        # Now check if this type has UploadFile fields (for multipart/form data)
        # This happens AFTER JSON decoding so value is a dict
        file_fields = _get_upload_file_fields(target_type)
        if file_fields and isinstance(value, dict):
            # Process file fields: convert JSON file representations to UploadFile instances
            processed_value = _process_upload_file_fields(value, file_fields)
            value = processed_value

        if dict in (target_type, origin):
            converted[key] = value
            continue

        if _is_typed_dict(target_type):
            converted[key] = value
            continue

        if is_dataclass(target_type) and isinstance(value, dict):
            try:
                # Add missing optional fields with None
                type_hints_for_dc = get_type_hints(target_type)
                value_with_defaults = value.copy()

                for field in dataclasses.fields(target_type):
                    if field.name not in value_with_defaults:
                        field_type = type_hints_for_dc.get(field.name, field.type)
                        # Check if field is Optional
                        origin = get_origin(field_type)
                        if origin in (Union, types.UnionType) and type(None) in get_args(field_type):
                            value_with_defaults[field.name] = None

                converted[key] = target_type(**value_with_defaults)  # type: ignore[operator]
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

        # Handle msgspec.Struct: use fast decoder path if we have raw JSON bytes
        if isinstance(target_type, type) and isinstance(value, (dict, bytes)):
            try:
                if issubclass(target_type, msgspec.Struct):
                    # Fast path: decode directly from JSON bytes using cached decoder
                    if isinstance(value, bytes) and params.get("_raw_json"):
                        decoder = _get_or_create_decoder(target_type)
                        converted[key] = decoder.decode(value)
                    # Fallback: construct from dict (after Rust validation)
                    elif isinstance(value, dict):
                        converted[key] = target_type(**value)
                    else:
                        converted[key] = value
                    continue
            except (TypeError, ValueError, msgspec.DecodeError) as err:
                if strict:
                    raise ValueError(
                        f"Failed to convert parameter '{key}' to msgspec.Struct {target_type}: {err}"
                    ) from err
                converted[key] = value
                continue

        # Pydantic BaseModel support
        if BaseModel is not None and isinstance(target_type, type) and issubclass(target_type, BaseModel):
            try:
                if (
                    isinstance(value, list)
                    and origin in (list, tuple)
                    and args
                    and isinstance(args[0], type)
                    and issubclass(args[0], BaseModel)
                ):
                    model_cls = args[0]
                    converted[key] = [
                        model_cls.model_validate(item) if isinstance(item, dict) else item for item in value
                    ]
                elif isinstance(value, dict):
                    model_cls = target_type
                    converted[key] = model_cls.model_validate(value)
                else:
                    converted[key] = value
                continue
            except Exception as err:
                if strict:
                    raise ValueError(
                        f"Failed to convert parameter '{key}' to Pydantic model {target_type}: {err}"
                    ) from err
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

    return converted
