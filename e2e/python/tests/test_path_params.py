"""E2E tests for path_params."""

import pytest
from typing import Any

async def test_boolean_path_parameter__true(client: Any) -> None:
    """Tests boolean path parameter with 'True' string value."""
    response = await client.get("/path/bool/True")

    assert response.status_code == 200
    response_data = response.json()
    assert "item_id" in response_data
    assert response_data["item_id"] == True


async def test_29_decimal_path_param_success(client: Any) -> None:
    """Path parameter with decimal/money value should be accepted."""
    response = await client.get("/prices/19.99")

    assert response.status_code == 200
    response_data = response.json()
    assert "amount" in response_data
    assert response_data["amount"] == "19.99"


async def test_integer_path_parameter_with_combined_lt_and_gt_constraints__success(client: Any) -> None:
    """Tests integer path parameter with both lt and gt validation (range check)."""
    response = await client.get("/path/param-lt-gt/2")

    assert response.status_code == 200
    response_data = response.json()
    assert "item_id" in response_data
    assert response_data["item_id"] == 2


async def test_33_string_pattern_path_success(client: Any) -> None:
    """Path parameter matching regex pattern should succeed."""
    response = await client.get("/repos/spikard-labs/spikard-http")

    assert response.status_code == 200
    response_data = response.json()
    assert "owner" in response_data
    assert response_data["owner"] == "spikard-labs"
    assert "repo" in response_data
    assert response_data["repo"] == "spikard-http"


async def test_31_string_minlength_path_failure(client: Any) -> None:
    """Path parameter with string below minLength constraint should fail."""
    response = await client.get("/users/ab")

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
    assert response_data["errors"][0]["loc"][0] == "path"
    assert response_data["errors"][0]["loc"][1] == "username"
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


async def test_35_negative_integer_path_param(client: Any) -> None:
    """Path parameter with negative integer should be parsed correctly."""
    response = await client.get("/offset/-100")

    assert response.status_code == 200
    response_data = response.json()
    assert "value" in response_data
    assert response_data["value"] == -100


async def test_enum_path_parameter__invalid_value(client: Any) -> None:
    """Tests enum path parameter with invalid enum value returns 422."""
    response = await client.get("/models/foo")

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
    assert response_data["errors"][0]["input"] == "foo"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "path"
    assert response_data["errors"][0]["loc"][1] == "model_name"
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


async def test_27_datetime_format_path_param_success(client: Any) -> None:
    """Path parameter with valid ISO 8601 datetime should be accepted."""
    response = await client.get("/bookings/2025-10-30T14:30:00Z")

    assert response.status_code == 200
    response_data = response.json()
    assert "timestamp" in response_data
    assert response_data["timestamp"] == "2025-10-30T14:30:00Z"


async def test_25_date_format_invalid_failure(client: Any) -> None:
    """Path parameter with invalid date format should fail validation."""
    response = await client.get("/events/2025-13-45")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "format" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["format"] == "date"
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == "2025-13-45"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "path"
    assert response_data["errors"][0]["loc"][1] == "date"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Invalid date format"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_integer_path_parameter_with_lt_constraint__success(client: Any) -> None:
    """Tests integer path parameter with lt (less than) validation."""
    response = await client.get("/path/param-lt/2")

    assert response.status_code == 200
    response_data = response.json()
    assert "item_id" in response_data
    assert response_data["item_id"] == 2


async def test_integer_path_parameter_with_gt_constraint__success(client: Any) -> None:
    """Tests integer path parameter with gt (greater than) validation succeeds."""
    response = await client.get("/path/param-gt/42")

    assert response.status_code == 200
    response_data = response.json()
    assert "item_id" in response_data
    assert response_data["item_id"] == 42


async def test_28_duration_format_path_param_success(client: Any) -> None:
    """Path parameter with valid ISO 8601 duration should be accepted."""
    response = await client.get("/delays/P1DT2H30M")

    assert response.status_code == 200
    response_data = response.json()
    assert "duration" in response_data
    assert response_data["duration"] == "P1DT2H30M"


async def test_path_parameter_type_syntax_with_override(client: Any) -> None:
    """Tests that explicit parameter schema overrides auto-generated type syntax schema."""
    response = await client.get("/type-syntax/items-count/50")

    assert response.status_code == 200
    response_data = response.json()
    assert "count" in response_data
    assert response_data["count"] == "50"


