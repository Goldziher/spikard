"""E2E tests for url_encoded."""

import pytest
from typing import Any

async def test_simple_form_submission_success() -> None:
    """Tests basic URL-encoded form with username and password."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_simple_form_submission_success

    app = create_app_url_encoded_simple_form_submission_success()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"password": "secret", "username": "johndoe"}
    response = await client.post("/login/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["password"] == "secret"
    assert response_data["username"] == "johndoe"


async def test_15_special_characters_field_names() -> None:
    """URL-encoded form with special characters in field names should be handled correctly."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_15_special_characters_field_names

    app = create_app_url_encoded_15_special_characters_field_names()
    client = TestClient(app)

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


async def test_pattern_validation_fail() -> None:
    """Tests form field with regex pattern constraint failure."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_pattern_validation_fail

    app = create_app_url_encoded_pattern_validation_fail()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"username": "john doe"}
    response = await client.post("/form/validated", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_22_additional_properties_strict_failure() -> None:
    """URL-encoded form with extra fields when additionalProperties is false should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_22_additional_properties_strict_failure

    app = create_app_url_encoded_22_additional_properties_strict_failure()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "theme=dark&unknown_field=value"
    response = await client.post("/settings", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_17_pattern_validation_failure() -> None:
    """URL-encoded form field violating regex pattern should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_17_pattern_validation_failure

    app = create_app_url_encoded_17_pattern_validation_failure()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "account_id=INVALID123"
    response = await client.post("/accounts", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_20_format_email_validation_failure() -> None:
    """URL-encoded form with invalid email format should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_20_format_email_validation_failure

    app = create_app_url_encoded_20_format_email_validation_failure()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "email=not-an-email"
    response = await client.post("/subscribe", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_multiple_values_for_same_field() -> None:
    """Tests form field with multiple values (array)."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_multiple_values_for_same_field

    app = create_app_url_encoded_multiple_values_for_same_field()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"tags": ["python", "fastapi", "web"]}
    response = await client.post("/form/tags", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["tags"] == ["python", "fastapi", "web"]


async def test_required_field_missing_validation_error() -> None:
    """Tests validation error when required form field is missing."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_required_field_missing_validation_error

    app = create_app_url_encoded_required_field_missing_validation_error()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"password": "secret"}
    response = await client.post("/login/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_13_array_field_success() -> None:
    """URL-encoded form with array field using bracket notation should parse correctly."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_13_array_field_success

    app = create_app_url_encoded_13_array_field_success()
    client = TestClient(app)

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


async def test_numeric_field_type_conversion() -> None:
    """Tests conversion of form string value to numeric type."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_numeric_field_type_conversion

    app = create_app_url_encoded_numeric_field_type_conversion()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"username": "johndoe", "age": "30"}
    response = await client.post("/form/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["username"] == "johndoe"
    assert response_data["age"] == "30"


async def test_special_characters_encoding() -> None:
    """Tests URL encoding of special characters in form data."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_special_characters_encoding

    app = create_app_url_encoded_special_characters_encoding()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"name": "John Doe", "description": "Test & Development"}
    response = await client.post("/form/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["name"] == "John Doe"
    assert response_data["description"] == "Test & Development"


async def test_boolean_field_conversion() -> None:
    """Tests conversion of form string value to boolean."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_boolean_field_conversion

    app = create_app_url_encoded_boolean_field_conversion()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"subscribe": "true", "username": "johndoe"}
    response = await client.post("/form/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["subscribe"] == "true"
    assert response_data["username"] == "johndoe"


async def test_empty_string_value() -> None:
    """Tests form field with empty string value."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_empty_string_value

    app = create_app_url_encoded_empty_string_value()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"description": "", "username": "johndoe"}
    response = await client.post("/form/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["description"] == ""
    assert response_data["username"] == "johndoe"


async def test_oauth2_password_grant_flow() -> None:
    """Tests OAuth2 password grant with form data."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_oauth2_password_grant_flow

    app = create_app_url_encoded_oauth2_password_grant_flow()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"username": "johndoe", "password": "secret", "scope": "", "grant_type": "password"}
    response = await client.post("/token", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["username"] == "johndoe"
    assert response_data["password"] == "secret"
    assert response_data["scope"] == ""
    assert response_data["grant_type"] == "password"


async def test_19_array_minitems_validation_failure() -> None:
    """URL-encoded array with fewer items than minItems should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_19_array_minitems_validation_failure

    app = create_app_url_encoded_19_array_minitems_validation_failure()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "tags[]=single"
    response = await client.post("/tags", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_optional_field_missing_success() -> None:
    """Tests form with optional field omitted."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_optional_field_missing_success

    app = create_app_url_encoded_optional_field_missing_success()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"username": "johndoe", "password": "secret"}
    response = await client.post("/register/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert response_data["username"] == "johndoe"
    assert response_data["password"] == "secret"


async def test_14_nested_object_bracket_notation() -> None:
    """URL-encoded form with nested object using bracket notation should parse correctly."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_14_nested_object_bracket_notation

    app = create_app_url_encoded_14_nested_object_bracket_notation()
    client = TestClient(app)

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


async def test_string_max_length_validation_fail() -> None:
    """Tests form field with max_length constraint failure."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_string_max_length_validation_fail

    app = create_app_url_encoded_string_max_length_validation_fail()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"username": "this_is_a_very_long_username_that_exceeds_limit"}
    response = await client.post("/form/validated", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_18_integer_minimum_validation_failure() -> None:
    """URL-encoded integer field below minimum should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_18_integer_minimum_validation_failure

    app = create_app_url_encoded_18_integer_minimum_validation_failure()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "quantity=0"
    response = await client.post("/products", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_21_integer_type_coercion_failure() -> None:
    """URL-encoded form with non-numeric value for integer field should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_21_integer_type_coercion_failure

    app = create_app_url_encoded_21_integer_type_coercion_failure()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "price=not-a-number"
    response = await client.post("/products", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_16_minlength_validation_failure() -> None:
    """URL-encoded form field violating minLength constraint should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_16_minlength_validation_failure

    app = create_app_url_encoded_16_minlength_validation_failure()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = "username=ab"
    response = await client.post("/users", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_string_min_length_validation_fail() -> None:
    """Tests form field with min_length constraint failure."""
    from spikard.testing import TestClient
    from app.main import create_app_url_encoded_string_min_length_validation_fail

    app = create_app_url_encoded_string_min_length_validation_fail()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/x-www-form-urlencoded",
    }
    json_data = {"username": "ab"}
    response = await client.post("/form/validated", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


