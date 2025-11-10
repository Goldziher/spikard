"""E2E tests for openapi."""

from spikard.testing import TestClient
from app.main import (
    create_app_openapi_openapi_spec_generation_basic,
    create_app_openapi_openapi_spec_with_api_key_security_scheme,
    create_app_openapi_openapi_spec_with_custom_metadata,
    create_app_openapi_openapi_spec_with_jwt_security_scheme,
    create_app_openapi_redoc_serving,
    create_app_openapi_swagger_ui_serving,
)


async def test_redoc_serving() -> None:
    """Tests that Redoc is served at the configured path with correct HTML."""

    app = create_app_openapi_redoc_serving()
    client = TestClient(app)

    headers = {
        "accept": "text/html",
    }
    response = await client.get("/redoc", headers=headers)

    assert response.status_code == 200


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


async def test_swagger_ui_serving() -> None:
    """Tests that Swagger UI is served at the configured path with correct HTML."""

    app = create_app_openapi_swagger_ui_serving()
    client = TestClient(app)

    headers = {
        "accept": "text/html",
    }
    response = await client.get("/docs", headers=headers)

    assert response.status_code == 200


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