async def test_20_uuid_v3_path_param_success(client: Any) -> None:
    """Path parameter with valid UUID v3 should be accepted."""
    response = await client.get("/items/e8b5a51d-11c8-3310-a6ab-367563f20686")

    assert response.status_code == 200
    response_data = response.json()
    assert "id" in response_data
    assert response_data["id"] == "e8b5a51d-11c8-3310-a6ab-367563f20686"


async def test_integer_path_parameter__invalid_string(client: Any) -> None:
    """Tests integer path parameter with non-numeric string returns 422."""
    response = await client.get("/path/int/foobar")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "foobar"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "path"
    assert response_data["errors"][0]["loc"][1] == "item_id"
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


async def test_30_string_minlength_path_success(client: Any) -> None:
    """Path parameter with string meeting minLength constraint should succeed."""
    response = await client.get("/users/alice")

    assert response.status_code == 200
    response_data = response.json()
    assert "username" in response_data
    assert response_data["username"] == "alice"


async def test_integer_path_parameter_with_le_constraint__success(client: Any) -> None:
    """Tests integer path parameter with le (less than or equal) validation at boundary."""
    response = await client.get("/path/param-le/3")

    assert response.status_code == 200
    response_data = response.json()
    assert "item_id" in response_data
    assert response_data["item_id"] == 3


async def test_path_parameter_type_syntax__invalid_uuid(client: Any) -> None:
    """Tests that :uuid type syntax properly validates and rejects invalid UUIDs."""
    response = await client.get("/type-syntax/items/not-a-uuid")

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
    assert response_data["errors"][0]["loc"][0] == "path"
    assert response_data["errors"][0]["loc"][1] == "id"
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


async def test_path_type_parameter__file_path(client: Any) -> None:
    """Tests path type parameter that captures remaining path segments."""
    response = await client.get("/files/home/johndoe/myfile.txt")

    assert response.status_code == 200
    response_data = response.json()
    assert "file_path" in response_data
    assert response_data["file_path"] == "home/johndoe/myfile.txt"


async def test_path_parameter_with_type_syntax__uuid(client: Any) -> None:
    """Tests path parameter with :uuid type syntax auto-generates correct schema."""
    response = await client.get("/type-syntax/items/550e8400-e29b-41d4-a716-446655440000")

    assert response.status_code == 200
    response_data = response.json()
    assert "id" in response_data
    assert response_data["id"] == "550e8400-e29b-41d4-a716-446655440000"


async def test_32_string_maxlength_path_failure(client: Any) -> None:
    """Path parameter with string exceeding maxLength constraint should fail."""
    response = await client.get("/users/this_username_is_way_too_long_to_be_valid")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "actual_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["actual_length"] == 42
    assert "max_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["max_length"] == 20
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "path"
    assert response_data["errors"][0]["loc"][1] == "username"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String length must not exceed 20"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_integer_path_parameter__success(client: Any) -> None:
    """Tests integer path parameter with valid value."""
    response = await client.get("/path/int/42")

    assert response.status_code == 200
    response_data = response.json()
    assert "item_id" in response_data
    assert response_data["item_id"] == 42


async def test_34_string_pattern_path_failure(client: Any) -> None:
    """Path parameter not matching regex pattern should fail."""
    response = await client.get("/repos/invalid@owner")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "pattern" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["pattern"] == "^[a-zA-Z0-9-]+$"
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == "invalid@owner"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "path"
    assert response_data["errors"][0]["loc"][1] == "owner"
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


async def test_21_uuid_v5_path_param_success(client: Any) -> None:
    """Path parameter with valid UUID v5 should be accepted."""
    response = await client.get("/items/630eb68f-e0fa-5ecc-887a-7c7a62614681")

    assert response.status_code == 200
    response_data = response.json()
    assert "id" in response_data
    assert response_data["id"] == "630eb68f-e0fa-5ecc-887a-7c7a62614681"


