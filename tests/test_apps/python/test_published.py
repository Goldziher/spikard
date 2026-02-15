"""E2E tests for published Spikard Python package.

These tests validate that:
1. The published package installs correctly from PyPI (0.10.1)
2. Core functionality works as expected via HTTP requests
3. Real server is spawned and tested via HTTP (not direct handler calls)
"""

import importlib.metadata

import pytest
from app import app

import spikard
from spikard import TestClient


def test_package_version() -> None:
    """Validate that the installed version is 0.10.1 from PyPI."""
    version = importlib.metadata.version("spikard")
    assert version == "0.10.1", f"Expected version 0.10.1, got {version}"


@pytest.mark.asyncio
async def test_health_check() -> None:
    """Validate health check endpoint returns correct response."""
    async with TestClient(app) as client:
        response = await client.get("/health")
        assert response.status_code == 200
        data = response.json()
        assert data == {"status": "ok"}


@pytest.mark.asyncio
async def test_query_parameters() -> None:
    """Validate query parameter handling."""
    async with TestClient(app) as client:
        response = await client.get("/query?name=test&age=25")
        assert response.status_code == 200
        data = response.json()
        assert data == {"name": "test", "age": 25}


@pytest.mark.asyncio
async def test_json_echo() -> None:
    """Validate JSON echo endpoint."""
    payload = {"message": "Hello from Python!"}
    async with TestClient(app) as client:
        response = await client.post("/echo", json=payload)
        assert response.status_code == 200
        data = response.json()
        assert data["received"] == payload
        assert data["method"] == "POST"


@pytest.mark.asyncio
async def test_path_parameters() -> None:
    """Validate path parameter extraction."""
    async with TestClient(app) as client:
        response = await client.get("/users/123")
        assert response.status_code == 200
        data = response.json()
        assert data["userId"] == "123"
        assert data["type"] == "string"


@pytest.mark.asyncio
async def test_put_method() -> None:
    """Validate PUT method and path parameters."""
    payload = {"name": "Widget"}
    async with TestClient(app) as client:
        response = await client.put("/items/1", json=payload)
        assert response.status_code == 200
        data = response.json()
        assert data["itemId"] == "1"
        assert data["updated"] == payload
        assert data["method"] == "PUT"


@pytest.mark.asyncio
async def test_delete_method() -> None:
    """Validate DELETE method and path parameters."""
    async with TestClient(app) as client:
        response = await client.delete("/items/1")
        assert response.status_code == 200
        data = response.json()
        assert data["itemId"] == "1"
        assert data["deleted"] is True
        assert data["method"] == "DELETE"


@pytest.mark.asyncio
async def test_patch_method() -> None:
    """Validate PATCH method and path parameters."""
    payload = {"name": "Updated"}
    async with TestClient(app) as client:
        response = await client.patch("/items/1", json=payload)
        assert response.status_code == 200
        data = response.json()
        assert data["itemId"] == "1"
        assert data["patched"] == payload
        assert data["method"] == "PATCH"


@pytest.mark.asyncio
async def test_header_extraction() -> None:
    """Validate custom header extraction."""
    async with TestClient(app) as client:
        response = await client.get("/headers", headers={"X-Custom-Header": "test-value"})
        assert response.status_code == 200
        data = response.json()
        assert data == {"x-custom-header": "test-value"}


@pytest.mark.asyncio
async def test_cookie_extraction() -> None:
    """Validate session cookie extraction."""
    async with TestClient(app) as client:
        response = await client.get("/cookies", headers={"Cookie": "session=abc123"})
        assert response.status_code == 200
        data = response.json()
        assert data == {"session": "abc123"}


@pytest.mark.asyncio
async def test_404_not_found() -> None:
    """Validate 404 not found response."""
    async with TestClient(app) as client:
        response = await client.get("/nonexistent")
        assert response.status_code == 404


@pytest.mark.asyncio
async def test_error_500() -> None:
    """Validate 500 error handling."""
    async with TestClient(app) as client:
        response = await client.get("/error")
        assert response.status_code == 500


def test_imports() -> None:
    """Validate that all necessary imports are available from spikard."""
    # Verify public exports are available and callable
    assert callable(spikard.get)
    assert callable(spikard.post)
    assert callable(spikard.put)
    assert callable(spikard.delete)
    assert callable(spikard.patch)
    assert callable(spikard.Header)
    assert callable(spikard.Cookie)
    assert callable(spikard.Query)
    assert callable(spikard.Path)
    assert callable(spikard.Spikard)
    assert callable(spikard.TestClient)
