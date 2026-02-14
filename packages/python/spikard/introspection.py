"""Function signature introspection for automatic parameter validation.

This module provides the main entry point for parsing function signatures
and converting them to JSON Schema for validation in Rust.
"""

import inspect
import re
from typing import TYPE_CHECKING, Any, get_args, get_origin

from spikard._internal import (
    field_definition_to_json_schema,
    parse_fn_signature,
)
from spikard.datastructures import UploadFile

if TYPE_CHECKING:
    from collections.abc import Callable


def extract_parameter_schema(func: Callable[..., Any], path: str | None = None) -> dict[str, Any] | None:
    """Extract JSON Schema from function signature for parameter validation.

    This analyzes the function's type hints using the universal FieldDefinition IR
    and creates a JSON Schema that describes all parameters, their types, and
    validation rules. This works with:
    - Native Python types (str, int, list, dict, etc.)
    - Dataclasses
    - TypedDict
    - Annotated types with constraints
    - msgspec types (optional dependency)
    - attrs classes (optional dependency)

    NOTE: This extracts query/path/header/cookie parameters only. Body parameters
    are handled separately via request_schema from extract_schemas().

    Args:
        func: The function to introspect
        path: The URL path pattern (e.g., "/users/{user_id}") to extract path parameters

    Returns:
        JSON Schema dict or None if no parameters
    """
    parsed_sig = parse_fn_signature(func)

    if not parsed_sig.parameters:
        return None

    path_param_names: set[str] = set()
    if path:
        path_param_names = set(re.findall(r"\{(\w+)\}", path))

    sig = inspect.signature(func)
    params_list = [p for p in sig.parameters.values() if p.name not in ("self", "cls", "request", "req")]
    first_param_is_body = False
    if params_list:
        first_param_name = params_list[0].name
        first_field_def = parsed_sig.parameters.get(first_param_name)
        if first_field_def and (
            _is_structured_type(first_field_def.annotation) or _is_upload_file_type(first_field_def.annotation)
        ):
            first_param_is_body = True

    schema: dict[str, Any] = {"type": "object", "properties": {}, "required": []}

    for idx, (param_name, field_def) in enumerate(parsed_sig.parameters.items()):
        if param_name in {"request", "req"}:
            continue
        if idx == 0 and (first_param_is_body or param_name in {"body", "_body"}):
            continue

        normalized_name = param_name.strip("_")

        param_schema = field_definition_to_json_schema(field_def)

        if "source" not in param_schema:
            if normalized_name in path_param_names:
                param_schema["source"] = "path"
            else:
                param_schema["source"] = "query"

        schema["properties"][normalized_name] = param_schema

        if field_def.is_required:
            schema["required"].append(normalized_name)

    if not schema["properties"]:
        return None

    return schema


def _is_upload_file_type(annotation: Any) -> bool:
    """Check if an annotation is UploadFile or list[UploadFile].

    Args:
        annotation: The type annotation to check

    Returns:
        True if it's UploadFile or list[UploadFile]
    """
    if annotation is UploadFile:
        return True

    if get_origin(annotation) is list:
        args = get_args(annotation)
        if args and args[0] is UploadFile:
            return True

    return False


def _is_structured_type(annotation: Any) -> bool:
    """Check if an annotation is a structured type (body parameter).

    Detects dataclasses, TypedDicts, NamedTuples, msgspec.Struct,
    attrs classes, or any class with similar structure via duck-typing.

    Args:
        annotation: The type annotation to check

    Returns:
        True if it's a structured type suitable for request body
    """
    if not isinstance(annotation, type):
        return False

    if hasattr(annotation, "__dataclass_fields__"):
        return True

    if hasattr(annotation, "__annotations__") and hasattr(annotation, "__total__"):
        return True

    if hasattr(annotation, "_fields") and hasattr(annotation, "_field_types"):
        return True

    try:
        import msgspec  # noqa: PLC0415

        if issubclass(annotation, msgspec.Struct):
            return True
    except (ImportError, TypeError, AttributeError):
        pass

    if hasattr(annotation, "__attrs_attrs__"):
        return True

    return bool(hasattr(annotation, "model_dump") or hasattr(annotation, "to_dict"))
