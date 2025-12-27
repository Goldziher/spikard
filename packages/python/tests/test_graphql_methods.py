"""Test the GraphQL methods added to TestClient."""

from __future__ import annotations

import pytest

from spikard import Spikard
from spikard.testing import TestClient


@pytest.fixture
def graphql_app() -> Spikard:
    """Create a simple GraphQL test app."""
    app = Spikard()

    @app.post("/graphql")
    async def handle_graphql(body: dict[str, object]) -> dict[str, object]:
        """Simple GraphQL endpoint for testing."""
        query = body.get("query", "")

        # Simple mock responses based on query
        if "hello" in str(query).lower():
            return {
                "data": {
                    "hello": "world",
                }
            }
        elif "error" in str(query).lower():
            return {
                "errors": [
                    {
                        "message": "Test error",
                        "code": "TEST_ERROR",
                    }
                ]
            }
        else:
            return {
                "data": {
                    "query": query,
                }
            }

    return app


@pytest.mark.asyncio
async def test_graphql_simple_query(graphql_app: Spikard) -> None:
    """Test sending a simple GraphQL query."""
    async with TestClient(graphql_app) as client:
        response = await client.graphql("query { hello }")

        assert response.status_code == 200
        data = response.json()
        assert data["data"]["hello"] == "world"


@pytest.mark.asyncio
async def test_graphql_with_variables(graphql_app: Spikard) -> None:
    """Test sending a GraphQL query with variables."""
    async with TestClient(graphql_app) as client:
        response = await client.graphql(
            "query Hello($name: String!) { hello }",
            variables={"name": "Alice"},
        )

        assert response.status_code == 200
        data = response.json()
        assert "data" in data


@pytest.mark.asyncio
async def test_graphql_with_operation_name(graphql_app: Spikard) -> None:
    """Test sending a GraphQL query with operation name."""
    async with TestClient(graphql_app) as client:
        response = await client.graphql(
            "query HelloQuery { hello }",
            operation_name="HelloQuery",
        )

        assert response.status_code == 200
        data = response.json()
        assert "data" in data


@pytest.mark.asyncio
async def test_graphql_with_errors(graphql_app: Spikard) -> None:
    """Test GraphQL response with errors."""
    async with TestClient(graphql_app) as client:
        response = await client.graphql("query { error }")

        assert response.status_code == 200
        body = response.json()
        assert "errors" in body
        assert len(body["errors"]) > 0
        assert body["errors"][0]["message"] == "Test error"


@pytest.mark.asyncio
async def test_graphql_with_status(graphql_app: Spikard) -> None:
    """Test graphql_with_status method."""
    async with TestClient(graphql_app) as client:
        status, response = await client.graphql_with_status("query { hello }")

        assert status == 200
        assert response.status_code == 200
        data = response.json()
        assert data["data"]["hello"] == "world"


@pytest.mark.asyncio
async def test_graphql_data_extraction(graphql_app: Spikard) -> None:
    """Test extracting data from GraphQL response."""
    async with TestClient(graphql_app) as client:
        response = await client.graphql("query { hello }")

        # Should not raise an error
        body = response.json()
        data = body.get("data")
        assert isinstance(data, dict)
        assert "hello" in data


@pytest.mark.asyncio
async def test_graphql_errors_extraction(graphql_app: Spikard) -> None:
    """Test extracting errors from GraphQL response."""
    async with TestClient(graphql_app) as client:
        response = await client.graphql("query { error }")

        # Should not raise an error
        body = response.json()
        errors = body.get("errors", [])
        assert isinstance(errors, list)
        assert len(errors) > 0


@pytest.mark.asyncio
async def test_graphql_no_data_field() -> None:
    """Test GraphQL response without data field."""
    app = Spikard()

    @app.post("/graphql")
    async def handle(body: dict[str, object]) -> dict[str, object]:
        return {"errors": [{"message": "Failed"}]}

    async with TestClient(app) as client:
        response = await client.graphql("query { test }")

        # Should return response without data field
        body = response.json()
        assert "errors" in body
        assert body.get("data") is None


@pytest.mark.asyncio
async def test_graphql_no_errors_field_returns_empty_list(graphql_app: Spikard) -> None:
    """Test that accessing graphql response without errors field returns empty list."""
    async with TestClient(graphql_app) as client:
        response = await client.graphql("query { hello }")

        # Should return response without errors field
        body = response.json()
        errors = body.get("errors", [])
        assert isinstance(errors, list)
        assert len(errors) == 0
