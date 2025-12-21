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

_MSGSPEC_DECODER_CACHE: dict[type, msgspec.json.Decoder[typing.Any]] = {}

_HANDLER_METADATA_CACHE: dict[int, tuple[dict[str, Any] | None, set[str] | None, str | None]] = {}


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
    _HANDLER_METADATA_CACHE.clear()


def supports_msgspec_decoder(target_type: type) -> bool:
    """Return True when msgspec decoding is safe for the given type."""
    if BaseModel is not None and isinstance(target_type, type) and issubclass(target_type, BaseModel):
        return False

    if isinstance(target_type, type) and hasattr(target_type, "_fields"):
        return False

    if _is_upload_file_type(target_type):
        return False

    file_fields = _get_upload_file_fields(target_type)
    return not (file_fields and any(file_fields.values()))


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


def _handler_metadata(
    handler_func: Callable[..., Any],
) -> tuple[dict[str, Any] | None, set[str] | None, str | None]:
    """Cache handler signature and type hints for conversion.

    `inspect.signature()` and `get_type_hints()` are both expensive. We call them on
    every request unless we cache. Since handlers are long-lived, caching provides
    a significant speedup under load.
    """
    cache_key = id(handler_func)
    cached = _HANDLER_METADATA_CACHE.get(cache_key)
    if cached is not None:
        return cached

    try:
        type_hints = get_type_hints(handler_func)
    except (AttributeError, NameError, TypeError, ValueError):
        type_hints = None

    try:
        sig = inspect.signature(handler_func)
    except (ValueError, TypeError, AttributeError):
        _HANDLER_METADATA_CACHE[cache_key] = (type_hints, None, None)
        return type_hints, None, None

    handler_params: set[str] = set(sig.parameters.keys())
    first_param_name: str | None = None
    for param_name in sig.parameters:
        if param_name not in ("self", "cls"):
            first_param_name = param_name
            break

    _HANDLER_METADATA_CACHE[cache_key] = (type_hints, handler_params, first_param_name)
    return type_hints, handler_params, first_param_name


def _default_dec_hook(type_: type, obj: Any) -> Any:
    """Default decoder hook that tries custom decoders.

    This is called by msgspec when it encounters a type it doesn't know
    how to convert. We try:
    1. Custom user-registered decoders
    2. Raise NotImplementedError to let msgspec handle it

    Note: msgspec natively handles dataclass, NamedTuple, and msgspec.Struct,
    so those types won't reach this hook.
    """
    if type_ is UploadFile and isinstance(obj, dict):
        return _convert_file_json_to_upload_file(obj)

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

        if field_value is None:
            continue

        if isinstance(field_value, list):
            result[field_name] = [
                _convert_file_json_to_upload_file(f) if isinstance(f, dict) else f for f in field_value
            ]
        elif isinstance(field_value, dict) and "filename" in field_value:
            result[field_name] = _convert_file_json_to_upload_file(field_value)

    return result


