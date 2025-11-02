"""E2E tests for url_encoded."""

from typing import Any


async def test_simple_form_submission__success(client: Any) -> None:
    """Tests basic URL-encoded form with username and password."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"password": "secret", "username": "johndoe"}
    response = await client.post("/login/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "username" in response_data
    assert response_data["username"] == "johndoe"


async def test_15_special_characters_field_names(client: Any) -> None:
    """URL-encoded form with special characters in field names should be handled correctly."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "user-name=JohnDoe&contact.email=john%40example.com"
    response = await client.post("/data", headers=headers, json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "contact.email" in response_data
    assert response_data["contact.email"] == "john@example.com"
    assert "user-name" in response_data
    assert response_data["user-name"] == "JohnDoe"


async def test_pattern_validation__fail(client: Any) -> None:
    """Tests form field with regex pattern constraint failure."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"username": "john doe"}
    response = await client.post("/form/validated", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "pattern" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["pattern"] == "^[a-z0-9_]+$"
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "john doe"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "username"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should match pattern '^[a-z0-9_]+$'"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_pattern_mismatch"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_22_additional_properties_strict_failure(client: Any) -> None:
    """URL-encoded form with extra fields when additionalProperties is false should fail."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "theme=dark&unknown_field=value"
    response = await client.post("/settings", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "property" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["property"] == "unknown_field"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "unknown_field"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Additional properties are not allowed"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_17_pattern_validation_failure(client: Any) -> None:
    """URL-encoded form field violating regex pattern should fail."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "account_id=INVALID123"
    response = await client.post("/accounts", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "pattern" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["pattern"] == "^ACC-[0-9]{6}$"
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == "INVALID123"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "account_id"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String does not match pattern '^ACC-[0-9]{6}$'"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_20_format_email_validation_failure(client: Any) -> None:
    """URL-encoded form with invalid email format should fail."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "email=not-an-email"
    response = await client.post("/subscribe", headers=headers, json=json_data)

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
    assert response_data["errors"][0]["loc"][0] == "body"
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


async def test_multiple_values_for_same_field(client: Any) -> None:
    """Tests form field with multiple values (array)."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"tags": ["python", "fastapi", "web"]}
    response = await client.post("/form/tags", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "tags" in response_data
    assert len(response_data["tags"]) == 3
    assert response_data["tags"][0] == "python"
    assert response_data["tags"][1] == "fastapi"
    assert response_data["tags"][2] == "web"


async def test_required_field_missing__validation_error(client: Any) -> None:
    """Tests validation error when required form field is missing."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"password": "secret"}
    response = await client.post("/login/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == ""
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "username"
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


async def test_13_array_field_success(client: Any) -> None:
    """URL-encoded form with array field using bracket notation should parse correctly."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "tags[]=python&tags[]=rust&tags[]=typescript"
    response = await client.post("/register", headers=headers, json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "tags" in response_data
    assert len(response_data["tags"]) == 3
    assert response_data["tags"][0] == "python"
    assert response_data["tags"][1] == "rust"
    assert response_data["tags"][2] == "typescript"


async def test_numeric_field_type_conversion(client: Any) -> None:
    """Tests conversion of form string value to numeric type."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"age": "30", "username": "johndoe"}
    response = await client.post("/form/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "age" in response_data
    assert response_data["age"] == 30
    assert "username" in response_data
    assert response_data["username"] == "johndoe"


async def test_special_characters_encoding(client: Any) -> None:
    """Tests URL encoding of special characters in form data."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"name": "John Doe", "description": "Test & Development"}
    response = await client.post("/form/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "description" in response_data
    assert response_data["description"] == "Test & Development"
    assert "name" in response_data
    assert response_data["name"] == "John Doe"


async def test_boolean_field_conversion(client: Any) -> None:
    """Tests conversion of form string value to boolean."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"username": "johndoe", "subscribe": "true"}
    response = await client.post("/form/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "subscribe" in response_data
    assert response_data["subscribe"]
    assert "username" in response_data
    assert response_data["username"] == "johndoe"


async def test_empty_string_value(client: Any) -> None:
    """Tests form field with empty string value."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"username": "johndoe", "description": ""}
    response = await client.post("/form/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "description" in response_data
    assert response_data["description"] == ""
    assert "username" in response_data
    assert response_data["username"] == "johndoe"


async def test_oauth2_password_grant_flow(client: Any) -> None:
    """Tests OAuth2 password grant with form data."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"username": "johndoe", "grant_type": "password", "password": "secret", "scope": ""}
    response = await client.post("/token", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "access_token" in response_data
    assert response_data["access_token"] == "johndoe"
    assert "token_type" in response_data
    assert response_data["token_type"] == "bearer"


async def test_19_array_minitems_validation_failure(client: Any) -> None:
    """URL-encoded array with fewer items than minItems should fail."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "tags[]=single"
    response = await client.post("/tags", headers=headers, json=json_data)

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
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "tags"
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


async def test_optional_field_missing__success(client: Any) -> None:
    """Tests form with optional field omitted."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"username": "johndoe", "password": "secret"}
    response = await client.post("/register/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "email" in response_data
    assert response_data["email"] is None
    assert "username" in response_data
    assert response_data["username"] == "johndoe"


async def test_14_nested_object_bracket_notation(client: Any) -> None:
    """URL-encoded form with nested object using bracket notation should parse correctly."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "user[name]=John%20Doe&user[email]=john@example.com&user[age]=30"
    response = await client.post("/profile", headers=headers, json=json_data)

    assert response.status_code == 201
    response_data = response.json()
    assert "user" in response_data
    assert "age" in response_data["user"]
    assert response_data["user"]["age"] == 30
    assert "email" in response_data["user"]
    assert response_data["user"]["email"] == "john@example.com"
    assert "name" in response_data["user"]
    assert response_data["user"]["name"] == "John Doe"


async def test_string_max_length_validation__fail(client: Any) -> None:
    """Tests form field with max_length constraint failure."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"username": "this_is_a_very_long_username_that_exceeds_limit"}
    response = await client.post("/form/validated", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "max_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["max_length"] == 20
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "this_is_a_very_long_username_that_exceeds_limit"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "username"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should have at most 20 characters"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_too_long"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_18_integer_minimum_validation_failure(client: Any) -> None:
    """URL-encoded integer field below minimum should fail."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "quantity=0"
    response = await client.post("/products", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "actual_value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["actual_value"] == 0
    assert "minimum" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["minimum"] == 1
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "quantity"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Value must be at least 1"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_21_integer_type_coercion_failure(client: Any) -> None:
    """URL-encoded form with non-numeric value for integer field should fail."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "price=not-a-number"
    response = await client.post("/products", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == "not-a-number"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "price"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Value is not a valid integer"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_16_minlength_validation_failure(client: Any) -> None:
    """URL-encoded form field violating minLength constraint should fail."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "username=ab"
    response = await client.post("/users", headers=headers, json=json_data)

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
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == "ab"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
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


async def test_string_min_length_validation__fail(client: Any) -> None:
    """Tests form field with min_length constraint failure."""
    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"username": "ab"}
    response = await client.post("/form/validated", headers=headers, json=json_data)

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
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "username"
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
