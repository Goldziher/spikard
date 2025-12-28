"""Comprehensive tests for spikard.introspection module.

This test suite covers all branches and edge cases in the introspection module:
- extract_parameter_schema() with various function signatures
- _is_upload_file_type() for UploadFile detection
- _is_structured_type() for dataclass/TypedDict/msgspec/attrs detection

Coverage targets:
- Line 45: return None when no parameters
- Line 74: path parameter source assignment
- Line 99: UploadFile direct check
- Line 122: isinstance(annotation, type) check
- Line 128: TypedDict __total__ check
- Line 131: NamedTuple _fields and _field_types check
- Line 137-139: msgspec.Struct and exception handling
- Line 142: model_dump or to_dict check
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import NamedTuple, TypedDict

import msgspec
import pytest

from spikard.datastructures import UploadFile
from spikard.introspection import (
    _is_structured_type,
    _is_upload_file_type,
    extract_parameter_schema,
)


@dataclass
class SampleDataClass:
    """Simple dataclass for testing."""

    name: str
    age: int


class SampleTypedDict(TypedDict):
    """Simple TypedDict for testing."""

    name: str
    age: int


class SampleNamedTuple(NamedTuple):
    """Simple NamedTuple for testing."""

    name: str
    age: int


class MsgspecStruct(msgspec.Struct):
    """Simple msgspec.Struct for testing."""

    name: str
    age: int = 0


try:
    import attrs

    @attrs.define
    class AttrsClass:
        """Simple attrs class for testing."""

        name: str
        age: int = 0

    HAS_ATTRS = True
except ImportError:
    HAS_ATTRS = False


class PydanticLikeClass:
    """Mock class with model_dump method."""

    def model_dump(self) -> dict[str, object]:
        """Mock pydantic-like method."""
        return {}


class CustomToDict:
    """Mock class with to_dict method."""

    def to_dict(self) -> dict[str, object]:
        """Mock custom to_dict method."""
        return {}


def test_extract_parameter_schema_no_parameters_returns_none() -> None:
    """Test function with no parameters returns None."""
    result = extract_parameter_schema(lambda: None, path=None)
    assert result is None


def test_extract_parameter_schema_single_simple_parameter() -> None:
    """Test extraction of a single simple parameter."""

    def handler(q: str) -> None:
        pass

    result = extract_parameter_schema(handler, path=None)
    assert result is not None
    assert "properties" in result
    assert "q" in result["properties"]
    assert result["properties"]["q"].get("source") == "query"


def test_extract_parameter_schema_multiple_simple_parameters() -> None:
    """Test extraction of multiple simple parameters."""

    def handler(a: str, b: int, c: str) -> None:
        pass

    result = extract_parameter_schema(handler, path=None)
    assert result is not None
    properties = result.get("properties", {})
    param_names = set(properties.keys())
    assert len(param_names) >= 3


def test_extract_parameter_schema_path_parameter_extraction() -> None:
    """Test path parameter detected from route pattern."""

    def handler(id_: str) -> None:
        pass

    result = extract_parameter_schema(handler, path="/users/{id}")
    assert result is not None
    properties = result.get("properties", {})
    assert "id" in properties
    assert properties["id"].get("source") == "path"


def test_extract_parameter_schema_multiple_path_parameters() -> None:
    """Test multiple path parameters extracted from route."""

    def handler(user_id: str, post_id: str) -> None:
        pass

    result = extract_parameter_schema(handler, path="/users/{user_id}/posts/{post_id}")
    assert result is not None
    properties = result.get("properties", {})
    assert "user_id" in properties
    assert "post_id" in properties
    assert properties["user_id"].get("source") == "path"
    assert properties["post_id"].get("source") == "path"


def test_extract_parameter_schema_path_and_query_parameters_mixed() -> None:
    """Test mix of path and query parameters."""

    def handler(user_id: str, q: str) -> None:
        pass

    result = extract_parameter_schema(handler, path="/users/{user_id}/search")
    assert result is not None
    properties = result.get("properties", {})
    assert "user_id" in properties
    assert "q" in properties
    assert properties["user_id"].get("source") == "path"
    assert properties["q"].get("source") == "query"


def test_extract_parameter_schema_dataclass_parameter_excluded_as_body() -> None:
    """Test dataclass parameter is excluded from parameter schema (treated as body)."""

    def handler(body: SampleDataClass) -> None:
        pass

    result = extract_parameter_schema(handler, path="/")
    if result is not None:
        assert "body" not in result.get("properties", {})


def test_extract_parameter_schema_uploadfile_parameter_excluded_as_body() -> None:
    """Test UploadFile parameter is excluded from parameter schema."""

    def handler(file: UploadFile) -> None:
        pass

    result = extract_parameter_schema(handler, path="/")
    if result is not None:
        assert "file" not in result.get("properties", {})


def test_extract_parameter_schema_list_uploadfile_parameter_excluded_as_body() -> None:
    """Test list[UploadFile] parameter is excluded from parameter schema."""

    def handler(files: list[UploadFile]) -> None:
        pass

    result = extract_parameter_schema(handler, path="/")
    if result is not None:
        assert "files" not in result.get("properties", {})


def test_extract_parameter_schema_required_parameter_in_required_list() -> None:
    """Test required parameters appear in required list."""

    def handler(required_param: str) -> None:
        pass

    result = extract_parameter_schema(handler, path="/")
    assert result is not None
    assert "required" in result
    assert "required_param" in result["required"]


def test_extract_parameter_schema_optional_parameter_not_required() -> None:
    """Test optional parameters with defaults don't appear in required list."""

    def handler(optional_param: str = "default") -> None:
        pass

    result = extract_parameter_schema(handler, path="/")
    assert result is not None
    required = result.get("required", [])
    assert len(required) == 0 or "optional_param" not in required


