"""JSON-RPC 2.0 e2e tests generated from fixtures."""

import pytest
from httpx import AsyncClient
from app.main import *


@pytest.mark.asyncio
async def test_user_create_success_1():
    """Test user.create."""
    app = create_app_user_create()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.create",
                "params": {
                    "userData": {
                        "name": "Charlie Brown",
                        "email": "charlie@example.com",
                        "password": "SecurePass123!",
                        "role": "user",
                    }
                },
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "result" in data
        assert data["id"] == 1
        # Result should match expected structure
        result = data["result"]
        assert isinstance(result, dict)


@pytest.mark.asyncio
async def test_user_create_email_already_exists_error():
    """Test user.create - email_already_exists error case."""
    app = create_app_user_create()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.create",
                "params": {
                    "userData": {"name": "Duplicate User", "email": "alice@example.com", "password": "SecurePass123!"}
                },
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "error" in data
        assert data["error"]["code"] == 409
        assert "message" in data["error"]
        assert data["id"] == 1


@pytest.mark.asyncio
async def test_user_create_invalid_email_error():
    """Test user.create - invalid_email error case."""
    app = create_app_user_create()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.create",
                "params": {"userData": {"name": "Test User", "email": "not-an-email", "password": "SecurePass123!"}},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "error" in data
        assert data["error"]["code"] == -32602
        assert "message" in data["error"]
        assert data["id"] == 1


@pytest.mark.asyncio
async def test_user_create_password_too_short_error():
    """Test user.create - password_too_short error case."""
    app = create_app_user_create()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.create",
                "params": {"userData": {"name": "Test User", "email": "test@example.com", "password": "short"}},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "error" in data
        assert data["error"]["code"] == -32602
        assert "message" in data["error"]
        assert data["id"] == 1


@pytest.mark.asyncio
async def test_user_delete_success_1():
    """Test user.delete."""
    app = create_app_user_delete()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.delete",
                "params": {"userId": "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d"},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "result" in data
        assert data["id"] == 1
        # Result should match expected structure
        result = data["result"]
        assert isinstance(result, dict)


@pytest.mark.asyncio
async def test_user_delete_user_not_found_error():
    """Test user.delete - user_not_found error case."""
    app = create_app_user_delete()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.delete",
                "params": {"userId": "00000000-0000-0000-0000-000000000000"},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "error" in data
        assert data["error"]["code"] == 404
        assert "message" in data["error"]
        assert data["id"] == 1


@pytest.mark.asyncio
async def test_user_delete_invalid_uuid_error():
    """Test user.delete - invalid_uuid error case."""
    app = create_app_user_delete()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.delete",
                "params": {"userId": "invalid-id"},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "error" in data
        assert data["error"]["code"] == -32602
        assert "message" in data["error"]
        assert data["id"] == 1


@pytest.mark.asyncio
async def test_user_getbyid_success_1():
    """Test user.getById."""
    app = create_app_user_getbyid()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.getById",
                "params": {"userId": "550e8400-e29b-41d4-a716-446655440000"},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "result" in data
        assert data["id"] == 1
        # Result should match expected structure
        result = data["result"]
        assert isinstance(result, dict)


@pytest.mark.asyncio
async def test_user_getbyid_success_2():
    """Test user.getById."""
    app = create_app_user_getbyid()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.getById",
                "params": {"userId": "7c9e6679-7425-40de-944b-e07fc1f90ae7"},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "result" in data
        assert data["id"] == 1
        # Result should match expected structure
        result = data["result"]
        assert isinstance(result, dict)


@pytest.mark.asyncio
async def test_user_getbyid_user_not_found_error():
    """Test user.getById - user_not_found error case."""
    app = create_app_user_getbyid()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.getById",
                "params": {"userId": "00000000-0000-0000-0000-000000000000"},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "error" in data
        assert data["error"]["code"] == 404
        assert "message" in data["error"]
        assert data["id"] == 1


@pytest.mark.asyncio
async def test_user_getbyid_invalid_uuid_error():
    """Test user.getById - invalid_uuid error case."""
    app = create_app_user_getbyid()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.getById",
                "params": {"userId": "not-a-uuid"},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "error" in data
        assert data["error"]["code"] == -32602
        assert "message" in data["error"]
        assert data["id"] == 1


@pytest.mark.asyncio
async def test_user_getbyid_batch_request():
    """Test user.getById - batch request."""
    app = create_app_user_getbyid()
    async with AsyncClient(app=app, base_url="http://test") as client:
        batch_request = [
            {
                "jsonrpc": "2.0",
                "method": "user.getById",
                "params": {"userId": "550e8400-e29b-41d4-a716-446655440000"},
                "id": 1,
            },
            {
                "jsonrpc": "2.0",
                "method": "user.getById",
                "params": {"userId": "7c9e6679-7425-40de-944b-e07fc1f90ae7"},
                "id": 2,
            },
        ]
        response = await client.post("/rpc", json=batch_request)
        assert response.status_code == 200
        # Batch requests return array of responses
        responses = response.json()
        assert isinstance(responses, list)
        assert len(responses) >= 1


