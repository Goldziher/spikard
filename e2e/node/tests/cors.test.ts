/**
 * E2E tests for cors
 * @generated
 */

import { describe, test, expect } from "vitest";
import { TestClient } from "@spikard/node";
import { createAppCors07CorsPreflightHeaderNotAllowed, createAppCorsCorsPreflightRequest, createAppCorsCorsWithCredentials, createAppCors08CorsMaxAge, createAppCors10CorsOriginNull, createAppCorsCorsWildcardOrigin, createAppCorsCorsRequestBlocked, createAppCorsSimpleCorsRequest, createAppCors09CorsExposeHeaders, createAppCors06CorsPreflightMethodNotAllowed } from "../app/main.js";

describe("cors", () => {
	test("07_cors_preflight_header_not_allowed", async () => {
		const app = createAppCors07CorsPreflightHeaderNotAllowed();
		const client = new TestClient(app);

		const headers = {
			"Access-Control-Request-Headers": "X-Custom-Header",
			"Origin": "https://example.com",
			"Access-Control-Request-Method": "POST",
		};
		const response = await client.options("/api/data", {headers});

		expect(response.statusCode).toBe(403);
	});

	test("CORS preflight request", async () => {
		const app = createAppCorsCorsPreflightRequest();
		const client = new TestClient(app);

		const headers = {
			"Access-Control-Request-Headers": "Content-Type, X-Custom-Header",
			"Origin": "https://example.com",
			"Access-Control-Request-Method": "POST",
		};
		const response = await client.options("/items/", {headers});

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
	});

	test("CORS with credentials", async () => {
		const app = createAppCorsCorsWithCredentials();
		const client = new TestClient(app);

		const headers = {
			"Origin": "https://app.example.com",
			"Cookie": "session=abc123",
		};
		const response = await client.get("/api/user/profile", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("username");
		expect(responseData["username"]).toBe("john");
	});

	test("08_cors_max_age", async () => {
		const app = createAppCors08CorsMaxAge();
		const client = new TestClient(app);

		const headers = {
			"Access-Control-Request-Method": "POST",
			"Origin": "https://example.com",
			"Access-Control-Request-Headers": "Content-Type",
		};
		const response = await client.options("/api/data", {headers});

		expect(response.statusCode).toBe(204);
	});

	test("10_cors_origin_null", async () => {
		const app = createAppCors10CorsOriginNull();
		const client = new TestClient(app);

		const headers = {
			"Origin": "null",
		};
		const response = await client.get("/api/data", headers);

		expect(response.statusCode).toBe(403);
		const responseData = response.json();
		expect(responseData).toHaveProperty("error");
		expect(responseData["error"]).toBe("Origin 'null' is not allowed");
	});

	test("CORS wildcard origin", async () => {
		const app = createAppCorsCorsWildcardOrigin();
		const client = new TestClient(app);

		const headers = {
			"Origin": "https://random-site.com",
		};
		const response = await client.get("/public/data", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("data");
		expect(responseData["data"]).toBe("public");
	});

	test("CORS request blocked", async () => {
		const app = createAppCorsCorsRequestBlocked();
		const client = new TestClient(app);

		const headers = {
			"Origin": "https://malicious-site.com",
		};
		const response = await client.get("/items/", headers);

		expect(response.statusCode).toBe(403);
		const responseData = response.json();
		expect(responseData).toHaveProperty("detail");
		expect(responseData["detail"]).toBe("CORS request from origin 'https://malicious-site.com' not allowed");
	});

	test("Simple CORS request", async () => {
		const app = createAppCorsSimpleCorsRequest();
		const client = new TestClient(app);

		const headers = {
			"Origin": "https://example.com",
		};
		const response = await client.get("/items/", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("items");
		expect(responseData["items"].length).toBe(0);
	});

	test("09_cors_expose_headers", async () => {
		const app = createAppCors09CorsExposeHeaders();
		const client = new TestClient(app);

		const headers = {
			"Origin": "https://example.com",
		};
		const response = await client.get("/api/data", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
	});

	test("06_cors_preflight_method_not_allowed", async () => {
		const app = createAppCors06CorsPreflightMethodNotAllowed();
		const client = new TestClient(app);

		const headers = {
			"Origin": "https://example.com",
			"Access-Control-Request-Method": "DELETE",
			"Access-Control-Request-Headers": "Content-Type",
		};
		const response = await client.options("/api/data", {headers});

		expect(response.statusCode).toBe(403);
	});

});
