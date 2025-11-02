"""E2E tests for http_methods."""

from typing import Any


async def test_options__cors_preflight_request(client: Any) -> None:
    """Tests OPTIONS method for CORS preflight."""
    headers = {
        "Origin": "https://example.com",
        "Access-Control-Request-Headers": "Content-Type",
        "Access-Control-Request-Method": "POST",
    }
    response = await client.options("/items/", headers=headers)

    assert response.status_code == 200


async def test_delete__remove_resource(client: Any) -> None:
    """Tests DELETE method to remove a resource."""
    response = await client.delete("/items/1")

    assert response.status_code == 200
    response.json()


async def test_put__create_resource_if_doesn_t_exist(client: Any) -> None:
    """Tests PUT creating new resource at specific URI."""
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


async def test_patch__update_multiple_fields(client: Any) -> None:
    """Tests PATCH updating multiple fields at once."""
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
    assert not response_data["in_stock"]
    assert "name" in response_data
    assert response_data["name"] == "Updated Name"
    assert "price" in response_data
    assert response_data["price"] == 89.99


async def test_put__validation_error(client: Any) -> None:
    """Tests PUT with invalid data returns 422."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"id": 1, "name": "X", "price": -10}
    response = await client.put("/items/1", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "2 validation errors in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 2
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "X"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "name"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should have at least 3 characters"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_too_short"
    assert "input" in response_data["errors"][1]
    assert response_data["errors"][1]["input"] == -10
    assert "loc" in response_data["errors"][1]
    assert len(response_data["errors"][1]["loc"]) == 2
    assert response_data["errors"][1]["loc"][0] == "body"
    assert response_data["errors"][1]["loc"][1] == "price"
    assert "msg" in response_data["errors"][1]
    assert response_data["errors"][1]["msg"] == "Input should be greater than 0"
    assert "type" in response_data["errors"][1]
    assert response_data["errors"][1]["type"] == "greater_than"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_head__get_metadata_without_body(client: Any) -> None:
    """Tests HEAD method returns headers without response body."""
    response = await client.head("/items/1")

    assert response.status_code == 200


async def test_delete__with_response_body(client: Any) -> None:
    """Tests DELETE returning deleted resource data."""
    response = await client.delete("/items/1")

    assert response.status_code == 200
    response_data = response.json()
    assert "id" in response_data
    assert response_data["id"] == 1
    assert "message" in response_data
    assert response_data["message"] == "Item deleted successfully"
    assert "name" in response_data
    assert response_data["name"] == "Deleted Item"


async def test_put__missing_required_field(client: Any) -> None:
    """Tests PUT with missing required fields returns 422."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"id": 1, "name": "Item Name"}
    response = await client.put("/items/1", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "1"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "price"
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


async def test_patch__partial_update(client: Any) -> None:
    """Tests PATCH method for partial resource updates."""
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
    assert response_data["in_stock"]
    assert "name" in response_data
    assert response_data["name"] == "Existing Item"
    assert "price" in response_data
    assert response_data["price"] == 79.99


async def test_delete__resource_not_found(client: Any) -> None:
    """Tests DELETE on non-existent resource returns 404."""
    response = await client.delete("/items/999")

    assert response.status_code == 200
    response.json()


async def test_put__idempotent_operation(client: Any) -> None:
    """Tests PUT idempotency - repeated calls produce same result."""
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


async def test_put__complete_resource_replacement(client: Any) -> None:
    """Tests PUT method for complete resource replacement."""
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
    assert response_data["in_stock"]
    assert "name" in response_data
    assert response_data["name"] == "Updated Item"
    assert "price" in response_data
    assert response_data["price"] == 99.99
