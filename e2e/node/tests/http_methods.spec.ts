/**
 * E2E tests for http_methods
 * @generated
 */

import { TestClient } from "@spikard/node";
import { describe, expect, test } from "vitest";
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

describe("http_methods", () => {
	test("OPTIONS - CORS preflight request", async () => {
		const app = createAppHttpMethodsOptionsCorsPreflightRequest();
		const client = new TestClient(app);

		const headers = {
			"Access-Control-Request-Headers": "Content-Type",
			Origin: "https://example.com",
			"Access-Control-Request-Method": "POST",
		};
		const response = await client.options("/items/", { headers });

		expect(response.statusCode).toBe(200);
		const responseHeaders = response.headers();
		expect(responseHeaders["access-control-allow-methods"]).toBe("GET, POST, PUT, DELETE, OPTIONS");
		expect(responseHeaders["access-control-max-age"]).toBe("86400");
		expect(responseHeaders["access-control-allow-origin"]).toBe("https://example.com");
		expect(responseHeaders["access-control-allow-headers"]).toBe("Content-Type");
	});

	test("DELETE - Remove resource", async () => {
		const app = createAppHttpMethodsDeleteRemoveResource();
		const client = new TestClient(app);

		const response = await client.delete("/items/1");

		expect(response.statusCode).toBe(200);
	});

	test("PUT - Create resource if doesn t exist", async () => {
		const app = createAppHttpMethodsPutCreateResourceIfDoesnTExist();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { id: 999, name: "New Item", price: 49.99 };
		const response = await client.put("/items/999", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("id");
		expect(responseData.id).toBe(999);
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("New Item");
		expect(responseData).toHaveProperty("price");
		expect(responseData.price).toBe(49.99);
	});

	test("PATCH - Update multiple fields", async () => {
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

	test("PUT - Validation error", async () => {
		const app = createAppHttpMethodsPutValidationError();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { id: 1, name: "X", price: -10 };
		const response = await client.put("/items/1", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("HEAD - Get metadata without body", async () => {
		const app = createAppHttpMethodsHeadGetMetadataWithoutBody();
		const client = new TestClient(app);

		const response = await client.head("/items/1");

		expect(response.statusCode).toBe(200);
		const responseHeaders = response.headers();
		expect(responseHeaders["content-length"]).toBe("85");
		expect(responseHeaders["content-type"]).toBe("application/json");
	});

	test("DELETE - With response body", async () => {
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

	test("PUT - Missing required field", async () => {
		const app = createAppHttpMethodsPutMissingRequiredField();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { id: 1, name: "Item Name" };
		const response = await client.put("/items/1", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("PATCH - Partial update", async () => {
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

	test("DELETE - Resource not found", async () => {
		const app = createAppHttpMethodsDeleteResourceNotFound();
		const client = new TestClient(app);

		const response = await client.delete("/items/999");

		expect(response.statusCode).toBe(200);
	});

	test("PUT - Idempotent operation", async () => {
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

	test("PUT - Complete resource replacement", async () => {
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
});
