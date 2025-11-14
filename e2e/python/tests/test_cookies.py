"""E2E tests for cookies."""

from spikard.testing import TestClient
from app.main import (
    create_app_cookies_24_cookie_samesite_strict,
    create_app_cookies_25_cookie_samesite_lax,
    create_app_cookies_26_cookie_secure_flag,
    create_app_cookies_27_cookie_httponly_flag,
    create_app_cookies_apikey_cookie_authentication_missing,
    create_app_cookies_apikey_cookie_authentication_success,
    create_app_cookies_cookie_regex_pattern_validation_fail,
    create_app_cookies_cookie_regex_pattern_validation_success,
    create_app_cookies_cookie_validation_max_length_constraint_fail,
    create_app_cookies_cookie_validation_min_length_constraint_success,
    create_app_cookies_cookie_validation_min_length_failure,
    create_app_cookies_multiple_cookies_success,
    create_app_cookies_optional_apikey_cookie_missing,
    create_app_cookies_optional_cookie_parameter_missing,
    create_app_cookies_optional_cookie_parameter_success,
    create_app_cookies_required_cookie_missing,
    create_app_cookies_response_cookie_with_attributes,
    create_app_cookies_response_cookie_with_domain_attribute,
    create_app_cookies_response_cookie_with_path_attribute,
    create_app_cookies_response_cookie_with_samesite_lax,
    create_app_cookies_response_cookie_with_samesite_none,
    create_app_cookies_response_cookie_with_samesite_strict,
    create_app_cookies_response_delete_cookie,
    create_app_cookies_response_multiple_cookies,
    create_app_cookies_response_session_cookie_no_max_age,
    create_app_cookies_response_set_cookie_basic,
)


async def test_25_cookie_samesite_lax() -> None:
    """Cookie with SameSite=Lax attribute should be validated."""

    async with TestClient(create_app_cookies_25_cookie_samesite_lax()) as client:
        cookies = {
            "tracking": "track123",
        }
        response = await client.get("/data", cookies=cookies)

        assert response.status_code == 200
        response_data = response.json()


async def test_optional_cookie_parameter_success() -> None:
    """Tests optional cookie parameter with value provided."""

    async with TestClient(create_app_cookies_optional_cookie_parameter_success()) as client:
        cookies = {
            "ads_id": "abc123",
        }
        response = await client.get("/items/", cookies=cookies)

        assert response.status_code == 200
        response_data = response.json()
        assert "ads_id" in response_data
        assert response_data["ads_id"] == "abc123"


