/**
 * E2E tests for json_bodies
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
import {
	createAppJsonBodies29NestedObjectValidationSuccess,
	createAppJsonBodies30NestedObjectMissingField,
	createAppJsonBodies31NullablePropertyNullValue,
	createAppJsonBodies32SchemaRefDefinitions,
	createAppJsonBodies33AllofSchemaComposition,
	createAppJsonBodies34AdditionalPropertiesFalse,
	createAppJsonBodies35OneofSchemaSuccess,
	createAppJsonBodies36OneofSchemaMultipleMatchFailure,
	createAppJsonBodies37OneofSchemaNoMatchFailure,
	createAppJsonBodies38AnyofSchemaSuccess,
	createAppJsonBodies39AnyofSchemaMultipleMatchSuccess,
	createAppJsonBodies40AnyofSchemaFailure,
	createAppJsonBodies41NotSchemaSuccess,
	createAppJsonBodies42NotSchemaFailure,
	createAppJsonBodies43ConstValidationSuccess,
	createAppJsonBodies44ConstValidationFailure,
	createAppJsonBodies45MinpropertiesValidationSuccess,
	createAppJsonBodies46MinpropertiesValidationFailure,
	createAppJsonBodies47MaxpropertiesValidationFailure,
	createAppJsonBodies48DependenciesValidationSuccess,
	createAppJsonBodies49DependenciesValidationFailure,
	createAppJsonBodies50DeepNesting4Levels,
	createAppJsonBodiesArrayOfObjectsSuccess,
	createAppJsonBodiesArrayOfPrimitiveValues,
	createAppJsonBodiesBodyWithQueryParameters,
	createAppJsonBodiesBooleanFieldSuccess,
	createAppJsonBodiesDateFieldSuccess,
	createAppJsonBodiesDatetimeFieldSuccess,
	createAppJsonBodiesDeeplyNestedObjects,
	createAppJsonBodiesEmptyArrayValidationFail,
	createAppJsonBodiesEmptyJsonObject,
	createAppJsonBodiesEnumFieldInvalidValue,
	createAppJsonBodiesEnumFieldSuccess,
	createAppJsonBodiesExtraFieldsIgnoredNoAdditionalproperties,
	createAppJsonBodiesFieldTypeValidationInvalidType,
	createAppJsonBodiesNestedObjectSuccess,
	createAppJsonBodiesNullValueForOptionalField,
	createAppJsonBodiesNumericGeValidationFail,
	createAppJsonBodiesNumericLeValidationSuccess,
	createAppJsonBodiesOptionalFieldsOmitted,
	createAppJsonBodiesPatchPartialUpdate,
	createAppJsonBodiesRequiredFieldMissingValidationError,
	createAppJsonBodiesSimpleJsonObjectSuccess,
	createAppJsonBodiesStringMaxLengthValidationFail,
	createAppJsonBodiesStringMinLengthValidationFail,
	createAppJsonBodiesStringPatternValidationFail,
	createAppJsonBodiesStringPatternValidationSuccess,
	createAppJsonBodiesUuidFieldInvalidFormat,
	createAppJsonBodiesUuidFieldSuccess,
} from "../app/main.ts";

	Deno.test("json_bodies: UUID field - invalid format", async () => {
		const app = createAppJsonBodiesUuidFieldInvalidFormat();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", item_id: "not-a-valid-uuid" };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: 44_const_validation_failure", async () => {
		const app = createAppJsonBodies44ConstValidationFailure();
		const client = new TestClient(app);

		const json = { version: "2.0", data: "test" };
		const response = await client.post("/api/v1/data", { json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: Boolean field - success", async () => {
		const app = createAppJsonBodiesBooleanFieldSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 42.0, in_stock: true };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Item");
		assert(Object.hasOwn(responseData, "price"));
		assertEquals(responseData.price, 42.0);
		assert(Object.hasOwn(responseData, "in_stock"));
		assertEquals(responseData.in_stock, true);
	});

	Deno.test("json_bodies: Numeric le validation - success", async () => {
		const app = createAppJsonBodiesNumericLeValidationSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 100.0 };
		const response = await client.post("/items/validated", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Item");
		assert(Object.hasOwn(responseData, "price"));
		assertEquals(responseData.price, 100.0);
	});

	Deno.test("json_bodies: Deeply nested objects", async () => {
		const app = createAppJsonBodiesDeeplyNestedObjects();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Product", price: 100.0, seller: { name: "John Doe", address: { street: "123 Main St", city: "Springfield", country: { name: "USA", code: "US" } } } };
		const response = await client.post("/items/nested", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Product");
		assert(Object.hasOwn(responseData, "price"));
		assertEquals(responseData.price, 100.0);
		assert(Object.hasOwn(responseData, "seller"));
		assert(Object.hasOwn(responseData.seller, "name"));
		assertEquals(responseData.seller.name, "John Doe");
		assert(Object.hasOwn(responseData.seller, "address"));
		assert(Object.hasOwn(responseData.seller.address, "street"));
		assertEquals(responseData.seller.address.street, "123 Main St");
		assert(Object.hasOwn(responseData.seller.address, "city"));
		assertEquals(responseData.seller.address.city, "Springfield");
		assert(Object.hasOwn(responseData.seller.address, "country"));
		assert(Object.hasOwn(responseData.seller.address.country, "name"));
		assertEquals(responseData.seller.address.country.name, "USA");
		assert(Object.hasOwn(responseData.seller.address.country, "code"));
		assertEquals(responseData.seller.address.country.code, "US");
	});

	Deno.test("json_bodies: Optional fields - omitted", async () => {
		const app = createAppJsonBodiesOptionalFieldsOmitted();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Foo", price: 35.4 };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Foo");
		assert(Object.hasOwn(responseData, "price"));
		assertEquals(responseData.price, 35.4);
		assert(Object.hasOwn(responseData, "description"));
		assertEquals(responseData.description, null);
		assert(Object.hasOwn(responseData, "tax"));
		assertEquals(responseData.tax, null);
	});

	Deno.test("json_bodies: UUID field - success", async () => {
		const app = createAppJsonBodiesUuidFieldSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", item_id: "c892496f-b1fd-4b91-bdb8-b46f92df1716" };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Item");
		assert(Object.hasOwn(responseData, "item_id"));
		assertEquals(responseData.item_id, "c892496f-b1fd-4b91-bdb8-b46f92df1716");
	});

	Deno.test("json_bodies: Date field - success", async () => {
		const app = createAppJsonBodiesDateFieldSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Conference", event_date: "2024-03-15" };
		const response = await client.post("/events/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Conference");
		assert(Object.hasOwn(responseData, "event_date"));
		assertEquals(responseData.event_date, "2024-03-15");
	});

	Deno.test("json_bodies: 47_maxproperties_validation_failure", async () => {
		const app = createAppJsonBodies47MaxpropertiesValidationFailure();
		const client = new TestClient(app);

		const json = { host: "localhost", port: 8080, ssl: true, debug: false };
		const response = await client.post("/config", { json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: 46_minproperties_validation_failure", async () => {
		const app = createAppJsonBodies46MinpropertiesValidationFailure();
		const client = new TestClient(app);

		const json = { host: "localhost" };
		const response = await client.post("/config", { json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: String min_length validation - fail", async () => {
		const app = createAppJsonBodiesStringMinLengthValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "ab", price: 35.4 };
		const response = await client.post("/items/validated", { headers, json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: Field type validation - invalid type", async () => {
		const app = createAppJsonBodiesFieldTypeValidationInvalidType();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Foo", description: "A very nice Item", price: "not a number", tax: 3.2 };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: 36_oneof_schema_multiple_match_failure", async () => {
		const app = createAppJsonBodies36OneofSchemaMultipleMatchFailure();
		const client = new TestClient(app);

		const json = { credit_card: "1234567812345678", paypal_email: "user@example.com" };
		const response = await client.post("/payment", { json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: Nested object - success", async () => {
		const app = createAppJsonBodiesNestedObjectSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Foo", price: 42.0, image: { url: "https://example.com/image.jpg", name: "Product Image" } };
		const response = await client.post("/items/nested", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Foo");
		assert(Object.hasOwn(responseData, "price"));
		assertEquals(responseData.price, 42.0);
		assert(Object.hasOwn(responseData, "image"));
		assert(Object.hasOwn(responseData.image, "url"));
		assertEquals(responseData.image.url, "https://example.com/image.jpg");
		assert(Object.hasOwn(responseData.image, "name"));
		assertEquals(responseData.image.name, "Product Image");
	});

	Deno.test("json_bodies: 41_not_schema_success", async () => {
		const app = createAppJsonBodies41NotSchemaSuccess();
		const client = new TestClient(app);

		const json = { username: "john_doe" };
		const response = await client.post("/users", { json });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("json_bodies: String max_length validation - fail", async () => {
		const app = createAppJsonBodiesStringMaxLengthValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "This is a very long name that exceeds the maximum length", price: 35.4 };
		const response = await client.post("/items/validated", { headers, json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: 50_deep_nesting_4_levels", async () => {
		const app = createAppJsonBodies50DeepNesting4Levels();
		const client = new TestClient(app);

		const json = { user: { profile: { contact: { address: { street: "123 Main St" } } } } };
		const response = await client.post("/data", { json });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("json_bodies: 48_dependencies_validation_success", async () => {
		const app = createAppJsonBodies48DependenciesValidationSuccess();
		const client = new TestClient(app);

		const json = { name: "John Doe", credit_card: "1234567812345678", billing_address: "123 Main St" };
		const response = await client.post("/billing", { json });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("json_bodies: PATCH partial update", async () => {
		const app = createAppJsonBodiesPatchPartialUpdate();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { price: 45.0 };
		const response = await client.patch("/items/1", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Original Item");
		assert(Object.hasOwn(responseData, "price"));
		assertEquals(responseData.price, 45.0);
		assert(Object.hasOwn(responseData, "description"));
		assertEquals(responseData.description, "Original description");
	});

	Deno.test("json_bodies: 30_nested_object_missing_field", async () => {
		const app = createAppJsonBodies30NestedObjectMissingField();
		const client = new TestClient(app);

		const json = { profile: { name: "John Doe" } };
		const response = await client.post("/users", { json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: Datetime field - success", async () => {
		const app = createAppJsonBodiesDatetimeFieldSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Meeting", created_at: "2024-03-15T10:30:00Z" };
		const response = await client.post("/events/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Meeting");
		assert(Object.hasOwn(responseData, "created_at"));
		assertEquals(responseData.created_at, "2024-03-15T10:30:00Z");
	});

	Deno.test("json_bodies: String pattern validation - success", async () => {
		const app = createAppJsonBodiesStringPatternValidationSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", sku: "ABC1234" };
		const response = await client.post("/items/validated", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Item");
		assert(Object.hasOwn(responseData, "sku"));
		assertEquals(responseData.sku, "ABC1234");
	});

	Deno.test("json_bodies: Extra fields ignored no additionalProperties", async () => {
		const app = createAppJsonBodiesExtraFieldsIgnoredNoAdditionalproperties();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 42.0, extra_field: "this should be ignored", another_extra: 123 };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Item");
		assert(Object.hasOwn(responseData, "price"));
		assertEquals(responseData.price, 42.0);
	});

	Deno.test("json_bodies: 40_anyof_schema_failure", async () => {
		const app = createAppJsonBodies40AnyofSchemaFailure();
		const client = new TestClient(app);

		const json = { name: "John Doe" };
		const response = await client.post("/contact", { json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: 39_anyof_schema_multiple_match_success", async () => {
		const app = createAppJsonBodies39AnyofSchemaMultipleMatchSuccess();
		const client = new TestClient(app);

		const json = { name: "John Doe", email: "john@example.com", phone: "+1-555-0100" };
		const response = await client.post("/contact", { json });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("json_bodies: Array of primitive values", async () => {
		const app = createAppJsonBodiesArrayOfPrimitiveValues();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Product", tags: ["electronics", "gadget", "new"], ratings: [4.5, 4.8, 5.0, 4.2] };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Product");
		assert(Object.hasOwn(responseData, "tags"));
		assertEquals(responseData.tags.length, 3);
		assertEquals(responseData.tags[0], "electronics");
		assertEquals(responseData.tags[1], "gadget");
		assertEquals(responseData.tags[2], "new");
		assert(Object.hasOwn(responseData, "ratings"));
		assertEquals(responseData.ratings.length, 4);
		assertEquals(responseData.ratings[0], 4.5);
		assertEquals(responseData.ratings[1], 4.8);
		assertEquals(responseData.ratings[2], 5.0);
		assertEquals(responseData.ratings[3], 4.2);
	});

	Deno.test("json_bodies: Numeric ge validation - fail", async () => {
		const app = createAppJsonBodiesNumericGeValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 0.5 };
		const response = await client.post("/items/validated", { headers, json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: 37_oneof_schema_no_match_failure", async () => {
		const app = createAppJsonBodies37OneofSchemaNoMatchFailure();
		const client = new TestClient(app);

		const json = { bitcoin_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa" };
		const response = await client.post("/payment", { json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: Empty array validation - fail", async () => {
		const app = createAppJsonBodiesEmptyArrayValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Product", tags: [] };
		const response = await client.post("/items/list-validated", { headers, json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: 38_anyof_schema_success", async () => {
		const app = createAppJsonBodies38AnyofSchemaSuccess();
		const client = new TestClient(app);

		const json = { name: "John Doe", email: "john@example.com" };
		const response = await client.post("/contact", { json });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("json_bodies: Empty JSON object", async () => {
		const app = createAppJsonBodiesEmptyJsonObject();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {  };
		const response = await client.post("/items/optional-all", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, null);
		assert(Object.hasOwn(responseData, "description"));
		assertEquals(responseData.description, null);
		assert(Object.hasOwn(responseData, "price"));
		assertEquals(responseData.price, null);
		assert(Object.hasOwn(responseData, "tax"));
		assertEquals(responseData.tax, null);
	});

	Deno.test("json_bodies: String pattern validation - fail", async () => {
		const app = createAppJsonBodiesStringPatternValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", sku: "ABC-123" };
		const response = await client.post("/items/validated", { headers, json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: 49_dependencies_validation_failure", async () => {
		const app = createAppJsonBodies49DependenciesValidationFailure();
		const client = new TestClient(app);

		const json = { name: "John Doe", credit_card: "1234567812345678" };
		const response = await client.post("/billing", { json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: Simple JSON object - success", async () => {
		const app = createAppJsonBodiesSimpleJsonObjectSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Foo", description: "A very nice Item", price: 35.4, tax: 3.2 };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Foo");
		assert(Object.hasOwn(responseData, "description"));
		assertEquals(responseData.description, "A very nice Item");
		assert(Object.hasOwn(responseData, "price"));
		assertEquals(responseData.price, 35.4);
		assert(Object.hasOwn(responseData, "tax"));
		assertEquals(responseData.tax, 3.2);
	});

	Deno.test("json_bodies: Required field missing - validation error", async () => {
		const app = createAppJsonBodiesRequiredFieldMissingValidationError();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { description: "A very nice Item", price: 35.4 };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: 35_oneof_schema_success", async () => {
		const app = createAppJsonBodies35OneofSchemaSuccess();
		const client = new TestClient(app);

		const json = { credit_card: "1234567812345678" };
		const response = await client.post("/payment", { json });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("json_bodies: Enum field - invalid value", async () => {
		const app = createAppJsonBodiesEnumFieldInvalidValue();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", category: "furniture" };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: Enum field - success", async () => {
		const app = createAppJsonBodiesEnumFieldSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", category: "electronics" };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Item");
		assert(Object.hasOwn(responseData, "category"));
		assertEquals(responseData.category, "electronics");
	});

	Deno.test("json_bodies: 33_allof_schema_composition", async () => {
		const app = createAppJsonBodies33AllofSchemaComposition();
		const client = new TestClient(app);

		const json = { name: "Product", price: 29.99 };
		const response = await client.post("/items", { json });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("json_bodies: 45_minproperties_validation_success", async () => {
		const app = createAppJsonBodies45MinpropertiesValidationSuccess();
		const client = new TestClient(app);

		const json = { host: "localhost", port: 8080 };
		const response = await client.post("/config", { json });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("json_bodies: Body with query parameters", async () => {
		const app = createAppJsonBodiesBodyWithQueryParameters();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 42.0 };
		const response = await client.post("/items/?limit=10&limit=10", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "item"));
		assert(Object.hasOwn(responseData.item, "name"));
		assertEquals(responseData.item.name, "Item");
		assert(Object.hasOwn(responseData.item, "price"));
		assertEquals(responseData.item.price, 42.0);
		assert(Object.hasOwn(responseData, "limit"));
		assertEquals(responseData.limit, 10);
	});

	Deno.test("json_bodies: 42_not_schema_failure", async () => {
		const app = createAppJsonBodies42NotSchemaFailure();
		const client = new TestClient(app);

		const json = { username: "admin" };
		const response = await client.post("/users", { json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: 43_const_validation_success", async () => {
		const app = createAppJsonBodies43ConstValidationSuccess();
		const client = new TestClient(app);

		const json = { version: "1.0", data: "test" };
		const response = await client.post("/api/v1/data", { json });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("json_bodies: 32_schema_ref_definitions", async () => {
		const app = createAppJsonBodies32SchemaRefDefinitions();
		const client = new TestClient(app);

		const json = { product: { name: "Widget", price: 9.99 } };
		const response = await client.post("/products", { json });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("json_bodies: 29_nested_object_validation_success", async () => {
		const app = createAppJsonBodies29NestedObjectValidationSuccess();
		const client = new TestClient(app);

		const json = { profile: { name: "John Doe", email: "john@example.com" } };
		const response = await client.post("/users", { json });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("json_bodies: 34_additional_properties_false", async () => {
		const app = createAppJsonBodies34AdditionalPropertiesFalse();
		const client = new TestClient(app);

		const json = { name: "John", email: "john@example.com", extra_field: "should fail" };
		const response = await client.post("/users", { json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("json_bodies: Null value for optional field", async () => {
		const app = createAppJsonBodiesNullValueForOptionalField();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 42.0, description: null, tax: null };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Item");
		assert(Object.hasOwn(responseData, "price"));
		assertEquals(responseData.price, 42.0);
		assert(Object.hasOwn(responseData, "description"));
		assertEquals(responseData.description, null);
		assert(Object.hasOwn(responseData, "tax"));
		assertEquals(responseData.tax, null);
	});

	Deno.test("json_bodies: 31_nullable_property_null_value", async () => {
		const app = createAppJsonBodies31NullablePropertyNullValue();
		const client = new TestClient(app);

		const json = { name: "Test User", description: null };
		const response = await client.post("/users", { json });

		assertEquals(response.statusCode, 201);
	});

	Deno.test("json_bodies: Array of objects - success", async () => {
		const app = createAppJsonBodiesArrayOfObjectsSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Product Bundle", tags: ["electronics", "gadget"], images: [{ url: "https://example.com/img1.jpg", name: "Front" }, { url: "https://example.com/img2.jpg", name: "Back" }] };
		const response = await client.post("/items/list", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Product Bundle");
		assert(Object.hasOwn(responseData, "tags"));
		assertEquals(responseData.tags.length, 2);
		assertEquals(responseData.tags[0], "electronics");
		assertEquals(responseData.tags[1], "gadget");
		assert(Object.hasOwn(responseData, "images"));
		assertEquals(responseData.images.length, 2);
		assert(Object.hasOwn(responseData.images[0], "url"));
		assertEquals(responseData.images[0].url, "https://example.com/img1.jpg");
		assert(Object.hasOwn(responseData.images[0], "name"));
		assertEquals(responseData.images[0].name, "Front");
		assert(Object.hasOwn(responseData.images[1], "url"));
		assertEquals(responseData.images[1].url, "https://example.com/img2.jpg");
		assert(Object.hasOwn(responseData.images[1], "name"));
		assertEquals(responseData.images[1].name, "Back");
	});