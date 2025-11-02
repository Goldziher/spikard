"""E2E tests for json_bodies."""

import pytest
from typing import Any

async def test_uuid_field_invalid_format() -> None:
    """Tests UUID field with invalid UUID format."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_uuid_field_invalid_format

    app = create_app_json_bodies_uuid_field_invalid_format()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"item_id": "not-a-valid-uuid", "name": "Item"}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_44_const_validation_failure() -> None:
    """Field with const constraint not matching exact value should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_44_const_validation_failure

    app = create_app_json_bodies_44_const_validation_failure()
    client = TestClient(app)

    json_data = {"data": "test", "version": "2.0"}
    response = await client.post("/api/v1/data", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_boolean_field_success() -> None:
    """Tests JSON object with boolean field."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_boolean_field_success

    app = create_app_json_bodies_boolean_field_success()
    client = TestClient(app)

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


async def test_numeric_le_validation_success() -> None:
    """Tests numeric field with le (less than or equal) constraint at boundary."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_numeric_le_validation_success

    app = create_app_json_bodies_numeric_le_validation_success()
    client = TestClient(app)

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


async def test_deeply_nested_objects() -> None:
    """Tests deeply nested JSON structure (3+ levels)."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_deeply_nested_objects

    app = create_app_json_bodies_deeply_nested_objects()
    client = TestClient(app)

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


async def test_optional_fields_omitted() -> None:
    """Tests object with optional fields omitted."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_optional_fields_omitted

    app = create_app_json_bodies_optional_fields_omitted()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Foo", "price": 35.4}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "name" in response_data
    assert response_data["name"] == "Foo"
    assert "price" in response_data
    assert response_data["price"] == 35.4


async def test_uuid_field_success() -> None:
    """Tests UUID field with valid UUID format."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_uuid_field_success

    app = create_app_json_bodies_uuid_field_success()
    client = TestClient(app)

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


async def test_date_field_success() -> None:
    """Tests date field with ISO date format."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_date_field_success

    app = create_app_json_bodies_date_field_success()
    client = TestClient(app)

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


async def test_47_maxproperties_validation_failure() -> None:
    """Object with more properties than maxProperties should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_47_maxproperties_validation_failure

    app = create_app_json_bodies_47_maxproperties_validation_failure()
    client = TestClient(app)

    json_data = {"debug": False, "host": "localhost", "port": 8080, "ssl": True}
    response = await client.post("/config", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_46_minproperties_validation_failure() -> None:
    """Object with fewer properties than minProperties should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_46_minproperties_validation_failure

    app = create_app_json_bodies_46_minproperties_validation_failure()
    client = TestClient(app)

    json_data = {"host": "localhost"}
    response = await client.post("/config", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_string_min_length_validation_fail() -> None:
    """Tests string field with min_length constraint failure."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_string_min_length_validation_fail

    app = create_app_json_bodies_string_min_length_validation_fail()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "ab", "price": 35.4}
    response = await client.post("/items/validated", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_field_type_validation_invalid_type() -> None:
    """Tests type validation error when field has wrong type."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_field_type_validation_invalid_type

    app = create_app_json_bodies_field_type_validation_invalid_type()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"description": "A very nice Item", "name": "Foo", "price": "not a number", "tax": 3.2}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_36_oneof_schema_multiple_match_failure() -> None:
    """oneOf schema composition - fails when multiple schemas match."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_36_oneof_schema_multiple_match_failure

    app = create_app_json_bodies_36_oneof_schema_multiple_match_failure()
    client = TestClient(app)

    json_data = {"credit_card": "1234567812345678", "paypal_email": "user@example.com"}
    response = await client.post("/payment", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_nested_object_success() -> None:
    """Tests nested JSON objects."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_nested_object_success

    app = create_app_json_bodies_nested_object_success()
    client = TestClient(app)

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


async def test_41_not_schema_success() -> None:
    """not schema - value must not match the schema."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_41_not_schema_success

    app = create_app_json_bodies_41_not_schema_success()
    client = TestClient(app)

    json_data = {"username": "john_doe"}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 201


async def test_string_max_length_validation_fail() -> None:
    """Tests string field with max_length constraint failure."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_string_max_length_validation_fail

    app = create_app_json_bodies_string_max_length_validation_fail()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "This is a very long name that exceeds the maximum length", "price": 35.4}
    response = await client.post("/items/validated", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_50_deep_nesting_4_levels() -> None:
    """Deeply nested object with 4+ levels should validate correctly."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_50_deep_nesting_4_levels

    app = create_app_json_bodies_50_deep_nesting_4_levels()
    client = TestClient(app)

    json_data = {"user": {"profile": {"contact": {"address": {"street": "123 Main St"}}}}}
    response = await client.post("/data", json=json_data)

    assert response.status_code == 201


async def test_48_dependencies_validation_success() -> None:
    """Dependencies constraint - when A present, B is required and provided."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_48_dependencies_validation_success

    app = create_app_json_bodies_48_dependencies_validation_success()
    client = TestClient(app)

    json_data = {"billing_address": "123 Main St", "credit_card": "1234567812345678", "name": "John Doe"}
    response = await client.post("/billing", json=json_data)

    assert response.status_code == 201


async def test_patch_partial_update() -> None:
    """Tests PATCH request with partial object update."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_patch_partial_update

    app = create_app_json_bodies_patch_partial_update()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"price": 45.0}
    response = await client.patch("/items/1", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "price" in response_data
    assert response_data["price"] == 45.0


async def test_30_nested_object_missing_field() -> None:
    """Nested object missing required field should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_30_nested_object_missing_field

    app = create_app_json_bodies_30_nested_object_missing_field()
    client = TestClient(app)

    json_data = {"profile": {"name": "John Doe"}}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_datetime_field_success() -> None:
    """Tests datetime field with ISO datetime format."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_datetime_field_success

    app = create_app_json_bodies_datetime_field_success()
    client = TestClient(app)

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


async def test_string_pattern_validation_success() -> None:
    """Tests string field with regex pattern constraint success."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_string_pattern_validation_success

    app = create_app_json_bodies_string_pattern_validation_success()
    client = TestClient(app)

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


async def test_extra_fields_ignored_no_additionalproperties() -> None:
    """Tests that extra fields not in model are ignored."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_extra_fields_ignored_no_additionalproperties

    app = create_app_json_bodies_extra_fields_ignored_no_additionalproperties()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"another_extra": 123, "extra_field": "this should be ignored", "name": "Item", "price": 42.0}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()
    assert "another_extra" in response_data
    assert response_data["another_extra"] == 123
    assert "extra_field" in response_data
    assert response_data["extra_field"] == "this should be ignored"
    assert "name" in response_data
    assert response_data["name"] == "Item"
    assert "price" in response_data
    assert response_data["price"] == 42.0


async def test_40_anyof_schema_failure() -> None:
    """anyOf schema composition - fails when no schemas match."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_40_anyof_schema_failure

    app = create_app_json_bodies_40_anyof_schema_failure()
    client = TestClient(app)

    json_data = {"name": "John Doe"}
    response = await client.post("/contact", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_39_anyof_schema_multiple_match_success() -> None:
    """anyOf schema composition - succeeds when multiple schemas match."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_39_anyof_schema_multiple_match_success

    app = create_app_json_bodies_39_anyof_schema_multiple_match_success()
    client = TestClient(app)

    json_data = {"email": "john@example.com", "name": "John Doe", "phone": "+1-555-0100"}
    response = await client.post("/contact", json=json_data)

    assert response.status_code == 201


async def test_array_of_primitive_values() -> None:
    """Tests array field containing primitive values (strings, numbers)."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_array_of_primitive_values

    app = create_app_json_bodies_array_of_primitive_values()
    client = TestClient(app)

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


async def test_numeric_ge_validation_fail() -> None:
    """Tests numeric field with ge (greater than or equal) constraint failure."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_numeric_ge_validation_fail

    app = create_app_json_bodies_numeric_ge_validation_fail()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item", "price": 0.5}
    response = await client.post("/items/validated", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_37_oneof_schema_no_match_failure() -> None:
    """oneOf schema composition - fails when no schemas match."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_37_oneof_schema_no_match_failure

    app = create_app_json_bodies_37_oneof_schema_no_match_failure()
    client = TestClient(app)

    json_data = {"bitcoin_address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"}
    response = await client.post("/payment", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_empty_array_validation_fail() -> None:
    """Tests array field with min_items constraint failure."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_empty_array_validation_fail

    app = create_app_json_bodies_empty_array_validation_fail()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Product", "tags": []}
    response = await client.post("/items/list-validated", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_38_anyof_schema_success() -> None:
    """anyOf schema composition - at least one schema must match."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_38_anyof_schema_success

    app = create_app_json_bodies_38_anyof_schema_success()
    client = TestClient(app)

    json_data = {"email": "john@example.com", "name": "John Doe"}
    response = await client.post("/contact", json=json_data)

    assert response.status_code == 201


async def test_empty_json_object() -> None:
    """Tests empty JSON object when all fields are optional."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_empty_json_object

    app = create_app_json_bodies_empty_json_object()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {}
    response = await client.post("/items/optional-all", headers=headers, json=json_data)

    assert response.status_code == 200
    response_data = response.json()


async def test_string_pattern_validation_fail() -> None:
    """Tests string field with regex pattern constraint failure."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_string_pattern_validation_fail

    app = create_app_json_bodies_string_pattern_validation_fail()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"name": "Item", "sku": "ABC-123"}
    response = await client.post("/items/validated", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_49_dependencies_validation_failure() -> None:
    """Dependencies constraint - when A present, B is required but missing."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_49_dependencies_validation_failure

    app = create_app_json_bodies_49_dependencies_validation_failure()
    client = TestClient(app)

    json_data = {"credit_card": "1234567812345678", "name": "John Doe"}
    response = await client.post("/billing", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_simple_json_object_success() -> None:
    """Tests simple JSON object with all required fields."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_simple_json_object_success

    app = create_app_json_bodies_simple_json_object_success()
    client = TestClient(app)

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


async def test_required_field_missing_validation_error() -> None:
    """Tests validation error when required field is missing."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_required_field_missing_validation_error

    app = create_app_json_bodies_required_field_missing_validation_error()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"description": "A very nice Item", "price": 35.4}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_35_oneof_schema_success() -> None:
    """oneOf schema composition - exactly one schema must match."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_35_oneof_schema_success

    app = create_app_json_bodies_35_oneof_schema_success()
    client = TestClient(app)

    json_data = {"credit_card": "1234567812345678"}
    response = await client.post("/payment", json=json_data)

    assert response.status_code == 201


async def test_enum_field_invalid_value() -> None:
    """Tests enum field with value not in enum."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_enum_field_invalid_value

    app = create_app_json_bodies_enum_field_invalid_value()
    client = TestClient(app)

    headers = {
        "Content-Type": "application/json",
    }
    json_data = {"category": "furniture", "name": "Item"}
    response = await client.post("/items/", headers=headers, json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_enum_field_success() -> None:
    """Tests enum field with valid enum value."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_enum_field_success

    app = create_app_json_bodies_enum_field_success()
    client = TestClient(app)

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


async def test_33_allof_schema_composition() -> None:
    """JSON Schema allOf composition should validate all schemas."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_33_allof_schema_composition

    app = create_app_json_bodies_33_allof_schema_composition()
    client = TestClient(app)

    json_data = {"name": "Product", "price": 29.99}
    response = await client.post("/items", json=json_data)

    assert response.status_code == 201


async def test_45_minproperties_validation_success() -> None:
    """Object with properties meeting minProperties constraint should succeed."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_45_minproperties_validation_success

    app = create_app_json_bodies_45_minproperties_validation_success()
    client = TestClient(app)

    json_data = {"host": "localhost", "port": 8080}
    response = await client.post("/config", json=json_data)

    assert response.status_code == 201


async def test_body_with_query_parameters() -> None:
    """Tests JSON body combined with query parameters."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_body_with_query_parameters

    app = create_app_json_bodies_body_with_query_parameters()
    client = TestClient(app)

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
    assert "name" in response_data
    assert response_data["name"] == "Item"
    assert "price" in response_data
    assert response_data["price"] == 42.0
    assert response_data["limit"] == 10


async def test_42_not_schema_failure() -> None:
    """not schema - fails when value matches the prohibited schema."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_42_not_schema_failure

    app = create_app_json_bodies_42_not_schema_failure()
    client = TestClient(app)

    json_data = {"username": "admin"}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_43_const_validation_success() -> None:
    """Field with const constraint matching exact value should succeed."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_43_const_validation_success

    app = create_app_json_bodies_43_const_validation_success()
    client = TestClient(app)

    json_data = {"data": "test", "version": "1.0"}
    response = await client.post("/api/v1/data", json=json_data)

    assert response.status_code == 201


async def test_32_schema_ref_definitions() -> None:
    """JSON Schema $ref with definitions should validate correctly."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_32_schema_ref_definitions

    app = create_app_json_bodies_32_schema_ref_definitions()
    client = TestClient(app)

    json_data = {"product": {"name": "Widget", "price": 9.99}}
    response = await client.post("/products", json=json_data)

    assert response.status_code == 201


async def test_29_nested_object_validation_success() -> None:
    """Nested object in JSON body should validate correctly."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_29_nested_object_validation_success

    app = create_app_json_bodies_29_nested_object_validation_success()
    client = TestClient(app)

    json_data = {"profile": {"email": "john@example.com", "name": "John Doe"}}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 201


async def test_34_additional_properties_false() -> None:
    """Schema with additionalProperties false should reject extra fields."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_34_additional_properties_false

    app = create_app_json_bodies_34_additional_properties_false()
    client = TestClient(app)

    json_data = {"email": "john@example.com", "extra_field": "should fail", "name": "John"}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_null_value_for_optional_field() -> None:
    """Tests explicitly setting optional field to null."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_null_value_for_optional_field

    app = create_app_json_bodies_null_value_for_optional_field()
    client = TestClient(app)

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


async def test_31_nullable_property_null_value() -> None:
    """Nullable property with null value should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_31_nullable_property_null_value

    app = create_app_json_bodies_31_nullable_property_null_value()
    client = TestClient(app)

    json_data = {"description": None, "name": "Test User"}
    response = await client.post("/users", json=json_data)

    assert response.status_code == 201


async def test_array_of_objects_success() -> None:
    """Tests array field containing objects."""
    from spikard.testing import TestClient
    from app.main import create_app_json_bodies_array_of_objects_success

    app = create_app_json_bodies_array_of_objects_success()
    client = TestClient(app)

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


