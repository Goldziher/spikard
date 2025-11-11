"""E2E tests for openapi."""

from spikard.testing import TestClient
from app.main import (
    create_app_openapi_openapi_generation_disabled,
    create_app_openapi_openapi_spec_generation_basic,
    create_app_openapi_openapi_spec_with_api_key_security_scheme,
    create_app_openapi_openapi_spec_with_custom_metadata,
    create_app_openapi_openapi_spec_with_jwt_security_scheme,
    create_app_openapi_openapi_with_multiple_security_schemes,
    create_app_openapi_openapi_with_multiple_server_configurations,
    create_app_openapi_openapi_with_oauth2_flows,
    create_app_openapi_openapi_with_reusable_component_schemas,
    create_app_openapi_openapi_with_tags_and_external_documentation,
    create_app_openapi_redoc_serving,
    create_app_openapi_redoc_with_customization_options,
    create_app_openapi_swagger_ui_serving,
    create_app_openapi_swagger_ui_with_customization_options,
)


async def test_redoc_with_customization_options() -> None:
    """Tests Redoc serving with custom theme, layout options, and UI preferences."""

    app = create_app_openapi_redoc_with_customization_options()
    client = TestClient(app)

    headers = {
        "accept": "text/html",
    }
    response = await client.get("/redoc", headers=headers)

    assert response.status_code == 200


async def test_redoc_serving() -> None:
    """Tests that Redoc is served at the configured path with correct HTML."""

    app = create_app_openapi_redoc_serving()
    client = TestClient(app)

    headers = {
        "accept": "text/html",
    }
    response = await client.get("/redoc", headers=headers)

    assert response.status_code == 200


async def test_openapi_with_multiple_security_schemes() -> None:
    """Tests OpenAPI spec generation with both JWT and API key security schemes simultaneously."""

    app = create_app_openapi_openapi_with_multiple_security_schemes()
    client = TestClient(app)

    headers = {
        "accept": "application/json",
    }
    response = await client.get("/openapi.json", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_openapi_spec_with_jwt_security_scheme() -> None:
    """Tests that JWT authentication is auto-detected and included in OpenAPI security schemes."""

    app = create_app_openapi_openapi_spec_with_jwt_security_scheme()
    client = TestClient(app)

    headers = {
        "accept": "application/json",
    }
    response = await client.get("/openapi.json", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_swagger_ui_with_customization_options() -> None:
    """Tests Swagger UI serving with custom CSS, query parameters, and persistence settings."""

    app = create_app_openapi_swagger_ui_with_customization_options()
    client = TestClient(app)

    headers = {
        "accept": "text/html",
    }
    response = await client.get("/docs", headers=headers)

    assert response.status_code == 200


async def test_openapi_with_multiple_server_configurations() -> None:
    """Tests OpenAPI spec generation with production, staging, and development server URLs."""

    app = create_app_openapi_openapi_with_multiple_server_configurations()
    client = TestClient(app)

    headers = {
        "accept": "application/json",
    }
    response = await client.get("/openapi.json", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_swagger_ui_serving() -> None:
    """Tests that Swagger UI is served at the configured path with correct HTML."""

    app = create_app_openapi_swagger_ui_serving()
    client = TestClient(app)

    headers = {
        "accept": "text/html",
    }
    response = await client.get("/docs", headers=headers)

    assert response.status_code == 200


async def test_openapi_with_reusable_component_schemas() -> None:
    """Tests OpenAPI spec generation with reusable schemas in components section."""

    app = create_app_openapi_openapi_with_reusable_component_schemas()
    client = TestClient(app)

    headers = {
        "accept": "application/json",
    }
    response = await client.get("/openapi.json", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_openapi_spec_with_api_key_security_scheme() -> None:
    """Tests that API key authentication is auto-detected and included in OpenAPI security schemes."""

    app = create_app_openapi_openapi_spec_with_api_key_security_scheme()
    client = TestClient(app)

    headers = {
        "accept": "application/json",
    }
    response = await client.get("/openapi.json", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_openapi_with_oauth2_flows() -> None:
    """Tests OpenAPI spec generation with OAuth2 authorization code and client credentials flows."""

    app = create_app_openapi_openapi_with_oauth2_flows()
    client = TestClient(app)

    headers = {
        "accept": "application/json",
    }
    response = await client.get("/openapi.json", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_openapi_with_tags_and_external_documentation() -> None:
    """Tests OpenAPI spec generation with endpoint tags and external documentation links."""

    app = create_app_openapi_openapi_with_tags_and_external_documentation()
    client = TestClient(app)

    headers = {
        "accept": "application/json",
    }
    response = await client.get("/openapi.json", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_openapi_generation_disabled() -> None:
    """Tests that OpenAPI endpoints return 404 when documentation generation is disabled."""

    app = create_app_openapi_openapi_generation_disabled()
    client = TestClient(app)

    headers = {
        "accept": "application/json",
    }
    response = await client.get("/openapi.json", headers=headers)

    assert response.status_code == 404
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "OpenAPI documentation is not enabled"
    assert "status" in response_data
    assert response_data["status"] == 404
    assert "title" in response_data
    assert response_data["title"] == "Not Found"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/not-found"


async def test_openapi_spec_with_custom_metadata() -> None:
    """Tests that custom contact, license, and server info are included in OpenAPI spec."""

    app = create_app_openapi_openapi_spec_with_custom_metadata()
    client = TestClient(app)

    headers = {
        "accept": "application/json",
    }
    response = await client.get("/openapi.json", headers=headers)

    assert response.status_code == 200
    response_data = response.json()


async def test_openapi_spec_generation_basic() -> None:
    """Tests that OpenAPI spec is generated and available at /openapi.json with correct structure."""

    app = create_app_openapi_openapi_spec_generation_basic()
    client = TestClient(app)

    headers = {
        "accept": "application/json",
    }
    response = await client.get("/openapi.json", headers=headers)

    assert response.status_code == 200
    response_data = response.json()
