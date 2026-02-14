"""E2E tests for json_bodies."""

from spikard.testing import TestClient
from app.main import (
    create_app_json_bodies_29_nested_object_validation_success,
    create_app_json_bodies_30_nested_object_missing_field,
    create_app_json_bodies_31_nullable_property_null_value,
    create_app_json_bodies_32_schema_ref_definitions,
    create_app_json_bodies_33_allof_schema_composition,
    create_app_json_bodies_34_additional_properties_false,
    create_app_json_bodies_35_oneof_schema_success,
    create_app_json_bodies_36_oneof_schema_multiple_match_failure,
    create_app_json_bodies_37_oneof_schema_no_match_failure,
    create_app_json_bodies_38_anyof_schema_success,
    create_app_json_bodies_39_anyof_schema_multiple_match_success,
    create_app_json_bodies_40_anyof_schema_failure,
    create_app_json_bodies_41_not_schema_success,
    create_app_json_bodies_42_not_schema_failure,
    create_app_json_bodies_43_const_validation_success,
    create_app_json_bodies_44_const_validation_failure,
    create_app_json_bodies_45_minproperties_validation_success,
    create_app_json_bodies_46_minproperties_validation_failure,
    create_app_json_bodies_47_maxproperties_validation_failure,
    create_app_json_bodies_48_dependencies_validation_success,
    create_app_json_bodies_49_dependencies_validation_failure,
    create_app_json_bodies_50_deep_nesting_4_levels,
    create_app_json_bodies_array_of_objects_success,
    create_app_json_bodies_array_of_primitive_values,
    create_app_json_bodies_body_with_query_parameters,
    create_app_json_bodies_boolean_field_success,
    create_app_json_bodies_date_field_success,
    create_app_json_bodies_datetime_field_success,
    create_app_json_bodies_deeply_nested_objects,
    create_app_json_bodies_empty_array_validation_fail,
    create_app_json_bodies_empty_json_object,
    create_app_json_bodies_enum_field_invalid_value,
    create_app_json_bodies_enum_field_success,
    create_app_json_bodies_extra_fields_ignored_no_additionalproperties,
    create_app_json_bodies_field_type_validation_invalid_type,
    create_app_json_bodies_nested_object_success,
    create_app_json_bodies_null_value_for_optional_field,
    create_app_json_bodies_numeric_ge_validation_fail,
    create_app_json_bodies_numeric_le_validation_success,
    create_app_json_bodies_optional_fields_omitted,
    create_app_json_bodies_patch_partial_update,
    create_app_json_bodies_required_field_missing_validation_error,
    create_app_json_bodies_simple_json_object_success,
    create_app_json_bodies_string_max_length_validation_fail,
    create_app_json_bodies_string_min_length_validation_fail,
    create_app_json_bodies_string_pattern_validation_fail,
    create_app_json_bodies_string_pattern_validation_success,
    create_app_json_bodies_uuid_field_invalid_format,
    create_app_json_bodies_uuid_field_success,
)


