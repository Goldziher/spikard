/**
 * E2E tests for http_methods
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
import {
	createAppHttpMethodsDeleteRemoveResource,
	createAppHttpMethodsDeleteResourceNotFound,
	createAppHttpMethodsDeleteWithResponseBody,
	createAppHttpMethodsHeadGetMetadataWithoutBody,
	createAppHttpMethodsOptionsCorsPreflightRequest,
	createAppHttpMethodsPatchPartialUpdate,
	createAppHttpMethodsPatchUpdateMultipleFields,
	createAppHttpMethodsPutCompleteResourceReplacement,
	createAppHttpMethodsPutCreateResourceIfDoesnTExist,
	createAppHttpMethodsPutIdempotentOperation,
	createAppHttpMethodsPutMissingRequiredField,
	createAppHttpMethodsPutValidationError,
} from "../app/main.ts";

Deno.test("http_methods: OPTIONS - CORS preflight request", async () => {
	const app = createAppHttpMethodsOptionsCorsPreflightRequest();
	const client = new TestClient(app);

	const headers = {
		"Access-Control-Request-Headers": "Content-Type",
		Origin: "https://example.com",
		"Access-Control-Request-Method": "POST",
	};
	const response = await client.options("/items/", { headers });

	assertEquals(response.statusCode, 200);
	const responseHeaders = response.headers();
	assertEquals(responseHeaders["access-control-allow-methods"], "GET, POST, PUT, DELETE, OPTIONS");
	assertEquals(responseHeaders["access-control-allow-origin"], "https://example.com");
	assertEquals(responseHeaders["access-control-allow-headers"], "Content-Type");
	assertEquals(responseHeaders["access-control-max-age"], "86400");
});

Deno.test("http_methods: DELETE - Remove resource", async () => {
	const app = createAppHttpMethodsDeleteRemoveResource();
	const client = new TestClient(app);

	const response = await client.delete("/items/1");

	assertEquals(response.statusCode, 200);
});

Deno.test("http_methods: PUT - Create resource if doesn t exist", async () => {
	const app = createAppHttpMethodsPutCreateResourceIfDoesnTExist();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { id: 999, name: "New Item", price: 49.99 };
	const response = await client.put("/items/999", { headers, json });

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "id"));
	assertEquals(responseData.id, 999);
	assert(Object.hasOwn(responseData, "name"));
	assertEquals(responseData.name, "New Item");
	assert(Object.hasOwn(responseData, "price"));
	assertEquals(responseData.price, 49.99);
});

Deno.test("http_methods: PATCH - Update multiple fields", async () => {
	const app = createAppHttpMethodsPatchUpdateMultipleFields();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { in_stock: false, name: "Updated Name", price: 89.99 };
	const response = await client.patch("/items/1", { headers, json });

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "id"));
	assertEquals(responseData.id, 1);
	assert(Object.hasOwn(responseData, "in_stock"));
	assertEquals(responseData.in_stock, false);
	assert(Object.hasOwn(responseData, "name"));
	assertEquals(responseData.name, "Updated Name");
	assert(Object.hasOwn(responseData, "price"));
	assertEquals(responseData.price, 89.99);
});

Deno.test("http_methods: PUT - Validation error", async () => {
	const app = createAppHttpMethodsPutValidationError();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { id: 1, name: "X", price: -10 };
	const response = await client.put("/items/1", { headers, json });

	assertEquals(response.statusCode, 422);
});

Deno.test("http_methods: HEAD - Get metadata without body", async () => {
	const app = createAppHttpMethodsHeadGetMetadataWithoutBody();
	const client = new TestClient(app);

	const response = await client.head("/items/1");

	assertEquals(response.statusCode, 200);
	const responseHeaders = response.headers();
	assertEquals(responseHeaders["content-length"], "85");
	assertEquals(responseHeaders["content-type"], "application/json");
});

Deno.test("http_methods: DELETE - With response body", async () => {
	const app = createAppHttpMethodsDeleteWithResponseBody();
	const client = new TestClient(app);

	const response = await client.delete("/items/1");

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "id"));
	assertEquals(responseData.id, 1);
	assert(Object.hasOwn(responseData, "message"));
	assertEquals(responseData.message, "Item deleted successfully");
	assert(Object.hasOwn(responseData, "name"));
	assertEquals(responseData.name, "Deleted Item");
});

Deno.test("http_methods: PUT - Missing required field", async () => {
	const app = createAppHttpMethodsPutMissingRequiredField();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { id: 1, name: "Item Name" };
	const response = await client.put("/items/1", { headers, json });

	assertEquals(response.statusCode, 422);
});

Deno.test("http_methods: PATCH - Partial update", async () => {
	const app = createAppHttpMethodsPatchPartialUpdate();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { price: 79.99 };
	const response = await client.patch("/items/1", { headers, json });

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "id"));
	assertEquals(responseData.id, 1);
	assert(Object.hasOwn(responseData, "in_stock"));
	assertEquals(responseData.in_stock, true);
	assert(Object.hasOwn(responseData, "name"));
	assertEquals(responseData.name, "Existing Item");
	assert(Object.hasOwn(responseData, "price"));
	assertEquals(responseData.price, 79.99);
});

Deno.test("http_methods: DELETE - Resource not found", async () => {
	const app = createAppHttpMethodsDeleteResourceNotFound();
	const client = new TestClient(app);

	const response = await client.delete("/items/999");

	assertEquals(response.statusCode, 200);
});

Deno.test("http_methods: PUT - Idempotent operation", async () => {
	const app = createAppHttpMethodsPutIdempotentOperation();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { id: 1, name: "Fixed Name", price: 50.0 };
	const response = await client.put("/items/1", { headers, json });

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "id"));
	assertEquals(responseData.id, 1);
	assert(Object.hasOwn(responseData, "name"));
	assertEquals(responseData.name, "Fixed Name");
	assert(Object.hasOwn(responseData, "price"));
	assertEquals(responseData.price, 50.0);
});

Deno.test("http_methods: PUT - Complete resource replacement", async () => {
	const app = createAppHttpMethodsPutCompleteResourceReplacement();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { description: "Completely replaced", id: 1, in_stock: true, name: "Updated Item", price: 99.99 };
	const response = await client.put("/items/1", { headers, json });

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "description"));
	assertEquals(responseData.description, "Completely replaced");
	assert(Object.hasOwn(responseData, "id"));
	assertEquals(responseData.id, 1);
	assert(Object.hasOwn(responseData, "in_stock"));
	assertEquals(responseData.in_stock, true);
	assert(Object.hasOwn(responseData, "name"));
	assertEquals(responseData.name, "Updated Item");
	assert(Object.hasOwn(responseData, "price"));
	assertEquals(responseData.price, 99.99);
});
