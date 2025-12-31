"""E2E tests for published Spikard Python package.

These tests validate that:
1. The published package installs correctly from PyPI (0.7.3)
2. Core functionality works as expected via HTTP requests
3. Real server is spawned and tested via HTTP (not direct handler calls)
"""

import importlib.metadata

import pytest
from app import app

from spikard import TestClient


def test_package_version() -> None:
    """Validate that the installed version is 0.7.3 from PyPI."""
    version = importlib.metadata.version("spikard")
    assert version == "0.7.3", f"Expected version 0.7.3, got {version}"


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
