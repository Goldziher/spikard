"""Function signature introspection for automatic parameter validation.

This module provides the main entry point for parsing function signatures
and converting them to JSON Schema for validation in Rust.
"""

from collections.abc import Callable
from typing import Any

from spikard._internal import (
    field_definition_to_json_schema,
    parse_fn_signature,
)


def extract_parameter_schema(func: Callable[..., Any]) -> dict[str, Any] | None:
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

    Args:
        func: The function to introspect

    Returns:
        JSON Schema dict or None if no parameters
    """
    # Parse the function signature into FieldDefinitions
    parsed_sig = parse_fn_signature(func)

    # If no parameters, return None
    if not parsed_sig.parameters:
        return None

    schema: dict[str, Any] = {"type": "object", "properties": {}, "required": []}

    # Convert each parameter to JSON Schema
    for param_name, field_def in parsed_sig.parameters.items():
        # Convert FieldDefinition to JSON Schema
        param_schema = field_definition_to_json_schema(field_def)

        # All parameters come from query by default
        # (This will be extended to support path, header, cookie params)
        param_schema["source"] = "query"

        schema["properties"][param_name] = param_schema

        # Check if required
        if field_def.is_required:
            schema["required"].append(param_name)

    return schema