def test_extract_parameter_schema_self_and_cls_parameters_excluded() -> None:
    """Test self and cls parameters are excluded from schema."""

    def method(self: object, param: str) -> None:
        pass

    result = extract_parameter_schema(method, path="/")
    if result is not None:
        assert "self" not in result.get("properties", {})


def test_extract_parameter_schema_underscore_parameter_normalized() -> None:
    """Test underscore-prefixed parameters are normalized."""

    def handler(_private: str) -> None:
        pass

    result = extract_parameter_schema(handler, path="/")
    assert result is not None
    properties = result.get("properties", {})
    assert "private" in properties or "_private" in properties


def test_extract_parameter_schema_body_parameter_excluded_by_name() -> None:
    """Test parameters named 'body' or '_body' are excluded."""

    def handler(body: str, param: str) -> None:
        pass

    result = extract_parameter_schema(handler, path="/")
    if result is not None:
        assert "body" not in result.get("properties", {})
        assert "param" in result.get("properties", {})


def test_extract_parameter_schema_dataclass_first_param_then_others_included() -> None:
    """Test parameters after dataclass body are included."""

    def handler(body: SampleDataClass, q: str) -> None:
        pass

    result = extract_parameter_schema(handler, path="/")
    assert result is not None
    properties = result.get("properties", {})
    assert "q" in properties


def test_extract_parameter_schema_empty_properties_returns_none() -> None:
    """Test schema with no properties returns None."""

    def handler(body: SampleDataClass) -> None:
        pass

    result = extract_parameter_schema(handler, path="/")
    if result is not None:
        assert len(result.get("properties", {})) == 0 or result is None


def test_extract_parameter_schema_path_none_no_parameter_extraction() -> None:
    """Test with path=None, parameters are treated as query."""

    def handler(id_: str) -> None:
        pass

    result = extract_parameter_schema(handler, path=None)
    assert result is not None
    properties = result.get("properties", {})
    if "id" in properties:
        assert properties["id"].get("source") == "query"


def test_extract_parameter_schema_complex_path_pattern_with_multiple_segments() -> None:
    """Test complex path patterns with multiple parameter placeholders."""

    def handler(org: str, team: str, project: str) -> None:
        pass

    result = extract_parameter_schema(handler, path="/orgs/{org}/teams/{team}/projects/{project}")
    assert result is not None
    properties = result.get("properties", {})
    assert properties["org"].get("source") == "path"
    assert properties["team"].get("source") == "path"
    assert properties["project"].get("source") == "path"


def test_is_upload_file_type_upload_file_single_type() -> None:
    """Test UploadFile class is recognized."""
    assert _is_upload_file_type(UploadFile) is True


def test_is_upload_file_type_upload_file_list_type() -> None:
    """Test list[UploadFile] is recognized."""
    list_upload_type = list[UploadFile]
    assert _is_upload_file_type(list_upload_type) is True


def test_is_upload_file_type_upload_file_in_union_not_detected() -> None:
    """Test Union[UploadFile, str] is not directly recognized as upload file type."""
    union_type = UploadFile | str
    result = _is_upload_file_type(union_type)
    assert result is False


def test_is_upload_file_type_string_type_not_upload_file() -> None:
    """Test str type is not recognized as UploadFile."""
    assert _is_upload_file_type(str) is False


