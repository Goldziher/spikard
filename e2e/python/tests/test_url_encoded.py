"""E2E tests for url_encoded."""

from spikard.testing import TestClient
from app.main import (
    create_app_url_encoded_13_array_field_success,
    create_app_url_encoded_14_nested_object_bracket_notation,
    create_app_url_encoded_15_special_characters_field_names,
    create_app_url_encoded_16_minlength_validation_failure,
    create_app_url_encoded_17_pattern_validation_failure,
    create_app_url_encoded_18_integer_minimum_validation_failure,
    create_app_url_encoded_19_array_minitems_validation_failure,
    create_app_url_encoded_20_format_email_validation_failure,
    create_app_url_encoded_21_integer_type_coercion_failure,
    create_app_url_encoded_22_additional_properties_strict_failure,
    create_app_url_encoded_boolean_field_conversion,
    create_app_url_encoded_empty_string_value,
    create_app_url_encoded_multiple_values_for_same_field,
    create_app_url_encoded_numeric_field_type_conversion,
    create_app_url_encoded_oauth2_password_grant_flow,
    create_app_url_encoded_optional_field_missing_success,
    create_app_url_encoded_pattern_validation_fail,
    create_app_url_encoded_required_field_missing_validation_error,
    create_app_url_encoded_simple_form_submission_success,
    create_app_url_encoded_special_characters_encoding,
    create_app_url_encoded_string_max_length_validation_fail,
    create_app_url_encoded_string_min_length_validation_fail,
)


