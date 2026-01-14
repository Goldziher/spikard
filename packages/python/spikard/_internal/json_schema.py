"""JSON Schema generation from FieldDefinition.

This module converts the universal FieldDefinition IR into JSON Schema format
that can be passed to Rust for validation and caching.
"""

from datetime import date, datetime, time, timedelta
from enum import Enum
from pathlib import Path, PurePath
from typing import Any
from uuid import UUID

from spikard._internal.field_definition import FieldDefinition  # noqa: TC001

__all__ = ("field_definition_to_json_schema",)


def field_definition_to_json_schema(field: FieldDefinition) -> dict[str, Any]:
    """Convert a FieldDefinition to JSON Schema.

    Args:
        field: The FieldDefinition to convert

    Returns:
        JSON Schema dictionary
    """
    schema: dict[str, Any] = {}

    if field.is_optional:
        non_none_types = [arg for arg in field.args if arg is not type(None)]
        if len(non_none_types) == 1:
            inner_field = (
                field.inner_types[0]
                if field.inner_types and field.inner_types[0].annotation is not type(None)
                else None
            )
            if inner_field:
                schema = field_definition_to_json_schema(inner_field)
            else:
                schema = _annotation_to_json_schema(non_none_types[0])

            _apply_constraints(schema, field.extra)

            return schema

    if field.is_union and not field.is_optional:
        schema["anyOf"] = [
            field_definition_to_json_schema(inner) for inner in field.inner_types if not inner.is_none_type
        ]
        return schema

    if field.is_non_string_sequence:
        schema["type"] = "array"
        if field.inner_types:
            schema["items"] = field_definition_to_json_schema(field.inner_types[0])
        else:
            schema["items"] = {"type": "string"}

        if "min_items" in field.extra:
            schema["minItems"] = field.extra["min_items"]
        if "max_items" in field.extra:
            schema["maxItems"] = field.extra["max_items"]
        if "min_length" in field.extra and "min_items" not in field.extra:
            schema["minItems"] = field.extra["min_length"]
        if "max_length" in field.extra and "max_items" not in field.extra:
            schema["maxItems"] = field.extra["max_length"]

        return schema

    if field.is_literal:
        schema["enum"] = list(field.args)
        return schema

    if field.is_subclass_of(Enum):
        try:
            enum_class = field.annotation
            schema["enum"] = [item.value for item in enum_class]
        except (AttributeError, TypeError, ValueError):
            schema["type"] = "string"
        return schema

    schema.update(_annotation_to_json_schema(field.annotation))

    _apply_constraints(schema, field.extra)

    return schema


def _annotation_to_json_schema(python_type: Any) -> dict[str, Any]:
    """Convert a Python type annotation to basic JSON Schema.

    Args:
        python_type: The Python type

    Returns:
        Basic JSON Schema dict
    """
    schema: dict[str, Any] | None = None

    if python_type is type(None):
        schema = {"type": "null"}
    elif python_type in (str, "str"):
        schema = {"type": "string"}
    elif python_type in (int, "int"):
        schema = {"type": "integer"}
    elif python_type in (float, "float"):
        schema = {"type": "number"}
    elif python_type in (bool, "bool"):
        schema = {"type": "boolean"}
    elif python_type is datetime:
        schema = {"type": "string", "format": "date-time"}
    elif python_type is date:
        schema = {"type": "string", "format": "date"}
    elif python_type is time:
        schema = {"type": "string", "format": "time"}
    elif python_type is timedelta:
        schema = {"type": "string", "format": "duration"}
    elif python_type is UUID:
        schema = {"type": "string", "format": "uuid"}
    elif python_type in (Path, PurePath) or (
        hasattr(python_type, "__origin__") and python_type.__origin__ in (Path, PurePath)
    ):
        schema = {"type": "string"}
    elif python_type is dict:
        schema = {"type": "object"}
    elif str(python_type) == "typing.Any" or python_type is Any:
        schema = {}
    else:
        schema = {"type": "string"}

    return schema


def _apply_constraints(schema: dict[str, Any], constraints: dict[str, Any]) -> None:
    """Apply validation constraints to a JSON Schema.

    Modifies the schema dict in place.

    Args:
        schema: The JSON Schema dict to modify
        constraints: Dictionary of constraint names to values
    """
    if "source" in constraints:
        schema["source"] = constraints["source"]

    if "min_length" in constraints:
        schema["minLength"] = constraints["min_length"]
    if "max_length" in constraints:
        schema["maxLength"] = constraints["max_length"]
    if "pattern" in constraints:
        schema["pattern"] = constraints["pattern"]

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

    if constraints.get("lower_case"):
        schema["pattern"] = "^[a-z]*$"
    if constraints.get("upper_case"):
        schema["pattern"] = "^[A-Z]*$"
