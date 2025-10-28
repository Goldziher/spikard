"""Function signature introspection for automatic parameter validation.

This module provides the main entry point for parsing function signatures
and converting them to JSON Schema for validation in Rust.
"""

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

    schema: dict[str, Any] = {"type": "object", "properties": {}, "required": []}

    # Convert each parameter to JSON Schema
    for param_name, field_def in parsed_sig.parameters.items():
        # Convert FieldDefinition to JSON Schema
        param_schema = field_definition_to_json_schema(field_def)

        # Determine parameter source
        if param_name in path_param_names:
            param_schema["source"] = "path"
        else:
            # Default to query params for now
            # (Will be extended to support header, cookie params via annotations)
            param_schema["source"] = "query"

        schema["properties"][param_name] = param_schema

        # Check if required
        if field_def.is_required:
            schema["required"].append(param_name)

    return schema
