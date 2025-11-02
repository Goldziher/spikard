"""E2E tests for json_bodies."""

import pytest
from typing import Any

async def test_uuid_field__invalid_format(client: Any) -> None:
    """Tests UUID field with invalid UUID format."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"item_id": "not-a-valid-uuid", "name": "Item"}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "not-a-valid-uuid"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "item_id"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be a valid UUID"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "uuid_parsing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_44_const_validation_failure(client: Any) -> None:
    """Field with const constraint not matching exact value should fail."""
    json_data = {"data": "test", "version": "2.0"}
    response = await client.post("/api/v1/data", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "const" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["const"] == "1.0"
    assert "value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["value"] == "2.0"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "version"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Value must be exactly '1.0'"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_boolean_field__success(client: Any) -> None:
    """Tests JSON object with boolean field."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"in_stock": True, "name": "Item", "price": 42.0}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "in_stock" in response_data
    assert response_data["in_stock"] == True
    assert "name" in response_data
    assert response_data["name"] == "Item"
    assert "price" in response_data
    assert response_data["price"] == 42.0


async def test_numeric_le_validation__success(client: Any) -> None:
    """Tests numeric field with le (less than or equal) constraint at boundary."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item", "price": 100.0}
    response = await client.post("/items/validated", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "Item"
    assert "price" in response_data
    assert response_data["price"] == 100.0


async def test_deeply_nested_objects(client: Any) -> None:
    """Tests deeply nested JSON structure (3+ levels)."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Product", "price": 100.0, "seller": {"address": {"city": "Springfield", "country": {"code": "US", "name": "USA"}, "street": "123 Main St"}, "name": "John Doe"}}
    response = await client.post("/items/nested", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "Product"
    assert "price" in response_data
    assert response_data["price"] == 100.0
    assert "seller" in response_data
    assert "address" in response_data["seller"]
    assert "city" in response_data["seller"]["address"]
    assert response_data["seller"]["address"]["city"] == "Springfield"
    assert "country" in response_data["seller"]["address"]
    assert "code" in response_data["seller"]["address"]["country"]
    assert response_data["seller"]["address"]["country"]["code"] == "US"
    assert "name" in response_data["seller"]["address"]["country"]
    assert response_data["seller"]["address"]["country"]["name"] == "USA"
    assert "street" in response_data["seller"]["address"]
    assert response_data["seller"]["address"]["street"] == "123 Main St"
    assert "name" in response_data["seller"]
    assert response_data["seller"]["name"] == "John Doe"


async def test_optional_fields__omitted(client: Any) -> None:
    """Tests object with optional fields omitted."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Foo", "price": 35.4}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "description" in response_data
    assert response_data["description"] == None
    assert "name" in response_data
    assert response_data["name"] == "Foo"
    assert "price" in response_data
    assert response_data["price"] == 35.4
    assert "tax" in response_data
    assert response_data["tax"] == None


async def test_uuid_field__success(client: Any) -> None:
    """Tests UUID field with valid UUID format."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"item_id": "c892496f-b1fd-4b91-bdb8-b46f92df1716", "name": "Item"}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "item_id" in response_data
    assert response_data["item_id"] == "c892496f-b1fd-4b91-bdb8-b46f92df1716"
    assert "name" in response_data
    assert response_data["name"] == "Item"


async def test_date_field__success(client: Any) -> None:
    """Tests date field with ISO date format."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"event_date": "2024-03-15", "name": "Conference"}
    response = await client.post("/events/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "event_date" in response_data
    assert response_data["event_date"] == "2024-03-15"
    assert "name" in response_data
    assert response_data["name"] == "Conference"


async def test_47_maxproperties_validation_failure(client: Any) -> None:
    """Object with more properties than maxProperties should fail."""
    json_data = {"debug": False, "host": "localhost", "port": 8080, "ssl": True}
    response = await client.post("/config", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "actual_properties" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["actual_properties"] == 4
    assert "max_properties" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["max_properties"] == 3
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 1
    assert response_data["errors"][0]["loc"][0] == "body"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Object must have at most 3 properties"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_46_minproperties_validation_failure(client: Any) -> None:
    """Object with fewer properties than minProperties should fail."""
    json_data = {"host": "localhost"}
    response = await client.post("/config", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "actual_properties" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["actual_properties"] == 1
    assert "min_properties" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["min_properties"] == 2
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 1
    assert response_data["errors"][0]["loc"][0] == "body"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Object must have at least 2 properties"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_string_min_length_validation__fail(client: Any) -> None:
    """Tests string field with min_length constraint failure."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "ab", "price": 35.4}
    response = await client.post("/items/validated", headers=headers, json=json_data)

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
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "name"
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


async def test_field_type_validation__invalid_type(client: Any) -> None:
    """Tests type validation error when field has wrong type."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"description": "A very nice Item", "name": "Foo", "price": "not a number", "tax": 3.2}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "not a number"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "price"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be a valid number"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "float_parsing"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_36_oneof_schema_multiple_match_failure(client: Any) -> None:
    """oneOf schema composition - fails when multiple schemas match."""
    json_data = {"credit_card": "1234567812345678", "paypal_email": "user@example.com"}
    response = await client.post("/payment", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "matched_schemas" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["matched_schemas"] == 2
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 1
    assert response_data["errors"][0]["loc"][0] == "body"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Must match exactly one schema (oneOf), but matched 2"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_nested_object__success(client: Any) -> None:
    """Tests nested JSON objects."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"image": {"name": "Product Image", "url": "https://example.com/image.jpg"}, "name": "Foo", "price": 42.0}
    response = await client.post("/items/nested", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "image" in response_data
    assert "name" in response_data["image"]
    assert response_data["image"]["name"] == "Product Image"
    assert "url" in response_data["image"]
    assert response_data["image"]["url"] == "https://example.com/image.jpg"
    assert "name" in response_data
    assert response_data["name"] == "Foo"
    assert "price" in response_data
    assert response_data["price"] == 42.0


async def test_41_not_schema_success(client: Any) -> None:
    """not schema - value must not match the schema."""
    json_data = {"username": "john_doe"}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 201


async def test_string_max_length_validation__fail(client: Any) -> None:
    """Tests string field with max_length constraint failure."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "This is a very long name that exceeds the maximum length", "price": 35.4}
    response = await client.post("/items/validated", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "max_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["max_length"] == 50
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "This is a very long name that exceeds the maximum length"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "name"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should have at most 50 characters"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_too_long"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_50_deep_nesting_4_levels(client: Any) -> None:
    """Deeply nested object with 4+ levels should validate correctly."""
    json_data = {"user": {"profile": {"contact": {"address": {"street": "123 Main St"}}}}}
    response = await client.post("/data", json=json_data)

    assert response.status_code == 201


async def test_48_dependencies_validation_success(client: Any) -> None:
    """Dependencies constraint - when A present, B is required and provided."""
    json_data = {"billing_address": "123 Main St", "credit_card": "1234567812345678", "name": "John Doe"}
    response = await client.post("/billing", json=json_data)

    assert response.status_code == 201


async def test_patch_partial_update(client: Any) -> None:
    """Tests PATCH request with partial object update."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"price": 45.0}
    response = await client.patch("/items/1", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "description" in response_data
    assert response_data["description"] == "Original description"
    assert "name" in response_data
    assert response_data["name"] == "Original Item"
    assert "price" in response_data
    assert response_data["price"] == 45.0


async def test_30_nested_object_missing_field(client: Any) -> None:
    """Nested object missing required field should fail validation."""
    json_data = {"profile": {"name": "John Doe"}}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "required" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["required"] == True
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 3
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "profile"
    assert response_data["errors"][0]["loc"][2] == "email"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Field required"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_datetime_field__success(client: Any) -> None:
    """Tests datetime field with ISO datetime format."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"created_at": "2024-03-15T10:30:00Z", "name": "Meeting"}
    response = await client.post("/events/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "created_at" in response_data
    assert response_data["created_at"] == "2024-03-15T10:30:00Z"
    assert "name" in response_data
    assert response_data["name"] == "Meeting"


async def test_string_pattern_validation__success(client: Any) -> None:
    """Tests string field with regex pattern constraint success."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item", "sku": "ABC1234"}
    response = await client.post("/items/validated", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "Item"
    assert "sku" in response_data
    assert response_data["sku"] == "ABC1234"


async def test_extra_fields_ignored_no_additionalproperties(client: Any) -> None:
    """Tests that extra fields not in model are ignored."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"another_extra": 123, "extra_field": "this should be ignored", "name": "Item", "price": 42.0}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "Item"
    assert "price" in response_data
    assert response_data["price"] == 42.0


async def test_40_anyof_schema_failure(client: Any) -> None:
    """anyOf schema composition - fails when no schemas match."""
    json_data = {"name": "John Doe"}
    response = await client.post("/contact", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "matched_schemas" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["matched_schemas"] == 0
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 1
    assert response_data["errors"][0]["loc"][0] == "body"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Must match at least one schema (anyOf), but matched 0"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_39_anyof_schema_multiple_match_success(client: Any) -> None:
    """anyOf schema composition - succeeds when multiple schemas match."""
    json_data = {"email": "john@example.com", "name": "John Doe", "phone": "+1-555-0100"}
    response = await client.post("/contact", json=json_data)

    assert response.status_code == 201


async def test_array_of_primitive_values(client: Any) -> None:
    """Tests array field containing primitive values (strings, numbers)."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Product", "ratings": [4.5, 4.8, 5.0, 4.2], "tags": ["electronics", "gadget", "new"]}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "Product"
    assert "ratings" in response_data
    assert len(response_data["ratings"]) == 4
    assert response_data["ratings"][0] == 4.5
    assert response_data["ratings"][1] == 4.8
    assert response_data["ratings"][2] == 5.0
    assert response_data["ratings"][3] == 4.2
    assert "tags" in response_data
    assert len(response_data["tags"]) == 3
    assert response_data["tags"][0] == "electronics"
    assert response_data["tags"][1] == "gadget"
    assert response_data["tags"][2] == "new"


async def test_numeric_ge_validation__fail(client: Any) -> None:
    """Tests numeric field with ge (greater than or equal) constraint failure."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item", "price": 0.5}
    response = await client.post("/items/validated", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "ge" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["ge"] == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == 0.5
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "price"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be greater than or equal to 1"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "greater_than_equal"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_37_oneof_schema_no_match_failure(client: Any) -> None:
    """oneOf schema composition - fails when no schemas match."""
    json_data = {"bitcoin_address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"}
    response = await client.post("/payment", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "matched_schemas" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["matched_schemas"] == 0
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 1
    assert response_data["errors"][0]["loc"][0] == "body"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Must match exactly one schema (oneOf), but matched 0"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_empty_array_validation__fail(client: Any) -> None:
    """Tests array field with min_items constraint failure."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Product", "tags": []}
    response = await client.post("/items/list-validated", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "min_length" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["min_length"] == 1
    assert "input" in response_data["errors"][0]
    assert len(response_data["errors"][0]["input"]) == 0
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "tags"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "List should have at least 1 item after validation"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "too_short"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_38_anyof_schema_success(client: Any) -> None:
    """anyOf schema composition - at least one schema must match."""
    json_data = {"email": "john@example.com", "name": "John Doe"}
    response = await client.post("/contact", json=json_data)

    assert response.status_code == 201


async def test_empty_json_object(client: Any) -> None:
    """Tests empty JSON object when all fields are optional."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {}
    response = await client.post("/items/optional-all", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "description" in response_data
    assert response_data["description"] == None
    assert "name" in response_data
    assert response_data["name"] == None
    assert "price" in response_data
    assert response_data["price"] == None
    assert "tax" in response_data
    assert response_data["tax"] == None


async def test_string_pattern_validation__fail(client: Any) -> None:
    """Tests string field with regex pattern constraint failure."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item", "sku": "ABC-123"}
    response = await client.post("/items/validated", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "pattern" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["pattern"] == "^[A-Z]{3}[0-9]{4}$"
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "ABC-123"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "sku"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "String should match pattern '^[A-Z]{3}[0-9]{4}$'"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "string_pattern_mismatch"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_49_dependencies_validation_failure(client: Any) -> None:
    """Dependencies constraint - when A present, B is required but missing."""
    json_data = {"credit_card": "1234567812345678", "name": "John Doe"}
    response = await client.post("/billing", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "dependency" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["dependency"] == "credit_card"
    assert "required_fields" in response_data["errors"][0]["ctx"]
    assert len(response_data["errors"][0]["ctx"]["required_fields"]) == 1
    assert response_data["errors"][0]["ctx"]["required_fields"][0] == "billing_address"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 1
    assert response_data["errors"][0]["loc"][0] == "body"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "When 'credit_card' is present, 'billing_address' is required"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_simple_json_object__success(client: Any) -> None:
    """Tests simple JSON object with all required fields."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"description": "A very nice Item", "name": "Foo", "price": 35.4, "tax": 3.2}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "description" in response_data
    assert response_data["description"] == "A very nice Item"
    assert "name" in response_data
    assert response_data["name"] == "Foo"
    assert "price" in response_data
    assert response_data["price"] == 35.4
    assert "tax" in response_data
    assert response_data["tax"] == 3.2


async def test_required_field_missing__validation_error(client: Any) -> None:
    """Tests validation error when required field is missing."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"description": "A very nice Item", "price": 35.4}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == ""
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "name"
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


async def test_35_oneof_schema_success(client: Any) -> None:
    """oneOf schema composition - exactly one schema must match."""
    json_data = {"credit_card": "1234567812345678"}
    response = await client.post("/payment", json=json_data)

    assert response.status_code == 201


async def test_enum_field__invalid_value(client: Any) -> None:
    """Tests enum field with value not in enum."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"category": "furniture", "name": "Item"}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "expected" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["expected"] == "'electronics', 'clothing' or 'books'"
    assert "input" in response_data["errors"][0]
    assert response_data["errors"][0]["input"] == "furniture"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "category"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Input should be 'electronics', 'clothing' or 'books'"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "enum"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_enum_field__success(client: Any) -> None:
    """Tests enum field with valid enum value."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"category": "electronics", "name": "Item"}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "category" in response_data
    assert response_data["category"] == "electronics"
    assert "name" in response_data
    assert response_data["name"] == "Item"


async def test_33_allof_schema_composition(client: Any) -> None:
    """JSON Schema allOf composition should validate all schemas."""
    json_data = {"name": "Product", "price": 29.99}
    response = await client.post("/items", json=json_data)

    assert response.status_code == 201


async def test_45_minproperties_validation_success(client: Any) -> None:
    """Object with properties meeting minProperties constraint should succeed."""
    json_data = {"host": "localhost", "port": 8080}
    response = await client.post("/config", json=json_data)

    assert response.status_code == 201


async def test_body_with_query_parameters(client: Any) -> None:
    """Tests JSON body combined with query parameters."""
    params = {
        "limit": 10,
    }
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item", "price": 42.0}
    response = await client.post("/items/?limit=10", query_params=params, headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "item" in response_data
    assert "name" in response_data["item"]
    assert response_data["item"]["name"] == "Item"
    assert "price" in response_data["item"]
    assert response_data["item"]["price"] == 42.0
    assert "limit" in response_data
    assert response_data["limit"] == 10


async def test_42_not_schema_failure(client: Any) -> None:
    """not schema - fails when value matches the prohibited schema."""
    json_data = {"username": "admin"}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "prohibited_value" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["prohibited_value"] == "admin"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "username"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Must not match the schema"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_43_const_validation_success(client: Any) -> None:
    """Field with const constraint matching exact value should succeed."""
    json_data = {"data": "test", "version": "1.0"}
    response = await client.post("/api/v1/data", json=json_data)

    assert response.status_code == 201


async def test_32_schema_ref_definitions(client: Any) -> None:
    """JSON Schema $ref with definitions should validate correctly."""
    json_data = {"product": {"name": "Widget", "price": 9.99}}
    response = await client.post("/products", json=json_data)

    assert response.status_code == 201


async def test_29_nested_object_validation_success(client: Any) -> None:
    """Nested object in JSON body should validate correctly."""
    json_data = {"profile": {"email": "john@example.com", "name": "John Doe"}}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 201


async def test_34_additional_properties_false(client: Any) -> None:
    """Schema with additionalProperties false should reject extra fields."""
    json_data = {"email": "john@example.com", "extra_field": "should fail", "name": "John"}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    assert "detail" in response_data
    assert response_data["detail"] == "1 validation error in request"
    assert "errors" in response_data
    assert len(response_data["errors"]) == 1
    assert "ctx" in response_data["errors"][0]
    assert "additional_properties" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["additional_properties"] == False
    assert "unexpected_field" in response_data["errors"][0]["ctx"]
    assert response_data["errors"][0]["ctx"]["unexpected_field"] == "extra_field"
    assert "loc" in response_data["errors"][0]
    assert len(response_data["errors"][0]["loc"]) == 2
    assert response_data["errors"][0]["loc"][0] == "body"
    assert response_data["errors"][0]["loc"][1] == "extra_field"
    assert "msg" in response_data["errors"][0]
    assert response_data["errors"][0]["msg"] == "Additional properties are not allowed"
    assert "type" in response_data["errors"][0]
    assert response_data["errors"][0]["type"] == "validation_error"
    assert "status" in response_data
    assert response_data["status"] == 422
    assert "title" in response_data
    assert response_data["title"] == "Request Validation Failed"
    assert "type" in response_data
    assert response_data["type"] == "https://spikard.dev/errors/validation-error"


async def test_null_value_for_optional_field(client: Any) -> None:
    """Tests explicitly setting optional field to null."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"description": None, "name": "Item", "price": 42.0, "tax": None}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "description" in response_data
    assert response_data["description"] == None
    assert "name" in response_data
    assert response_data["name"] == "Item"
    assert "price" in response_data
    assert response_data["price"] == 42.0
    assert "tax" in response_data
    assert response_data["tax"] == None


async def test_31_nullable_property_null_value(client: Any) -> None:
    """Nullable property with null value should be accepted."""
    json_data = {"description": None, "name": "Test User"}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 201


async def test_array_of_objects__success(client: Any) -> None:
    """Tests array field containing objects."""
    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"images": [{"name": "Front", "url": "https://example.com/img1.jpg"}, {"name": "Back", "url": "https://example.com/img2.jpg"}], "name": "Product Bundle", "tags": ["electronics", "gadget"]}
    response = await client.post("/items/list", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "images" in response_data
    assert len(response_data["images"]) == 2
    assert "name" in response_data["images"][0]
    assert response_data["images"][0]["name"] == "Front"
    assert "url" in response_data["images"][0]
    assert response_data["images"][0]["url"] == "https://example.com/img1.jpg"
    assert "name" in response_data["images"][1]
    assert response_data["images"][1]["name"] == "Back"
    assert "url" in response_data["images"][1]
    assert response_data["images"][1]["url"] == "https://example.com/img2.jpg"
    assert "name" in response_data
    assert response_data["name"] == "Product Bundle"
    assert "tags" in response_data
    assert len(response_data["tags"]) == 2
    assert response_data["tags"][0] == "electronics"
    assert response_data["tags"][1] == "gadget"


