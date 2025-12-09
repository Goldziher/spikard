/**
 * E2E tests for cors
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { describe, expect, test } from "vitest";
import {
	createAppCors06CorsPreflightMethodNotAllowed,
	createAppCors07CorsPreflightHeaderNotAllowed,
	createAppCors08CorsMaxAge,
	createAppCors09CorsExposeHeaders,
	createAppCors10CorsOriginNull,
	createAppCorsCorsMultipleAllowedOrigins,
	createAppCorsCorsOriginCaseSensitivity,
	createAppCorsCorsPreflightForDeleteMethod,
	createAppCorsCorsPreflightForPutMethod,
	createAppCorsCorsPreflightRequest,
	createAppCorsCorsPrivateNetworkAccess,
	createAppCorsCorsRegexPatternMatchingForOrigins,
	createAppCorsCorsRequestBlocked,
	createAppCorsCorsSafelistedHeadersWithoutPreflight,
	createAppCorsCorsVaryHeaderForProperCaching,
	createAppCorsCorsWildcardOrigin,
	createAppCorsCorsWithCredentials,
	createAppCorsSimpleCorsRequest,
} from "../app/main.ts";

describe("cors", () => {
	test("07_cors_preflight_header_not_allowed", async () => {
		const app = createAppCors07CorsPreflightHeaderNotAllowed();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://example.com",
			"Access-Control-Request-Method": "POST",
			"Access-Control-Request-Headers": "X-Custom-Header",
		};
		const response = await client.options("/api/data", { headers });

		expect(response.statusCode).toBe(403);
	});

	test("CORS Vary header for proper caching", async () => {
		const app = createAppCorsCorsVaryHeaderForProperCaching();
		const client = new TestClient(app);

		const headers = {
			"Cache-Control": "max-age=3600",
			Origin: "https://app.example.com",
		};
		const response = await client.get("/api/cached-resource", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("data");
		expect(responseData.data).toBe("cacheable resource");
		const responseHeaders = response.headers();
		expect(responseHeaders["cache-control"]).toBe("public, max-age=3600");
		expect(responseHeaders["access-control-allow-origin"]).toBe("https://app.example.com");
		expect(responseHeaders.vary).toBe("Origin");
	});

	test("CORS preflight for PUT method", async () => {
		const app = createAppCorsCorsPreflightForPutMethod();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://app.example.com",
			"Access-Control-Request-Headers": "Content-Type, X-Custom-Header",
			"Access-Control-Request-Method": "PUT",
		};
		const response = await client.options("/api/resource/123", { headers });

		expect(response.statusCode).toBe(204);
		const responseHeaders = response.headers();
		expect(responseHeaders["access-control-allow-methods"]).toBe("GET, POST, PUT, PATCH, DELETE");
		expect(responseHeaders["access-control-max-age"]).toBe("3600");
		expect(responseHeaders["access-control-allow-origin"]).toBe("https://app.example.com");
		expect(responseHeaders.vary).toBe("Origin");
		expect(responseHeaders["access-control-allow-headers"]).toBe("Content-Type, X-Custom-Header");
	});

	test("CORS preflight for DELETE method", async () => {
		const app = createAppCorsCorsPreflightForDeleteMethod();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://app.example.com",
			"Access-Control-Request-Method": "DELETE",
		};
		const response = await client.options("/api/resource/456", { headers });

		expect(response.statusCode).toBe(204);
		const responseHeaders = response.headers();
		expect(responseHeaders["access-control-max-age"]).toBe("3600");
		expect(responseHeaders["access-control-allow-origin"]).toBe("https://app.example.com");
		expect(responseHeaders.vary).toBe("Origin");
		expect(responseHeaders["access-control-allow-methods"]).toBe("GET, POST, PUT, PATCH, DELETE");
	});

	test("CORS multiple allowed origins", async () => {
		const app = createAppCorsCorsMultipleAllowedOrigins();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://admin.example.com",
		};
		const response = await client.get("/api/data", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("data");
		expect(responseData.data).toBe("resource data");
		const responseHeaders = response.headers();
		expect(responseHeaders.vary).toBe("Origin");
		expect(responseHeaders["access-control-allow-origin"]).toBe("https://admin.example.com");
	});

	test("CORS preflight request", async () => {
		const app = createAppCorsCorsPreflightRequest();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://example.com",
			"Access-Control-Request-Method": "POST",
			"Access-Control-Request-Headers": "Content-Type, X-Custom-Header",
		};
		const response = await client.options("/items/", { headers });

		expect(response.statusCode).toBe(200);
		const responseHeaders = response.headers();
		expect(responseHeaders["access-control-max-age"]).toBe("600");
		expect(responseHeaders["access-control-allow-methods"]).toBe("GET, POST, PUT, DELETE, OPTIONS");
		expect(responseHeaders["access-control-allow-headers"]).toBe("Content-Type, X-Custom-Header");
		expect(responseHeaders["access-control-allow-origin"]).toBe("https://example.com");
	});

	test("CORS with credentials", async () => {
		const app = createAppCorsCorsWithCredentials();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://app.example.com",
			Cookie: "session=abc123",
		};
		const response = await client.get("/api/user/profile", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("username");
		expect(responseData.username).toBe("john");
		const responseHeaders = response.headers();
		expect(responseHeaders.vary).toBe("Origin");
		expect(responseHeaders["access-control-allow-origin"]).toBe("https://app.example.com");
		expect(responseHeaders["access-control-allow-credentials"]).toBe("true");
	});

	test("CORS regex pattern matching for origins", async () => {
		const app = createAppCorsCorsRegexPatternMatchingForOrigins();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://subdomain.example.com",
		};
		const response = await client.get("/api/data", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("data");
		expect(responseData.data).toBe("resource data");
		const responseHeaders = response.headers();
		expect(responseHeaders.vary).toBe("Origin");
		expect(responseHeaders["access-control-allow-origin"]).toBe("https://subdomain.example.com");
	});

	test("08_cors_max_age", async () => {
		const app = createAppCors08CorsMaxAge();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://example.com",
			"Access-Control-Request-Method": "POST",
			"Access-Control-Request-Headers": "Content-Type",
		};
		const response = await client.options("/api/data", { headers });

		expect(response.statusCode).toBe(204);
		const responseHeaders = response.headers();
		expect(responseHeaders["access-control-max-age"]).toBe("3600");
		expect(responseHeaders["access-control-allow-headers"]).toBe("Content-Type");
		expect(responseHeaders["access-control-allow-origin"]).toBe("https://example.com");
		expect(responseHeaders["access-control-allow-methods"]).toBe("POST");
	});

	test("10_cors_origin_null", async () => {
		const app = createAppCors10CorsOriginNull();
		const client = new TestClient(app);

		const headers = {
			Origin: "null",
		};
		const response = await client.get("/api/data", headers);

		expect(response.statusCode).toBe(403);
	});

	test("CORS wildcard origin", async () => {
		const app = createAppCorsCorsWildcardOrigin();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://random-site.com",
		};
		const response = await client.get("/public/data", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("data");
		expect(responseData.data).toBe("public");
		const responseHeaders = response.headers();
		expect(responseHeaders["access-control-allow-origin"]).toBe("*");
	});

	test("CORS safelisted headers without preflight", async () => {
		const app = createAppCorsCorsSafelistedHeadersWithoutPreflight();
		const client = new TestClient(app);

		const headers = {
			Accept: "application/json",
			"Content-Type": "text/plain",
			Origin: "https://app.example.com",
			"Accept-Language": "en-US",
		};
		const response = await client.post("/api/form", { headers });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Success");
		const responseHeaders = response.headers();
		expect(responseHeaders["access-control-allow-origin"]).toBe("https://app.example.com");
		expect(responseHeaders.vary).toBe("Origin");
	});

	test("CORS Private Network Access", async () => {
		const app = createAppCorsCorsPrivateNetworkAccess();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://public.example.com",
			"Access-Control-Request-Method": "GET",
			"Access-Control-Request-Private-Network": "true",
		};
		const response = await client.options("/api/local-resource", { headers });

		expect(response.statusCode).toBe(204);
		const responseHeaders = response.headers();
		expect(responseHeaders["access-control-allow-methods"]).toBe("GET, POST");
		expect(responseHeaders["access-control-allow-origin"]).toBe("https://public.example.com");
		expect(responseHeaders.vary).toBe("Origin");
		expect(responseHeaders["access-control-allow-private-network"]).toBe("true");
	});

	test("CORS origin case sensitivity", async () => {
		const app = createAppCorsCorsOriginCaseSensitivity();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://EXAMPLE.COM",
		};
		const response = await client.get("/api/data", headers);

		expect(response.statusCode).toBe(200);
		const responseHeaders = response.headers();
		expect(responseHeaders.vary).toBe("Origin");
	});

	test("CORS request blocked", async () => {
		const app = createAppCorsCorsRequestBlocked();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://malicious-site.com",
		};
		const response = await client.get("/items/", headers);

		expect(response.statusCode).toBe(403);
	});

	test("Simple CORS request", async () => {
		const app = createAppCorsSimpleCorsRequest();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://example.com",
		};
		const response = await client.get("/items/", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("items");
		expect(responseData.items.length).toBe(0);
		const responseHeaders = response.headers();
		expect(responseHeaders["access-control-allow-origin"]).toBe("https://example.com");
		expect(responseHeaders.vary).toBe("Origin");
	});

	test("09_cors_expose_headers", async () => {
		const app = createAppCors09CorsExposeHeaders();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://example.com",
		};
		const response = await client.get("/api/data", headers);

		expect(response.statusCode).toBe(200);
		const responseHeaders = response.headers();
		expect(responseHeaders["x-total-count"]).toBe("42");
		expect(responseHeaders["x-request-id"]).toBe("abc123");
		expect(responseHeaders["access-control-allow-origin"]).toBe("https://example.com");
		expect(responseHeaders["access-control-expose-headers"]).toBe("X-Total-Count, X-Request-Id");
	});

	test("06_cors_preflight_method_not_allowed", async () => {
		const app = createAppCors06CorsPreflightMethodNotAllowed();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://example.com",
			"Access-Control-Request-Method": "DELETE",
			"Access-Control-Request-Headers": "Content-Type",
		};
		const response = await client.options("/api/data", { headers });

		expect(response.statusCode).toBe(403);
	});
});
