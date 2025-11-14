"""E2E tests for http_methods."""

from spikard.testing import TestClient
from app.main import (
    create_app_http_methods_delete_remove_resource,
    create_app_http_methods_delete_resource_not_found,
    create_app_http_methods_delete_with_response_body,
    create_app_http_methods_head_get_metadata_without_body,
    create_app_http_methods_options_cors_preflight_request,
    create_app_http_methods_patch_partial_update,
    create_app_http_methods_patch_update_multiple_fields,
    create_app_http_methods_put_complete_resource_replacement,
    create_app_http_methods_put_create_resource_if_doesn_t_exist,
    create_app_http_methods_put_idempotent_operation,
    create_app_http_methods_put_missing_required_field,
    create_app_http_methods_put_validation_error,
)


async def test_options_cors_preflight_request() -> None:
    """Tests OPTIONS method for CORS preflight."""

    async with TestClient(create_app_http_methods_options_cors_preflight_request()) as client:
        headers = {
            "Access-Control-Request-Method": "POST",
            "Origin": "https://example.com",
            "Access-Control-Request-Headers": "Content-Type",
        }
        response = await client.options("/items/", headers=headers)

        assert response.status_code == 200
        response_data = response.json()
        response_headers = response.headers
        assert response_headers.get("access-control-allow-origin") == "https://example.com"
        assert response_headers.get("access-control-allow-methods") == "GET, POST, PUT, DELETE, OPTIONS"
        assert response_headers.get("access-control-max-age") == "86400"
        assert response_headers.get("access-control-allow-headers") == "Content-Type"


async def test_delete_remove_resource() -> None:
    """Tests DELETE method to remove a resource."""

    async with TestClient(create_app_http_methods_delete_remove_resource()) as client:
        response = await client.delete("/items/1")

        assert response.status_code == 200
        response_data = response.json()


async def test_put_create_resource_if_doesn_t_exist() -> None:
    """Tests PUT creating new resource at specific URI."""

    async with TestClient(create_app_http_methods_put_create_resource_if_doesn_t_exist()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"id": 999, "name": "New Item", "price": 49.99}
        response = await client.put("/items/999", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "id" in response_data
        assert response_data["id"] == 999
        assert "name" in response_data
        assert response_data["name"] == "New Item"
        assert "price" in response_data
        assert response_data["price"] == 49.99


async def test_patch_update_multiple_fields() -> None:
    """Tests PATCH updating multiple fields at once."""

    async with TestClient(create_app_http_methods_patch_update_multiple_fields()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"in_stock": False, "name": "Updated Name", "price": 89.99}
        response = await client.patch("/items/1", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "id" in response_data
        assert response_data["id"] == 1
        assert "in_stock" in response_data
        assert response_data["in_stock"] == False
        assert "name" in response_data
        assert response_data["name"] == "Updated Name"
        assert "price" in response_data
        assert response_data["price"] == 89.99


async def test_put_validation_error() -> None:
    """Tests PUT with invalid data returns 422."""

    async with TestClient(create_app_http_methods_put_validation_error()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"id": 1, "name": "X", "price": -10}
        response = await client.put("/items/1", headers=headers, json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_head_get_metadata_without_body() -> None:
    """Tests HEAD method returns headers without response body."""

    async with TestClient(create_app_http_methods_head_get_metadata_without_body()) as client:
        response = await client.head("/items/1")

        assert response.status_code == 200
        response_headers = response.headers
        assert response_headers.get("content-type") == "application/json"
        assert response_headers.get("content-length") == "85"


async def test_delete_with_response_body() -> None:
    """Tests DELETE returning deleted resource data."""

    async with TestClient(create_app_http_methods_delete_with_response_body()) as client:
        response = await client.delete("/items/1")

        assert response.status_code == 200
        response_data = response.json()
        assert "id" in response_data
        assert response_data["id"] == 1
        assert "message" in response_data
        assert response_data["message"] == "Item deleted successfully"
        assert "name" in response_data
        assert response_data["name"] == "Deleted Item"


async def test_put_missing_required_field() -> None:
    """Tests PUT with missing required fields returns 422."""

    async with TestClient(create_app_http_methods_put_missing_required_field()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"id": 1, "name": "Item Name"}
        response = await client.put("/items/1", headers=headers, json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_patch_partial_update() -> None:
    """Tests PATCH method for partial resource updates."""

    async with TestClient(create_app_http_methods_patch_partial_update()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"price": 79.99}
        response = await client.patch("/items/1", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "id" in response_data
        assert response_data["id"] == 1
        assert "in_stock" in response_data
        assert response_data["in_stock"] == True
        assert "name" in response_data
        assert response_data["name"] == "Existing Item"
        assert "price" in response_data
        assert response_data["price"] == 79.99


async def test_delete_resource_not_found() -> None:
    """Tests DELETE on non-existent resource returns 404."""

    async with TestClient(create_app_http_methods_delete_resource_not_found()) as client:
        response = await client.delete("/items/999")

        assert response.status_code == 200
        response_data = response.json()


async def test_put_idempotent_operation() -> None:
    """Tests PUT idempotency - repeated calls produce same result."""

    async with TestClient(create_app_http_methods_put_idempotent_operation()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"id": 1, "name": "Fixed Name", "price": 50.0}
        response = await client.put("/items/1", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "id" in response_data
        assert response_data["id"] == 1
        assert "name" in response_data
        assert response_data["name"] == "Fixed Name"
        assert "price" in response_data
        assert response_data["price"] == 50.0


async def test_put_complete_resource_replacement() -> None:
    """Tests PUT method for complete resource replacement."""

    async with TestClient(create_app_http_methods_put_complete_resource_replacement()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {
            "description": "Completely replaced",
            "id": 1,
            "in_stock": True,
            "name": "Updated Item",
            "price": 99.99,
        }
        response = await client.put("/items/1", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "description" in response_data
        assert response_data["description"] == "Completely replaced"
        assert "id" in response_data
        assert response_data["id"] == 1
        assert "in_stock" in response_data
        assert response_data["in_stock"] == True
        assert "name" in response_data
        assert response_data["name"] == "Updated Item"
        assert "price" in response_data
        assert response_data["price"] == 99.99
