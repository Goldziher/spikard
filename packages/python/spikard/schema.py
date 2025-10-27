"""Schema extraction utilities."""

import inspect
from dataclasses import is_dataclass
from typing import Any, Callable, Optional, get_type_hints


def extract_schemas(
    func: Callable,
) -> tuple[Optional[dict[str, Any]], Optional[dict[str, Any]]]:
    """Extract request and response schemas from function signature.

    Args:
        func: Handler function

    Returns:
        Tuple of (request_schema, response_schema)
    """
    try:
        type_hints = get_type_hints(func)
    except Exception:
        return None, None

    # Extract request schema from first parameter (after self/cls if present)
    sig = inspect.signature(func)
    params = list(sig.parameters.values())

    request_schema = None
    if params:
        # Skip self/cls
        first_param = params[0]
        if first_param.name not in ("self", "cls"):
            param_type = type_hints.get(first_param.name)
            if param_type:
                request_schema = extract_json_schema(param_type)

    # Extract response schema from return annotation
    response_schema = None
    return_type = type_hints.get("return")
    if return_type:
        response_schema = extract_json_schema(return_type)

    return request_schema, response_schema


def extract_json_schema(schema_source: Any) -> Optional[dict[str, Any]]:
    """Extract JSON Schema from various sources.

    Supports:
    - Pydantic v2 models (model_json_schema)
    - msgspec structs (__json_schema__)
    - attrs classes
    - dataclasses
    - Raw dict (assumed to be JSON Schema)

    Args:
        schema_source: Type or dict to extract schema from

    Returns:
        JSON Schema dict, or None if extraction fails
    """
    # Already a dict - assume it's JSON Schema
    if isinstance(schema_source, dict):
        return schema_source

    # Handle None, primitives
    if schema_source is None or schema_source in (int, str, float, bool):
        return None

    try:
        # Pydantic v2
        if hasattr(schema_source, "model_json_schema"):
            return schema_source.model_json_schema()

        # msgspec
        if hasattr(schema_source, "__json_schema__"):
            return schema_source.__json_schema__()

        # attrs
        if hasattr(schema_source, "__attrs_attrs__"):
            return _attrs_to_json_schema(schema_source)

        # Dataclasses
        if is_dataclass(schema_source):
            return _dataclass_to_json_schema(schema_source)

    except Exception:
        pass

    return None


def _dataclass_to_json_schema(cls: type) -> dict[str, Any]:
    """Convert dataclass to JSON Schema.

    Args:
        cls: Dataclass type

    Returns:
        JSON Schema dict
    """
    import dataclasses

    fields = dataclasses.fields(cls)
    properties = {}
    required = []

    for field in fields:
        field_name = field.name
        field_type = field.type

        # Simple type mapping
        json_type = _python_type_to_json_type(field_type)
        properties[field_name] = {"type": json_type}

        # Check if required
        if field.default is dataclasses.MISSING and field.default_factory is dataclasses.MISSING:
            required.append(field_name)

    schema = {
        "type": "object",
        "properties": properties,
    }

    if required:
        schema["required"] = required

    return schema


def _attrs_to_json_schema(cls: type) -> dict[str, Any]:
    """Convert attrs class to JSON Schema.

    Args:
        cls: attrs class type

    Returns:
        JSON Schema dict
    """
    import attr

    fields = attr.fields(cls)
    properties = {}
    required = []

    for field in fields:
        field_name = field.name
        field_type = field.type

        json_type = _python_type_to_json_type(field_type)
        properties[field_name] = {"type": json_type}

        if field.default is attr.NOTHING:
            required.append(field_name)

    schema = {
        "type": "object",
        "properties": properties,
    }

    if required:
        schema["required"] = required

    return schema


def _python_type_to_json_type(py_type: Any) -> str:
    """Map Python type to JSON Schema type.

    Args:
        py_type: Python type

    Returns:
        JSON Schema type string
    """
    type_map = {
        int: "integer",
        str: "string",
        float: "number",
        bool: "boolean",
        list: "array",
        dict: "object",
    }

    return type_map.get(py_type, "string")
