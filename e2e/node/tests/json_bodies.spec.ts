/**
 * E2E tests for json_bodies
 * @generated
 */

import { TestClient } from "@spikard/node";
import { describe, expect, test } from "vitest";
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

describe("json_bodies", () => {
	test("UUID field - invalid format", async () => {
		const app = createAppJsonBodiesUuidFieldInvalidFormat();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", item_id: "not-a-valid-uuid" };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("44_const_validation_failure", async () => {
		const app = createAppJsonBodies44ConstValidationFailure();
		const client = new TestClient(app);

		const json = { version: "2.0", data: "test" };
		const response = await client.post("/api/v1/data", { json });

		expect(response.statusCode).toBe(422);
	});

	test("Boolean field - success", async () => {
		const app = createAppJsonBodiesBooleanFieldSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 42.0, in_stock: true };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Item");
		expect(responseData).toHaveProperty("price");
		expect(responseData.price).toBe(42.0);
		expect(responseData).toHaveProperty("in_stock");
		expect(responseData.in_stock).toBe(true);
	});

	test("Numeric le validation - success", async () => {
		const app = createAppJsonBodiesNumericLeValidationSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 100.0 };
		const response = await client.post("/items/validated", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Item");
		expect(responseData).toHaveProperty("price");
		expect(responseData.price).toBe(100.0);
	});

	test("Deeply nested objects", async () => {
		const app = createAppJsonBodiesDeeplyNestedObjects();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {
			name: "Product",
			price: 100.0,
			seller: {
				name: "John Doe",
				address: { street: "123 Main St", city: "Springfield", country: { name: "USA", code: "US" } },
			},
		};
		const response = await client.post("/items/nested", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Product");
		expect(responseData).toHaveProperty("price");
		expect(responseData.price).toBe(100.0);
		expect(responseData).toHaveProperty("seller");
		expect(responseData.seller).toHaveProperty("name");
		expect(responseData.seller.name).toBe("John Doe");
		expect(responseData.seller).toHaveProperty("address");
		expect(responseData.seller.address).toHaveProperty("street");
		expect(responseData.seller.address.street).toBe("123 Main St");
		expect(responseData.seller.address).toHaveProperty("city");
		expect(responseData.seller.address.city).toBe("Springfield");
		expect(responseData.seller.address).toHaveProperty("country");
		expect(responseData.seller.address.country).toHaveProperty("name");
		expect(responseData.seller.address.country.name).toBe("USA");
		expect(responseData.seller.address.country).toHaveProperty("code");
		expect(responseData.seller.address.country.code).toBe("US");
	});

	test("Optional fields - omitted", async () => {
		const app = createAppJsonBodiesOptionalFieldsOmitted();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Foo", price: 35.4 };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Foo");
		expect(responseData).toHaveProperty("price");
		expect(responseData.price).toBe(35.4);
		expect(responseData).toHaveProperty("description");
		expect(responseData.description).toBe(null);
		expect(responseData).toHaveProperty("tax");
		expect(responseData.tax).toBe(null);
	});

	test("UUID field - success", async () => {
		const app = createAppJsonBodiesUuidFieldSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", item_id: "c892496f-b1fd-4b91-bdb8-b46f92df1716" };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Item");
		expect(responseData).toHaveProperty("item_id");
		expect(responseData.item_id).toBe("c892496f-b1fd-4b91-bdb8-b46f92df1716");
	});

	test("Date field - success", async () => {
		const app = createAppJsonBodiesDateFieldSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Conference", event_date: "2024-03-15" };
		const response = await client.post("/events/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Conference");
		expect(responseData).toHaveProperty("event_date");
		expect(responseData.event_date).toBe("2024-03-15");
	});

	test("47_maxproperties_validation_failure", async () => {
		const app = createAppJsonBodies47MaxpropertiesValidationFailure();
		const client = new TestClient(app);

		const json = { host: "localhost", port: 8080, ssl: true, debug: false };
		const response = await client.post("/config", { json });

		expect(response.statusCode).toBe(422);
	});

	test("46_minproperties_validation_failure", async () => {
		const app = createAppJsonBodies46MinpropertiesValidationFailure();
		const client = new TestClient(app);

		const json = { host: "localhost" };
		const response = await client.post("/config", { json });

		expect(response.statusCode).toBe(422);
	});

	test("String min_length validation - fail", async () => {
		const app = createAppJsonBodiesStringMinLengthValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "ab", price: 35.4 };
		const response = await client.post("/items/validated", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("Field type validation - invalid type", async () => {
		const app = createAppJsonBodiesFieldTypeValidationInvalidType();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Foo", description: "A very nice Item", price: "not a number", tax: 3.2 };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("36_oneof_schema_multiple_match_failure", async () => {
		const app = createAppJsonBodies36OneofSchemaMultipleMatchFailure();
		const client = new TestClient(app);

		const json = { credit_card: "1234567812345678", paypal_email: "user@example.com" };
		const response = await client.post("/payment", { json });

		expect(response.statusCode).toBe(422);
	});

	test("Nested object - success", async () => {
		const app = createAppJsonBodiesNestedObjectSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Foo", price: 42.0, image: { url: "https://example.com/image.jpg", name: "Product Image" } };
		const response = await client.post("/items/nested", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Foo");
		expect(responseData).toHaveProperty("price");
		expect(responseData.price).toBe(42.0);
		expect(responseData).toHaveProperty("image");
		expect(responseData.image).toHaveProperty("url");
		expect(responseData.image.url).toBe("https://example.com/image.jpg");
		expect(responseData.image).toHaveProperty("name");
		expect(responseData.image.name).toBe("Product Image");
	});

	test("41_not_schema_success", async () => {
		const app = createAppJsonBodies41NotSchemaSuccess();
		const client = new TestClient(app);

		const json = { username: "john_doe" };
		const response = await client.post("/users", { json });

		expect(response.statusCode).toBe(201);
	});

	test("String max_length validation - fail", async () => {
		const app = createAppJsonBodiesStringMaxLengthValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "This is a very long name that exceeds the maximum length", price: 35.4 };
		const response = await client.post("/items/validated", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("50_deep_nesting_4_levels", async () => {
		const app = createAppJsonBodies50DeepNesting4Levels();
		const client = new TestClient(app);

		const json = { user: { profile: { contact: { address: { street: "123 Main St" } } } } };
		const response = await client.post("/data", { json });

		expect(response.statusCode).toBe(201);
	});

	test("48_dependencies_validation_success", async () => {
		const app = createAppJsonBodies48DependenciesValidationSuccess();
		const client = new TestClient(app);

		const json = { name: "John Doe", credit_card: "1234567812345678", billing_address: "123 Main St" };
		const response = await client.post("/billing", { json });

		expect(response.statusCode).toBe(201);
	});

	test("PATCH partial update", async () => {
		const app = createAppJsonBodiesPatchPartialUpdate();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { price: 45.0 };
		const response = await client.patch("/items/1", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Original Item");
		expect(responseData).toHaveProperty("price");
		expect(responseData.price).toBe(45.0);
		expect(responseData).toHaveProperty("description");
		expect(responseData.description).toBe("Original description");
	});

	test("30_nested_object_missing_field", async () => {
		const app = createAppJsonBodies30NestedObjectMissingField();
		const client = new TestClient(app);

		const json = { profile: { name: "John Doe" } };
		const response = await client.post("/users", { json });

		expect(response.statusCode).toBe(422);
	});

	test("Datetime field - success", async () => {
		const app = createAppJsonBodiesDatetimeFieldSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Meeting", created_at: "2024-03-15T10:30:00Z" };
		const response = await client.post("/events/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Meeting");
		expect(responseData).toHaveProperty("created_at");
		expect(responseData.created_at).toBe("2024-03-15T10:30:00Z");
	});

	test("String pattern validation - success", async () => {
		const app = createAppJsonBodiesStringPatternValidationSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", sku: "ABC1234" };
		const response = await client.post("/items/validated", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Item");
		expect(responseData).toHaveProperty("sku");
		expect(responseData.sku).toBe("ABC1234");
	});

	test("Extra fields ignored no additionalProperties", async () => {
		const app = createAppJsonBodiesExtraFieldsIgnoredNoAdditionalproperties();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 42.0, extra_field: "this should be ignored", another_extra: 123 };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Item");
		expect(responseData).toHaveProperty("price");
		expect(responseData.price).toBe(42.0);
	});

	test("40_anyof_schema_failure", async () => {
		const app = createAppJsonBodies40AnyofSchemaFailure();
		const client = new TestClient(app);

		const json = { name: "John Doe" };
		const response = await client.post("/contact", { json });

		expect(response.statusCode).toBe(422);
	});

	test("39_anyof_schema_multiple_match_success", async () => {
		const app = createAppJsonBodies39AnyofSchemaMultipleMatchSuccess();
		const client = new TestClient(app);

		const json = { name: "John Doe", email: "john@example.com", phone: "+1-555-0100" };
		const response = await client.post("/contact", { json });

		expect(response.statusCode).toBe(201);
	});

	test("Array of primitive values", async () => {
		const app = createAppJsonBodiesArrayOfPrimitiveValues();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Product", tags: ["electronics", "gadget", "new"], ratings: [4.5, 4.8, 5.0, 4.2] };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Product");
		expect(responseData).toHaveProperty("tags");
		expect(responseData.tags.length).toBe(3);
		expect(responseData.tags[0]).toBe("electronics");
		expect(responseData.tags[1]).toBe("gadget");
		expect(responseData.tags[2]).toBe("new");
		expect(responseData).toHaveProperty("ratings");
		expect(responseData.ratings.length).toBe(4);
		expect(responseData.ratings[0]).toBe(4.5);
		expect(responseData.ratings[1]).toBe(4.8);
		expect(responseData.ratings[2]).toBe(5.0);
		expect(responseData.ratings[3]).toBe(4.2);
	});

	test("Numeric ge validation - fail", async () => {
		const app = createAppJsonBodiesNumericGeValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 0.5 };
		const response = await client.post("/items/validated", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("37_oneof_schema_no_match_failure", async () => {
		const app = createAppJsonBodies37OneofSchemaNoMatchFailure();
		const client = new TestClient(app);

		const json = { bitcoin_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa" };
		const response = await client.post("/payment", { json });

		expect(response.statusCode).toBe(422);
	});

	test("Empty array validation - fail", async () => {
		const app = createAppJsonBodiesEmptyArrayValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Product", tags: [] };
		const response = await client.post("/items/list-validated", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("38_anyof_schema_success", async () => {
		const app = createAppJsonBodies38AnyofSchemaSuccess();
		const client = new TestClient(app);

		const json = { name: "John Doe", email: "john@example.com" };
		const response = await client.post("/contact", { json });

		expect(response.statusCode).toBe(201);
	});

	test("Empty JSON object", async () => {
		const app = createAppJsonBodiesEmptyJsonObject();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {};
		const response = await client.post("/items/optional-all", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe(null);
		expect(responseData).toHaveProperty("description");
		expect(responseData.description).toBe(null);
		expect(responseData).toHaveProperty("price");
		expect(responseData.price).toBe(null);
		expect(responseData).toHaveProperty("tax");
		expect(responseData.tax).toBe(null);
	});

	test("String pattern validation - fail", async () => {
		const app = createAppJsonBodiesStringPatternValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", sku: "ABC-123" };
		const response = await client.post("/items/validated", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("49_dependencies_validation_failure", async () => {
		const app = createAppJsonBodies49DependenciesValidationFailure();
		const client = new TestClient(app);

		const json = { name: "John Doe", credit_card: "1234567812345678" };
		const response = await client.post("/billing", { json });

		expect(response.statusCode).toBe(422);
	});

	test("Simple JSON object - success", async () => {
		const app = createAppJsonBodiesSimpleJsonObjectSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Foo", description: "A very nice Item", price: 35.4, tax: 3.2 };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Foo");
		expect(responseData).toHaveProperty("description");
		expect(responseData.description).toBe("A very nice Item");
		expect(responseData).toHaveProperty("price");
		expect(responseData.price).toBe(35.4);
		expect(responseData).toHaveProperty("tax");
		expect(responseData.tax).toBe(3.2);
	});

	test("Required field missing - validation error", async () => {
		const app = createAppJsonBodiesRequiredFieldMissingValidationError();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { description: "A very nice Item", price: 35.4 };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("35_oneof_schema_success", async () => {
		const app = createAppJsonBodies35OneofSchemaSuccess();
		const client = new TestClient(app);

		const json = { credit_card: "1234567812345678" };
		const response = await client.post("/payment", { json });

		expect(response.statusCode).toBe(201);
	});

	test("Enum field - invalid value", async () => {
		const app = createAppJsonBodiesEnumFieldInvalidValue();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", category: "furniture" };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("Enum field - success", async () => {
		const app = createAppJsonBodiesEnumFieldSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", category: "electronics" };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Item");
		expect(responseData).toHaveProperty("category");
		expect(responseData.category).toBe("electronics");
	});

	test("33_allof_schema_composition", async () => {
		const app = createAppJsonBodies33AllofSchemaComposition();
		const client = new TestClient(app);

		const json = { name: "Product", price: 29.99 };
		const response = await client.post("/items", { json });

		expect(response.statusCode).toBe(201);
	});

	test("45_minproperties_validation_success", async () => {
		const app = createAppJsonBodies45MinpropertiesValidationSuccess();
		const client = new TestClient(app);

		const json = { host: "localhost", port: 8080 };
		const response = await client.post("/config", { json });

		expect(response.statusCode).toBe(201);
	});

	test("Body with query parameters", async () => {
		const app = createAppJsonBodiesBodyWithQueryParameters();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 42.0 };
		const response = await client.post("/items/?limit=10&limit=10", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("item");
		expect(responseData.item).toHaveProperty("name");
		expect(responseData.item.name).toBe("Item");
		expect(responseData.item).toHaveProperty("price");
		expect(responseData.item.price).toBe(42.0);
		expect(responseData).toHaveProperty("limit");
		expect(responseData.limit).toBe(10);
	});

	test("42_not_schema_failure", async () => {
		const app = createAppJsonBodies42NotSchemaFailure();
		const client = new TestClient(app);

		const json = { username: "admin" };
		const response = await client.post("/users", { json });

		expect(response.statusCode).toBe(422);
	});

	test("43_const_validation_success", async () => {
		const app = createAppJsonBodies43ConstValidationSuccess();
		const client = new TestClient(app);

		const json = { version: "1.0", data: "test" };
		const response = await client.post("/api/v1/data", { json });

		expect(response.statusCode).toBe(201);
	});

	test("32_schema_ref_definitions", async () => {
		const app = createAppJsonBodies32SchemaRefDefinitions();
		const client = new TestClient(app);

		const json = { product: { name: "Widget", price: 9.99 } };
		const response = await client.post("/products", { json });

		expect(response.statusCode).toBe(201);
	});

	test("29_nested_object_validation_success", async () => {
		const app = createAppJsonBodies29NestedObjectValidationSuccess();
		const client = new TestClient(app);

		const json = { profile: { name: "John Doe", email: "john@example.com" } };
		const response = await client.post("/users", { json });

		expect(response.statusCode).toBe(201);
	});

	test("34_additional_properties_false", async () => {
		const app = createAppJsonBodies34AdditionalPropertiesFalse();
		const client = new TestClient(app);

		const json = { name: "John", email: "john@example.com", extra_field: "should fail" };
		const response = await client.post("/users", { json });

		expect(response.statusCode).toBe(422);
	});

	test("Null value for optional field", async () => {
		const app = createAppJsonBodiesNullValueForOptionalField();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 42.0, description: null, tax: null };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Item");
		expect(responseData).toHaveProperty("price");
		expect(responseData.price).toBe(42.0);
		expect(responseData).toHaveProperty("description");
		expect(responseData.description).toBe(null);
		expect(responseData).toHaveProperty("tax");
		expect(responseData.tax).toBe(null);
	});

	test("31_nullable_property_null_value", async () => {
		const app = createAppJsonBodies31NullablePropertyNullValue();
		const client = new TestClient(app);

		const json = { name: "Test User", description: null };
		const response = await client.post("/users", { json });

		expect(response.statusCode).toBe(201);
	});

	test("Array of objects - success", async () => {
		const app = createAppJsonBodiesArrayOfObjectsSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {
			name: "Product Bundle",
			tags: ["electronics", "gadget"],
			images: [
				{ url: "https://example.com/img1.jpg", name: "Front" },
				{ url: "https://example.com/img2.jpg", name: "Back" },
			],
		};
		const response = await client.post("/items/list", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Product Bundle");
		expect(responseData).toHaveProperty("tags");
		expect(responseData.tags.length).toBe(2);
		expect(responseData.tags[0]).toBe("electronics");
		expect(responseData.tags[1]).toBe("gadget");
		expect(responseData).toHaveProperty("images");
		expect(responseData.images.length).toBe(2);
		expect(responseData.images[0]).toHaveProperty("url");
		expect(responseData.images[0].url).toBe("https://example.com/img1.jpg");
		expect(responseData.images[0]).toHaveProperty("name");
		expect(responseData.images[0].name).toBe("Front");
		expect(responseData.images[1]).toHaveProperty("url");
		expect(responseData.images[1].url).toBe("https://example.com/img2.jpg");
		expect(responseData.images[1]).toHaveProperty("name");
		expect(responseData.images[1].name).toBe("Back");
	});
});
