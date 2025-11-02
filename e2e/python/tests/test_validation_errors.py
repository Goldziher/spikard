"""E2E tests for validation_errors."""

import pytest
from typing import Any

async def test_invalid_uuid_format() -> None:
    """Tests validation error when UUID format is invalid."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_invalid_uuid_format

    app = create_app_validation_errors_invalid_uuid_format()
    client = TestClient(app)

    response = await client.get("/items/not-a-uuid")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "path"
    assert response_data["errors"][0]["loc"][1] == "item_id"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_invalid_boolean_value() -> None:
    """Tests validation error when boolean value is invalid."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_invalid_boolean_value

    app = create_app_validation_errors_invalid_boolean_value()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/?q=test&is_active=maybe", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "is_active"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_missing_required_query_parameter() -> None:
    """Tests validation error when required query param is missing."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_missing_required_query_parameter

    app = create_app_validation_errors_missing_required_query_parameter()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "q"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_array_max_items_constraint_violation() -> None:
    """Tests validation error when array has more items than max_items."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_array_max_items_constraint_violation

    app = create_app_validation_errors_array_max_items_constraint_violation()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item", "price": 10.0, "tags": ["tag1", "tag2", "tag3", "tag4", "tag5", "tag6", "tag7", "tag8", "tag9", "tag10", "tag11"]}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert len(response_data["errors"][0]["input"]) == 11
    assert response_data["errors"][0]["input"][0] == "tag1"
    assert response_data["errors"][0]["input"][1] == "tag2"
    assert response_data["errors"][0]["input"][2] == "tag3"
    assert response_data["errors"][0]["input"][3] == "tag4"
    assert response_data["errors"][0]["input"][4] == "tag5"
    assert response_data["errors"][0]["input"][5] == "tag6"
    assert response_data["errors"][0]["input"][6] == "tag7"
    assert response_data["errors"][0]["input"][7] == "tag8"
    assert response_data["errors"][0]["input"][8] == "tag9"
    assert response_data["errors"][0]["input"][9] == "tag10"
    assert response_data["errors"][0]["input"][10] == "tag11"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "tags"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_numeric_constraint_violation_gt_greater_than() -> None:
    """Tests validation error when value violates gt constraint."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_numeric_constraint_violation_gt_greater_than

    app = create_app_validation_errors_numeric_constraint_violation_gt_greater_than()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/?q=test&price=0", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "price"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_string_regex_pattern_mismatch() -> None:
    """Tests validation error when string doesn't match regex pattern."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_string_regex_pattern_mismatch

    app = create_app_validation_errors_string_regex_pattern_mismatch()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/?q=invalid!", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "q"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_invalid_enum_value() -> None:
    """Tests validation error when value is not in allowed enum values."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_invalid_enum_value

    app = create_app_validation_errors_invalid_enum_value()
    client = TestClient(app)

    response = await client.get("/models/invalid_model")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "path"
    assert response_data["errors"][0]["loc"][1] == "model_name"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_string_min_length_constraint_violation() -> None:
    """Tests validation error when string is shorter than min_length."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_string_min_length_constraint_violation

    app = create_app_validation_errors_string_min_length_constraint_violation()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/?q=ab", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "q"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_multiple_validation_errors() -> None:
    """Tests multiple validation errors returned in single response."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_multiple_validation_errors

    app = create_app_validation_errors_multiple_validation_errors()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "X", "price": -10, "quantity": "not_a_number"}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "3 validation errors in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 3
    assert "ctx" in response_data["errors"][0]
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "name"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "ctx" in response_data["errors"][1]
    assert "input" in response_data["errors"][1]
    assert "loc" in response_data["errors"][1]
    assert len(response_data["errors"][1]["loc"]) == 2
    assert response_data["errors"][1]["loc"][0] == "body"
    assert response_data["errors"][1]["loc"][1] == "price"
    assert "msg" in response_data["errors"][1]
    assert "type" in response_data["errors"][1]
    assert "input" in response_data["errors"][2]
    assert "loc" in response_data["errors"][2]
    assert len(response_data["errors"][2]["loc"]) == 2
    assert response_data["errors"][2]["loc"][0] == "body"
    assert response_data["errors"][2]["loc"][1] == "quantity"
    assert "msg" in response_data["errors"][2]
    assert "type" in response_data["errors"][2]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_string_max_length_constraint_violation() -> None:
    """Tests validation error when string exceeds max_length."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_string_max_length_constraint_violation

    app = create_app_validation_errors_string_max_length_constraint_violation()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/?q=this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "q"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_nested_object_validation_error() -> None:
    """Tests validation error in nested object field."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_nested_object_validation_error

    app = create_app_validation_errors_nested_object_validation_error()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Product", "price": 10.0, "seller": {"address": {"city": "SF", "zip_code": "123"}, "name": "Jo"}}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "3 validation errors in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 3
    assert "ctx" in response_data["errors"][0]
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 4
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "seller"
    assert response_data["errors"][0]["loc"][2] == "address"
    assert response_data["errors"][0]["loc"][3] == "city"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "ctx" in response_data["errors"][1]
    assert "input" in response_data["errors"][1]
    assert "loc" in response_data["errors"][1]
    assert len(response_data["errors"][1]["loc"]) == 4
    assert response_data["errors"][1]["loc"][0] == "body"
    assert response_data["errors"][1]["loc"][1] == "seller"
    assert response_data["errors"][1]["loc"][2] == "address"
    assert response_data["errors"][1]["loc"][3] == "zip_code"
    assert "msg" in response_data["errors"][1]
    assert "type" in response_data["errors"][1]
    assert "ctx" in response_data["errors"][2]
    assert "input" in response_data["errors"][2]
    assert "loc" in response_data["errors"][2]
    assert len(response_data["errors"][2]["loc"]) == 3
    assert response_data["errors"][2]["loc"][0] == "body"
    assert response_data["errors"][2]["loc"][1] == "seller"
    assert response_data["errors"][2]["loc"][2] == "name"
    assert "msg" in response_data["errors"][2]
    assert "type" in response_data["errors"][2]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_10_nested_error_path() -> None:
    """Validation error in nested object should have correct path in loc."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_10_nested_error_path

    app = create_app_validation_errors_10_nested_error_path()
    client = TestClient(app)

    json_data = {"profile": {"contact": {"email": "invalid"}}}
    response = await client.post("/profiles", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 4
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "profile"
    assert response_data["errors"][0]["loc"][2] == "contact"
    assert response_data["errors"][0]["loc"][3] == "email"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_invalid_datetime_format() -> None:
    """Tests validation error when datetime format is invalid."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_invalid_datetime_format

    app = create_app_validation_errors_invalid_datetime_format()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"created_at": "not-a-datetime", "name": "Item", "price": 10.0}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "created_at"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_array_item_validation_error() -> None:
    """Tests validation error for invalid item within array."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_array_item_validation_error

    app = create_app_validation_errors_array_item_validation_error()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item", "price": 10.0, "tags": ["tag1", "tag2", 123]}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 3
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "tags"
    assert response_data["errors"][0]["loc"][2] == "2"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_missing_required_body_field() -> None:
    """Tests validation error when required body field is missing."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_missing_required_body_field

    app = create_app_validation_errors_missing_required_body_field()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item"}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert "name" in response_data["errors"][0]["input"]
    assert response_data["errors"][0]["input"]["name"] == "Item"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "price"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_body_field_type_error_string_for_float() -> None:
    """Tests validation error when body field has wrong type."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_body_field_type_error_string_for_float

    app = create_app_validation_errors_body_field_type_error_string_for_float()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item", "price": "not_a_float"}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "price"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_malformed_json_body() -> None:
    """Tests validation error when request body contains malformed JSON."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_malformed_json_body

    app = create_app_validation_errors_malformed_json_body()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = "{\"name\": \"Item\", \"price\": }"
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 400
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Invalid request format"


async def test_query_param_type_error_string_provided_for_int() -> None:
    """Tests validation error when string is provided for integer query param."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_query_param_type_error_string_provided_for_int

    app = create_app_validation_errors_query_param_type_error_string_provided_for_int()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/?q=test&skip=not_a_number", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "skip"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_header_validation_error() -> None:
    """Tests validation error when required header is missing."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_header_validation_error

    app = create_app_validation_errors_header_validation_error()
    client = TestClient(app)

    response = await client.get("/items/?q=test")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "header"
    assert response_data["errors"][0]["loc"][1] == "x-token"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_09_multiple_validation_errors() -> None:
    """Multiple validation errors should be returned together in batch."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_09_multiple_validation_errors

    app = create_app_validation_errors_09_multiple_validation_errors()
    client = TestClient(app)

    json_data = {"age": 15, "email": "invalid-email", "name": "ab"}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "3 validation errors in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 3
    assert "ctx" in response_data["errors"][0]
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "age"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "ctx" in response_data["errors"][1]
    assert "input" in response_data["errors"][1]
    assert "loc" in response_data["errors"][1]
    assert len(response_data["errors"][1]["loc"]) == 2
    assert response_data["errors"][1]["loc"][0] == "body"
    assert response_data["errors"][1]["loc"][1] == "email"
    assert "msg" in response_data["errors"][1]
    assert "type" in response_data["errors"][1]
    assert "ctx" in response_data["errors"][2]
    assert "input" in response_data["errors"][2]
    assert "loc" in response_data["errors"][2]
    assert len(response_data["errors"][2]["loc"]) == 2
    assert response_data["errors"][2]["loc"][0] == "body"
    assert response_data["errors"][2]["loc"][1] == "name"
    assert "msg" in response_data["errors"][2]
    assert "type" in response_data["errors"][2]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_numeric_constraint_violation_le_less_than_or_equal() -> None:
    """Tests validation error when value violates le constraint."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_numeric_constraint_violation_le_less_than_or_equal

    app = create_app_validation_errors_numeric_constraint_violation_le_less_than_or_equal()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/?q=test&limit=101", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "input" in response_data["errors"][0]
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "limit"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_array_min_items_constraint_violation() -> None:
    """Tests validation error when array has fewer items than min_items."""
    from spikard.testing import TestClient
    from app.main import create_app_validation_errors_array_min_items_constraint_violation

    app = create_app_validation_errors_array_min_items_constraint_violation()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item", "price": 10.0, "tags": []}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert len(response_data["errors"][0]["input"]) == 0
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "tags"
    assert "msg" in response_data["errors"][0]
    assert "type" in response_data["errors"][0]
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


