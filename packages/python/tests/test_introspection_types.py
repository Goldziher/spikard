"""Comprehensive tests for type introspection covering all Python type systems.

This test suite validates that our FieldDefinition-based introspection system correctly
handles all supported Python type systems and generates valid JSON schemas.
"""

from dataclasses import dataclass
from enum import Enum
from typing import Annotated, NamedTuple, TypedDict
from uuid import UUID

import jsonschema
import pytest
from pydantic import Field

# Import msgspec if available
try:
    from msgspec import Struct

    HAS_MSGSPEC = True
except ImportError:
    HAS_MSGSPEC = False

from spikard.introspection import extract_parameter_schema


class TestBasicTypes:
    """Test native Python types."""

    def test_basic_string(self) -> None:
        """Test basic string parameter."""

        def func(name: str) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None
        assert schema["properties"]["name"] == {"type": "string", "source": "query"}
        assert "name" in schema["required"]

    def test_basic_int(self) -> None:
        """Test basic integer parameter."""

        def func(age: int) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None
        assert schema["properties"]["age"] == {"type": "integer", "source": "query"}
        assert "age" in schema["required"]

    def test_basic_float(self) -> None:
        """Test basic float parameter."""

        def func(score: float) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None
        assert schema["properties"]["score"] == {"type": "number", "source": "query"}
        assert "score" in schema["required"]

    def test_basic_bool(self) -> None:
        """Test basic boolean parameter."""

        def func(active: bool) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None
        assert schema["properties"]["active"] == {"type": "boolean", "source": "query"}
        assert "active" in schema["required"]

    def test_optional_types(self) -> None:
        """Test optional types."""

        def func(name: str, nickname: str | None = None) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None
        assert "name" in schema["required"]
        assert "nickname" not in schema["required"]
        assert schema["properties"]["nickname"]["type"] == "string"

    def test_list_types(self) -> None:
        """Test list/array types."""

        def func(tags: list[str], scores: list[int]) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None
        assert schema["properties"]["tags"] == {
            "type": "array",
            "items": {"type": "string"},
            "source": "query",
        }
        assert schema["properties"]["scores"] == {
            "type": "array",
            "items": {"type": "integer"},
            "source": "query",
        }


class TestPydanticTypes:
    """Test Pydantic Field() with constraints."""

    def test_field_required(self) -> None:
        """Test Pydantic Field(...) marks parameter as required."""

        def func(username: str = Field(...)) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None
        assert "username" in schema["required"]

    def test_field_with_constraints(self) -> None:
        """Test Pydantic Field with validation constraints."""

        def func(
            username: str = Field(..., min_length=3, max_length=50, pattern=r"^[a-zA-Z0-9_]+$"),
            email: str = Field(..., pattern=r"^[^@]+@[^@]+\.[^@]+$"),
        ) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None

        # Check username constraints
        username_schema = schema["properties"]["username"]
        assert username_schema["type"] == "string"
        assert username_schema.get("minLength") == 3
        assert username_schema.get("maxLength") == 50
        assert username_schema.get("pattern") == r"^[a-zA-Z0-9_]+$"

        # Check email constraints
        email_schema = schema["properties"]["email"]
        assert email_schema["type"] == "string"
        assert email_schema.get("pattern") == r"^[^@]+@[^@]+\.[^@]+$"

        # Both should be required
        assert "username" in schema["required"]
        assert "email" in schema["required"]

    def test_field_optional_with_default(self) -> None:
        """Test Pydantic Field with default value."""

        def func(role: str = Field(default="user")) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None
        assert "role" not in schema["required"]


class TestAnnotatedTypes:
    """Test Annotated types with constraints."""

    def test_annotated_with_annotated_types(self) -> None:
        """Test Annotated with annotated_types constraints."""
        try:
            from annotated_types import Ge, Le, MaxLen, MinLen

            def func(
                age: Annotated[int, Ge(0), Le(150)],
                tags: Annotated[list[str], MinLen(1), MaxLen(10)],
            ) -> None:
                pass

            schema = extract_parameter_schema(func)
            assert schema is not None

            # Check age constraints
            age_schema = schema["properties"]["age"]
            assert age_schema["type"] == "integer"
            assert age_schema.get("minimum") == 0
            assert age_schema.get("maximum") == 150

            # Check tags constraints
            tags_schema = schema["properties"]["tags"]
            assert tags_schema["type"] == "array"
            assert tags_schema.get("minItems") == 1
            assert tags_schema.get("maxItems") == 10

        except ImportError:
            pytest.skip("annotated_types not installed")


