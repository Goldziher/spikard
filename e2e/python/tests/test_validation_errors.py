"""E2E tests for validation_errors."""

from app.main import (
    create_app_validation_errors_09_multiple_validation_errors,
    create_app_validation_errors_10_nested_error_path,
    create_app_validation_errors_array_item_validation_error,
    create_app_validation_errors_array_max_items_constraint_violation,
    create_app_validation_errors_array_min_items_constraint_violation,
    create_app_validation_errors_body_field_type_error_string_for_float,
    create_app_validation_errors_header_validation_error,
    create_app_validation_errors_invalid_boolean_value,
    create_app_validation_errors_invalid_datetime_format,
    create_app_validation_errors_invalid_enum_value,
    create_app_validation_errors_invalid_uuid_format,
    create_app_validation_errors_malformed_json_body,
    create_app_validation_errors_missing_required_body_field,
    create_app_validation_errors_missing_required_query_parameter,
    create_app_validation_errors_multiple_validation_errors,
    create_app_validation_errors_nested_object_validation_error,
    create_app_validation_errors_numeric_constraint_violation_gt_greater_than,
    create_app_validation_errors_numeric_constraint_violation_le_less_than_or_equal,
    create_app_validation_errors_query_param_type_error_string_provided_for_int,
    create_app_validation_errors_string_max_length_constraint_violation,
    create_app_validation_errors_string_min_length_constraint_violation,
    create_app_validation_errors_string_regex_pattern_mismatch,
)

from spikard.testing import TestClient


async def test_invalid_uuid_format() -> None:
    """Tests validation error when UUID format is invalid."""

    app = create_app_validation_errors_invalid_uuid_format()
    client = TestClient(app)

    response = await client.get("/items/not-a-uuid")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_invalid_boolean_value() -> None:
    """Tests validation error when boolean value is invalid."""

    app = create_app_validation_errors_invalid_boolean_value()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/?q=test&is_active=maybe", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_missing_required_query_parameter() -> None:
    """Tests validation error when required query param is missing."""

    app = create_app_validation_errors_missing_required_query_parameter()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_array_max_items_constraint_violation() -> None:
    """Tests validation error when array has more items than max_items."""

    app = create_app_validation_errors_array_max_items_constraint_violation()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {
        "name": "Item",
        "price": 10.0,
        "tags": ["tag1", "tag2", "tag3", "tag4", "tag5", "tag6", "tag7", "tag8", "tag9", "tag10", "tag11"],
    }
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_numeric_constraint_violation_gt_greater_than() -> None:
    """Tests validation error when value violates gt constraint."""

    app = create_app_validation_errors_numeric_constraint_violation_gt_greater_than()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/?q=test&price=0", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_string_regex_pattern_mismatch() -> None:
    """Tests validation error when string doesn't match regex pattern."""

    app = create_app_validation_errors_string_regex_pattern_mismatch()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/?q=invalid!", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_invalid_enum_value() -> None:
    """Tests validation error when value is not in allowed enum values."""

    app = create_app_validation_errors_invalid_enum_value()
    client = TestClient(app)

    response = await client.get("/models/invalid_model")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_string_min_length_constraint_violation() -> None:
    """Tests validation error when string is shorter than min_length."""

    app = create_app_validation_errors_string_min_length_constraint_violation()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/?q=ab", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_multiple_validation_errors() -> None:
    """Tests multiple validation errors returned in single response."""

    app = create_app_validation_errors_multiple_validation_errors()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "X", "price": -10, "quantity": "not_a_number"}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_string_max_length_constraint_violation() -> None:
    """Tests validation error when string exceeds max_length."""

    app = create_app_validation_errors_string_max_length_constraint_violation()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get(
        "/items/?q=this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter",
        headers=headers,
    )

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_nested_object_validation_error() -> None:
    """Tests validation error in nested object field."""

    app = create_app_validation_errors_nested_object_validation_error()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {
        "name": "Product",
        "price": 10.0,
        "seller": {"address": {"city": "SF", "zip_code": "123"}, "name": "Jo"},
    }
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_10_nested_error_path() -> None:
    """Validation error in nested object should have correct path in loc."""

    app = create_app_validation_errors_10_nested_error_path()
    client = TestClient(app)

    json_data = {"profile": {"contact": {"email": "invalid"}}}
    response = await client.post("/profiles", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_invalid_datetime_format() -> None:
    """Tests validation error when datetime format is invalid."""

    app = create_app_validation_errors_invalid_datetime_format()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"created_at": "not-a-datetime", "name": "Item", "price": 10.0}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_array_item_validation_error() -> None:
    """Tests validation error for invalid item within array."""

    app = create_app_validation_errors_array_item_validation_error()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item", "price": 10.0, "tags": ["tag1", "tag2", 123]}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_missing_required_body_field() -> None:
    """Tests validation error when required body field is missing."""

    app = create_app_validation_errors_missing_required_body_field()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item"}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_body_field_type_error_string_for_float() -> None:
    """Tests validation error when body field has wrong type."""

    app = create_app_validation_errors_body_field_type_error_string_for_float()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item", "price": "not_a_float"}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_malformed_json_body() -> None:
    """Tests validation error when request body contains malformed JSON."""

    app = create_app_validation_errors_malformed_json_body()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = '{"name": "Item", "price": }'
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 400
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Invalid request format"


async def test_query_param_type_error_string_provided_for_int() -> None:
    """Tests validation error when string is provided for integer query param."""

    app = create_app_validation_errors_query_param_type_error_string_provided_for_int()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/?q=test&skip=not_a_number", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_header_validation_error() -> None:
    """Tests validation error when required header is missing."""

    app = create_app_validation_errors_header_validation_error()
    client = TestClient(app)

    response = await client.get("/items/?q=test")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_09_multiple_validation_errors() -> None:
    """Multiple validation errors should be returned together in batch."""

    app = create_app_validation_errors_09_multiple_validation_errors()
    client = TestClient(app)

    json_data = {"age": 15, "email": "invalid-email", "name": "ab"}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_numeric_constraint_violation_le_less_than_or_equal() -> None:
    """Tests validation error when value violates le constraint."""

    app = create_app_validation_errors_numeric_constraint_violation_le_less_than_or_equal()
    client = TestClient(app)

    headers = {
        "x-token": "test-token",
    }
    response = await client.get("/items/?q=test&limit=101", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_array_min_items_constraint_violation() -> None:
    """Tests validation error when array has fewer items than min_items."""

    app = create_app_validation_errors_array_min_items_constraint_violation()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item", "price": 10.0, "tags": []}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data
