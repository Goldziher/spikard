"""E2E tests for validation_errors."""


async def test_invalid_uuid_format() -> None:
    """Tests validation error when UUID format is invalid."""
    from app.main import create_app_validation_errors_Invalid_UUID_format

    from spikard.testing import TestClient

    app = create_app_validation_errors_Invalid_UUID_format()
    client = TestClient(app)

    response = await client.get("/items/not-a-uuid")

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
    assert response_data["errors"][0]["loc"][1] == "item_id"
    assert "msg" in response_data["errors"][0]
    assert (
        response_data["errors"][0]["msg"]
        == "Input should be a valid UUID, invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `n` at 0"
    )
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "uuid_parsing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_invalid_boolean_value() -> None:
    """Tests validation error when boolean value is invalid."""
    from app.main import create_app_validation_errors_Invalid_boolean_value

    from spikard.testing import TestClient

    app = create_app_validation_errors_Invalid_boolean_value()
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
    assert response_data["errors"][0]["input"] == "maybe"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "is_active"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be a valid boolean, unable to interpret input"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "bool_parsing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_missing_required_query_parameter() -> None:
    """Tests validation error when required query param is missing."""
    from app.main import create_app_validation_errors_Missing_required_query_parameter

    from spikard.testing import TestClient

    app = create_app_validation_errors_Missing_required_query_parameter()
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
    assert response_data["errors"][0]["input"] is None
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "q"
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


async def test_array_max_items_constraint_violation() -> None:
    """Tests validation error when array has more items than max_items."""
    from app.main import create_app_validation_errors_Array_max_items_constraint_violation

    from spikard.testing import TestClient

    app = create_app_validation_errors_Array_max_items_constraint_violation()
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
    assert (
        response_data["errors"][0]["msg"]
        == '["tag1","tag2","tag3","tag4","tag5","tag6","tag7","tag8","tag9","tag10","tag11"] has more than 10 items'
    )
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_numeric_constraint_violation__gt_greater_than() -> None:
    """Tests validation error when value violates gt constraint."""
    from app.main import create_app_validation_errors_Numeric_constraint_violation___gt__greater_than

    from spikard.testing import TestClient

    app = create_app_validation_errors_Numeric_constraint_violation___gt__greater_than()
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
    assert "gt" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["gt"] == 0
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "0"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "price"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be greater than 0"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "greater_than"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_string_regex_pattern_mismatch() -> None:
    """Tests validation error when string doesn't match regex pattern."""
    from app.main import create_app_validation_errors_String_regex_pattern_mismatch

    from spikard.testing import TestClient

    app = create_app_validation_errors_String_regex_pattern_mismatch()
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
    assert "pattern" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["pattern"] == "^[a-zA-Z0-9_-]+$"
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "invalid!"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "q"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should match pattern '^[a-zA-Z0-9_-]+$'"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_pattern_mismatch"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_invalid_enum_value() -> None:
    """Tests validation error when value is not in allowed enum values."""
    from app.main import create_app_validation_errors_Invalid_enum_value

    from spikard.testing import TestClient

    app = create_app_validation_errors_Invalid_enum_value()
    client = TestClient(app)

    response = await client.get("/models/invalid_model")

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
    assert response_data["errors"][0]["input"] == "invalid_model"
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


async def test_string_min_length_constraint_violation() -> None:
    """Tests validation error when string is shorter than min_length."""
    from app.main import create_app_validation_errors_String_min_length_constraint_violation

    from spikard.testing import TestClient

    app = create_app_validation_errors_String_min_length_constraint_violation()
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
    assert "min_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["min_length"] == 3
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "ab"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "q"
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