async def test_simple_form_submission_success() -> None:
    """Tests basic URL-encoded form with username and password."""

    async with TestClient(create_app_url_encoded_simple_form_submission_success()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = {"password": "secret", "username": "johndoe"}
        response = await client.post("/login/", headers=headers, data=form_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "username" in response_data
        assert response_data["username"] == "johndoe"


async def test_15_special_characters_field_names() -> None:
    """URL-encoded form with special characters in field names should be handled correctly."""

    async with TestClient(create_app_url_encoded_15_special_characters_field_names()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = "user-name=JohnDoe&contact.email=john%40example.com"
        response = await client.post("/data", headers=headers, data=form_data)

        assert response.status_code == 201
        response_data = response.json()
        assert "user-name" in response_data
        assert response_data["user-name"] == "JohnDoe"
        assert "contact.email" in response_data
        assert response_data["contact.email"] == "john@example.com"


async def test_pattern_validation_fail() -> None:
    """Tests form field with regex pattern constraint failure."""

    async with TestClient(create_app_url_encoded_pattern_validation_fail()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = {"username": "john doe"}
        response = await client.post("/form/validated", headers=headers, data=form_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_22_additional_properties_strict_failure() -> None:
    """URL-encoded form with extra fields when additionalProperties is false should fail."""

    async with TestClient(create_app_url_encoded_22_additional_properties_strict_failure()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = "theme=dark&unknown_field=value"
        response = await client.post("/settings", headers=headers, data=form_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_17_pattern_validation_failure() -> None:
    """URL-encoded form field violating regex pattern should fail."""

    async with TestClient(create_app_url_encoded_17_pattern_validation_failure()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = "account_id=INVALID123"
        response = await client.post("/accounts", headers=headers, data=form_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_20_format_email_validation_failure() -> None:
    """URL-encoded form with invalid email format should fail."""

    async with TestClient(create_app_url_encoded_20_format_email_validation_failure()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = "email=not-an-email"
        response = await client.post("/subscribe", headers=headers, data=form_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_multiple_values_for_same_field() -> None:
    """Tests form field with multiple values (array)."""

    async with TestClient(create_app_url_encoded_multiple_values_for_same_field()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = {"tags": ["python", "fastapi", "web"]}
        response = await client.post("/form/tags", headers=headers, data=form_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "tags" in response_data
        assert len(response_data["tags"]) == 3
        assert response_data["tags"][0] == "python"
        assert response_data["tags"][1] == "fastapi"
        assert response_data["tags"][2] == "web"


async def test_required_field_missing_validation_error() -> None:
    """Tests validation error when required form field is missing."""

    async with TestClient(create_app_url_encoded_required_field_missing_validation_error()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = {"password": "secret"}
        response = await client.post("/login/", headers=headers, data=form_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_13_array_field_success() -> None:
    """URL-encoded form with array field using bracket notation should parse correctly."""

    async with TestClient(create_app_url_encoded_13_array_field_success()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = "tags[]=python&tags[]=rust&tags[]=typescript"
        response = await client.post("/register", headers=headers, data=form_data)

        assert response.status_code == 201
        response_data = response.json()
        assert "tags" in response_data
        assert len(response_data["tags"]) == 3
        assert response_data["tags"][0] == "python"
        assert response_data["tags"][1] == "rust"
        assert response_data["tags"][2] == "typescript"


async def test_numeric_field_type_conversion() -> None:
    """Tests conversion of form string value to numeric type."""

    async with TestClient(create_app_url_encoded_numeric_field_type_conversion()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = {"age": "30", "username": "johndoe"}
        response = await client.post("/form/", headers=headers, data=form_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "username" in response_data
        assert response_data["username"] == "johndoe"
        assert "age" in response_data
        assert response_data["age"] == 30


async def test_special_characters_encoding() -> None:
    """Tests URL encoding of special characters in form data."""

    async with TestClient(create_app_url_encoded_special_characters_encoding()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = {"description": "Test & Development", "name": "John Doe"}
        response = await client.post("/form/", headers=headers, data=form_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "John Doe"
        assert "description" in response_data
        assert response_data["description"] == "Test & Development"


async def test_boolean_field_conversion() -> None:
    """Tests conversion of form string value to boolean."""

    async with TestClient(create_app_url_encoded_boolean_field_conversion()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = {"subscribe": "true", "username": "johndoe"}
        response = await client.post("/form/", headers=headers, data=form_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "username" in response_data
        assert response_data["username"] == "johndoe"
        assert "subscribe" in response_data
        assert response_data["subscribe"] == True


async def test_empty_string_value() -> None:
    """Tests form field with empty string value."""

    async with TestClient(create_app_url_encoded_empty_string_value()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = {"description": "", "username": "johndoe"}
        response = await client.post("/form/", headers=headers, data=form_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "username" in response_data
        assert response_data["username"] == "johndoe"
        assert "description" in response_data
        assert response_data["description"] == ""


async def test_oauth2_password_grant_flow() -> None:
    """Tests OAuth2 password grant with form data."""

    async with TestClient(create_app_url_encoded_oauth2_password_grant_flow()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = {"grant_type": "password", "password": "secret", "scope": "", "username": "johndoe"}
        response = await client.post("/token", headers=headers, data=form_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "access_token" in response_data
        assert response_data["access_token"] == "johndoe"
        assert "token_type" in response_data
        assert response_data["token_type"] == "bearer"


async def test_19_array_minitems_validation_failure() -> None:
    """URL-encoded array with fewer items than minItems should fail."""

    async with TestClient(create_app_url_encoded_19_array_minitems_validation_failure()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = "tags[]=single"
        response = await client.post("/tags", headers=headers, data=form_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_optional_field_missing_success() -> None:
    """Tests form with optional field omitted."""

    async with TestClient(create_app_url_encoded_optional_field_missing_success()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = {"password": "secret", "username": "johndoe"}
        response = await client.post("/register/", headers=headers, data=form_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "username" in response_data
        assert response_data["username"] == "johndoe"
        assert "email" in response_data
        assert response_data["email"] == None


async def test_14_nested_object_bracket_notation() -> None:
    """URL-encoded form with nested object using bracket notation should parse correctly."""

    async with TestClient(create_app_url_encoded_14_nested_object_bracket_notation()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = "user[name]=John%20Doe&user[email]=john@example.com&user[age]=30"
        response = await client.post("/profile", headers=headers, data=form_data)

        assert response.status_code == 201
        response_data = response.json()
        assert "user" in response_data
        assert "name" in response_data["user"]
        assert response_data["user"]["name"] == "John Doe"
        assert "email" in response_data["user"]
        assert response_data["user"]["email"] == "john@example.com"
        assert "age" in response_data["user"]
        assert response_data["user"]["age"] == 30


async def test_string_max_length_validation_fail() -> None:
    """Tests form field with max_length constraint failure."""

    async with TestClient(create_app_url_encoded_string_max_length_validation_fail()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = {"username": "this_is_a_very_long_username_that_exceeds_limit"}
        response = await client.post("/form/validated", headers=headers, data=form_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_18_integer_minimum_validation_failure() -> None:
    """URL-encoded integer field below minimum should fail."""

    async with TestClient(create_app_url_encoded_18_integer_minimum_validation_failure()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = "quantity=0"
        response = await client.post("/products", headers=headers, data=form_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_21_integer_type_coercion_failure() -> None:
    """URL-encoded form with non-numeric value for integer field should fail."""

    async with TestClient(create_app_url_encoded_21_integer_type_coercion_failure()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = "price=not-a-number"
        response = await client.post("/products", headers=headers, data=form_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_16_minlength_validation_failure() -> None:
    """URL-encoded form field violating minLength constraint should fail."""

    async with TestClient(create_app_url_encoded_16_minlength_validation_failure()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = "username=ab"
        response = await client.post("/users", headers=headers, data=form_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_string_min_length_validation_fail() -> None:
    """Tests form field with min_length constraint failure."""

    async with TestClient(create_app_url_encoded_string_min_length_validation_fail()) as client:
        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
        }
        form_data = {"username": "ab"}
        response = await client.post("/form/validated", headers=headers, data=form_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data
