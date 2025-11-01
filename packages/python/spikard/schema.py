"""Schema extraction utilities."""

import dataclasses
import inspect
from collections.abc import Callable
from typing import Any, Protocol, get_type_hints, runtime_checkable

import msgspec
from pydantic import TypeAdapter


@runtime_checkable
class PydanticV2Model(Protocol):
    """Protocol for Pydantic v2 models."""

    @classmethod
    def model_json_schema(cls, **kwargs: Any) -> dict[str, Any]:
        """Generate JSON Schema for Pydantic v2 model."""
        ...


@runtime_checkable
class PydanticV1Model(Protocol):
    """Protocol for Pydantic v1 models."""

    @classmethod
    def schema(cls, **kwargs: Any) -> dict[str, Any]:
        """Generate JSON Schema for Pydantic v1 model."""
        ...


def is_typeddict(obj: Any) -> bool:
    """Check if an object is a TypedDict class.

    TypedDict classes have __required_keys__ and __optional_keys__ attributes.

    Args:
        obj: Object to check

    Returns:
        True if obj is a TypedDict class
    """
    return isinstance(obj, type) and hasattr(obj, "__required_keys__") and hasattr(obj, "__optional_keys__")


def is_json_schema_dict(obj: Any) -> bool:
    """Check if an object is a plain JSON Schema dictionary.

    Detects dicts that contain common JSON Schema keys.

    Args:
        obj: Object to check

    Returns:
        True if obj appears to be a JSON Schema dict
    """
    if not isinstance(obj, dict):
        return False

    # Common JSON Schema keys
    json_schema_keys = {
        "type",
        "properties",
        "items",
        "additionalProperties",
        "$schema",
        "$ref",
        "$defs",
        "required",
        "enum",
        "minimum",
        "maximum",
        "minLength",
        "maxLength",
        "pattern",
        "format",
        "anyOf",
        "oneOf",
        "allOf",
    }

    return bool(set(obj.keys()) & json_schema_keys)


def resolve_msgspec_ref(schema: dict[str, Any]) -> dict[str, Any]:
    """Resolve $ref in msgspec schemas.

    msgspec returns schemas with $ref and $defs. This function resolves
    the top-level $ref and returns a clean schema.

    Args:
        schema: msgspec schema with potential $ref

    Returns:
        Resolved schema
    """
    if "$ref" in schema and "$defs" in schema:
        ref_path = schema["$ref"]
        if ref_path.startswith("#/$defs/"):
            ref_name = ref_path.split("/")[-1]
            if ref_name in schema["$defs"]:
                # Get the referenced schema
                resolved: dict[str, Any] = schema["$defs"][ref_name].copy()

                # Preserve other definitions
                other_defs = {k: v for k, v in schema["$defs"].items() if k != ref_name}
                if other_defs:
                    resolved["$defs"] = other_defs

                return resolved

    return schema


def extract_schemas(
    func: Callable[..., Any],
) -> tuple[dict[str, Any] | None, dict[str, Any] | None]:
    """Extract request and response schemas from function signature.

    Args:
        func: Handler function

    Returns:
        Tuple of (request_schema, response_schema)
    """
    try:
        type_hints = get_type_hints(func)
    except (AttributeError, NameError, TypeError, ValueError):
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


