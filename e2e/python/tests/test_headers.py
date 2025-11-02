"""E2E tests for headers."""

from typing import Any


async def test_header_regex_validation__success(client: Any) -> None:
    """Tests header with regex pattern validation success."""
    headers = {
        "X-Request-Id": "12345",
    }
    response = await client.get("/headers/pattern", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "x_request_id" in response_data
    assert response_data["x_request_id"] == "12345"


async def test_33_api_key_header_valid(client: Any) -> None:
    """X-API-Key header with valid format should be accepted."""
    headers = {
        "X-API-Key": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 200


async def test_content_type_header__application_json(client: Any) -> None:
    """Tests Content-Type header with JSON media type."""
    headers = {
        "Content-Type": "application/json",
    }
    response = await client.get("/headers/content-type", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "content_type" in response_data
    assert response_data["content_type"] == "application/json"


async def test_accept_language_header(client: Any) -> None:
    """Tests Accept-Language header for locale/i18n."""
    headers = {
        "Accept-Language": "en-US,en;q=0.9",
    }
    response = await client.get("/headers/accept-language", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "accept_language" in response_data
    assert response_data["accept_language"] == "en-US,en;q=0.9"


async def test_x_api_key_required_header__success(client: Any) -> None:
    """Tests required X-API-Key header with valid value."""
    headers = {
        "key": "secret",
    }
    response = await client.get("/users/me", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "username" in response_data
    assert response_data["username"] == "secret"


async def test_header_validation__max_length_constraint_fail(client: Any) -> None:
    """Tests header validation with max_length constraint failure."""
    headers = {
        "X-Session-Id": "this_is_way_too_long_for_validation",
    }
    response = await client.get("/headers/max-length", headers=headers)

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
    assert response_data["errors"][0]["input"] == "this_is_way_too_long_for_validation"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "header"
    assert response_data["errors"][0]["loc"][1] == "x-session-id"
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


async def test_x_api_key_required_header__missing(client: Any) -> None:
    """Tests required X-API-Key header when not provided, returns 403."""
    response = await client.get("/users/me")

    assert response.status_code == 403
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Not authenticated"


async def test_origin_header(client: Any) -> None:
    """Tests Origin header for CORS."""
    headers = {
        "Origin": "https://example.com",
    }
    response = await client.get("/headers/origin", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "origin" in response_data
    assert response_data["origin"] == "https://example.com"


async def test_user_agent_header__default_value(client: Any) -> None:
    """Tests optional User-Agent header when not provided, uses testclient default."""
    response = await client.get("/items/")

    assert response.status_code == 200
    response_data = response.json()
    assert "User-Agent" in response_data
    assert response_data["User-Agent"] == "testclient"


async def test_32_bearer_token_missing_prefix(client: Any) -> None:
    """Authorization header without Bearer prefix should fail validation."""
    headers = {
        "Authorization": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
    }
    response = await client.get("/protected", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "pattern" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["pattern"] == "^Bearer [A-Za-z0-9-._~+/]+=*$"
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "headers"
    assert response_data["errors"][0]["loc"][1] == "Authorization"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Invalid Bearer token format"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_optional_header_with_none_default__missing(client: Any) -> None:
    """Tests optional header parameter with None default when not provided."""
    response = await client.get("/items/")

    assert response.status_code == 200
    response_data = response.json()
    assert "strange_header" in response_data
    assert response_data["strange_header"] is None


async def test_header_regex_validation__fail(client: Any) -> None:
    """Tests header with regex pattern validation failure."""
    headers = {
        "X-Request-Id": "invalid-format",
    }
    response = await client.get("/headers/pattern", headers=headers)

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
    assert response_data["errors"][0]["input"] == "invalid-format"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "header"
    assert response_data["errors"][0]["loc"][1] == "x-request-id"
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


async def test_31_bearer_token_format_invalid(client: Any) -> None:
    """Authorization header with invalid Bearer token format should fail validation."""
    headers = {
        "Authorization": "Bearer invalid token with spaces",
    }
    response = await client.get("/protected", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "pattern" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["pattern"] == "^Bearer [A-Za-z0-9-._~+/]+=*$"
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == "Bearer invalid token with spaces"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "headers"
    assert response_data["errors"][0]["loc"][1] == "Authorization"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Invalid Bearer token format"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_x_api_key_optional_header__success(client: Any) -> None:
    """Tests optional X-API-Key header with valid value."""
    headers = {
        "key": "secret",
    }
    response = await client.get("/users/me", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "msg" in response_data
    assert response_data["msg"] == "Hello secret"


async def test_authorization_header__success(client: Any) -> None:
    """Tests Authorization header with valid Digest scheme."""
    headers = {
        "Authorization": "Digest foobar",
    }
    response = await client.get("/users/me", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "credentials" in response_data
    assert response_data["credentials"] == "foobar"
    assert "scheme" in response_data
    assert response_data["scheme"] == "Digest"


async def test_30_bearer_token_format_valid(client: Any) -> None:
    """Authorization header with valid Bearer token format should be accepted."""
    headers = {
        "Authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U",
    }
    response = await client.get("/protected", headers=headers)

    assert response.status_code == 200


async def test_authorization_header__missing(client: Any) -> None:
    """Tests missing Authorization header returns 403."""
    response = await client.get("/users/me")

    assert response.status_code == 403
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Not authenticated"


async def test_accept_header__json(client: Any) -> None:
    """Tests Accept header for content negotiation."""
    headers = {
        "Accept": "application/json",
    }
    response = await client.get("/headers/accept", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "accept" in response_data
    assert response_data["accept"] == "application/json"


async def test_accept_encoding_header(client: Any) -> None:
    """Tests Accept-Encoding header for compression negotiation."""
    headers = {
        "Accept-Encoding": "gzip, deflate, br",
    }
    response = await client.get("/headers/accept-encoding", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "accept_encoding" in response_data
    assert response_data["accept_encoding"] == "gzip, deflate, br"


async def test_authorization_header__wrong_scheme(client: Any) -> None:
    """Tests Authorization header with incorrect scheme returns 403."""
    headers = {
        "Authorization": "Other invalidauthorization",
    }
    response = await client.get("/users/me", headers=headers)

    assert response.status_code == 403
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Invalid authentication credentials"


async def test_header_validation__min_length_constraint(client: Any) -> None:
    """Tests header validation with min_length constraint."""
    headers = {
        "X-Token": "ab",
    }
    response = await client.get("/headers/validated", headers=headers)

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
    assert response_data["errors"][0]["loc"][0] == "header"
    assert response_data["errors"][0]["loc"][1] == "x-token"
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


async def test_basic_authentication__success(client: Any) -> None:
    """Tests Authorization header with Basic auth scheme."""
    headers = {
        "Authorization": "Basic dXNlcm5hbWU6cGFzc3dvcmQ=",
    }
    response = await client.get("/headers/basic-auth", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "password" in response_data
    assert response_data["password"] == "password"
    assert "username" in response_data
    assert response_data["username"] == "username"


async def test_bearer_token_authentication__missing(client: Any) -> None:
    """Tests missing Bearer token returns 401 Unauthorized."""
    response = await client.get("/headers/bearer-auth")

    assert response.status_code == 401
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "Not authenticated"


async def test_x_api_key_optional_header__missing(client: Any) -> None:
    """Tests optional X-API-Key header when not provided, returns fallback message."""
    response = await client.get("/users/me")

    assert response.status_code == 200
    response_data = response.json()
    assert "msg" in response_data
    assert response_data["msg"] == "Hello World"


async def test_multiple_custom_headers(client: Any) -> None:
    """Tests multiple custom headers in single request."""
    headers = {
        "X-Client-Version": "1.2.3",
        "X-Trace-Id": "trace-abc",
        "X-Request-Id": "req-12345",
    }
    response = await client.get("/headers/multiple", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "x_client_version" in response_data
    assert response_data["x_client_version"] == "1.2.3"
    assert "x_request_id" in response_data
    assert response_data["x_request_id"] == "req-12345"
    assert "x_trace_id" in response_data
    assert response_data["x_trace_id"] == "trace-abc"


async def test_34_api_key_header_invalid(client: Any) -> None:
    """X-API-Key header with invalid format should fail validation."""
    headers = {
        "X-API-Key": "invalid-key",
    }
    response = await client.get("/api/data", headers=headers)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "pattern" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["pattern"] == "^[a-f0-9]{32}$"
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == "invalid-key"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "headers"
    assert response_data["errors"][0]["loc"][1] == "X-API-Key"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Invalid API key format"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_bearer_token_authentication__success(client: Any) -> None:
    """Tests Authorization header with Bearer token scheme."""
    headers = {
        "Authorization": "Bearer valid_token_123",
    }
    response = await client.get("/headers/bearer-auth", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "token" in response_data
    assert response_data["token"] == "valid_token_123"


async def test_host_header(client: Any) -> None:
    """Tests Host header (standard HTTP header)."""
    headers = {
        "Host": "example.com:8080",
    }
    response = await client.get("/headers/host", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "host" in response_data
    assert response_data["host"] == "example.com:8080"


async def test_referer_header(client: Any) -> None:
    """Tests Referer header (standard misspelling)."""
    headers = {
        "Referer": "https://example.com/page",
    }
    response = await client.get("/headers/referer", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "referer" in response_data
    assert response_data["referer"] == "https://example.com/page"


async def test_header_with_underscore_conversion__explicit(client: Any) -> None:
    """Tests X-Token header converted to x_token parameter."""
    headers = {
        "X-Token": "secret123",
    }
    response = await client.get("/headers/underscore", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "x_token" in response_data
    assert response_data["x_token"] == "secret123"


async def test_header_case_insensitivity__access(client: Any) -> None:
    """Tests case-insensitive header access (Content-Type vs content-type)."""
    headers = {
        "content-type": "application/json",
    }
    json_data = {"test": "data"}
    response = await client.post("/echo", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "content_type_lower" in response_data
    assert response_data["content_type_lower"] == "application/json"
    assert "content_type_mixed" in response_data
    assert response_data["content_type_mixed"] == "application/json"
    assert "content_type_upper" in response_data
    assert response_data["content_type_upper"] == "application/json"


async def test_user_agent_header__custom_value(client: Any) -> None:
    """Tests User-Agent header with custom value."""
    headers = {
        "User-Agent": "Mozilla/5.0 Custom Browser",
    }
    response = await client.get("/items/", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
    assert "User-Agent" in response_data
    assert response_data["User-Agent"] == "Mozilla/5.0 Custom Browser"
