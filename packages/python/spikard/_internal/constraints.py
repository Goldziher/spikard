"""Extract JSON Schema constraints from type annotations.

This module contains adapted code from Litestar for extracting validation
constraints from Python type annotations, including Pydantic Field objects
and Annotated types.

Original inspiration: https://github.com/litestar-org/litestar
License: MIT (see ATTRIBUTIONS.md in project root)
"""

from typing import Any, get_args, get_origin


def _extract_from_metadata_item(item: Any) -> dict[str, Any]:
    """Extract constraints from a single metadata item.

    Args:
        item: Metadata item from Pydantic FieldInfo

    Returns:
        Dictionary of constraint names to values
    """
    constraints: dict[str, Any] = {}
    item_type = type(item).__name__

    if item_type == "MinLen":
        constraints["min_length"] = item.min_length
    elif item_type == "MaxLen":
        constraints["max_length"] = item.max_length
    elif item_type == "Gt":
        constraints["gt"] = item.gt
    elif item_type == "Ge":
        constraints["ge"] = item.ge
    elif item_type == "Lt":
        constraints["lt"] = item.lt
    elif item_type == "Le":
        constraints["le"] = item.le
    elif item_type == "MultipleOf":
        constraints["multiple_of"] = item.multiple_of
    elif item_type == "_PydanticGeneralMetadata":
        if hasattr(item, "pattern") and item.pattern is not None:
            constraints["pattern"] = item.pattern
        if hasattr(item, "min_length") and item.min_length is not None:
            constraints["min_length"] = item.min_length
        if hasattr(item, "max_length") and item.max_length is not None:
            constraints["max_length"] = item.max_length

    return constraints


def extract_constraints_from_field(field_info: Any) -> dict[str, Any]:
    """Extract JSON Schema constraints from a Pydantic FieldInfo object.

    Args:
        field_info: A Pydantic FieldInfo object (from Field())

    Returns:
        Dictionary of JSON Schema constraint properties
    """
    constraints: dict[str, Any] = {}

    if not hasattr(field_info, "metadata"):
        return constraints

    if (
        hasattr(field_info, "json_schema_extra")
        and isinstance(field_info.json_schema_extra, dict)
        and "source" in field_info.json_schema_extra
    ):
        constraints["source"] = field_info.json_schema_extra["source"]

    for item in field_info.metadata:
        item_constraints = _extract_from_metadata_item(item)
        constraints.update(item_constraints)

    return constraints


def extract_constraints_from_annotated(annotation: Any) -> dict[str, Any]:
    """Extract constraints from an Annotated type.

    Args:
        annotation: A type annotation, possibly Annotated

    Returns:
        Dictionary of JSON Schema constraint properties
    """
    constraints = {}

    origin = get_origin(annotation)
    if origin is not type(None):
        args = get_args(annotation)
        if len(args) > 1:
            for item in args[1:]:
                item_type = type(item).__name__

                if item_type == "MinLen":
                    constraints["minLength"] = item.min_length
                elif item_type == "MaxLen":
                    constraints["maxLength"] = item.max_length
                elif item_type == "Gt":
                    constraints["exclusiveMinimum"] = item.gt
                elif item_type == "Ge":
                    constraints["minimum"] = item.ge
                elif item_type == "Lt":
                    constraints["exclusiveMaximum"] = item.lt
                elif item_type == "Le":
                    constraints["maximum"] = item.le
                elif item_type == "MultipleOf":
                    constraints["multipleOf"] = item.multiple_of

    return constraints