async def test_string_path_parameter_with_max_length__failure(client: Any) -> None:
    """Tests string path parameter with max_length validation fails when too long."""
    response = await client.get("/path/param-maxlength/foobar")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "max_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["max_length"] == 3
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "foobar"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "path"
    assert response_data["errors"][0]["loc"][1] == "item_id"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should have at most 3 characters"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_too_long"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_string_path_parameter_with_min_length__failure(client: Any) -> None:
    """Tests string path parameter with min_length validation fails."""
    response = await client.get("/path/param-minlength/fo")

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
    assert response_data["errors"][0]["input"] == "fo"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "path"
    assert response_data["errors"][0]["loc"][1] == "item_id"
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


async def test_multiple_path_parameters__success(client: Any) -> None:
    """Tests multiple path parameters in single route."""
    response = await client.get("/1.0/1/abc/c892496f-b1fd-4b91-bdb8-b46f92df1716")

    assert response.status_code == 200
    response_data = response.json()
    assert "order_id" in response_data
    assert response_data["order_id"] == "c892496f-b1fd-4b91-bdb8-b46f92df1716"
    assert "service_id" in response_data
    assert response_data["service_id"] == 1
    assert "user_id" in response_data
    assert response_data["user_id"] == "abc"
    assert "version" in response_data
    assert response_data["version"] == 1.0


async def test_date_path_parameter__success(client: Any) -> None:
    """Tests date path parameter with ISO format date."""
    response = await client.get("/date/2023-07-15")

    assert response.status_code == 200
    response_data = response.json()
    assert "date_param" in response_data
    assert response_data["date_param"] == "2023-07-15"


async def test_integer_path_parameter_with_gt_constraint__failure(client: Any) -> None:
    """Tests integer path parameter with gt validation fails when value too small."""
    response = await client.get("/path/param-gt/2")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "gt" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["gt"] == 3
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == 2
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "path"
    assert response_data["errors"][0]["loc"][1] == "item_id"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be greater than 3"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "greater_than"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_24_date_format_path_param_success(client: Any) -> None:
    """Path parameter with valid ISO date format should be accepted."""
    response = await client.get("/events/2025-10-30")

    assert response.status_code == 200
    response_data = response.json()
    assert "date" in response_data
    assert response_data["date"] == "2025-10-30"


async def test_float_path_parameter__success(client: Any) -> None:
    """Tests float path parameter with valid value."""
    response = await client.get("/path/float/42.5")

    assert response.status_code == 200
    response_data = response.json()
    assert "item_id" in response_data
    assert response_data["item_id"] == 42.5


async def test_path_parameter_with_type_syntax__integer(client: Any) -> None:
    """Tests path parameter with :int type syntax auto-generates correct schema."""
    response = await client.get("/type-syntax/users/42")

    assert response.status_code == 200
    response_data = response.json()
    assert "user_id" in response_data
    assert response_data["user_id"] == "42"


async def test_string_path_parameter__success(client: Any) -> None:
    """Tests string path parameter with valid value."""
    response = await client.get("/path/str/foobar")

    assert response.status_code == 200
    response_data = response.json()
    assert "item_id" in response_data
    assert response_data["item_id"] == "foobar"


async def test_uuid_path_parameter__success(client: Any) -> None:
    """Tests UUID path parameter with valid UUID format."""
    response = await client.get("/items/ec38df32-ceda-4cfa-9b4a-1aeb94ad551a")

    assert response.status_code == 200
    response_data = response.json()
    assert "item_id" in response_data
    assert response_data["item_id"] == "ec38df32-ceda-4cfa-9b4a-1aeb94ad551a"


async def test_integer_path_parameter_with_ge_constraint__success(client: Any) -> None:
    """Tests integer path parameter with ge (greater than or equal) validation at boundary."""
    response = await client.get("/path/param-ge/3")

    assert response.status_code == 200
    response_data = response.json()
    assert "item_id" in response_data
    assert response_data["item_id"] == 3


async def test_enum_path_parameter__success(client: Any) -> None:
    """Tests enum path parameter with valid enum value."""
    response = await client.get("/models/alexnet")

    assert response.status_code == 200
    response_data = response.json()
    assert "model_name" in response_data
    assert response_data["model_name"] == "alexnet"


async def test_boolean_path_parameter__numeric_1(client: Any) -> None:
    """Tests boolean path parameter with '1' converts to true."""
    response = await client.get("/path/bool/1")

    assert response.status_code == 200
    response_data = response.json()
    assert "item_id" in response_data
    assert response_data["item_id"] == True


