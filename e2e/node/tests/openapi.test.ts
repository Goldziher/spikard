/**
 * E2E tests for openapi
 * @generated
 */

import { TestClient } from "@spikard/node";
import { describe, expect, test } from "vitest";
import {
	createAppOpenapiOpenapiSpecGenerationBasic,
	createAppOpenapiOpenapiSpecWithApiKeySecurityScheme,
	createAppOpenapiOpenapiSpecWithCustomMetadata,
	createAppOpenapiOpenapiSpecWithJwtSecurityScheme,
	createAppOpenapiRedocServing,
	createAppOpenapiSwaggerUiServing,
} from "../app/main.js";

describe("openapi", () => {
	test("Redoc serving", async () => {
		const app = createAppOpenapiRedocServing();
		const client = new TestClient(app);

		const headers = {
			accept: "text/html",
		};
		const response = await client.get("/redoc", headers);

		expect(response.statusCode).toBe(200);
	});

	test("OpenAPI spec with JWT security scheme", async () => {
		const app = createAppOpenapiOpenapiSpecWithJwtSecurityScheme();
		const client = new TestClient(app);

		const headers = {
			accept: "application/json",
		};
		const response = await client.get("/openapi.json", headers);

		expect(response.statusCode).toBe(200);
	});

	test("Swagger UI serving", async () => {
		const app = createAppOpenapiSwaggerUiServing();
		const client = new TestClient(app);

		const headers = {
			accept: "text/html",
		};
		const response = await client.get("/docs", headers);

		expect(response.statusCode).toBe(200);
	});

	test("OpenAPI spec with API key security scheme", async () => {
		const app = createAppOpenapiOpenapiSpecWithApiKeySecurityScheme();
		const client = new TestClient(app);

		const headers = {
			accept: "application/json",
		};
		const response = await client.get("/openapi.json", headers);

		expect(response.statusCode).toBe(200);
	});

	test("OpenAPI spec with custom metadata", async () => {
		const app = createAppOpenapiOpenapiSpecWithCustomMetadata();
		const client = new TestClient(app);

		const headers = {
			accept: "application/json",
		};
		const response = await client.get("/openapi.json", headers);

		expect(response.statusCode).toBe(200);
	});

	test("OpenAPI spec generation - basic", async () => {
		const app = createAppOpenapiOpenapiSpecGenerationBasic();
		const client = new TestClient(app);

		const headers = {
			accept: "application/json",
		};
		const response = await client.get("/openapi.json", headers);

		expect(response.statusCode).toBe(200);
	});
});