def extract_json_schema(schema_source: Any) -> dict[str, Any] | None:  # noqa: C901, PLR0912, PLR0915
    """Extract JSON Schema from various Python schema sources.

    Supports multiple schema formats through duck typing:
    - Plain JSON Schema dicts (passed through as-is)
    - TypedDict (via msgspec)
    - Pydantic v2 models (via model_json_schema())
    - Pydantic v1 models (via schema())
    - msgspec.Struct (via msgspec.json.schema())

    Logic flow:
    1. Check if plain JSON Schema dict → return as-is
    2. Check if TypedDict → use msgspec.json.schema() with ref resolution
    3. Check for Pydantic v2 → call model_json_schema()
    4. Check for Pydantic v1 → call schema()
    5. Try msgspec.json.schema() as fallback (for msgspec.Struct)
    6. Return None if unsupported

    Args:
        schema_source: Type or dict to extract schema from

    Returns:
        JSON Schema dict, or None if extraction fails

    Raises:
        TypeError: If schema_source type is not supported
    """
    # Handle None, primitives
    if schema_source is None or schema_source in (int, str, float, bool):
        return None

    # 1. Check if plain JSON Schema dict (not TypedDict)
    if isinstance(schema_source, dict) and is_json_schema_dict(schema_source):
        return dict(schema_source)

    # 2. Check if TypedDict
    if is_typeddict(schema_source):
        try:
            schema = msgspec.json.schema(schema_source)
            return resolve_msgspec_ref(schema)
        except Exception as e:
            raise TypeError(f"Failed to extract schema from TypedDict {schema_source.__name__}: {e}") from e

    # 3. Check for Pydantic v2 (via Protocol)
    if isinstance(schema_source, type) and isinstance(schema_source, PydanticV2Model):
        try:
            return schema_source.model_json_schema()
        except Exception as e:
            raise TypeError(f"Failed to extract schema from Pydantic v2 model {schema_source.__name__}: {e}") from e

    # 4. Check for Pydantic v1 (via Protocol)
    if isinstance(schema_source, type) and isinstance(schema_source, PydanticV1Model):
        try:
            return schema_source.schema()
        except Exception as e:
            raise TypeError(f"Failed to extract schema from Pydantic v1 model {schema_source.__name__}: {e}") from e

    # 5. Check for dataclass
    if isinstance(schema_source, type) and dataclasses.is_dataclass(schema_source):
        try:
            # Use Pydantic TypeAdapter for dataclass schema extraction
            adapter = TypeAdapter(schema_source)
            return adapter.json_schema()
        except Exception as e:
            raise TypeError(f"Failed to extract schema from dataclass {schema_source.__name__}: {e}") from e

    # 6. Check for NamedTuple (check for _fields attribute)
    if isinstance(schema_source, type) and hasattr(schema_source, "_fields"):
        try:
            # NamedTuple schema needs to be built manually as an object schema
            # (msgspec and Pydantic treat it as array, but we want object semantics for HTTP)
            type_hints = get_type_hints(schema_source)
            properties = {}
            # Only fields without defaults are required
            field_defaults = getattr(schema_source, "_field_defaults", {})
            required = [f for f in schema_source._fields if f not in field_defaults]

            for field_name in schema_source._fields:
                field_type = type_hints.get(field_name)
                if field_type:
                    # Get basic type mapping
                    if field_type is str:
                        properties[field_name] = {"type": "string"}
                    elif field_type is int:
                        properties[field_name] = {"type": "integer"}
                    elif field_type is float:
                        properties[field_name] = {"type": "number"}
                    elif field_type is bool:
                        properties[field_name] = {"type": "boolean"}
                    else:
                        # For complex types, recursively extract schema
                        field_schema = extract_json_schema(field_type)
                        if field_schema:
                            properties[field_name] = field_schema
                        else:
                            # Fallback to generic object
                            properties[field_name] = {}
                else:
                    properties[field_name] = {}

            return {
                "type": "object",
                "properties": properties,
                "required": required,
                "title": schema_source.__name__,
            }
        except Exception as e:
            raise TypeError(f"Failed to extract schema from NamedTuple {schema_source.__name__}: {e}") from e

    # 7. Try msgspec.json.schema() as fallback (for msgspec.Struct, attrs, etc.)
    try:
        schema = msgspec.json.schema(schema_source)
        return resolve_msgspec_ref(schema)
    except (TypeError, KeyError, AttributeError) as e:
        # msgspec doesn't support this type
        raise TypeError(
            f"Unsupported schema type: {type(schema_source).__name__}. "
            f"Supported types: Pydantic v1/v2 models, msgspec.Struct, TypedDict, dataclass, NamedTuple, or plain JSON Schema dict. "
            f"Error: {e}"
        ) from e

    return None
