"""E2E tests for cookies."""


async def test_25_cookie_samesite_lax() -> None:
    """Cookie with SameSite=Lax attribute should be validated."""
    from app.main import create_app_cookies_25_cookie_samesite_lax

    from spikard.testing import TestClient

    app = create_app_cookies_25_cookie_samesite_lax()
    client = TestClient(app)

    cookies = {
        "tracking": "track123",
    }
    response = await client.get("/data", cookies=cookies)

    assert response.status_code == 200


async def test_optional_cookie_parameter_success() -> None:
    """Tests optional cookie parameter with value provided."""
    from app.main import create_app_cookies_optional_cookie_parameter_success

    from spikard.testing import TestClient

    app = create_app_cookies_optional_cookie_parameter_success()
    client = TestClient(app)

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
    from app.main import create_app_cookies_cookie_regex_pattern_validation_fail

    from spikard.testing import TestClient

    app = create_app_cookies_cookie_regex_pattern_validation_fail()
    client = TestClient(app)

    cookies = {
        "tracking_id": "invalid-format",
    }
    response = await client.get("/cookies/pattern", cookies=cookies)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "pattern" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["pattern"] == "^[A-Z0-9]{8}$"
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "invalid-format"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "cookie"
    assert response_data["errors"][0]["loc"][1] == "tracking_id"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should match pattern '^[A-Z0-9]{8}$'"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_pattern_mismatch"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_response_session_cookie_no_max_age() -> None:
    """Tests setting session cookie without max_age (expires with browser)."""
    from app.main import create_app_cookies_response_session_cookie_no_max_age

    from spikard.testing import TestClient

    app = create_app_cookies_response_session_cookie_no_max_age()
    client = TestClient(app)

    json_data = {"value": "session_abc123"}
    response = await client.post("/cookies/session", json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Session cookie set"


async def test_27_cookie_httponly_flag() -> None:
    """Cookie with HttpOnly flag should prevent JavaScript access."""
    from app.main import create_app_cookies_27_cookie_httponly_flag

    from spikard.testing import TestClient

    app = create_app_cookies_27_cookie_httponly_flag()
    client = TestClient(app)

    cookies = {
        "session": "session_abc123",
    }
    response = await client.get("/secure", cookies=cookies)

    assert response.status_code == 200


async def test_response_cookie_with_attributes() -> None:
    """Tests setting a cookie with max_age, secure, httponly, and samesite attributes."""
    from app.main import create_app_cookies_response_cookie_with_attributes

    from spikard.testing import TestClient

    app = create_app_cookies_response_cookie_with_attributes()
    client = TestClient(app)

    response = await client.get("/cookie/set")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Cookie set"


async def test_24_cookie_samesite_strict() -> None:
    """Cookie with SameSite=Strict attribute should be validated."""
    from app.main import create_app_cookies_24_cookie_samesite_strict

    from spikard.testing import TestClient

    app = create_app_cookies_24_cookie_samesite_strict()
    client = TestClient(app)

    cookies = {
        "session_id": "abc123xyz789",
    }
    response = await client.get("/secure", cookies=cookies)

    assert response.status_code == 200


async def test_apikey_cookie_authentication_success() -> None:
    """Tests APIKeyCookie authentication with valid cookie."""
    from app.main import create_app_cookies_apikey_cookie_authentication_success

    from spikard.testing import TestClient

    app = create_app_cookies_apikey_cookie_authentication_success()
    client = TestClient(app)

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
    from app.main import create_app_cookies_cookie_validation_min_length_constraint_success

    from spikard.testing import TestClient

    app = create_app_cookies_cookie_validation_min_length_constraint_success()
    client = TestClient(app)

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
    from app.main import create_app_cookies_cookie_validation_min_length_failure

    from spikard.testing import TestClient

    app = create_app_cookies_cookie_validation_min_length_failure()
    client = TestClient(app)

    cookies = {
        "tracking_id": "ab",
    }
    response = await client.get("/items/", cookies=cookies)

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
    assert response_data["errors"][0]["loc"][0] == "cookie"
    assert response_data["errors"][0]["loc"][1] == "tracking_id"
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


async def test_cookie_validation_max_length_constraint_fail() -> None:
    """Tests cookie validation with max_length constraint failure."""
    from app.main import create_app_cookies_cookie_validation_max_length_constraint_fail

    from spikard.testing import TestClient

    app = create_app_cookies_cookie_validation_max_length_constraint_fail()
    client = TestClient(app)

    cookies = {
        "session_id": "this_cookie_value_is_way_too_long",
    }
    response = await client.get("/cookies/validated", cookies=cookies)

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
    assert response_data["errors"][0]["input"] == "this_cookie_value_is_way_too_long"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "cookie"
    assert response_data["errors"][0]["loc"][1] == "session_id"
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


async def test_required_cookie_missing() -> None:
    """Tests validation error when required cookie is missing."""
    from app.main import create_app_cookies_required_cookie_missing

    from spikard.testing import TestClient

    app = create_app_cookies_required_cookie_missing()
    client = TestClient(app)

    cookies = {
        "fatebook_tracker": "tracker456",
    }
    response = await client.get("/items/cookies", cookies=cookies)

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
    assert response_data["errors"][0]["loc"][0] == "cookie"
    assert response_data["errors"][0]["loc"][1] == "session_id"
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


async def test_optional_cookie_parameter_missing() -> None:
    """Tests optional cookie parameter returns None when not provided."""
    from app.main import create_app_cookies_optional_cookie_parameter_missing

    from spikard.testing import TestClient

    app = create_app_cookies_optional_cookie_parameter_missing()
    client = TestClient(app)

    response = await client.get("/items/")

    assert response.status_code == 200
    response_data = response.json()
    assert "ads_id" in response_data
    assert response_data["ads_id"] is None


async def test_apikey_cookie_authentication_missing() -> None:
    """Tests APIKeyCookie authentication returns 403 when cookie missing."""
    from app.main import create_app_cookies_apikey_cookie_authentication_missing

    from spikard.testing import TestClient

    app = create_app_cookies_apikey_cookie_authentication_missing()
    client = TestClient(app)

    response = await client.get("/users/me/auth")

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
    assert response_data["errors"][0]["loc"][0] == "cookie"
    assert response_data["errors"][0]["loc"][1] == "key"
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


async def test_response_multiple_cookies() -> None:
    """Tests setting multiple cookies in single response."""
    from app.main import create_app_cookies_response_multiple_cookies

    from spikard.testing import TestClient

    app = create_app_cookies_response_multiple_cookies()
    client = TestClient(app)

    json_data = {"session": "session123", "user": "john"}
    response = await client.post("/cookies/multiple", json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Multiple cookies set"


async def test_response_cookie_with_samesite_lax() -> None:
    """Tests setting cookie with SameSite lax attribute."""
    from app.main import create_app_cookies_response_cookie_with_samesite_lax

    from spikard.testing import TestClient

    app = create_app_cookies_response_cookie_with_samesite_lax()
    client = TestClient(app)

    json_data = {"value": "lax_cookie"}
    response = await client.post("/cookies/samesite-lax", json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Cookie set with SameSite=Lax"


async def test_response_delete_cookie() -> None:
    """Tests deleting a cookie by setting max_age to 0."""
    from app.main import create_app_cookies_response_delete_cookie

    from spikard.testing import TestClient

    app = create_app_cookies_response_delete_cookie()
    client = TestClient(app)

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
    from app.main import create_app_cookies_response_cookie_with_path_attribute

    from spikard.testing import TestClient

    app = create_app_cookies_response_cookie_with_path_attribute()
    client = TestClient(app)

    json_data = {"value": "path_test"}
    response = await client.post("/cookies/set-with-path", json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Cookie set with path"


async def test_optional_apikey_cookie_missing() -> None:
    """Tests optional APIKeyCookie (auto_error=False) returns None without 403."""
    from app.main import create_app_cookies_optional_apikey_cookie_missing

    from spikard.testing import TestClient

    app = create_app_cookies_optional_apikey_cookie_missing()
    client = TestClient(app)

    response = await client.get("/users/me")

    assert response.status_code == 200
    response_data = response.json()
    assert "msg" in response_data
    assert response_data["msg"] == "Create an account first"


async def test_response_cookie_with_samesite_strict() -> None:
    """Tests setting cookie with SameSite strict attribute."""
    from app.main import create_app_cookies_response_cookie_with_samesite_strict

    from spikard.testing import TestClient

    app = create_app_cookies_response_cookie_with_samesite_strict()
    client = TestClient(app)

    json_data = {"value": "strict_cookie"}
    response = await client.post("/cookies/samesite-strict", json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Cookie set with SameSite=Strict"


async def test_response_cookie_with_samesite_none() -> None:
    """Tests setting cookie with SameSite none (requires Secure)."""
    from app.main import create_app_cookies_response_cookie_with_samesite_none

    from spikard.testing import TestClient

    app = create_app_cookies_response_cookie_with_samesite_none()
    client = TestClient(app)

    json_data = {"value": "none_cookie"}
    response = await client.post("/cookies/samesite-none", json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Cookie set with SameSite=None"


async def test_cookie_regex_pattern_validation_success() -> None:
    """Tests cookie with regex pattern validation success."""
    from app.main import create_app_cookies_cookie_regex_pattern_validation_success

    from spikard.testing import TestClient

    app = create_app_cookies_cookie_regex_pattern_validation_success()
    client = TestClient(app)

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
    from app.main import create_app_cookies_response_set_cookie_basic

    from spikard.testing import TestClient

    app = create_app_cookies_response_set_cookie_basic()
    client = TestClient(app)

    response = await client.post("/cookie/")

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Come to the dark side, we have cookies"


async def test_multiple_cookies_success() -> None:
    """Tests multiple cookie parameters in a single request."""
    from app.main import create_app_cookies_multiple_cookies_success

    from spikard.testing import TestClient

    app = create_app_cookies_multiple_cookies_success()
    client = TestClient(app)

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
    from app.main import create_app_cookies_26_cookie_secure_flag

    from spikard.testing import TestClient

    app = create_app_cookies_26_cookie_secure_flag()
    client = TestClient(app)

    cookies = {
        "auth_token": "secure_token_xyz",
    }
    response = await client.get("/secure", cookies=cookies)

    assert response.status_code == 200


async def test_response_cookie_with_domain_attribute() -> None:
    """Tests setting cookie with specific domain."""
    from app.main import create_app_cookies_response_cookie_with_domain_attribute

    from spikard.testing import TestClient

    app = create_app_cookies_response_cookie_with_domain_attribute()
    client = TestClient(app)

    json_data = {"value": "domain_test"}
    response = await client.post("/cookies/set-with-domain", json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "message" in response_data
    assert response_data["message"] == "Cookie set with domain"
