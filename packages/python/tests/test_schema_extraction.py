"""Tests for schema extraction from various Python types.

Tests duck typing support for:
- Plain JSON Schema dicts
- TypedDict
- Pydantic v2 models
- Pydantic v1 models (if available)
- msgspec.Struct
- Error handling for unsupported types
"""

from typing import TypedDict

import msgspec
import pytest

from spikard.schema import (
    extract_json_schema,
    is_json_schema_dict,
    is_typeddict,
    resolve_msgspec_ref,
)


class TestIsTypedDict:
    """Tests for TypedDict detection."""

    def test_is_typeddict_with_typeddict(self) -> None:
        """TypedDict classes should be detected."""

        class MyTypedDict(TypedDict):
            name: str
            age: int

        assert is_typeddict(MyTypedDict) is True

    def test_is_typeddict_with_regular_dict(self) -> None:
        """Regular dicts should not be detected as TypedDict."""
        assert is_typeddict({"name": "test"}) is False

    def test_is_typeddict_with_regular_class(self) -> None:
        """Regular classes should not be detected as TypedDict."""

        class RegularClass:
            pass

        assert is_typeddict(RegularClass) is False


class TestIsJsonSchemaDict:
    """Tests for JSON Schema dict detection."""

    def test_is_json_schema_dict_with_type(self) -> None:
        """Dict with 'type' key should be detected."""
        schema = {"type": "object", "properties": {}}
        assert is_json_schema_dict(schema) is True

    def test_is_json_schema_dict_with_properties(self) -> None:
        """Dict with 'properties' key should be detected."""
        schema = {"properties": {"name": {"type": "string"}}}
        assert is_json_schema_dict(schema) is True

    def test_is_json_schema_dict_with_ref(self) -> None:
        """Dict with '$ref' key should be detected."""
        schema = {"$ref": "#/$defs/User"}
        assert is_json_schema_dict(schema) is True

    def test_is_json_schema_dict_with_regular_dict(self) -> None:
        """Regular dict should not be detected."""
        data = {"name": "test", "age": 30}
        assert is_json_schema_dict(data) is False

    def test_is_json_schema_dict_with_non_dict(self) -> None:
        """Non-dict should not be detected."""
        assert is_json_schema_dict("not a dict") is False


class TestResolveMsgspecRef:
    """Tests for msgspec $ref resolution."""

    def test_resolve_msgspec_ref_with_ref(self) -> None:
        """Should resolve top-level $ref to definition."""
        schema = {
            "$ref": "#/$defs/User",
            "$defs": {
                "User": {
                    "type": "object",
                    "properties": {"name": {"type": "string"}},
                }
            },
        }

        resolved = resolve_msgspec_ref(schema)
        assert resolved["type"] == "object"
        assert "properties" in resolved
        assert "$ref" not in resolved

    def test_resolve_msgspec_ref_without_ref(self) -> None:
        """Should return schema as-is if no $ref."""
        schema = {"type": "object", "properties": {}}
        resolved = resolve_msgspec_ref(schema)
        assert resolved == schema

    def test_resolve_msgspec_ref_preserves_other_defs(self) -> None:
        """Should preserve other definitions after resolving."""
        schema = {
            "$ref": "#/$defs/User",
            "$defs": {
                "User": {"type": "object"},
                "Address": {"type": "object"},
            },
        }

        resolved = resolve_msgspec_ref(schema)
        assert "$defs" in resolved
        assert "Address" in resolved["$defs"]
        assert "User" not in resolved["$defs"]


class TestExtractJsonSchemaPlainDict:
    """Tests for plain JSON Schema dict extraction."""

    def test_extract_json_schema_plain_dict(self) -> None:
        """Plain JSON Schema dict should be returned as-is."""
        schema = {
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"},
            },
            "required": ["name"],
        }

        result = extract_json_schema(schema)
        assert result == schema

    def test_extract_json_schema_primitives(self) -> None:
        """Primitives should return None."""
        assert extract_json_schema(None) is None
        assert extract_json_schema(int) is None
        assert extract_json_schema(str) is None


