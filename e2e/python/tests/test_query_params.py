"""E2E tests for query_params."""

import pytest
from typing import Any

async def test_string_validation_with_regex__success(client: Any) -> None:
    """Tests string parameter with regex pattern validation - matching pattern."""
    params = {
        "item_query": "fixedquery",
    }
    response = await client.get("/items/", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "item_query" in response_data
    assert response_data["item_query"] == "fixedquery"


async def test_49_integer_gt_constraint_success(client: Any) -> None:
    """Integer query parameter greater than exclusive minimum should succeed."""
    params = {
        "limit": "5",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "limit" in response_data
    assert response_data["limit"] == 5


async def test_enum_query_parameter__invalid_value(client: Any) -> None:
    """Tests enum query parameter with value not in enum."""
    params = {
        "model": "vgg16",
    }
    response = await client.get("/query/enum", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "expected" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["expected"] == "'alexnet', 'resnet' or 'lenet'"
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "vgg16"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "model"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be 'alexnet', 'resnet' or 'lenet'"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "enum"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_68_array_uniqueitems_success(client: Any) -> None:
    """Array query parameter with unique items should succeed."""
    params = {
        "ids": ["1", "2", "3", "4"],
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "ids" in response_data
    assert len(response_data["ids"]) == 4
    assert response_data["ids"][0] == 1
    assert response_data["ids"][1] == 2
    assert response_data["ids"][2] == 3
    assert response_data["ids"][3] == 4


async def test_47_pattern_validation_email_success(client: Any) -> None:
    """String query parameter matching regex pattern should succeed."""
    params = {
        "email": "user@example.com",
    }
    response = await client.get("/subscribe", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "email" in response_data
    assert response_data["email"] == "user@example.com"


async def test_required_integer_query_parameter__success(client: Any) -> None:
    """Tests a required integer query parameter with valid value."""
    params = {
        "query": 42,
    }
    response = await client.get("/query/int", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "foo bar 42"


async def test_required_string_query_parameter__missing(client: Any) -> None:
    """Tests a required string query parameter without providing value, should return 422."""
    response = await client.get("/query")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == None
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "query"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Field required"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "missing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_57_boolean_empty_string_coercion(client: Any) -> None:
    """Boolean query parameter with empty string should coerce to false."""
    params = {
        "active": "",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "active" in response_data
    assert response_data["active"] == False


async def test_52_integer_le_constraint_boundary(client: Any) -> None:
    """Integer query parameter equal to maximum should succeed with le constraint."""
    params = {
        "limit": "100",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "limit" in response_data
    assert response_data["limit"] == 100


async def test_list_with_default_empty_array__no_values_provided(client: Any) -> None:
    """Tests list parameter with default=[] when no values provided, should return empty list."""
    response = await client.get("/query/list-default")

    assert response.status_code == 200
    response_data = response.json()
    assert len(response_data) == 0


async def test_date_query_parameter__success(client: Any) -> None:
    """Tests date query parameter with valid ISO date format."""
    params = {
        "event_date": "2024-01-15",
    }
    response = await client.get("/query/date", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "event_date" in response_data
    assert response_data["event_date"] == "2024-01-15"


async def test_string_query_param_with_max_length_constraint__fail(client: Any) -> None:
    """Tests string query parameter with max_length validation failure."""
    params = {
        "name": "this_is_way_too_long",
    }
    response = await client.get("/query/str-max-length", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "max_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["max_length"] == 10
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "this_is_way_too_long"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "name"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should have at most 10 characters"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_too_long"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_45_string_minlength_validation_failure(client: Any) -> None:
    """String query parameter below minLength constraint should fail validation."""
    params = {
        "term": "ab",
    }
    response = await client.get("/search", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "actual_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["actual_length"] == 2
    assert "min_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["min_length"] == 3
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "term"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String length must be at least 3"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_integer_with_default_value__override(client: Any) -> None:
    """Tests integer parameter with default value when overridden with custom value."""
    params = {
        "query": 50,
    }
    response = await client.get("/query/int/default", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "foo bar 50"


async def test_67_multipleof_constraint_failure(client: Any) -> None:
    """Integer query parameter that is not multiple of constraint should fail."""
    params = {
        "quantity": "17",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "multiple_of" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["multiple_of"] == 5
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == 17
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "quantity"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Value must be a multiple of 5"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_58_format_email_success(client: Any) -> None:
    """Query parameter with valid email format should be accepted."""
    params = {
        "email": "user@example.com",
    }
    response = await client.get("/subscribe", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "email" in response_data
    assert response_data["email"] == "user@example.com"


async def test_integer_query_param_with_ge_constraint__boundary(client: Any) -> None:
    """Tests integer query parameter with ge validation at boundary value."""
    params = {
        "value": "10",
    }
    response = await client.get("/query/int-ge", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "value" in response_data
    assert response_data["value"] == 10


async def test_integer_query_param_with_gt_constraint__valid(client: Any) -> None:
    """Tests integer query parameter with gt validation, value above limit."""
    params = {
        "value": "1",
    }
    response = await client.get("/query/int-gt", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "value" in response_data
    assert response_data["value"] == 1


async def test_required_integer_query_parameter__invalid_type(client: Any) -> None:
    """Tests integer query parameter with non-numeric string, should return 422."""
    params = {
        "query": "baz",
    }
    response = await client.get("/query/int", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "baz"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "query"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be a valid integer, unable to parse string as an integer"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "int_parsing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_required_integer_query_parameter__float_value(client: Any) -> None:
    """Tests integer query parameter with float string value, should return 422."""
    params = {
        "query": "42.5",
    }
    response = await client.get("/query/int", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == 42.5
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "query"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be a valid integer, unable to parse string as an integer"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "int_parsing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_query_parameter_with_url_encoded_special_characters(client: Any) -> None:
    """Tests query parameter with URL encoded special characters."""
    params = {
        "name": "test&value=123",
    }
    response = await client.get("/query/basic", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "test&value=123"


async def test_59_format_email_failure(client: Any) -> None:
    """Query parameter with invalid email format should fail validation."""
    params = {
        "email": "not-an-email",
    }
    response = await client.get("/subscribe", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "format" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["format"] == "email"
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == "not-an-email"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "email"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Invalid email format"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_43_scientific_notation_float(client: Any) -> None:
    """Query parameter with scientific notation float should parse correctly."""
    params = {
        "threshold": "1.5e-3",
    }
    response = await client.get("/stats", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "threshold" in response_data
    assert response_data["threshold"] == 0.0015


async def test_63_format_uri_success(client: Any) -> None:
    """Query parameter with valid URI should be accepted."""
    params = {
        "url": "https://example.com/path?query=value",
    }
    response = await client.get("/redirect", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "url" in response_data
    assert response_data["url"] == "https://example.com/path?query=value"


async def test_boolean_query_parameter__numeric_1(client: Any) -> None:
    """Tests boolean query parameter with '1' converts to true."""
    params = {
        "flag": "1",
    }
    response = await client.get("/query/bool", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "flag" in response_data
    assert response_data["flag"] == True


async def test_string_query_param_with_min_length_constraint__fail(client: Any) -> None:
    """Tests string query parameter with min_length validation failure."""
    params = {
        "name": "ab",
    }
    response = await client.get("/query/str-min-length", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "min_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["min_length"] == 3
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "ab"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "name"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should have at least 3 characters"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_too_short"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_optional_string_query_parameter__provided(client: Any) -> None:
    """Tests optional string parameter with value provided."""
    params = {
        "query": "baz",
    }
    response = await client.get("/query/optional", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "foo bar baz"


async def test_list_of_integers__multiple_values(client: Any) -> None:
    """Tests list query parameter with multiple integer values using same key."""
    params = {
        "device_ids": [1, 2],
    }
    response = await client.get("/query/list", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert len(response_data) == 2
    assert response_data[0] == 1
    assert response_data[1] == 2


async def test_integer_query_param_with_lt_constraint__valid(client: Any) -> None:
    """Tests integer query parameter with lt validation, value below limit."""
    params = {
        "value": "49",
    }
    response = await client.get("/query/int-lt", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "value" in response_data
    assert response_data["value"] == 49


async def test_42_negative_integer_query_param(client: Any) -> None:
    """Query parameter with negative integer value should be parsed correctly."""
    params = {
        "offset": "-10",
    }
    response = await client.get("/items/negative", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "offset" in response_data
    assert response_data["offset"] == -10


async def test_46_string_maxlength_validation_failure(client: Any) -> None:
    """String query parameter exceeding maxLength constraint should fail validation."""
    params = {
        "term": "this_is_way_too_long",
    }
    response = await client.get("/search", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "actual_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["actual_length"] == 21
    assert "max_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["max_length"] == 10
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "term"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String length must not exceed 10"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_56_array_maxitems_constraint_failure(client: Any) -> None:
    """Array query parameter exceeding maxItems constraint should fail validation."""
    params = {
        "tags": ["a", "b", "c", "d", "e", "f"],
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "actual_items" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["actual_items"] == 6
    assert "max_items" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["max_items"] == 5
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "tags"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Array must not contain more than 5 items"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_string_query_param_with_regex_pattern__fail(client: Any) -> None:
    """Tests string query parameter with regex pattern validation failure."""
    params = {
        "code": "abc123",
    }
    response = await client.get("/query/pattern", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "pattern" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["pattern"] == "^[0-9]{3,}$"
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "abc123"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "code"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should match pattern '^[0-9]{3,}$'"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_pattern_mismatch"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_44_string_minlength_validation_success(client: Any) -> None:
    """String query parameter meeting minLength constraint should succeed."""
    params = {
        "term": "foo",
    }
    response = await client.get("/search", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "term" in response_data
    assert response_data["term"] == "foo"


async def test_61_format_ipv4_failure(client: Any) -> None:
    """Query parameter with invalid IPv4 address should fail validation."""
    params = {
        "ip": "999.999.999.999",
    }
    response = await client.get("/network", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "format" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["format"] == "ipv4"
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == "999.999.999.999"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "ip"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Invalid IPv4 address format"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_48_pattern_validation_email_failure(client: Any) -> None:
    """String query parameter not matching regex pattern should fail validation."""
    params = {
        "email": "invalid-email",
    }
    response = await client.get("/subscribe", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "pattern" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["pattern"] == "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == "invalid-email"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "email"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String does not match pattern"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_required_integer_query_parameter__missing(client: Any) -> None:
    """Tests a required integer query parameter without providing value, should return 422."""
    response = await client.get("/query/int")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == None
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "query"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Field required"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "missing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_query_parameter_with_special_characters__url_encoding(client: Any) -> None:
    """Tests query parameters with special characters that need URL encoding."""
    params = {
        "email": "x@test.com",
        "special": "&@A.ac",
    }
    response = await client.get("/test", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "email" in response_data
    assert response_data["email"] == "x@test.com"
    assert "special" in response_data
    assert response_data["special"] == "&@A.ac"


async def test_list_query_parameter__required_but_missing(client: Any) -> None:
    """Tests required list parameter without any values, should return 422."""
    response = await client.get("/query/list")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == None
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "device_ids"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Field required"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "missing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_required_string_query_parameter__success(client: Any) -> None:
    """Tests a required string query parameter with valid value."""
    params = {
        "query": "baz",
    }
    response = await client.get("/query", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "foo bar baz"


async def test_66_multipleof_constraint_success(client: Any) -> None:
    """Integer query parameter that is multiple of constraint should succeed."""
    params = {
        "quantity": "15",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "quantity" in response_data
    assert response_data["quantity"] == 15


async def test_53_integer_le_constraint_failure(client: Any) -> None:
    """Integer query parameter exceeding maximum should fail validation."""
    params = {
        "limit": "101",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "maximum" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["maximum"] == 100
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == 101
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "limit"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Value must not exceed 100"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_multiple_query_parameters_with_different_types(client: Any) -> None:
    """Tests multiple query parameters of different types in single request."""
    params = {
        "active": "true",
        "score": "95.5",
        "age": "30",
        "name": "john",
    }
    response = await client.get("/query/multi-type", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "active" in response_data
    assert response_data["active"] == True
    assert "age" in response_data
    assert response_data["age"] == 30
    assert "name" in response_data
    assert response_data["name"] == "john"
    assert "score" in response_data
    assert response_data["score"] == 95.5


async def test_71_array_separator_semicolon(client: Any) -> None:
    """Array query parameter with semicolon separator should parse correctly."""
    params = {
        "colors": "red;green;blue",
    }
    response = await client.get("/items?colors=red;green;blue", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "colors" in response_data
    assert len(response_data["colors"]) == 3
    assert response_data["colors"][0] == "red"
    assert response_data["colors"][1] == "green"
    assert response_data["colors"][2] == "blue"


async def test_70_array_separator_pipe(client: Any) -> None:
    """Array query parameter with pipe separator should parse correctly."""
    params = {
        "tags": "python|rust|typescript",
    }
    response = await client.get("/items?tags=python|rust|typescript", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "tags" in response_data
    assert len(response_data["tags"]) == 3
    assert response_data["tags"][0] == "python"
    assert response_data["tags"][1] == "rust"
    assert response_data["tags"][2] == "typescript"


async def test_integer_with_default_value__not_provided(client: Any) -> None:
    """Tests integer parameter with default value when not provided."""
    response = await client.get("/query/int/default")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "foo bar 10"


async def test_boolean_query_parameter__true(client: Any) -> None:
    """Tests boolean query parameter with 'true' string value."""
    params = {
        "flag": "true",
    }
    response = await client.get("/query/bool", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "flag" in response_data
    assert response_data["flag"] == True


async def test_integer_query_param_with_le_constraint__boundary(client: Any) -> None:
    """Tests integer query parameter with le validation at boundary value."""
    params = {
        "value": "100",
    }
    response = await client.get("/query/int-le", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "value" in response_data
    assert response_data["value"] == 100


async def test_float_query_param_with_ge_constraint__success(client: Any) -> None:
    """Tests float query parameter with ge validation at boundary."""
    params = {
        "price": "0.01",
    }
    response = await client.get("/query/float-ge", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "price" in response_data
    assert response_data["price"] == 0.01


async def test_51_integer_ge_constraint_boundary(client: Any) -> None:
    """Integer query parameter equal to minimum should succeed with ge constraint."""
    params = {
        "offset": "0",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "offset" in response_data
    assert response_data["offset"] == 0


async def test_optional_integer_query_parameter__missing(client: Any) -> None:
    """Tests optional integer parameter without value, should not error."""
    response = await client.get("/query/int/optional")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "foo bar None"


async def test_69_array_uniqueitems_failure(client: Any) -> None:
    """Array query parameter with duplicate items should fail when uniqueItems is true."""
    params = {
        "ids": ["1", "2", "2", "3"],
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "duplicate_index" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["duplicate_index"] == 2
    assert "duplicate_value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["duplicate_value"] == 2
    assert "unique_items" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["unique_items"] == True
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "ids"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Array items must be unique"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_72_array_separator_space(client: Any) -> None:
    """Array query parameter with space separator should parse correctly."""
    params = {
        "keywords": "rust web framework",
    }
    response = await client.get("/search?keywords=rust%20web%20framework", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "keywords" in response_data
    assert len(response_data["keywords"]) == 3
    assert response_data["keywords"][0] == "rust"
    assert response_data["keywords"][1] == "web"
    assert response_data["keywords"][2] == "framework"


async def test_string_validation_with_regex__failure(client: Any) -> None:
    """Tests string parameter with regex pattern validation - non-matching pattern returns 422."""
    params = {
        "item_query": "nonregexquery",
    }
    response = await client.get("/items/", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "pattern" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["pattern"] == "^fixedquery$"
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "nonregexquery"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "item_query"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should match pattern '^fixedquery$'"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_pattern_mismatch"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_65_format_hostname_success(client: Any) -> None:
    """Query parameter with valid hostname should be accepted."""
    params = {
        "host": "api.example.com",
    }
    response = await client.get("/dns", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "host" in response_data
    assert response_data["host"] == "api.example.com"


async def test_query_parameter_with_url_encoded_space(client: Any) -> None:
    """Tests query parameter with URL encoded space character."""
    params = {
        "name": "hello world",
    }
    response = await client.get("/query/basic", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "hello world"


async def test_list_of_strings__multiple_values(client: Any) -> None:
    """Tests list of string query parameters with multiple values."""
    params = {
        "q": ["foo", "bar"],
    }
    response = await client.get("/items/", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "q" in response_data
    assert len(response_data["q"]) == 2
    assert response_data["q"][0] == "foo"
    assert response_data["q"][1] == "bar"


async def test_optional_query_parameter_with_default_value(client: Any) -> None:
    """Tests optional query parameter that uses default when not provided."""
    response = await client.get("/query/optional-default")

    assert response.status_code == 200
    response_data = response.json()
    assert "limit" in response_data
    assert response_data["limit"] == 10


async def test_62_format_ipv6_success(client: Any) -> None:
    """Query parameter with valid IPv6 address should be accepted."""
    params = {
        "ip": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
    }
    response = await client.get("/network/ipv6", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "ip" in response_data
    assert response_data["ip"] == "2001:0db8:85a3:0000:0000:8a2e:0370:7334"


async def test_array_query_parameter__single_value(client: Any) -> None:
    """Tests array query parameter with single value."""
    params = {
        "tags": "apple",
    }
    response = await client.get("/query/list-default", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert len(response_data) == 1
    assert response_data[0] == "apple"


async def test_optional_string_query_parameter__missing(client: Any) -> None:
    """Tests optional string parameter without value, should return None/null."""
    response = await client.get("/query/optional")

    assert response.status_code == 200
    response_data = response.json()
    assert response_data == "foo bar None"


async def test_datetime_query_parameter__success(client: Any) -> None:
    """Tests datetime query parameter with valid ISO datetime format."""
    params = {
        "timestamp": "2024-01-15T10:30:00Z",
    }
    response = await client.get("/query/datetime", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "timestamp" in response_data
    assert response_data["timestamp"] == "2024-01-15T10:30:00Z"


async def test_uuid_query_parameter__invalid_format(client: Any) -> None:
    """Tests UUID query parameter with invalid UUID format."""
    params = {
        "item_id": "not-a-uuid",
    }
    response = await client.get("/query/uuid", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "not-a-uuid"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "item_id"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be a valid UUID"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "uuid_parsing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_array_query_parameter__empty_array(client: Any) -> None:
    """Tests array query parameter when no values are provided."""
    response = await client.get("/query/list-default")

    assert response.status_code == 200
    response_data = response.json()
    assert len(response_data) == 0


async def test_enum_query_parameter__success(client: Any) -> None:
    """Tests enum query parameter with valid enum value."""
    params = {
        "model": "alexnet",
    }
    response = await client.get("/query/enum", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "model" in response_data
    assert response_data["model"] == "alexnet"


async def test_uuid_query_parameter__success(client: Any) -> None:
    """Tests UUID query parameter with valid UUID format."""
    params = {
        "item_id": "c892496f-b1fd-4b91-bdb8-b46f92df1716",
    }
    response = await client.get("/query/uuid", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "item_id" in response_data
    assert response_data["item_id"] == "c892496f-b1fd-4b91-bdb8-b46f92df1716"


async def test_50_integer_gt_constraint_failure(client: Any) -> None:
    """Integer query parameter equal to exclusive minimum should fail validation."""
    params = {
        "limit": "0",
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "exclusive_minimum" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["exclusive_minimum"] == 0
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == 0
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "limit"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Value must be greater than 0"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_64_format_uri_failure(client: Any) -> None:
    """Query parameter with invalid URI should fail validation."""
    params = {
        "url": "not a uri",
    }
    response = await client.get("/redirect", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "format" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["format"] == "uri"
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == "not a uri"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "url"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Invalid URI format"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_54_array_minitems_constraint_success(client: Any) -> None:
    """Array query parameter meeting minItems constraint should succeed."""
    params = {
        "ids": ["1", "2", "3"],
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "ids" in response_data
    assert len(response_data["ids"]) == 3
    assert response_data["ids"][0] == 1
    assert response_data["ids"][1] == 2
    assert response_data["ids"][2] == 3


async def test_55_array_minitems_constraint_failure(client: Any) -> None:
    """Array query parameter below minItems constraint should fail validation."""
    params = {
        "ids": ["1"],
    }
    response = await client.get("/items", query_params=params)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "actual_items" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["actual_items"] == 1
    assert "min_items" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["min_items"] == 2
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "ids"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Array must contain at least 2 items"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_60_format_ipv4_success(client: Any) -> None:
    """Query parameter with valid IPv4 address should be accepted."""
    params = {
        "ip": "192.168.1.1",
    }
    response = await client.get("/network", query_params=params)

    assert response.status_code == 200
    response_data = response.json()
    assert "ip" in response_data
    assert response_data["ip"] == "192.168.1.1"