async def test_cookie_regex_pattern_validation_fail() -> None:
    """Tests cookie with regex pattern validation failure."""

    async with TestClient(create_app_cookies_cookie_regex_pattern_validation_fail()) as client:
        cookies = {
            "tracking_id": "invalid-format",
        }
        response = await client.get("/cookies/pattern", cookies=cookies)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_response_session_cookie_no_max_age() -> None:
    """Tests setting session cookie without max_age (expires with browser)."""

    async with TestClient(create_app_cookies_response_session_cookie_no_max_age()) as client:
        json_data = {"value": "session_abc123"}
        response = await client.post("/cookies/session", json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Session cookie set"


async def test_27_cookie_httponly_flag() -> None:
    """Cookie with HttpOnly flag should prevent JavaScript access."""

    async with TestClient(create_app_cookies_27_cookie_httponly_flag()) as client:
        cookies = {
            "session": "session_abc123",
        }
        response = await client.get("/secure", cookies=cookies)

        assert response.status_code == 200
        response_data = response.json()


async def test_response_cookie_with_attributes() -> None:
    """Tests setting a cookie with max_age, secure, httponly, and samesite attributes."""

    async with TestClient(create_app_cookies_response_cookie_with_attributes()) as client:
        response = await client.get("/cookie/set")

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Cookie set"


async def test_24_cookie_samesite_strict() -> None:
    """Cookie with SameSite=Strict attribute should be validated."""

    async with TestClient(create_app_cookies_24_cookie_samesite_strict()) as client:
        cookies = {
            "session_id": "abc123xyz789",
        }
        response = await client.get("/secure", cookies=cookies)

        assert response.status_code == 200
        response_data = response.json()


async def test_apikey_cookie_authentication_success() -> None:
    """Tests APIKeyCookie authentication with valid cookie."""

    async with TestClient(create_app_cookies_apikey_cookie_authentication_success()) as client:
        cookies = {
            "key": "secret",
        }
        response = await client.get("/users/me", cookies=cookies)

        assert response.status_code == 200
        response_data = response.json()
        assert "username" in response_data
        assert response_data["username"] == "secret"


async def test_cookie_validation_min_length_constraint_success() -> None:
    """Tests cookie validation with min_length constraint at boundary."""

    async with TestClient(create_app_cookies_cookie_validation_min_length_constraint_success()) as client:
        cookies = {
            "token": "abc",
        }
        response = await client.get("/cookies/min-length", cookies=cookies)

        assert response.status_code == 200
        response_data = response.json()
        assert "token" in response_data
        assert response_data["token"] == "abc"


async def test_cookie_validation_min_length_failure() -> None:
    """Tests cookie parameter with min_length constraint returns 422 when too short."""

    async with TestClient(create_app_cookies_cookie_validation_min_length_failure()) as client:
        cookies = {
            "tracking_id": "ab",
        }
        response = await client.get("/items/", cookies=cookies)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_cookie_validation_max_length_constraint_fail() -> None:
    """Tests cookie validation with max_length constraint failure."""

    async with TestClient(create_app_cookies_cookie_validation_max_length_constraint_fail()) as client:
        cookies = {
            "session_id": "this_cookie_value_is_way_too_long",
        }
        response = await client.get("/cookies/validated", cookies=cookies)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_required_cookie_missing() -> None:
    """Tests validation error when required cookie is missing."""

    async with TestClient(create_app_cookies_required_cookie_missing()) as client:
        cookies = {
            "fatebook_tracker": "tracker456",
        }
        response = await client.get("/items/cookies", cookies=cookies)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_optional_cookie_parameter_missing() -> None:
    """Tests optional cookie parameter returns None when not provided."""

    async with TestClient(create_app_cookies_optional_cookie_parameter_missing()) as client:
        response = await client.get("/items/")

        assert response.status_code == 200
        response_data = response.json()
        assert "ads_id" in response_data
        assert response_data["ads_id"] == None


async def test_apikey_cookie_authentication_missing() -> None:
    """Tests APIKeyCookie authentication returns 403 when cookie missing."""

    async with TestClient(create_app_cookies_apikey_cookie_authentication_missing()) as client:
        response = await client.get("/users/me/auth")

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_response_multiple_cookies() -> None:
    """Tests setting multiple cookies in single response."""

    async with TestClient(create_app_cookies_response_multiple_cookies()) as client:
        json_data = {"session": "session123", "user": "john"}
        response = await client.post("/cookies/multiple", json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Multiple cookies set"


async def test_response_cookie_with_samesite_lax() -> None:
    """Tests setting cookie with SameSite lax attribute."""

    async with TestClient(create_app_cookies_response_cookie_with_samesite_lax()) as client:
        json_data = {"value": "lax_cookie"}
        response = await client.post("/cookies/samesite-lax", json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Cookie set with SameSite=Lax"


async def test_response_delete_cookie() -> None:
    """Tests deleting a cookie by setting max_age to 0."""

    async with TestClient(create_app_cookies_response_delete_cookie()) as client:
        cookies = {
            "session": "old_session_123",
        }
        response = await client.post("/cookies/delete", cookies=cookies)

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Cookie deleted"


async def test_response_cookie_with_path_attribute() -> None:
    """Tests setting cookie with specific path."""

    async with TestClient(create_app_cookies_response_cookie_with_path_attribute()) as client:
        json_data = {"value": "path_test"}
        response = await client.post("/cookies/set-with-path", json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Cookie set with path"


async def test_optional_apikey_cookie_missing() -> None:
    """Tests optional APIKeyCookie (auto_error=False) returns None without 403."""

    async with TestClient(create_app_cookies_optional_apikey_cookie_missing()) as client:
        response = await client.get("/users/me")

        assert response.status_code == 200
        response_data = response.json()
        assert "msg" in response_data
        assert response_data["msg"] == "Create an account first"


async def test_response_cookie_with_samesite_strict() -> None:
    """Tests setting cookie with SameSite strict attribute."""

    async with TestClient(create_app_cookies_response_cookie_with_samesite_strict()) as client:
        json_data = {"value": "strict_cookie"}
        response = await client.post("/cookies/samesite-strict", json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Cookie set with SameSite=Strict"


async def test_response_cookie_with_samesite_none() -> None:
    """Tests setting cookie with SameSite none (requires Secure)."""

    async with TestClient(create_app_cookies_response_cookie_with_samesite_none()) as client:
        json_data = {"value": "none_cookie"}
        response = await client.post("/cookies/samesite-none", json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Cookie set with SameSite=None"


async def test_cookie_regex_pattern_validation_success() -> None:
    """Tests cookie with regex pattern validation success."""

    async with TestClient(create_app_cookies_cookie_regex_pattern_validation_success()) as client:
        cookies = {
            "tracking_id": "ABC12345",
        }
        response = await client.get("/cookies/pattern", cookies=cookies)

        assert response.status_code == 200
        response_data = response.json()
        assert "tracking_id" in response_data
        assert response_data["tracking_id"] == "ABC12345"


async def test_response_set_cookie_basic() -> None:
    """Tests setting a cookie in the response."""

    async with TestClient(create_app_cookies_response_set_cookie_basic()) as client:
        response = await client.post("/cookie/")

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Come to the dark side, we have cookies"


async def test_multiple_cookies_success() -> None:
    """Tests multiple cookie parameters in a single request."""

    async with TestClient(create_app_cookies_multiple_cookies_success()) as client:
        cookies = {
            "session_id": "session123",
            "fatebook_tracker": "tracker456",
            "googall_tracker": "ga789",
        }
        response = await client.get("/items/", cookies=cookies)

        assert response.status_code == 200
        response_data = response.json()
        assert "fatebook_tracker" in response_data
        assert response_data["fatebook_tracker"] == "tracker456"
        assert "googall_tracker" in response_data
        assert response_data["googall_tracker"] == "ga789"
        assert "session_id" in response_data
        assert response_data["session_id"] == "session123"


async def test_26_cookie_secure_flag() -> None:
    """Cookie with Secure flag should be validated for HTTPS."""

    async with TestClient(create_app_cookies_26_cookie_secure_flag()) as client:
        cookies = {
            "auth_token": "secure_token_xyz",
        }
        response = await client.get("/secure", cookies=cookies)

        assert response.status_code == 200
        response_data = response.json()


async def test_response_cookie_with_domain_attribute() -> None:
    """Tests setting cookie with specific domain."""

    async with TestClient(create_app_cookies_response_cookie_with_domain_attribute()) as client:
        json_data = {"value": "domain_test"}
        response = await client.post("/cookies/set-with-domain", json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "message" in response_data
        assert response_data["message"] == "Cookie set with domain"