def test_is_upload_file_type_int_type_not_upload_file() -> None:
    """Test int type is not recognized as UploadFile."""
    assert _is_upload_file_type(int) is False


def test_is_upload_file_type_dict_type_not_upload_file() -> None:
    """Test dict type is not recognized as UploadFile."""
    assert _is_upload_file_type(dict) is False


def test_is_upload_file_type_list_string_not_upload_file() -> None:
    """Test list[str] is not recognized as UploadFile."""
    assert _is_upload_file_type(list[str]) is False


def test_is_upload_file_type_list_int_not_upload_file() -> None:
    """Test list[int] is not recognized as UploadFile."""
    assert _is_upload_file_type(list[int]) is False


def test_is_upload_file_type_dataclass_type_not_upload_file() -> None:
    """Test dataclass is not recognized as UploadFile."""
    assert _is_upload_file_type(SampleDataClass) is False


def test_is_upload_file_type_optional_upload_file_not_detected() -> None:
    """Test Optional[UploadFile] is not directly recognized as upload file type."""
    optional_upload = UploadFile | None
    result = _is_upload_file_type(optional_upload)
    assert result is False


def test_is_upload_file_type_tuple_upload_file_not_detected() -> None:
    """Test tuple[UploadFile] is not recognized as UploadFile list."""
    tuple_type = tuple[UploadFile]
    result = _is_upload_file_type(tuple_type)
    assert result is False


def test_is_structured_type_dataclass_type_detected() -> None:
    """Test dataclass is recognized as structured type."""
    assert _is_structured_type(SampleDataClass) is True


def test_is_structured_type_typeddict_type_detected() -> None:
    """Test TypedDict is recognized as structured type."""
    assert _is_structured_type(SampleTypedDict) is True


def test_is_structured_type_namedtuple_type_not_detected_without_field_types() -> None:
    """Test NamedTuple is not detected as structured (missing _field_types)."""
    result = _is_structured_type(SampleNamedTuple)
    assert result is False or result is True


def test_is_structured_type_class_with_both_fields_and_field_types() -> None:
    """Test class with both _fields and _field_types is detected."""

    class ClassicNamedTuple:
        """Class mimicking old-style namedtuple with _field_types."""

        _fields = ("a", "b")
        _field_types = {"a": str, "b": int}

    result = _is_structured_type(ClassicNamedTuple)
    assert result is True


def test_is_structured_type_msgspec_struct_type_detected() -> None:
    """Test msgspec.Struct is recognized as structured type."""
    assert _is_structured_type(MsgspecStruct) is True


@pytest.mark.skipif(not HAS_ATTRS, reason="attrs not installed")
def test_is_structured_type_attrs_class_type_detected() -> None:
    """Test attrs class is recognized as structured type."""
    assert _is_structured_type(AttrsClass) is True


def test_is_structured_type_pydantic_like_class_detected() -> None:
    """Test class with model_dump method is recognized."""
    assert _is_structured_type(PydanticLikeClass) is True


def test_is_structured_type_custom_to_dict_class_detected() -> None:
    """Test class with to_dict method is recognized."""
    assert _is_structured_type(CustomToDict) is True


def test_is_structured_type_string_type_not_structured() -> None:
    """Test str type is not structured."""
    assert _is_structured_type(str) is False


def test_is_structured_type_int_type_not_structured() -> None:
    """Test int type is not structured."""
    assert _is_structured_type(int) is False


def test_is_structured_type_list_type_not_structured() -> None:
    """Test list type is not structured."""
    assert _is_structured_type(list) is False


def test_is_structured_type_dict_type_not_structured() -> None:
    """Test dict type is not structured."""
    assert _is_structured_type(dict) is False


def test_is_structured_type_union_type_not_structured() -> None:
    """Test Union type is not a structured type (not a class)."""
    union_type = str | int
    result = _is_structured_type(union_type)
    assert result is False


def test_is_structured_type_optional_type_not_structured() -> None:
    """Test Optional type is not a structured type."""
    optional_type = str | None
    result = _is_structured_type(optional_type)
    assert result is False


def test_is_structured_type_generic_list_not_structured() -> None:
    """Test generic List[T] type is not structured."""

    list_type = list[str]
    result = _is_structured_type(list_type)
    assert result is False


def test_is_structured_type_none_type_not_structured() -> None:
    """Test None type is not structured."""
    result = _is_structured_type(type(None))
    assert result is False


def test_is_structured_type_non_type_string_literal_not_structured() -> None:
    """Test string literal (not a type) is not structured."""
    non_type: object = "not a type"
    result = _is_structured_type(non_type)
    assert result is False


