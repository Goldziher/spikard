"""JSON Schema generation from FieldDefinition.

This module converts the universal FieldDefinition IR into JSON Schema format
that can be passed to Rust for validation and caching.
"""

from datetime import date, datetime, time, timedelta
from enum import Enum
from pathlib import Path, PurePath
from typing import Any
from uuid import UUID

from spikard._internal.field_definition import FieldDefinition

__all__ = ("field_definition_to_json_schema",)


def field_definition_to_json_schema(field: FieldDefinition) -> dict[str, Any]:
    """Convert a FieldDefinition to JSON Schema.

    Args:
        field: The FieldDefinition to convert

    Returns:
        JSON Schema dictionary
    """
    schema: dict[str, Any] = {}

    # Handle union types (including Optional)
    if field.is_optional:
        # For Optional[T], get the non-None type and make it nullable
        non_none_types = [arg for arg in field.args if arg is not type(None)]
        if len(non_none_types) == 1:
            # Simple Optional[T]
            inner_field = (
                field.inner_types[0]
                if field.inner_types and field.inner_types[0].annotation is not type(None)
                else None
            )
            if inner_field:
                # Recursively convert the inner type
                schema = field_definition_to_json_schema(inner_field)
            else:
                # Fallback to basic type
                schema = _annotation_to_json_schema(non_none_types[0])

            # Apply constraints from the parent field (which may have Pydantic Field constraints)
            _apply_constraints(schema, field.extra)

            # Optional fields are handled by not including them in required array
            return schema

    # Handle union types (non-Optional)
    if field.is_union and not field.is_optional:
        # Use anyOf for union types
        schema["anyOf"] = [
            field_definition_to_json_schema(inner) for inner in field.inner_types if not inner.is_none_type
        ]
        return schema

    # Handle list/array types
    if field.is_non_string_sequence:
        schema["type"] = "array"
        if field.inner_types:
            schema["items"] = field_definition_to_json_schema(field.inner_types[0])
        else:
            schema["items"] = {"type": "string"}  # Default to string items

        # Apply array-specific constraints from extra
        if "min_items" in field.extra:
            schema["minItems"] = field.extra["min_items"]
        if "max_items" in field.extra:
            schema["maxItems"] = field.extra["max_items"]
        # Note: min_length/max_length on arrays should also be treated as minItems/maxItems
        if "min_length" in field.extra and "min_items" not in field.extra:
            schema["minItems"] = field.extra["min_length"]
        if "max_length" in field.extra and "max_items" not in field.extra:
            schema["maxItems"] = field.extra["max_length"]

        return schema

    # Handle literal types
    if field.is_literal:
        schema["enum"] = list(field.args)
        return schema

    # Handle enum types
    if field.is_subclass_of(Enum):
        try:
            # Get enum values
            enum_class = field.annotation
            schema["enum"] = [item.value for item in enum_class]
        except Exception:
            # If we can't get enum values, just use string type
            schema["type"] = "string"
        return schema

    # Convert base type to JSON Schema
    schema.update(_annotation_to_json_schema(field.annotation))

    # Apply constraints from field.extra
    _apply_constraints(schema, field.extra)

    return schema


def _annotation_to_json_schema(python_type: Any) -> dict[str, Any]:
    """Convert a Python type annotation to basic JSON Schema.

    Args:
        python_type: The Python type

    Returns:
        Basic JSON Schema dict
    """
    # Handle None type
    if python_type is type(None):
        return {"type": "null"}

    # Handle string types
    if python_type is str or python_type == "str":
        return {"type": "string"}

    # Handle integer types
    if python_type is int or python_type == "int":
        return {"type": "integer"}

    # Handle float types
    if python_type is float or python_type == "float":
        return {"type": "number"}

    # Handle boolean types
    if python_type is bool or python_type == "bool":
        return {"type": "boolean"}

    # Handle date/datetime/time/timedelta with format
    if python_type is datetime:
        return {"type": "string", "format": "date-time"}

    if python_type is date:
        return {"type": "string", "format": "date"}

    if python_type is time:
        return {"type": "string", "format": "time"}

    if python_type is timedelta:
        return {"type": "string", "format": "duration"}

    # Handle UUID with format
    if python_type is UUID:
        return {"type": "string", "format": "uuid"}

    # Handle Path types as strings
    if python_type in (Path, PurePath) or (
        hasattr(python_type, "__origin__") and python_type.__origin__ in (Path, PurePath)
    ):
        return {"type": "string"}

    # Handle dict/object types
    if python_type is dict:
        return {"type": "object"}

    # Handle Any
    if str(python_type) == "typing.Any" or python_type is Any:
        return {}  # No type constraint for Any

    # Default to string for unknown types
    return {"type": "string"}


def _apply_constraints(schema: dict[str, Any], constraints: dict[str, Any]) -> None:
    """Apply validation constraints to a JSON Schema.

    Modifies the schema dict in place.

    Args:
        schema: The JSON Schema dict to modify
        constraints: Dictionary of constraint names to values
    """
    # String constraints
    if "min_length" in constraints:
        schema["minLength"] = constraints["min_length"]
    if "max_length" in constraints:
        schema["maxLength"] = constraints["max_length"]
    if "pattern" in constraints:
        schema["pattern"] = constraints["pattern"]

    # Numeric constraints
    if "gt" in constraints:
        schema["exclusiveMinimum"] = constraints["gt"]
    if "ge" in constraints:
        schema["minimum"] = constraints["ge"]
    if "lt" in constraints:
        schema["exclusiveMaximum"] = constraints["lt"]
    if "le" in constraints:
        schema["maximum"] = constraints["le"]
    if "multiple_of" in constraints:
        schema["multipleOf"] = constraints["multiple_of"]

    # Case constraints (pattern-based)
    if constraints.get("lower_case"):
        schema["pattern"] = "^[a-z]*$"
    if constraints.get("upper_case"):
        schema["pattern"] = "^[A-Z]*$"