async def test_multiple_validation_errors() -> None:
    """Tests multiple validation errors returned in single response."""
    from app.main import create_app_validation_errors_Multiple_validation_errors

    from spikard.testing import TestClient

    app = create_app_validation_errors_Multiple_validation_errors()
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
    assert "min_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["min_length"] == 3
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "X"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "name"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should have at least 3 characters"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_too_short"
    assert "ctx" in response_data["errors"][1]
    assert "gt" in response_data["errors"][1]["ctx"]
    assert response_data["errors"][1]["ctx"]["gt"] == 0
    assert "input" in response_data["errors"][1]
    assert response_data["errors"][1]["input"] == -10
    assert "loc" in response_data["errors"][1]
    assert len(response_data["errors"][1]["loc"]) == 2
    assert response_data["errors"][1]["loc"][0] == "body"
    assert response_data["errors"][1]["loc"][1] == "price"
    assert "msg" in response_data["errors"][1]
    assert response_data["errors"][1]["msg"] == "Input should be greater than 0"
    assert "type" in response_data["errors"][1]
    assert response_data["errors"][1]["type"] == "greater_than"
    assert "input" in response_data["errors"][2]
    assert response_data["errors"][2]["input"] == "not_a_number"
    assert "loc" in response_data["errors"][2]
    assert len(response_data["errors"][2]["loc"]) == 2
    assert response_data["errors"][2]["loc"][0] == "body"
    assert response_data["errors"][2]["loc"][1] == "quantity"
    assert "msg" in response_data["errors"][2]
    assert response_data["errors"][2]["msg"] == "Input should be a valid integer, unable to parse string as an integer"
    assert "type" in response_data["errors"][2]
    assert response_data["errors"][2]["type"] == "int_parsing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_string_max_length_constraint_violation() -> None:
    """Tests validation error when string exceeds max_length."""
    from app.main import create_app_validation_errors_String_max_length_constraint_violation

    from spikard.testing import TestClient

    app = create_app_validation_errors_String_max_length_constraint_violation()
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
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "max_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["max_length"] == 50
    assert "input" in response_data["errors"][0]
    assert (
        response_data["errors"][0]["input"]
        == "this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter"
    )
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "q"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should have at most 50 characters"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_too_long"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_nested_object_validation_error() -> None:
    """Tests validation error in nested object field."""
    from app.main import create_app_validation_errors_Nested_object_validation_error

    from spikard.testing import TestClient

    app = create_app_validation_errors_Nested_object_validation_error()
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
    assert "detail" in response_data
    assert response_data["detail"] == "3 validation errors in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 3
    assert "ctx" in response_data["errors"][0]
    assert "min_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["min_length"] == 3
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "SF"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 4
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "seller"
    assert response_data["errors"][0]["loc"][2] == "address"
    assert response_data["errors"][0]["loc"][3] == "city"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should have at least 3 characters"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_too_short"
    assert "ctx" in response_data["errors"][1]
    assert "min_length" in response_data["errors"][1]["ctx"]
    assert response_data["errors"][1]["ctx"]["min_length"] == 5
    assert "input" in response_data["errors"][1]
    assert response_data["errors"][1]["input"] == "123"
    assert "loc" in response_data["errors"][1]
    assert len(response_data["errors"][1]["loc"]) == 4
    assert response_data["errors"][1]["loc"][0] == "body"
    assert response_data["errors"][1]["loc"][1] == "seller"
    assert response_data["errors"][1]["loc"][2] == "address"
    assert response_data["errors"][1]["loc"][3] == "zip_code"
    assert "msg" in response_data["errors"][1]
    assert response_data["errors"][1]["msg"] == "String should have at least 5 characters"
    assert "type" in response_data["errors"][1]
    assert response_data["errors"][1]["type"] == "string_too_short"
    assert "ctx" in response_data["errors"][2]
    assert "min_length" in response_data["errors"][2]["ctx"]
    assert response_data["errors"][2]["ctx"]["min_length"] == 3
    assert "input" in response_data["errors"][2]
    assert response_data["errors"][2]["input"] == "Jo"
    assert "loc" in response_data["errors"][2]
    assert len(response_data["errors"][2]["loc"]) == 3
    assert response_data["errors"][2]["loc"][0] == "body"
    assert response_data["errors"][2]["loc"][1] == "seller"
    assert response_data["errors"][2]["loc"][2] == "name"
    assert "msg" in response_data["errors"][2]
    assert response_data["errors"][2]["msg"] == "String should have at least 3 characters"
    assert "type" in response_data["errors"][2]
    assert response_data["errors"][2]["type"] == "string_too_short"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_10_nested_error_path() -> None:
    """Validation error in nested object should have correct path in loc."""
    from app.main import create_app_validation_errors_10_nested_error_path

    from spikard.testing import TestClient

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
    assert "pattern" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["pattern"] == "^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$"
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "invalid"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 4
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "profile"
    assert response_data["errors"][0]["loc"][2] == "contact"
    assert response_data["errors"][0]["loc"][3] == "email"
    assert "msg" in response_data["errors"][0]
    assert (
        response_data["errors"][0]["msg"]
        == "String should match pattern '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$'"
    )
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_pattern_mismatch"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_invalid_datetime_format() -> None:
    """Tests validation error when datetime format is invalid."""
    from app.main import create_app_validation_errors_Invalid_datetime_format

    from spikard.testing import TestClient

    app = create_app_validation_errors_Invalid_datetime_format()
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
    assert response_data["errors"][0]["input"] == "not-a-datetime"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "created_at"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be a valid datetime"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "datetime_parsing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_array_item_validation_error() -> None:
    """Tests validation error for invalid item within array."""
    from app.main import create_app_validation_errors_Array_item_validation_error

    from spikard.testing import TestClient

    app = create_app_validation_errors_Array_item_validation_error()
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
    assert response_data["errors"][0]["input"] == 123
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 3
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "tags"
    assert response_data["errors"][0]["loc"][2] == "2"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be a valid unknown"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "type_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_missing_required_body_field() -> None:
    """Tests validation error when required body field is missing."""
    from app.main import create_app_validation_errors_Missing_required_body_field

    from spikard.testing import TestClient

    app = create_app_validation_errors_Missing_required_body_field()
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
    assert response_data["errors"][0]["msg"] == "Field required"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "missing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_body_field_type_error__string_for_float() -> None:
    """Tests validation error when body field has wrong type."""
    from app.main import create_app_validation_errors_Body_field_type_error___string_for_float

    from spikard.testing import TestClient

    app = create_app_validation_errors_Body_field_type_error___string_for_float()
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
    assert response_data["errors"][0]["input"] == "not_a_float"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "price"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be a valid number, unable to parse string as a number"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "float_parsing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_malformed_json_body() -> None:
    """Tests validation error when request body contains malformed JSON."""
    from app.main import create_app_validation_errors_Malformed_JSON_body

    from spikard.testing import TestClient

    app = create_app_validation_errors_Malformed_JSON_body()
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


