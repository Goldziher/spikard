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
        if "error" in str(query).lower():
            return {
                "errors": [
                    {
                        "message": "Test error",
                        "code": "TEST_ERROR",
                    }
                ]
            }
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


@pytest.mark.asyncio
async def test_graphql_subscription_returns_first_event() -> None:
    """Test GraphQL subscription helper over WebSocket."""
    app = Spikard()
    state: dict[str, str] = {"operation_id": "1"}

    @app.websocket("/graphql")
    async def graphql_ws(message: dict[str, object]) -> dict[str, object] | None:
        msg_type = message.get("type")
        if msg_type == "connection_init":
            return {"type": "connection_ack"}
        if msg_type == "subscribe":
            op_id = message.get("id", "1")
            state["operation_id"] = str(op_id)
            return {
                "id": state["operation_id"],
                "type": "next",
                "payload": {"data": {"ticker": "AAPL"}},
            }
        if msg_type == "complete":
            return {"id": state["operation_id"], "type": "complete"}
        return None

    async with TestClient(app) as client:
        snapshot = await client.graphql_subscription("subscription { ticker }")

        assert snapshot["acknowledged"] is True
        assert snapshot["event"] == {"data": {"ticker": "AAPL"}}
        assert snapshot["errors"] == []
        assert snapshot["complete_received"] is True


@pytest.mark.asyncio
async def test_graphql_subscription_surfaces_connection_error() -> None:
    """Test GraphQL subscription helper raises on init rejection."""
    app = Spikard()

    @app.websocket("/graphql")
    async def graphql_ws(message: dict[str, object]) -> dict[str, object] | None:
        if message.get("type") == "connection_init":
            return {"type": "connection_error", "payload": {"message": "denied"}}
        return None

    async with TestClient(app) as client:
        with pytest.raises(RuntimeError, match="connection_error"):
            await client.graphql_subscription("subscription { privateFeed }")
