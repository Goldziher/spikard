"""Function signature introspection for automatic parameter validation.

This module provides the main entry point for parsing function signatures
and converting them to JSON Schema for validation in Rust.
"""

import inspect
import re
from collections.abc import Callable
from typing import Any

from spikard._internal import (
    field_definition_to_json_schema,
    parse_fn_signature,
)


def extract_parameter_schema(func: Callable[..., Any], path: str | None = None) -> dict[str, Any] | None:
    """Extract JSON Schema from function signature for parameter validation.

    This analyzes the function's type hints using the universal FieldDefinition IR
    and creates a JSON Schema that describes all parameters, their types, and
    validation rules. This works with:
    - Native Python types (str, int, list, dict, etc.)
    - Pydantic models and Field() (optional dependency)
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
    # Parse the function signature into FieldDefinitions
    parsed_sig = parse_fn_signature(func)

    # If no parameters, return None
    if not parsed_sig.parameters:
        return None

    # Extract path parameter names from the path pattern
    path_param_names: set[str] = set()
    if path:
        # Match {param_name} patterns in the path
        path_param_names = set(re.findall(r"\{(\w+)\}", path))

    # Get function signature to check for body parameter
    # The first parameter (after self/cls) is treated as body if it's a structured type
    sig = inspect.signature(func)
    params_list = [p for p in sig.parameters.values() if p.name not in ("self", "cls")]
    first_param_is_body = False
    if params_list:
        first_param_name = params_list[0].name
        first_field_def = parsed_sig.parameters.get(first_param_name)
        if first_field_def and _is_structured_type(first_field_def.annotation):
            first_param_is_body = True

    schema: dict[str, Any] = {"type": "object", "properties": {}, "required": []}

    # Convert each parameter to JSON Schema
    for idx, (param_name, field_def) in enumerate(parsed_sig.parameters.items()):
        # Skip the first parameter if it's a structured body type
        # (it's handled by request_schema from extract_schemas)
        if idx == 0 and first_param_is_body:
            continue

        # Convert FieldDefinition to JSON Schema
        param_schema = field_definition_to_json_schema(field_def)

        # Determine parameter source (if not already set from Field constraints)
        if "source" not in param_schema:
            if param_name in path_param_names:
                param_schema["source"] = "path"
            else:
                # Default to query params
                param_schema["source"] = "query"

        schema["properties"][param_name] = param_schema

        # Check if required
        if field_def.is_required:
            schema["required"].append(param_name)

    # If no properties, return None
    if not schema["properties"]:
        return None

    return schema


def _is_structured_type(annotation: Any) -> bool:
    """Check if an annotation is a structured type (body parameter).

    Detects dataclasses, Pydantic models, TypedDicts, NamedTuples, msgspec.Struct,
    attrs classes, or any class with similar structure via duck-typing.

    Args:
        annotation: The type annotation to check

    Returns:
        True if it's a structured type suitable for request body
    """
    if not isinstance(annotation, type):
        return False

    # Check for dataclass
    if hasattr(annotation, "__dataclass_fields__"):
        return True

    # Check for TypedDict
    if hasattr(annotation, "__annotations__") and hasattr(annotation, "__total__"):
        return True

    # Check for NamedTuple
    if hasattr(annotation, "_fields") and hasattr(annotation, "_field_types"):
        return True

    # Check for Pydantic BaseModel
    try:
        from pydantic import BaseModel

        if issubclass(annotation, BaseModel):
            return True
    except (ImportError, TypeError):
        pass

    # Check for msgspec.Struct
    try:
        import msgspec

        if issubclass(annotation, msgspec.Struct):
            return True
    except (ImportError, TypeError, AttributeError):
        pass

    # Check for attrs class
    if hasattr(annotation, "__attrs_attrs__"):
        return True

    # Duck-typing: check if it has model_dump, dict, or to_dict methods
    # (common patterns for serializable classes)
    return bool(hasattr(annotation, "model_dump") or hasattr(annotation, "to_dict"))