@pytest.mark.asyncio
async def test_user_list_success_1():
    """Test user.list."""
    app = create_app_user_list()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.list",
                "params": {"options": {"page": 1, "perPage": 10}},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "result" in data
        assert data["id"] == 1
        # Result should match expected structure
        result = data["result"]
        assert isinstance(result, dict)


@pytest.mark.asyncio
async def test_user_list_success_2():
    """Test user.list."""
    app = create_app_user_list()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.list",
                "params": {"options": {"page": 1, "perPage": 5, "role": "admin"}},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "result" in data
        assert data["id"] == 1
        # Result should match expected structure
        result = data["result"]
        assert isinstance(result, dict)


@pytest.mark.asyncio
async def test_user_list_invalid_page_error():
    """Test user.list - invalid_page error case."""
    app = create_app_user_list()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.list",
                "params": {"options": {"page": 0}},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "error" in data
        assert data["error"]["code"] == -32602
        assert "message" in data["error"]
        assert data["id"] == 1


@pytest.mark.asyncio
async def test_user_list_perpage_too_large_error():
    """Test user.list - perpage_too_large error case."""
    app = create_app_user_list()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.list",
                "params": {"options": {"perPage": 200}},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "error" in data
        assert data["error"]["code"] == -32602
        assert "message" in data["error"]
        assert data["id"] == 1


@pytest.mark.asyncio
async def test_user_list_batch_request():
    """Test user.list - batch request."""
    app = create_app_user_list()
    async with AsyncClient(app=app, base_url="http://test") as client:
        batch_request = [
            {
                "jsonrpc": "2.0",
                "method": "user.list",
                "params": {"options": {"page": 1, "perPage": 10}},
                "id": 1,
            },
            {
                "jsonrpc": "2.0",
                "method": "user.list",
                "params": {"options": {"page": 1, "perPage": 5, "role": "admin"}},
                "id": 2,
            },
        ]
        response = await client.post("/rpc", json=batch_request)
        assert response.status_code == 200
        # Batch requests return array of responses
        responses = response.json()
        assert isinstance(responses, list)
        assert len(responses) >= 1


@pytest.mark.asyncio
async def test_user_update_success_1():
    """Test user.update."""
    app = create_app_user_update()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.update",
                "params": {"userId": "550e8400-e29b-41d4-a716-446655440000", "updates": {"role": "admin"}},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "result" in data
        assert data["id"] == 1
        # Result should match expected structure
        result = data["result"]
        assert isinstance(result, dict)


@pytest.mark.asyncio
async def test_user_update_success_2():
    """Test user.update."""
    app = create_app_user_update()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.update",
                "params": {
                    "userId": "7c9e6679-7425-40de-944b-e07fc1f90ae7",
                    "updates": {"name": "Robert Smith", "email": "robert@example.com"},
                },
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "result" in data
        assert data["id"] == 1
        # Result should match expected structure
        result = data["result"]
        assert isinstance(result, dict)


@pytest.mark.asyncio
async def test_user_update_user_not_found_error():
    """Test user.update - user_not_found error case."""
    app = create_app_user_update()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.update",
                "params": {"userId": "00000000-0000-0000-0000-000000000000", "updates": {"name": "New Name"}},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "error" in data
        assert data["error"]["code"] == 404
        assert "message" in data["error"]
        assert data["id"] == 1


@pytest.mark.asyncio
async def test_user_update_invalid_role_error():
    """Test user.update - invalid_role error case."""
    app = create_app_user_update()
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/rpc",
            json={
                "jsonrpc": "2.0",
                "method": "user.update",
                "params": {"userId": "550e8400-e29b-41d4-a716-446655440000", "updates": {"role": "superuser"}},
                "id": 1,
            },
        )
        assert response.status_code == 200
        data = response.json()
        assert data["jsonrpc"] == "2.0"
        assert "error" in data
        assert data["error"]["code"] == -32602
        assert "message" in data["error"]
        assert data["id"] == 1


@pytest.mark.asyncio
async def test_user_update_batch_request():
    """Test user.update - batch request."""
    app = create_app_user_update()
    async with AsyncClient(app=app, base_url="http://test") as client:
        batch_request = [
            {
                "jsonrpc": "2.0",
                "method": "user.update",
                "params": {"userId": "550e8400-e29b-41d4-a716-446655440000", "updates": {"role": "admin"}},
                "id": 1,
            },
            {
                "jsonrpc": "2.0",
                "method": "user.update",
                "params": {
                    "userId": "7c9e6679-7425-40de-944b-e07fc1f90ae7",
                    "updates": {"name": "Robert Smith", "email": "robert@example.com"},
                },
                "id": 2,
            },
        ]
        response = await client.post("/rpc", json=batch_request)
        assert response.status_code == 200
        # Batch requests return array of responses
        responses = response.json()
        assert isinstance(responses, list)
        assert len(responses) >= 1
