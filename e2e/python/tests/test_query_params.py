"""E2E tests for query_params."""

import pytest
from typing import Any

async def test_string_validation_with_regex_success() -> None:
    """Tests string parameter with regex pattern validation - matching pattern."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_string_validation_with_regex_success

    app = create_app_query_params_string_validation_with_regex_success()
    client = TestClient(app)

    params = {
        "item_query": "fixedquery",
    }
    response = await client.get("/items/", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["item_query"] == "fixedquery"


async def test_49_integer_gt_constraint_success() -> None:
    """Integer query parameter greater than exclusive minimum should succeed."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_49_integer_gt_constraint_success

    app = create_app_query_params_49_integer_gt_constraint_success()
    client = TestClient(app)

    params = {
        "limit": "5",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["limit"] == "5"


async def test_enum_query_parameter_invalid_value() -> None:
    """Tests enum query parameter with value not in enum."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_enum_query_parameter_invalid_value

    app = create_app_query_params_enum_query_parameter_invalid_value()
    client = TestClient(app)

    params = {
        "model": "vgg16",
    }
    response = await client.get("/query/enum", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_68_array_uniqueitems_success() -> None:
    """Array query parameter with unique items should succeed."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_68_array_uniqueitems_success

    app = create_app_query_params_68_array_uniqueitems_success()
    client = TestClient(app)

    params = {
        "ids": ["1", "2", "3", "4"],
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["ids"] == ["1", "2", "3", "4"]


async def test_47_pattern_validation_email_success() -> None:
    """String query parameter matching regex pattern should succeed."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_47_pattern_validation_email_success

    app = create_app_query_params_47_pattern_validation_email_success()
    client = TestClient(app)

    params = {
        "email": "user@example.com",
    }
    response = await client.get("/subscribe", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["email"] == "user@example.com"


async def test_required_integer_query_parameter_success() -> None:
    """Tests a required integer query parameter with valid value."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_required_integer_query_parameter_success

    app = create_app_query_params_required_integer_query_parameter_success()
    client = TestClient(app)

    params = {
        "query": 42,
    }
    response = await client.get("/query/int", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["query"] == 42


async def test_required_string_query_parameter_missing() -> None:
    """Tests a required string query parameter without providing value, should return 422."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_required_string_query_parameter_missing

    app = create_app_query_params_required_string_query_parameter_missing()
    client = TestClient(app)

    response = await client.get("/query")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_57_boolean_empty_string_coercion() -> None:
    """Boolean query parameter with empty string should coerce to false."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_57_boolean_empty_string_coercion

    app = create_app_query_params_57_boolean_empty_string_coercion()
    client = TestClient(app)

    params = {
        "active": "",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["active"] == ""


async def test_52_integer_le_constraint_boundary() -> None:
    """Integer query parameter equal to maximum should succeed with le constraint."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_52_integer_le_constraint_boundary

    app = create_app_query_params_52_integer_le_constraint_boundary()
    client = TestClient(app)

    params = {
        "limit": "100",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["limit"] == "100"


async def test_list_with_default_empty_array_no_values_provided() -> None:
    """Tests list parameter with default=[] when no values provided, should return empty list."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_list_with_default_empty_array_no_values_provided

    app = create_app_query_params_list_with_default_empty_array_no_values_provided()
    client = TestClient(app)

    response = await client.get("/query/list-default")

    assert response.status_code == 200
    response_data = response.json()


async def test_date_query_parameter_success() -> None:
    """Tests date query parameter with valid ISO date format."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_date_query_parameter_success

    app = create_app_query_params_date_query_parameter_success()
    client = TestClient(app)

    params = {
        "event_date": "2024-01-15",
    }
    response = await client.get("/query/date", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["event_date"] == "2024-01-15"


async def test_string_query_param_with_max_length_constraint_fail() -> None:
    """Tests string query parameter with max_length validation failure."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_string_query_param_with_max_length_constraint_fail

    app = create_app_query_params_string_query_param_with_max_length_constraint_fail()
    client = TestClient(app)

    params = {
        "name": "this_is_way_too_long",
    }
    response = await client.get("/query/str-max-length", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_45_string_minlength_validation_failure() -> None:
    """String query parameter below minLength constraint should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_45_string_minlength_validation_failure

    app = create_app_query_params_45_string_minlength_validation_failure()
    client = TestClient(app)

    params = {
        "term": "ab",
    }
    response = await client.get("/search", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_integer_with_default_value_override() -> None:
    """Tests integer parameter with default value when overridden with custom value."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_integer_with_default_value_override

    app = create_app_query_params_integer_with_default_value_override()
    client = TestClient(app)

    params = {
        "query": 50,
    }
    response = await client.get("/query/int/default", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["query"] == 50


async def test_67_multipleof_constraint_failure() -> None:
    """Integer query parameter that is not multiple of constraint should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_67_multipleof_constraint_failure

    app = create_app_query_params_67_multipleof_constraint_failure()
    client = TestClient(app)

    params = {
        "quantity": "17",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_58_format_email_success() -> None:
    """Query parameter with valid email format should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_58_format_email_success

    app = create_app_query_params_58_format_email_success()
    client = TestClient(app)

    params = {
        "email": "user@example.com",
    }
    response = await client.get("/subscribe", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["email"] == "user@example.com"


async def test_integer_query_param_with_ge_constraint_boundary() -> None:
    """Tests integer query parameter with ge validation at boundary value."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_integer_query_param_with_ge_constraint_boundary

    app = create_app_query_params_integer_query_param_with_ge_constraint_boundary()
    client = TestClient(app)

    params = {
        "value": "10",
    }
    response = await client.get("/query/int-ge", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["value"] == "10"


async def test_integer_query_param_with_gt_constraint_valid() -> None:
    """Tests integer query parameter with gt validation, value above limit."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_integer_query_param_with_gt_constraint_valid

    app = create_app_query_params_integer_query_param_with_gt_constraint_valid()
    client = TestClient(app)

    params = {
        "value": "1",
    }
    response = await client.get("/query/int-gt", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["value"] == "1"


async def test_required_integer_query_parameter_invalid_type() -> None:
    """Tests integer query parameter with non-numeric string, should return 422."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_required_integer_query_parameter_invalid_type

    app = create_app_query_params_required_integer_query_parameter_invalid_type()
    client = TestClient(app)

    params = {
        "query": "baz",
    }
    response = await client.get("/query/int", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_required_integer_query_parameter_float_value() -> None:
    """Tests integer query parameter with float string value, should return 422."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_required_integer_query_parameter_float_value

    app = create_app_query_params_required_integer_query_parameter_float_value()
    client = TestClient(app)

    params = {
        "query": "42.5",
    }
    response = await client.get("/query/int", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_query_parameter_with_url_encoded_special_characters() -> None:
    """Tests query parameter with URL encoded special characters."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_query_parameter_with_url_encoded_special_characters

    app = create_app_query_params_query_parameter_with_url_encoded_special_characters()
    client = TestClient(app)

    params = {
        "name": "test&value=123",
    }
    response = await client.get("/query/basic", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["name"] == "test&value=123"


async def test_59_format_email_failure() -> None:
    """Query parameter with invalid email format should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_59_format_email_failure

    app = create_app_query_params_59_format_email_failure()
    client = TestClient(app)

    params = {
        "email": "not-an-email",
    }
    response = await client.get("/subscribe", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_43_scientific_notation_float() -> None:
    """Query parameter with scientific notation float should parse correctly."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_43_scientific_notation_float

    app = create_app_query_params_43_scientific_notation_float()
    client = TestClient(app)

    params = {
        "threshold": "1.5e-3",
    }
    response = await client.get("/stats", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["threshold"] == "1.5e-3"


async def test_63_format_uri_success() -> None:
    """Query parameter with valid URI should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_63_format_uri_success

    app = create_app_query_params_63_format_uri_success()
    client = TestClient(app)

    params = {
        "url": "https://example.com/path?query=value",
    }
    response = await client.get("/redirect", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["url"] == "https://example.com/path?query=value"


async def test_boolean_query_parameter_numeric_1() -> None:
    """Tests boolean query parameter with '1' converts to true."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_boolean_query_parameter_numeric_1

    app = create_app_query_params_boolean_query_parameter_numeric_1()
    client = TestClient(app)

    params = {
        "flag": "1",
    }
    response = await client.get("/query/bool", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["flag"] == "1"


async def test_string_query_param_with_min_length_constraint_fail() -> None:
    """Tests string query parameter with min_length validation failure."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_string_query_param_with_min_length_constraint_fail

    app = create_app_query_params_string_query_param_with_min_length_constraint_fail()
    client = TestClient(app)

    params = {
        "name": "ab",
    }
    response = await client.get("/query/str-min-length", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_optional_string_query_parameter_provided() -> None:
    """Tests optional string parameter with value provided."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_optional_string_query_parameter_provided

    app = create_app_query_params_optional_string_query_parameter_provided()
    client = TestClient(app)

    params = {
        "query": "baz",
    }
    response = await client.get("/query/optional", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["query"] == "baz"


async def test_list_of_integers_multiple_values() -> None:
    """Tests list query parameter with multiple integer values using same key."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_list_of_integers_multiple_values

    app = create_app_query_params_list_of_integers_multiple_values()
    client = TestClient(app)

    params = {
        "device_ids": [1, 2],
    }
    response = await client.get("/query/list", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["device_ids"] == [1, 2]


async def test_integer_query_param_with_lt_constraint_valid() -> None:
    """Tests integer query parameter with lt validation, value below limit."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_integer_query_param_with_lt_constraint_valid

    app = create_app_query_params_integer_query_param_with_lt_constraint_valid()
    client = TestClient(app)

    params = {
        "value": "49",
    }
    response = await client.get("/query/int-lt", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["value"] == "49"


async def test_42_negative_integer_query_param() -> None:
    """Query parameter with negative integer value should be parsed correctly."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_42_negative_integer_query_param

    app = create_app_query_params_42_negative_integer_query_param()
    client = TestClient(app)

    params = {
        "offset": "-10",
    }
    response = await client.get("/items/negative", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["offset"] == "-10"


async def test_46_string_maxlength_validation_failure() -> None:
    """String query parameter exceeding maxLength constraint should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_46_string_maxlength_validation_failure

    app = create_app_query_params_46_string_maxlength_validation_failure()
    client = TestClient(app)

    params = {
        "term": "this_is_way_too_long",
    }
    response = await client.get("/search", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_56_array_maxitems_constraint_failure() -> None:
    """Array query parameter exceeding maxItems constraint should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_56_array_maxitems_constraint_failure

    app = create_app_query_params_56_array_maxitems_constraint_failure()
    client = TestClient(app)

    params = {
        "tags": ["a", "b", "c", "d", "e", "f"],
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_string_query_param_with_regex_pattern_fail() -> None:
    """Tests string query parameter with regex pattern validation failure."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_string_query_param_with_regex_pattern_fail

    app = create_app_query_params_string_query_param_with_regex_pattern_fail()
    client = TestClient(app)

    params = {
        "code": "abc123",
    }
    response = await client.get("/query/pattern", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_44_string_minlength_validation_success() -> None:
    """String query parameter meeting minLength constraint should succeed."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_44_string_minlength_validation_success

    app = create_app_query_params_44_string_minlength_validation_success()
    client = TestClient(app)

    params = {
        "term": "foo",
    }
    response = await client.get("/search", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["term"] == "foo"


async def test_61_format_ipv4_failure() -> None:
    """Query parameter with invalid IPv4 address should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_61_format_ipv4_failure

    app = create_app_query_params_61_format_ipv4_failure()
    client = TestClient(app)

    params = {
        "ip": "999.999.999.999",
    }
    response = await client.get("/network", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_48_pattern_validation_email_failure() -> None:
    """String query parameter not matching regex pattern should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_48_pattern_validation_email_failure

    app = create_app_query_params_48_pattern_validation_email_failure()
    client = TestClient(app)

    params = {
        "email": "invalid-email",
    }
    response = await client.get("/subscribe", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_required_integer_query_parameter_missing() -> None:
    """Tests a required integer query parameter without providing value, should return 422."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_required_integer_query_parameter_missing

    app = create_app_query_params_required_integer_query_parameter_missing()
    client = TestClient(app)

    response = await client.get("/query/int")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_query_parameter_with_special_characters_url_encoding() -> None:
    """Tests query parameters with special characters that need URL encoding."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_query_parameter_with_special_characters_url_encoding

    app = create_app_query_params_query_parameter_with_special_characters_url_encoding()
    client = TestClient(app)

    params = {
        "email": "x@test.com",
        "special": "&@A.ac",
    }
    response = await client.get("/test", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["email"] == "x@test.com"
    assert response_data["special"] == "&@A.ac"


async def test_list_query_parameter_required_but_missing() -> None:
    """Tests required list parameter without any values, should return 422."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_list_query_parameter_required_but_missing

    app = create_app_query_params_list_query_parameter_required_but_missing()
    client = TestClient(app)

    response = await client.get("/query/list")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_required_string_query_parameter_success() -> None:
    """Tests a required string query parameter with valid value."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_required_string_query_parameter_success

    app = create_app_query_params_required_string_query_parameter_success()
    client = TestClient(app)

    params = {
        "query": "baz",
    }
    response = await client.get("/query", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["query"] == "baz"


async def test_66_multipleof_constraint_success() -> None:
    """Integer query parameter that is multiple of constraint should succeed."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_66_multipleof_constraint_success

    app = create_app_query_params_66_multipleof_constraint_success()
    client = TestClient(app)

    params = {
        "quantity": "15",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["quantity"] == "15"


async def test_53_integer_le_constraint_failure() -> None:
    """Integer query parameter exceeding maximum should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_53_integer_le_constraint_failure

    app = create_app_query_params_53_integer_le_constraint_failure()
    client = TestClient(app)

    params = {
        "limit": "101",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_multiple_query_parameters_with_different_types() -> None:
    """Tests multiple query parameters of different types in single request."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_multiple_query_parameters_with_different_types

    app = create_app_query_params_multiple_query_parameters_with_different_types()
    client = TestClient(app)

    params = {
        "active": "true",
        "name": "john",
        "age": "30",
        "score": "95.5",
    }
    response = await client.get("/query/multi-type", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["active"] == "true"
    assert response_data["name"] == "john"
    assert response_data["age"] == "30"
    assert response_data["score"] == "95.5"


async def test_71_array_separator_semicolon() -> None:
    """Array query parameter with semicolon separator should parse correctly."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_71_array_separator_semicolon

    app = create_app_query_params_71_array_separator_semicolon()
    client = TestClient(app)

    params = {
        "colors": "red;green;blue",
    }
    response = await client.get("/items?colors=red;green;blue", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["colors"] == "red;green;blue"


async def test_70_array_separator_pipe() -> None:
    """Array query parameter with pipe separator should parse correctly."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_70_array_separator_pipe

    app = create_app_query_params_70_array_separator_pipe()
    client = TestClient(app)

    params = {
        "tags": "python|rust|typescript",
    }
    response = await client.get("/items?tags=python|rust|typescript", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["tags"] == "python|rust|typescript"


async def test_integer_with_default_value_not_provided() -> None:
    """Tests integer parameter with default value when not provided."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_integer_with_default_value_not_provided

    app = create_app_query_params_integer_with_default_value_not_provided()
    client = TestClient(app)

    response = await client.get("/query/int/default")

    assert response.status_code == 200
    response_data = response.json()


async def test_boolean_query_parameter_true() -> None:
    """Tests boolean query parameter with 'true' string value."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_boolean_query_parameter_true

    app = create_app_query_params_boolean_query_parameter_true()
    client = TestClient(app)

    params = {
        "flag": "true",
    }
    response = await client.get("/query/bool", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["flag"] == "true"


async def test_integer_query_param_with_le_constraint_boundary() -> None:
    """Tests integer query parameter with le validation at boundary value."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_integer_query_param_with_le_constraint_boundary

    app = create_app_query_params_integer_query_param_with_le_constraint_boundary()
    client = TestClient(app)

    params = {
        "value": "100",
    }
    response = await client.get("/query/int-le", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["value"] == "100"


async def test_float_query_param_with_ge_constraint_success() -> None:
    """Tests float query parameter with ge validation at boundary."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_float_query_param_with_ge_constraint_success

    app = create_app_query_params_float_query_param_with_ge_constraint_success()
    client = TestClient(app)

    params = {
        "price": "0.01",
    }
    response = await client.get("/query/float-ge", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["price"] == "0.01"


async def test_51_integer_ge_constraint_boundary() -> None:
    """Integer query parameter equal to minimum should succeed with ge constraint."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_51_integer_ge_constraint_boundary

    app = create_app_query_params_51_integer_ge_constraint_boundary()
    client = TestClient(app)

    params = {
        "offset": "0",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["offset"] == "0"


async def test_optional_integer_query_parameter_missing() -> None:
    """Tests optional integer parameter without value, should not error."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_optional_integer_query_parameter_missing

    app = create_app_query_params_optional_integer_query_parameter_missing()
    client = TestClient(app)

    response = await client.get("/query/int/optional")

    assert response.status_code == 200
    response_data = response.json()


async def test_69_array_uniqueitems_failure() -> None:
    """Array query parameter with duplicate items should fail when uniqueItems is true."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_69_array_uniqueitems_failure

    app = create_app_query_params_69_array_uniqueitems_failure()
    client = TestClient(app)

    params = {
        "ids": ["1", "2", "2", "3"],
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_72_array_separator_space() -> None:
    """Array query parameter with space separator should parse correctly."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_72_array_separator_space

    app = create_app_query_params_72_array_separator_space()
    client = TestClient(app)

    params = {
        "keywords": "rust web framework",
    }
    response = await client.get("/search?keywords=rust%20web%20framework", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["keywords"] == "rust web framework"


async def test_string_validation_with_regex_failure() -> None:
    """Tests string parameter with regex pattern validation - non-matching pattern returns 422."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_string_validation_with_regex_failure

    app = create_app_query_params_string_validation_with_regex_failure()
    client = TestClient(app)

    params = {
        "item_query": "nonregexquery",
    }
    response = await client.get("/items/", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_65_format_hostname_success() -> None:
    """Query parameter with valid hostname should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_65_format_hostname_success

    app = create_app_query_params_65_format_hostname_success()
    client = TestClient(app)

    params = {
        "host": "api.example.com",
    }
    response = await client.get("/dns", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["host"] == "api.example.com"


async def test_query_parameter_with_url_encoded_space() -> None:
    """Tests query parameter with URL encoded space character."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_query_parameter_with_url_encoded_space

    app = create_app_query_params_query_parameter_with_url_encoded_space()
    client = TestClient(app)

    params = {
        "name": "hello world",
    }
    response = await client.get("/query/basic", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["name"] == "hello world"


async def test_list_of_strings_multiple_values() -> None:
    """Tests list of string query parameters with multiple values."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_list_of_strings_multiple_values

    app = create_app_query_params_list_of_strings_multiple_values()
    client = TestClient(app)

    params = {
        "q": ["foo", "bar"],
    }
    response = await client.get("/items/", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["q"] == ["foo", "bar"]


async def test_optional_query_parameter_with_default_value() -> None:
    """Tests optional query parameter that uses default when not provided."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_optional_query_parameter_with_default_value

    app = create_app_query_params_optional_query_parameter_with_default_value()
    client = TestClient(app)

    response = await client.get("/query/optional-default")

    assert response.status_code == 200
    response_data = response.json()


async def test_62_format_ipv6_success() -> None:
    """Query parameter with valid IPv6 address should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_62_format_ipv6_success

    app = create_app_query_params_62_format_ipv6_success()
    client = TestClient(app)

    params = {
        "ip": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
    }
    response = await client.get("/network/ipv6", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["ip"] == "2001:0db8:85a3:0000:0000:8a2e:0370:7334"


async def test_array_query_parameter_single_value() -> None:
    """Tests array query parameter with single value."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_array_query_parameter_single_value

    app = create_app_query_params_array_query_parameter_single_value()
    client = TestClient(app)

    params = {
        "tags": "apple",
    }
    response = await client.get("/query/list-default", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["tags"] == "apple"


async def test_optional_string_query_parameter_missing() -> None:
    """Tests optional string parameter without value, should return None/null."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_optional_string_query_parameter_missing

    app = create_app_query_params_optional_string_query_parameter_missing()
    client = TestClient(app)

    response = await client.get("/query/optional")

    assert response.status_code == 200
    response_data = response.json()


async def test_datetime_query_parameter_success() -> None:
    """Tests datetime query parameter with valid ISO datetime format."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_datetime_query_parameter_success

    app = create_app_query_params_datetime_query_parameter_success()
    client = TestClient(app)

    params = {
        "timestamp": "2024-01-15T10:30:00Z",
    }
    response = await client.get("/query/datetime", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["timestamp"] == "2024-01-15T10:30:00Z"


async def test_uuid_query_parameter_invalid_format() -> None:
    """Tests UUID query parameter with invalid UUID format."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_uuid_query_parameter_invalid_format

    app = create_app_query_params_uuid_query_parameter_invalid_format()
    client = TestClient(app)

    params = {
        "item_id": "not-a-uuid",
    }
    response = await client.get("/query/uuid", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_array_query_parameter_empty_array() -> None:
    """Tests array query parameter when no values are provided."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_array_query_parameter_empty_array

    app = create_app_query_params_array_query_parameter_empty_array()
    client = TestClient(app)

    response = await client.get("/query/list-default")

    assert response.status_code == 200
    response_data = response.json()


async def test_enum_query_parameter_success() -> None:
    """Tests enum query parameter with valid enum value."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_enum_query_parameter_success

    app = create_app_query_params_enum_query_parameter_success()
    client = TestClient(app)

    params = {
        "model": "alexnet",
    }
    response = await client.get("/query/enum", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["model"] == "alexnet"


async def test_uuid_query_parameter_success() -> None:
    """Tests UUID query parameter with valid UUID format."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_uuid_query_parameter_success

    app = create_app_query_params_uuid_query_parameter_success()
    client = TestClient(app)

    params = {
        "item_id": "c892496f-b1fd-4b91-bdb8-b46f92df1716",
    }
    response = await client.get("/query/uuid", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["item_id"] == "c892496f-b1fd-4b91-bdb8-b46f92df1716"


async def test_50_integer_gt_constraint_failure() -> None:
    """Integer query parameter equal to exclusive minimum should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_50_integer_gt_constraint_failure

    app = create_app_query_params_50_integer_gt_constraint_failure()
    client = TestClient(app)

    params = {
        "limit": "0",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_64_format_uri_failure() -> None:
    """Query parameter with invalid URI should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_64_format_uri_failure

    app = create_app_query_params_64_format_uri_failure()
    client = TestClient(app)

    params = {
        "url": "not a uri",
    }
    response = await client.get("/redirect", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_54_array_minitems_constraint_success() -> None:
    """Array query parameter meeting minItems constraint should succeed."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_54_array_minitems_constraint_success

    app = create_app_query_params_54_array_minitems_constraint_success()
    client = TestClient(app)

    params = {
        "ids": ["1", "2", "3"],
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["ids"] == ["1", "2", "3"]


async def test_55_array_minitems_constraint_failure() -> None:
    """Array query parameter below minItems constraint should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_55_array_minitems_constraint_failure

    app = create_app_query_params_55_array_minitems_constraint_failure()
    client = TestClient(app)

    params = {
        "ids": ["1"],
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_60_format_ipv4_success() -> None:
    """Query parameter with valid IPv4 address should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_query_params_60_format_ipv4_success

    app = create_app_query_params_60_format_ipv4_success()
    client = TestClient(app)

    params = {
        "ip": "192.168.1.1",
    }
    response = await client.get("/network", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["ip"] == "192.168.1.1"