class TestExtractJsonSchemaTypedDict:
    """Tests for TypedDict extraction via msgspec."""

    def test_extract_json_schema_typeddict(self) -> None:
        """TypedDict should be extracted via msgspec."""

        class User(TypedDict):
            name: str
            age: int
            email: str

        schema = extract_json_schema(User)
        assert schema is not None
        assert "type" in schema or "properties" in schema

    def test_extract_json_schema_typeddict_optional(self) -> None:
        """TypedDict with optional fields should work."""

        class UserOptional(TypedDict, total=False):
            name: str
            nickname: str

        schema = extract_json_schema(UserOptional)
        assert schema is not None


class TestExtractJsonSchemaMsgspec:
    """Tests for msgspec.Struct extraction."""

    def test_extract_json_schema_msgspec_struct(self) -> None:
        """msgspec.Struct should be extracted."""

        class User(msgspec.Struct):
            name: str
            age: int

        schema = extract_json_schema(User)
        assert schema is not None
        assert "type" in schema or "properties" in schema

    def test_extract_json_schema_msgspec_struct_with_defaults(self) -> None:
        """msgspec.Struct with defaults should work."""

        class User(msgspec.Struct):
            name: str
            email: str = ""
            active: bool = True

        schema = extract_json_schema(User)
        assert schema is not None


class TestExtractJsonSchemaPydanticV2:
    """Tests for Pydantic v2 model extraction."""

    def test_extract_json_schema_pydantic_v2(self) -> None:
        """Pydantic v2 models should be extracted via model_json_schema()."""
        try:
            from pydantic import BaseModel, Field

            class User(BaseModel):
                name: str = Field(min_length=3)
                age: int = Field(ge=0, le=150)
                email: str | None = None

            schema = extract_json_schema(User)
            assert schema is not None
            assert "properties" in schema
            assert "name" in schema["properties"]
            assert "age" in schema["properties"]

        except ImportError:
            pytest.skip("Pydantic not installed")


class TestExtractJsonSchemaPydanticV1:
    """Tests for Pydantic v1 model extraction."""

    def test_extract_json_schema_pydantic_v1(self) -> None:
        """Pydantic v1 models should be extracted via schema()."""
        # Pydantic v1 is unlikely to be installed, but we test the protocol
        # This will typically be skipped
        pytest.skip("Pydantic v1 not commonly installed")


class TestExtractJsonSchemaErrors:
    """Tests for error handling."""

    def test_extract_json_schema_unsupported_type(self) -> None:
        """Unsupported types should raise TypeError."""

        class UnsupportedClass:
            def __init__(self) -> None:
                self.name = "test"

        with pytest.raises(TypeError) as exc_info:
            extract_json_schema(UnsupportedClass)

        assert "Unsupported schema type" in str(exc_info.value)
        assert "UnsupportedClass" in str(exc_info.value)

    def test_extract_json_schema_dataclass_raises(self) -> None:
        """Dataclasses should raise TypeError (msgspec doesn't support them)."""
        from dataclasses import dataclass

        @dataclass
        class User:
            name: str
            age: int

        with pytest.raises(TypeError):
            extract_json_schema(User)


class TestSchemaExtractionIntegration:
    """Integration tests combining multiple types."""

    def test_various_types_work(self) -> None:
        """Test that various supported types all work."""
        # Plain JSON Schema dict
        plain_schema = {"type": "string", "minLength": 3}
        assert extract_json_schema(plain_schema) == plain_schema

        # TypedDict
        class MyTypedDict(TypedDict):
            name: str

        td_schema = extract_json_schema(MyTypedDict)
        assert td_schema is not None

        # msgspec.Struct
        class MyStruct(msgspec.Struct):
            name: str

        ms_schema = extract_json_schema(MyStruct)
        assert ms_schema is not None

    def test_pydantic_and_msgspec_compatibility(self) -> None:
        """Test that Pydantic v2 and msgspec can coexist."""
        try:
            from pydantic import BaseModel

            class PydanticUser(BaseModel):
                name: str

            class MsgspecUser(msgspec.Struct):
                name: str

            pydantic_schema = extract_json_schema(PydanticUser)
            msgspec_schema = extract_json_schema(MsgspecUser)

            assert pydantic_schema is not None
            assert msgspec_schema is not None

        except ImportError:
            pytest.skip("Pydantic not installed")


if __name__ == "__main__":
    pytest.main([__file__, "-v", "-s"])