async def test_query_param_type_error__string_provided_for_int() -> None:
    """Tests validation error when string is provided for integer query param."""
    from app.main import create_app_validation_errors_Query_param_type_error___string_provided_for_int

    from spikard.testing import TestClient

    app = create_app_validation_errors_Query_param_type_error___string_provided_for_int()
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
    assert response_data["errors"][0]["input"] == "not_a_number"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "skip"
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


async def test_header_validation_error() -> None:
    """Tests validation error when required header is missing."""
    from app.main import create_app_validation_errors_Header_validation_error

    from spikard.testing import TestClient

    app = create_app_validation_errors_Header_validation_error()
    client = TestClient(app)

    response = await client.get("/items/?q=test")

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] is None
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "header"
    assert response_data["errors"][0]["loc"][1] == "x-token"
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


async def test_09_multiple_validation_errors() -> None:
    """Multiple validation errors should be returned together in batch."""
    from app.main import create_app_validation_errors_09_multiple_validation_errors

    from spikard.testing import TestClient

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
    assert "ge" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["ge"] == 18
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == 15
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "age"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be greater than or equal to 18"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "greater_than_equal"
    assert "ctx" in response_data["errors"][1]
    assert "pattern" in response_data["errors"][1]["ctx"]
    assert response_data["errors"][1]["ctx"]["pattern"] == "^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$"
    assert "input" in response_data["errors"][1]
    assert response_data["errors"][1]["input"] == "invalid-email"
    assert "loc" in response_data["errors"][1]
    assert len(response_data["errors"][1]["loc"]) == 2
    assert response_data["errors"][1]["loc"][0] == "body"
    assert response_data["errors"][1]["loc"][1] == "email"
    assert "msg" in response_data["errors"][1]
    assert (
        response_data["errors"][1]["msg"]
        == "String should match pattern '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$'"
    )
    assert "type" in response_data["errors"][1]
    assert response_data["errors"][1]["type"] == "string_pattern_mismatch"
    assert "ctx" in response_data["errors"][2]
    assert "min_length" in response_data["errors"][2]["ctx"]
    assert response_data["errors"][2]["ctx"]["min_length"] == 3
    assert "input" in response_data["errors"][2]
    assert response_data["errors"][2]["input"] == "ab"
    assert "loc" in response_data["errors"][2]
    assert len(response_data["errors"][2]["loc"]) == 2
    assert response_data["errors"][2]["loc"][0] == "body"
    assert response_data["errors"][2]["loc"][1] == "name"
    assert "msg" in response_data["errors"][2]
    assert response_data["errors"][2]["msg"] == "String should have at least 3 characters"
    assert "type" in response_data["errors"][2]
    assert response_data["errors"][2]["type"] == "string_too_short"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_numeric_constraint_violation__le_less_than_or_equal() -> None:
    """Tests validation error when value violates le constraint."""
    from app.main import create_app_validation_errors_Numeric_constraint_violation___le__less_than_or_equal

    from spikard.testing import TestClient

    app = create_app_validation_errors_Numeric_constraint_violation___le__less_than_or_equal()
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
    assert "le" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["le"] == 100
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "101"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "query"
    assert response_data["errors"][0]["loc"][1] == "limit"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be less than or equal to 100"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "less_than_equal"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_array_min_items_constraint_violation() -> None:
    """Tests validation error when array has fewer items than min_items."""
    from app.main import create_app_validation_errors_Array_min_items_constraint_violation

    from spikard.testing import TestClient

    app = create_app_validation_errors_Array_min_items_constraint_violation()
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
    assert response_data["errors"][0]["msg"] == "[] has less than 1 item"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"