async def test_uuid_field_invalid_format() -> None:
    """Tests UUID field with invalid UUID format."""

    async with TestClient(create_app_json_bodies_uuid_field_invalid_format()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"name": "Item", "item_id": "not-a-valid-uuid"}
        response = await client.post("/items/", headers=headers, json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_44_const_validation_failure() -> None:
    """Field with const constraint not matching exact value should fail."""

    async with TestClient(create_app_json_bodies_44_const_validation_failure()) as client:
        json_data = {"version": "2.0", "data": "test"}
        response = await client.post("/api/v1/data", json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_boolean_field_success() -> None:
    """Tests JSON object with boolean field."""

    async with TestClient(create_app_json_bodies_boolean_field_success()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"name": "Item", "price": 42.0, "in_stock": True}
        response = await client.post("/items/", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "Item"
        assert "price" in response_data
        assert response_data["price"] == 42.0
        assert "in_stock" in response_data
        assert response_data["in_stock"] == True


async def test_numeric_le_validation_success() -> None:
    """Tests numeric field with le (less than or equal) constraint at boundary."""

    async with TestClient(create_app_json_bodies_numeric_le_validation_success()) as client:
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

    async with TestClient(create_app_json_bodies_deeply_nested_objects()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {
            "name": "Product",
            "price": 100.0,
            "seller": {
                "name": "John Doe",
                "address": {"street": "123 Main St", "city": "Springfield", "country": {"name": "USA", "code": "US"}},
            },
        }
        response = await client.post("/items/nested", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "Product"
        assert "price" in response_data
        assert response_data["price"] == 100.0
        assert "seller" in response_data
        assert "name" in response_data["seller"]
        assert response_data["seller"]["name"] == "John Doe"
        assert "address" in response_data["seller"]
        assert "street" in response_data["seller"]["address"]
        assert response_data["seller"]["address"]["street"] == "123 Main St"
        assert "city" in response_data["seller"]["address"]
        assert response_data["seller"]["address"]["city"] == "Springfield"
        assert "country" in response_data["seller"]["address"]
        assert "name" in response_data["seller"]["address"]["country"]
        assert response_data["seller"]["address"]["country"]["name"] == "USA"
        assert "code" in response_data["seller"]["address"]["country"]
        assert response_data["seller"]["address"]["country"]["code"] == "US"


async def test_optional_fields_omitted() -> None:
    """Tests object with optional fields omitted."""

    async with TestClient(create_app_json_bodies_optional_fields_omitted()) as client:
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
        assert "description" in response_data
        assert response_data["description"] == None
        assert "tax" in response_data
        assert response_data["tax"] == None


async def test_uuid_field_success() -> None:
    """Tests UUID field with valid UUID format."""

    async with TestClient(create_app_json_bodies_uuid_field_success()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"name": "Item", "item_id": "c892496f-b1fd-4b91-bdb8-b46f92df1716"}
        response = await client.post("/items/", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "Item"
        assert "item_id" in response_data
        assert response_data["item_id"] == "c892496f-b1fd-4b91-bdb8-b46f92df1716"


async def test_date_field_success() -> None:
    """Tests date field with ISO date format."""

    async with TestClient(create_app_json_bodies_date_field_success()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"name": "Conference", "event_date": "2024-03-15"}
        response = await client.post("/events/", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "Conference"
        assert "event_date" in response_data
        assert response_data["event_date"] == "2024-03-15"


async def test_47_maxproperties_validation_failure() -> None:
    """Object with more properties than maxProperties should fail."""

    async with TestClient(create_app_json_bodies_47_maxproperties_validation_failure()) as client:
        json_data = {"host": "localhost", "port": 8080, "ssl": True, "debug": False}
        response = await client.post("/config", json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_46_minproperties_validation_failure() -> None:
    """Object with fewer properties than minProperties should fail."""

    async with TestClient(create_app_json_bodies_46_minproperties_validation_failure()) as client:
        json_data = {"host": "localhost"}
        response = await client.post("/config", json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_string_min_length_validation_fail() -> None:
    """Tests string field with min_length constraint failure."""

    async with TestClient(create_app_json_bodies_string_min_length_validation_fail()) as client:
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

    async with TestClient(create_app_json_bodies_field_type_validation_invalid_type()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"name": "Foo", "description": "A very nice Item", "price": "not a number", "tax": 3.2}
        response = await client.post("/items/", headers=headers, json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_36_oneof_schema_multiple_match_failure() -> None:
    """oneOf schema composition - fails when multiple schemas match."""

    async with TestClient(create_app_json_bodies_36_oneof_schema_multiple_match_failure()) as client:
        json_data = {"credit_card": "1234567812345678", "paypal_email": "user@example.com"}
        response = await client.post("/payment", json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_nested_object_success() -> None:
    """Tests nested JSON objects."""

    async with TestClient(create_app_json_bodies_nested_object_success()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {
            "name": "Foo",
            "price": 42.0,
            "image": {"url": "https://example.com/image.jpg", "name": "Product Image"},
        }
        response = await client.post("/items/nested", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "Foo"
        assert "price" in response_data
        assert response_data["price"] == 42.0
        assert "image" in response_data
        assert "url" in response_data["image"]
        assert response_data["image"]["url"] == "https://example.com/image.jpg"
        assert "name" in response_data["image"]
        assert response_data["image"]["name"] == "Product Image"


async def test_41_not_schema_success() -> None:
    """not schema - value must not match the schema."""

    async with TestClient(create_app_json_bodies_41_not_schema_success()) as client:
        json_data = {"username": "john_doe"}
        response = await client.post("/users", json=json_data)

        assert response.status_code == 201


async def test_string_max_length_validation_fail() -> None:
    """Tests string field with max_length constraint failure."""

    async with TestClient(create_app_json_bodies_string_max_length_validation_fail()) as client:
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

    async with TestClient(create_app_json_bodies_50_deep_nesting_4_levels()) as client:
        json_data = {"user": {"profile": {"contact": {"address": {"street": "123 Main St"}}}}}
        response = await client.post("/data", json=json_data)

        assert response.status_code == 201


async def test_48_dependencies_validation_success() -> None:
    """Dependencies constraint - when A present, B is required and provided."""

    async with TestClient(create_app_json_bodies_48_dependencies_validation_success()) as client:
        json_data = {"name": "John Doe", "credit_card": "1234567812345678", "billing_address": "123 Main St"}
        response = await client.post("/billing", json=json_data)

        assert response.status_code == 201


async def test_patch_partial_update() -> None:
    """Tests PATCH request with partial object update."""

    async with TestClient(create_app_json_bodies_patch_partial_update()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"price": 45.0}
        response = await client.patch("/items/1", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "Original Item"
        assert "price" in response_data
        assert response_data["price"] == 45.0
        assert "description" in response_data
        assert response_data["description"] == "Original description"


async def test_30_nested_object_missing_field() -> None:
    """Nested object missing required field should fail validation."""

    async with TestClient(create_app_json_bodies_30_nested_object_missing_field()) as client:
        json_data = {"profile": {"name": "John Doe"}}
        response = await client.post("/users", json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_datetime_field_success() -> None:
    """Tests datetime field with ISO datetime format."""

    async with TestClient(create_app_json_bodies_datetime_field_success()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"name": "Meeting", "created_at": "2024-03-15T10:30:00Z"}
        response = await client.post("/events/", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "Meeting"
        assert "created_at" in response_data
        assert response_data["created_at"] == "2024-03-15T10:30:00Z"


async def test_string_pattern_validation_success() -> None:
    """Tests string field with regex pattern constraint success."""

    async with TestClient(create_app_json_bodies_string_pattern_validation_success()) as client:
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

    async with TestClient(create_app_json_bodies_extra_fields_ignored_no_additionalproperties()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"name": "Item", "price": 42.0, "extra_field": "this should be ignored", "another_extra": 123}
        response = await client.post("/items/", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "Item"
        assert "price" in response_data
        assert response_data["price"] == 42.0


async def test_40_anyof_schema_failure() -> None:
    """anyOf schema composition - fails when no schemas match."""

    async with TestClient(create_app_json_bodies_40_anyof_schema_failure()) as client:
        json_data = {"name": "John Doe"}
        response = await client.post("/contact", json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_39_anyof_schema_multiple_match_success() -> None:
    """anyOf schema composition - succeeds when multiple schemas match."""

    async with TestClient(create_app_json_bodies_39_anyof_schema_multiple_match_success()) as client:
        json_data = {"name": "John Doe", "email": "john@example.com", "phone": "+1-555-0100"}
        response = await client.post("/contact", json=json_data)

        assert response.status_code == 201


async def test_array_of_primitive_values() -> None:
    """Tests array field containing primitive values (strings, numbers)."""

    async with TestClient(create_app_json_bodies_array_of_primitive_values()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"name": "Product", "tags": ["electronics", "gadget", "new"], "ratings": [4.5, 4.8, 5.0, 4.2]}
        response = await client.post("/items/", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "Product"
        assert "tags" in response_data
        assert len(response_data["tags"]) == 3
        assert response_data["tags"][0] == "electronics"
        assert response_data["tags"][1] == "gadget"
        assert response_data["tags"][2] == "new"
        assert "ratings" in response_data
        assert len(response_data["ratings"]) == 4
        assert response_data["ratings"][0] == 4.5
        assert response_data["ratings"][1] == 4.8
        assert response_data["ratings"][2] == 5.0
        assert response_data["ratings"][3] == 4.2


async def test_numeric_ge_validation_fail() -> None:
    """Tests numeric field with ge (greater than or equal) constraint failure."""

    async with TestClient(create_app_json_bodies_numeric_ge_validation_fail()) as client:
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

    async with TestClient(create_app_json_bodies_37_oneof_schema_no_match_failure()) as client:
        json_data = {"bitcoin_address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"}
        response = await client.post("/payment", json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_empty_array_validation_fail() -> None:
    """Tests array field with min_items constraint failure."""

    async with TestClient(create_app_json_bodies_empty_array_validation_fail()) as client:
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

    async with TestClient(create_app_json_bodies_38_anyof_schema_success()) as client:
        json_data = {"name": "John Doe", "email": "john@example.com"}
        response = await client.post("/contact", json=json_data)

        assert response.status_code == 201


async def test_empty_json_object() -> None:
    """Tests empty JSON object when all fields are optional."""

    async with TestClient(create_app_json_bodies_empty_json_object()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {}
        response = await client.post("/items/optional-all", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == None
        assert "description" in response_data
        assert response_data["description"] == None
        assert "price" in response_data
        assert response_data["price"] == None
        assert "tax" in response_data
        assert response_data["tax"] == None


async def test_string_pattern_validation_fail() -> None:
    """Tests string field with regex pattern constraint failure."""

    async with TestClient(create_app_json_bodies_string_pattern_validation_fail()) as client:
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

    async with TestClient(create_app_json_bodies_49_dependencies_validation_failure()) as client:
        json_data = {"name": "John Doe", "credit_card": "1234567812345678"}
        response = await client.post("/billing", json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_simple_json_object_success() -> None:
    """Tests simple JSON object with all required fields."""

    async with TestClient(create_app_json_bodies_simple_json_object_success()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"name": "Foo", "description": "A very nice Item", "price": 35.4, "tax": 3.2}
        response = await client.post("/items/", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "Foo"
        assert "description" in response_data
        assert response_data["description"] == "A very nice Item"
        assert "price" in response_data
        assert response_data["price"] == 35.4
        assert "tax" in response_data
        assert response_data["tax"] == 3.2


async def test_required_field_missing_validation_error() -> None:
    """Tests validation error when required field is missing."""

    async with TestClient(create_app_json_bodies_required_field_missing_validation_error()) as client:
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

    async with TestClient(create_app_json_bodies_35_oneof_schema_success()) as client:
        json_data = {"credit_card": "1234567812345678"}
        response = await client.post("/payment", json=json_data)

        assert response.status_code == 201


async def test_enum_field_invalid_value() -> None:
    """Tests enum field with value not in enum."""

    async with TestClient(create_app_json_bodies_enum_field_invalid_value()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"name": "Item", "category": "furniture"}
        response = await client.post("/items/", headers=headers, json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_enum_field_success() -> None:
    """Tests enum field with valid enum value."""

    async with TestClient(create_app_json_bodies_enum_field_success()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"name": "Item", "category": "electronics"}
        response = await client.post("/items/", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "Item"
        assert "category" in response_data
        assert response_data["category"] == "electronics"


async def test_33_allof_schema_composition() -> None:
    """JSON Schema allOf composition should validate all schemas."""

    async with TestClient(create_app_json_bodies_33_allof_schema_composition()) as client:
        json_data = {"name": "Product", "price": 29.99}
        response = await client.post("/items", json=json_data)

        assert response.status_code == 201


async def test_45_minproperties_validation_success() -> None:
    """Object with properties meeting minProperties constraint should succeed."""

    async with TestClient(create_app_json_bodies_45_minproperties_validation_success()) as client:
        json_data = {"host": "localhost", "port": 8080}
        response = await client.post("/config", json=json_data)

        assert response.status_code == 201


async def test_body_with_query_parameters() -> None:
    """Tests JSON body combined with query parameters."""

    async with TestClient(create_app_json_bodies_body_with_query_parameters()) as client:
        params = {
            "limit": 10,
        }
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"name": "Item", "price": 42.0}
        response = await client.post("/items/?limit=10", params=params, headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "item" in response_data
        assert "name" in response_data["item"]
        assert response_data["item"]["name"] == "Item"
        assert "price" in response_data["item"]
        assert response_data["item"]["price"] == 42.0
        assert "limit" in response_data
        assert response_data["limit"] == 10


async def test_42_not_schema_failure() -> None:
    """not schema - fails when value matches the prohibited schema."""

    async with TestClient(create_app_json_bodies_42_not_schema_failure()) as client:
        json_data = {"username": "admin"}
        response = await client.post("/users", json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_43_const_validation_success() -> None:
    """Field with const constraint matching exact value should succeed."""

    async with TestClient(create_app_json_bodies_43_const_validation_success()) as client:
        json_data = {"version": "1.0", "data": "test"}
        response = await client.post("/api/v1/data", json=json_data)

        assert response.status_code == 201


async def test_32_schema_ref_definitions() -> None:
    """JSON Schema $ref with definitions should validate correctly."""

    async with TestClient(create_app_json_bodies_32_schema_ref_definitions()) as client:
        json_data = {"product": {"name": "Widget", "price": 9.99}}
        response = await client.post("/products", json=json_data)

        assert response.status_code == 201


async def test_29_nested_object_validation_success() -> None:
    """Nested object in JSON body should validate correctly."""

    async with TestClient(create_app_json_bodies_29_nested_object_validation_success()) as client:
        json_data = {"profile": {"name": "John Doe", "email": "john@example.com"}}
        response = await client.post("/users", json=json_data)

        assert response.status_code == 201


async def test_34_additional_properties_false() -> None:
    """Schema with additionalProperties false should reject extra fields."""

    async with TestClient(create_app_json_bodies_34_additional_properties_false()) as client:
        json_data = {"name": "John", "email": "john@example.com", "extra_field": "should fail"}
        response = await client.post("/users", json=json_data)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_null_value_for_optional_field() -> None:
    """Tests explicitly setting optional field to null."""

    async with TestClient(create_app_json_bodies_null_value_for_optional_field()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {"name": "Item", "price": 42.0, "description": None, "tax": None}
        response = await client.post("/items/", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "Item"
        assert "price" in response_data
        assert response_data["price"] == 42.0
        assert "description" in response_data
        assert response_data["description"] == None
        assert "tax" in response_data
        assert response_data["tax"] == None


async def test_31_nullable_property_null_value() -> None:
    """Nullable property with null value should be accepted."""

    async with TestClient(create_app_json_bodies_31_nullable_property_null_value()) as client:
        json_data = {"name": "Test User", "description": None}
        response = await client.post("/users", json=json_data)

        assert response.status_code == 201


async def test_array_of_objects_success() -> None:
    """Tests array field containing objects."""

    async with TestClient(create_app_json_bodies_array_of_objects_success()) as client:
        headers = {
            "Content-Type": "application/json",
        }
        json_data = {
            "name": "Product Bundle",
            "tags": ["electronics", "gadget"],
            "images": [
                {"url": "https://example.com/img1.jpg", "name": "Front"},
                {"url": "https://example.com/img2.jpg", "name": "Back"},
            ],
        }
        response = await client.post("/items/list", headers=headers, json=json_data)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "Product Bundle"
        assert "tags" in response_data
        assert len(response_data["tags"]) == 2
        assert response_data["tags"][0] == "electronics"
        assert response_data["tags"][1] == "gadget"
        assert "images" in response_data
        assert len(response_data["images"]) == 2
        assert "url" in response_data["images"][0]
        assert response_data["images"][0]["url"] == "https://example.com/img1.jpg"
        assert "name" in response_data["images"][0]
        assert response_data["images"][0]["name"] == "Front"
        assert "url" in response_data["images"][1]
        assert response_data["images"][1]["url"] == "https://example.com/img2.jpg"
        assert "name" in response_data["images"][1]
        assert response_data["images"][1]["name"] == "Back"
