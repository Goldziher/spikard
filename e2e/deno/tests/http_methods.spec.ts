/**
 * E2E tests for http_methods
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assertEquals } from "@std/assert";
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
		"Access-Control-Request-Method": "POST",
		Origin: "https://example.com",
	};
	const response = await client.options("/items/", { headers });

	assertEquals(response.statusCode, 200);
	const responseHeaders = response.headers();
	assertEquals(responseHeaders["access-control-allow-headers"], "Content-Type");
	assertEquals(responseHeaders["access-control-allow-origin"], "https://example.com");
	assertEquals(responseHeaders["access-control-max-age"], "86400");
	assertEquals(responseHeaders["access-control-allow-methods"], "GET, POST, PUT, DELETE, OPTIONS");
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
	expect(responseData).toHaveProperty("id");
	expect(responseData.id).toBe(999);
	expect(responseData).toHaveProperty("name");
	expect(responseData.name).toBe("New Item");
	expect(responseData).toHaveProperty("price");
	expect(responseData.price).toBe(49.99);
});

Deno.test("http_methods: PATCH - Update multiple fields", async () => {
	const app = createAppHttpMethodsPatchUpdateMultipleFields();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { in_stock: false, name: "Updated Name", price: 89.99 };
	const response = await client.patch("/items/1", { headers, json });

	expect(response.statusCode).toBe(200);
	const responseData = response.json();
	expect(responseData).toHaveProperty("id");
	expect(responseData.id).toBe(1);
	expect(responseData).toHaveProperty("in_stock");
	expect(responseData.in_stock).toBe(false);
	expect(responseData).toHaveProperty("name");
	expect(responseData.name).toBe("Updated Name");
	expect(responseData).toHaveProperty("price");
	expect(responseData.price).toBe(89.99);
});

Deno.test("http_methods: PUT - Validation error", async () => {
	const app = createAppHttpMethodsPutValidationError();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { id: 1, name: "X", price: -10 };
	const response = await client.put("/items/1", { headers, json });

	expect(response.statusCode).toBe(422);
});

Deno.test("http_methods: HEAD - Get metadata without body", async () => {
	const app = createAppHttpMethodsHeadGetMetadataWithoutBody();
	const client = new TestClient(app);

	const response = await client.head("/items/1");

	expect(response.statusCode).toBe(200);
	const responseHeaders = response.headers();
	expect(responseHeaders["content-type"]).toBe("application/json");
	expect(responseHeaders["content-length"]).toBe("85");
});

Deno.test("http_methods: DELETE - With response body", async () => {
	const app = createAppHttpMethodsDeleteWithResponseBody();
	const client = new TestClient(app);

	const response = await client.delete("/items/1");

	expect(response.statusCode).toBe(200);
	const responseData = response.json();
	expect(responseData).toHaveProperty("id");
	expect(responseData.id).toBe(1);
	expect(responseData).toHaveProperty("message");
	expect(responseData.message).toBe("Item deleted successfully");
	expect(responseData).toHaveProperty("name");
	expect(responseData.name).toBe("Deleted Item");
});

Deno.test("http_methods: PUT - Missing required field", async () => {
	const app = createAppHttpMethodsPutMissingRequiredField();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { id: 1, name: "Item Name" };
	const response = await client.put("/items/1", { headers, json });

	expect(response.statusCode).toBe(422);
});

Deno.test("http_methods: PATCH - Partial update", async () => {
	const app = createAppHttpMethodsPatchPartialUpdate();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { price: 79.99 };
	const response = await client.patch("/items/1", { headers, json });

	expect(response.statusCode).toBe(200);
	const responseData = response.json();
	expect(responseData).toHaveProperty("id");
	expect(responseData.id).toBe(1);
	expect(responseData).toHaveProperty("in_stock");
	expect(responseData.in_stock).toBe(true);
	expect(responseData).toHaveProperty("name");
	expect(responseData.name).toBe("Existing Item");
	expect(responseData).toHaveProperty("price");
	expect(responseData.price).toBe(79.99);
});

Deno.test("http_methods: DELETE - Resource not found", async () => {
	const app = createAppHttpMethodsDeleteResourceNotFound();
	const client = new TestClient(app);

	const response = await client.delete("/items/999");

	expect(response.statusCode).toBe(200);
});

Deno.test("http_methods: PUT - Idempotent operation", async () => {
	const app = createAppHttpMethodsPutIdempotentOperation();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { id: 1, name: "Fixed Name", price: 50.0 };
	const response = await client.put("/items/1", { headers, json });

	expect(response.statusCode).toBe(200);
	const responseData = response.json();
	expect(responseData).toHaveProperty("id");
	expect(responseData.id).toBe(1);
	expect(responseData).toHaveProperty("name");
	expect(responseData.name).toBe("Fixed Name");
	expect(responseData).toHaveProperty("price");
	expect(responseData.price).toBe(50.0);
});

Deno.test("http_methods: PUT - Complete resource replacement", async () => {
	const app = createAppHttpMethodsPutCompleteResourceReplacement();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { description: "Completely replaced", id: 1, in_stock: true, name: "Updated Item", price: 99.99 };
	const response = await client.put("/items/1", { headers, json });

	expect(response.statusCode).toBe(200);
	const responseData = response.json();
	expect(responseData).toHaveProperty("description");
	expect(responseData.description).toBe("Completely replaced");
	expect(responseData).toHaveProperty("id");
	expect(responseData.id).toBe(1);
	expect(responseData).toHaveProperty("in_stock");
	expect(responseData.in_stock).toBe(true);
	expect(responseData).toHaveProperty("name");
	expect(responseData.name).toBe("Updated Item");
	expect(responseData).toHaveProperty("price");
	expect(responseData.price).toBe(99.99);
});