class TestDataclasses:
    """Test dataclass parameters."""

    def test_dataclass_parameter(self) -> None:
        """Test function with dataclass parameter."""

        @dataclass
        class UserData:
            name: str
            age: int
            email: str | None = None

        def func(user: UserData) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None
        # Dataclass should be treated as object type
        # The specific behavior depends on how we want to handle complex types


class TestTypedDict:
    """Test TypedDict parameters."""

    def test_typed_dict_parameter(self) -> None:
        """Test function with TypedDict parameter."""

        class UserDict(TypedDict):
            name: str
            age: int
            email: str | None

        def func(user: UserDict) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None
        # TypedDict should be treated as object type


class TestNamedTuple:
    """Test NamedTuple parameters."""

    def test_named_tuple_parameter(self) -> None:
        """Test function with NamedTuple parameter."""

        class Point(NamedTuple):
            x: int
            y: int

        def func(point: Point) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None


@pytest.mark.skipif(not HAS_MSGSPEC, reason="msgspec not installed")
class TestMsgspecTypes:
    """Test msgspec Struct types."""

    def test_msgspec_struct(self) -> None:
        """Test msgspec Struct parameter."""
        from msgspec import Struct

        class User(Struct):
            name: str
            age: int

        def func(user: User) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None


class TestEnumTypes:
    """Test Enum types."""

    def test_enum_parameter(self) -> None:
        """Test enum parameter generates correct schema."""

        class Status(Enum):
            ACTIVE = "active"
            INACTIVE = "inactive"
            PENDING = "pending"

        def func(status: Status) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None
        status_schema = schema["properties"]["status"]
        assert "enum" in status_schema
        assert set(status_schema["enum"]) == {"active", "inactive", "pending"}


class TestSpecialTypes:
    """Test special Python types (UUID, date, datetime, etc.)."""

    def test_uuid_type(self) -> None:
        """Test UUID type generates format constraint."""
        from datetime import date, datetime

        def func(id: UUID, created: datetime, birth_date: date) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None

        # UUID should have format
        assert schema["properties"]["id"]["type"] == "string"
        assert schema["properties"]["id"].get("format") == "uuid"

        # Datetime should have format
        assert schema["properties"]["created"]["type"] == "string"
        assert schema["properties"]["created"].get("format") == "date-time"

        # Date should have format
        assert schema["properties"]["birth_date"]["type"] == "string"
        assert schema["properties"]["birth_date"].get("format") == "date"


class TestMixedScenarios:
    """Test complex scenarios with mixed types."""

    def test_mixed_required_and_optional(self) -> None:
        """Test mix of required and optional parameters."""

        def func(
            name: str,
            age: int | None = None,
            role: str = "user",
            tags: list[str] | None = None,  # type: ignore[assignment]
        ) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None
        assert "name" in schema["required"]
        assert "age" not in schema["required"]
        assert "role" not in schema["required"]
        assert "tags" not in schema["required"]

    def test_no_parameters(self) -> None:
        """Test function with no parameters."""

        def func() -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is None


class TestJSONSchemaValidation:
    """Validate that generated schemas are valid JSON Schema."""

    def test_generated_schema_is_valid_json_schema(self) -> None:
        """Test that generated schemas are valid according to JSON Schema spec."""

        def func(
            name: str,
            age: int,
            email: str | None = None,
            tags: list[str] | None = None,  # type: ignore[assignment]
        ) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None

        # Remove Spikard-specific keys that aren't part of JSON Schema
        cleaned_schema = schema.copy()
        for prop_schema in cleaned_schema["properties"].values():
            prop_schema.pop("source", None)

        # Validate against JSON Schema meta-schema
        try:
            # This will raise an exception if the schema is invalid
            jsonschema.Draft7Validator.check_schema(cleaned_schema)
        except jsonschema.SchemaError as e:
            pytest.fail(f"Generated schema is not valid JSON Schema: {e}")

    def test_schema_validates_correct_data(self) -> None:
        """Test that generated schema can validate correct data."""

        def func(name: str, age: int = Field(..., ge=0, le=150)) -> None:
            pass

        schema = extract_parameter_schema(func)
        assert schema is not None

        # Remove Spikard-specific keys
        cleaned_schema = schema.copy()
        for prop_schema in cleaned_schema["properties"].values():
            prop_schema.pop("source", None)

        # Create validator
        validator = jsonschema.Draft7Validator(cleaned_schema)

        # Valid data should pass
        valid_data = {"name": "Alice", "age": 30}
        errors = list(validator.iter_errors(valid_data))
        assert len(errors) == 0

        # Invalid data should fail
        invalid_data = {"name": "Alice", "age": 200}
        errors = list(validator.iter_errors(invalid_data))
        assert len(errors) > 0