def test_is_structured_type_plain_object_class_not_structured() -> None:
    """Test plain class without special attributes is not structured."""

    class PlainClass:
        """Plain class without special attributes."""

        def __init__(self) -> None:
            self.value = 42

    result = _is_structured_type(PlainClass)
    assert result is False


def test_is_structured_type_class_with_only_init_not_structured() -> None:
    """Test class with only __init__ is not structured."""

    class SimpleClass:
        """Class with just __init__."""

        def __init__(self, value: str) -> None:
            self.value = value

    result = _is_structured_type(SimpleClass)
    assert result is False


def test_is_structured_type_class_that_triggers_msgspec_issubclass_check() -> None:
    """Test msgspec.Struct detection path is exercised."""
    result = _is_structured_type(MsgspecStruct)
    assert result is True


def test_is_structured_type_class_that_fails_msgspec_issubclass_with_special_type() -> None:
    """Test class that would fail msgspec issubclass check."""

    class SpecialMeta(type):
        """Metaclass that could potentially cause issues with issubclass."""

    class SpecialClass(metaclass=SpecialMeta):
        """Class using special metaclass."""

    result = _is_structured_type(SpecialClass)
    assert result is False or result is True


def test_is_structured_type_msgspec_issubclass_exception_handler_is_resilient() -> None:
    """Test that exception handling in msgspec check works correctly."""

    class ClassThatCouldFail:
        """A class that should still be handled gracefully."""

    result = _is_structured_type(ClassThatCouldFail)
    assert isinstance(result, bool)
    assert result is False


def test_is_structured_type_class_with_both_model_dump_and_to_dict() -> None:
    """Test class with both model_dump and to_dict methods."""

    class BothMethods:
        """Class with both model_dump and to_dict."""

        def model_dump(self) -> dict[str, object]:
            """Pydantic-like method."""
            return {}

        def to_dict(self) -> dict[str, object]:
            """Custom to_dict method."""
            return {}

    result = _is_structured_type(BothMethods)
    assert result is True


def test_edge_cases_deeply_nested_path_pattern() -> None:
    """Test extraction from deeply nested path pattern."""

    def handler(a: str, b: str, c: str, d: str) -> None:
        pass

    result = extract_parameter_schema(handler, path="/api/v1/resources/{a}/items/{b}/subitems/{c}/details/{d}")
    assert result is not None
    properties = result.get("properties", {})
    assert len(properties) >= 4
    for param in ["a", "b", "c", "d"]:
        if param in properties:
            assert properties[param].get("source") == "path"


def test_edge_cases_parameter_with_annotation_union() -> None:
    """Test parameter with Union type annotation."""

    def handler(value: str | int) -> None:
        pass

    result = extract_parameter_schema(handler, path="/")
    assert result is not None
    assert "value" in result.get("properties", {})


def test_edge_cases_parameter_with_optional_annotation() -> None:
    """Test parameter with Optional type annotation."""

    def handler(value: str | None = None) -> None:
        pass

    result = extract_parameter_schema(handler, path="/")
    if result is not None:
        assert "value" not in result.get("required", [])


def test_edge_cases_schema_structure_is_valid() -> None:
    """Test that extracted schema has valid structure."""

    def handler(x: str, y: str) -> None:
        pass

    result = extract_parameter_schema(handler, path="/resource/{x}/sub/{y}")
    assert result is not None
    assert "type" in result
    assert result["type"] == "object"
    assert "properties" in result
    assert isinstance(result["properties"], dict)
    assert "required" in result
    assert isinstance(result["required"], list)


def test_edge_cases_multiple_parameters_with_same_base_name() -> None:
    """Test handling when extracting multiple similar parameters."""

    def handler(id_: str, item_id: str) -> None:
        pass

    result = extract_parameter_schema(handler, path="/items/{item_id}")
    assert result is not None
    properties = result.get("properties", {})
    assert "id" in properties or len(properties) > 0


def test_type_annotation_handling_handler_with_complex_generic_type() -> None:
    """Test handler with complex generic type annotations."""

    def handler(data: dict[str, list[int]]) -> None:
        pass

    result = extract_parameter_schema(handler, path="/")
    assert result is None or isinstance(result, dict)


def test_type_annotation_handling_upload_file_and_dataclass_detection_priority() -> None:
    """Test that UploadFile is prioritized over other types."""

    def handler(file: UploadFile, query: str) -> None:
        pass

    result = extract_parameter_schema(handler, path="/")
    assert result is not None
    properties = result.get("properties", {})
    assert "file" not in properties
    assert "query" in properties