def _coerce_file_dicts_for_scalar_fields(value: dict[str, Any], target_type: type) -> dict[str, Any]:
    """Coerce file metadata dicts into string/bytes for scalar-typed fields."""
    try:
        type_hints = get_type_hints(target_type)
    except (AttributeError, NameError, TypeError):
        return value

    result = value.copy()
    for field_name, field_type in type_hints.items():
        if field_name not in result:
            continue

        field_value = result[field_name]
        if not isinstance(field_value, dict) or "content" not in field_value:
            continue

        origin = get_origin(field_type)
        args = get_args(field_type)
        wants_str = field_type is str or (origin in (Union, types.UnionType) and str in args)
        wants_bytes = field_type is bytes or (origin in (Union, types.UnionType) and bytes in args)

        if not (wants_str or wants_bytes):
            continue

        content = field_value.get("content", "")
        content_encoding = field_value.get("content_encoding", "text")

        if wants_bytes:
            if isinstance(content, str):
                content = base64.b64decode(content) if content_encoding == "base64" else content.encode("utf-8")
            elif not isinstance(content, bytes):
                content = str(content).encode("utf-8")
            result[field_name] = content
        elif isinstance(content, bytes):
            result[field_name] = content.decode("utf-8", errors="replace")
        else:
            result[field_name] = str(content)

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
    type_hints, handler_params, _first_param_name = _handler_metadata(handler_func)
    if type_hints is None or handler_params is None:
        return True
    if not handler_params:
        return False

    for param_name in handler_params:
        if param_name not in type_hints:
            continue
        target_type = type_hints[param_name]
        origin = get_origin(target_type)
        args = get_args(target_type)

        if target_type in (dict, Any) or origin is dict:
            continue

        if _is_typed_dict(target_type):
            continue

        if target_type in (str, int, float, bool, bytes):
            continue

        if origin is Union or origin is types.UnionType:
            non_none_args = [arg for arg in args if arg is not type(None)]
            if non_none_args and all(arg in (str, int, float, bool, bytes) for arg in non_none_args):
                continue

        return True

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
    type_hints, handler_params, first_param_name = _handler_metadata(handler_func)
    if type_hints is None:
        return params

    if "body" in params and first_param_name and first_param_name != "body" and first_param_name not in params:
        params = params.copy()
        params[first_param_name] = params.pop("body")

    converted = {}
    for key, raw_value in params.items():
        if handler_params is not None and key not in handler_params:
            continue

        if key not in type_hints:
            converted[key] = raw_value
            continue

        target_type = type_hints[key]
        origin = get_origin(target_type)
        args = get_args(target_type)
        value = raw_value

        # Allow Rust to provide both a parsed JSON body (builtins) and the original raw bytes.
        # If the handler expects `bytes`, prefer raw bytes to avoid round-tripping through JSON.
        is_body_param_name = key == first_param_name
        if is_body_param_name and target_type is bytes and not isinstance(value, bytes):
            raw_body_bytes = params.get("_raw_body")
            if isinstance(raw_body_bytes, bytes):
                converted[key] = raw_body_bytes
                continue

        is_body_param = is_body_param_name and isinstance(value, bytes)
        if is_body_param:
            if not value:
                converted[key] = None if target_type in (type(None), None) else {}
                continue

            if target_type is bytes:
                converted[key] = value
                continue

            # Fast path: for msgspec.Struct request bodies coming from Rust as raw JSON bytes,
            # decode directly into the target struct using a cached msgspec decoder.
            #
            # The generic code path below first decodes JSON bytes into Python builtins
            # (`msgspec.json.decode`) and then (for msgspec.Struct) constructs the struct via
            # `target_type(**dict)`. That does extra work and can be significantly slower for
            # nested payloads.
            if (
                params.get("_raw_json")
                and isinstance(target_type, type)
                and issubclass(target_type, msgspec.Struct)
                and not _is_upload_file_type(target_type)
            ):
                try:
                    decoder = _get_or_create_decoder(target_type)
                    converted[key] = decoder.decode(value)
                    continue
                except (TypeError, ValueError, msgspec.DecodeError, msgspec.ValidationError):
                    if strict:
                        raise
                    converted[key] = value
                    continue

            try:
                decoded_value = msgspec.json.decode(value)

                if isinstance(decoded_value, dict) and key in decoded_value and _is_upload_file_type(target_type):
                    value = decoded_value[key]
                else:
                    value = decoded_value

            except (msgspec.DecodeError, ValueError):
                if strict:
                    raise
                converted[key] = value
                continue

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

        file_fields = _get_upload_file_fields(target_type)
        if file_fields and isinstance(value, dict):
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
                type_hints_for_dc = get_type_hints(target_type)
                value_with_defaults = value.copy()

                for field in dataclasses.fields(target_type):
                    if field.name not in value_with_defaults:
                        field_type = type_hints_for_dc.get(field.name, field.type)
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

        if isinstance(target_type, type) and isinstance(value, (dict, bytes)):
            try:
                if issubclass(target_type, msgspec.Struct):
                    if isinstance(value, bytes) and params.get("_raw_json"):
                        decoder = _get_or_create_decoder(target_type)
                        converted[key] = decoder.decode(value)
                    elif isinstance(value, dict):
                        value = _coerce_file_dicts_for_scalar_fields(value, target_type)
                        converted[key] = target_type(**value)
                    else:
                        converted[key] = value
                    continue
            except (TypeError, ValueError, msgspec.DecodeError, msgspec.ValidationError) as err:
                if strict:
                    raise ValueError(
                        f"Failed to convert parameter '{key}' to msgspec.Struct {target_type}: {err}"
                    ) from err
                converted[key] = value
                continue

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
