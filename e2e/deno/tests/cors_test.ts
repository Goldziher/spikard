/**
 * E2E tests for cors
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
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

	Deno.test("cors: 07_cors_preflight_header_not_allowed", async () => {
		const app = createAppCors07CorsPreflightHeaderNotAllowed();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://example.com",
			"Access-Control-Request-Method": "POST",
			"Access-Control-Request-Headers": "X-Custom-Header",
		};
		const response = await client.options("/api/data", { headers });

		assertEquals(response.statusCode, 403);
	});

	Deno.test("cors: CORS Vary header for proper caching", async () => {
		const app = createAppCorsCorsVaryHeaderForProperCaching();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://app.example.com",
			"Cache-Control": "max-age=3600",
		};
		const response = await client.get("/api/cached-resource", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "data"));
		assertEquals(responseData.data, "cacheable resource");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders.vary, "Origin");
		assertEquals(responseHeaders["cache-control"], "public, max-age=3600");
		assertEquals(responseHeaders["access-control-allow-origin"], "https://app.example.com");
	});

	Deno.test("cors: CORS preflight for PUT method", async () => {
		const app = createAppCorsCorsPreflightForPutMethod();
		const client = new TestClient(app);

		const headers = {
			"Access-Control-Request-Method": "PUT",
			"Access-Control-Request-Headers": "Content-Type, X-Custom-Header",
			Origin: "https://app.example.com",
		};
		const response = await client.options("/api/resource/123", { headers });

		assertEquals(response.statusCode, 204);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["access-control-allow-origin"], "https://app.example.com");
		assertEquals(responseHeaders["access-control-max-age"], "3600");
		assertEquals(responseHeaders.vary, "Origin");
		assertEquals(responseHeaders["access-control-allow-headers"], "Content-Type, X-Custom-Header");
		assertEquals(responseHeaders["access-control-allow-methods"], "GET, POST, PUT, PATCH, DELETE");
	});

	Deno.test("cors: CORS preflight for DELETE method", async () => {
		const app = createAppCorsCorsPreflightForDeleteMethod();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://app.example.com",
			"Access-Control-Request-Method": "DELETE",
		};
		const response = await client.options("/api/resource/456", { headers });

		assertEquals(response.statusCode, 204);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders.vary, "Origin");
		assertEquals(responseHeaders["access-control-allow-methods"], "GET, POST, PUT, PATCH, DELETE");
		assertEquals(responseHeaders["access-control-allow-origin"], "https://app.example.com");
		assertEquals(responseHeaders["access-control-max-age"], "3600");
	});

	Deno.test("cors: CORS multiple allowed origins", async () => {
		const app = createAppCorsCorsMultipleAllowedOrigins();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://admin.example.com",
		};
		const response = await client.get("/api/data", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "data"));
		assertEquals(responseData.data, "resource data");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["access-control-allow-origin"], "https://admin.example.com");
		assertEquals(responseHeaders.vary, "Origin");
	});

	Deno.test("cors: CORS preflight request", async () => {
		const app = createAppCorsCorsPreflightRequest();
		const client = new TestClient(app);

		const headers = {
			"Access-Control-Request-Method": "POST",
			Origin: "https://example.com",
			"Access-Control-Request-Headers": "Content-Type, X-Custom-Header",
		};
		const response = await client.options("/items/", { headers });

		assertEquals(response.statusCode, 200);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["access-control-max-age"], "600");
		assertEquals(responseHeaders["access-control-allow-headers"], "Content-Type, X-Custom-Header");
		assertEquals(responseHeaders["access-control-allow-origin"], "https://example.com");
		assertEquals(responseHeaders["access-control-allow-methods"], "GET, POST, PUT, DELETE, OPTIONS");
	});

	Deno.test("cors: CORS with credentials", async () => {
		const app = createAppCorsCorsWithCredentials();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://app.example.com",
			Cookie: "session=abc123",
		};
		const response = await client.get("/api/user/profile", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "username"));
		assertEquals(responseData.username, "john");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders.vary, "Origin");
		assertEquals(responseHeaders["access-control-allow-origin"], "https://app.example.com");
		assertEquals(responseHeaders["access-control-allow-credentials"], "true");
	});

	Deno.test("cors: CORS regex pattern matching for origins", async () => {
		const app = createAppCorsCorsRegexPatternMatchingForOrigins();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://subdomain.example.com",
		};
		const response = await client.get("/api/data", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "data"));
		assertEquals(responseData.data, "resource data");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["access-control-allow-origin"], "https://subdomain.example.com");
		assertEquals(responseHeaders.vary, "Origin");
	});

	Deno.test("cors: 08_cors_max_age", async () => {
		const app = createAppCors08CorsMaxAge();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://example.com",
			"Access-Control-Request-Method": "POST",
			"Access-Control-Request-Headers": "Content-Type",
		};
		const response = await client.options("/api/data", { headers });

		assertEquals(response.statusCode, 204);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["access-control-allow-methods"], "POST");
		assertEquals(responseHeaders["access-control-max-age"], "3600");
		assertEquals(responseHeaders["access-control-allow-origin"], "https://example.com");
		assertEquals(responseHeaders["access-control-allow-headers"], "Content-Type");
	});

	Deno.test("cors: 10_cors_origin_null", async () => {
		const app = createAppCors10CorsOriginNull();
		const client = new TestClient(app);

		const headers = {
			Origin: "null",
		};
		const response = await client.get("/api/data", headers);

		assertEquals(response.statusCode, 403);
	});

	Deno.test("cors: CORS wildcard origin", async () => {
		const app = createAppCorsCorsWildcardOrigin();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://random-site.com",
		};
		const response = await client.get("/public/data", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "data"));
		assertEquals(responseData.data, "public");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["access-control-allow-origin"], "*");
	});

	Deno.test("cors: CORS safelisted headers without preflight", async () => {
		const app = createAppCorsCorsSafelistedHeadersWithoutPreflight();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://app.example.com",
			"Content-Type": "text/plain",
			Accept: "application/json",
			"Accept-Language": "en-US",
		};
		const response = await client.post("/api/form", { headers });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Success");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders.vary, "Origin");
		assertEquals(responseHeaders["access-control-allow-origin"], "https://app.example.com");
	});

	Deno.test("cors: CORS Private Network Access", async () => {
		const app = createAppCorsCorsPrivateNetworkAccess();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://public.example.com",
			"Access-Control-Request-Method": "GET",
			"Access-Control-Request-Private-Network": "true",
		};
		const response = await client.options("/api/local-resource", { headers });

		assertEquals(response.statusCode, 204);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["access-control-allow-methods"], "GET, POST");
		assertEquals(responseHeaders.vary, "Origin");
		assertEquals(responseHeaders["access-control-allow-origin"], "https://public.example.com");
		assertEquals(responseHeaders["access-control-allow-private-network"], "true");
	});

	Deno.test("cors: CORS origin case sensitivity", async () => {
		const app = createAppCorsCorsOriginCaseSensitivity();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://EXAMPLE.COM",
		};
		const response = await client.get("/api/data", headers);

		assertEquals(response.statusCode, 200);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders.vary, "Origin");
	});

	Deno.test("cors: CORS request blocked", async () => {
		const app = createAppCorsCorsRequestBlocked();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://malicious-site.com",
		};
		const response = await client.get("/items/", headers);

		assertEquals(response.statusCode, 403);
	});

	Deno.test("cors: Simple CORS request", async () => {
		const app = createAppCorsSimpleCorsRequest();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://example.com",
		};
		const response = await client.get("/items/", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "items"));
		assertEquals(responseData.items.length, 0);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["access-control-allow-origin"], "https://example.com");
		assertEquals(responseHeaders.vary, "Origin");
	});

	Deno.test("cors: 09_cors_expose_headers", async () => {
		const app = createAppCors09CorsExposeHeaders();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://example.com",
		};
		const response = await client.get("/api/data", headers);

		assertEquals(response.statusCode, 200);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["x-request-id"], "abc123");
		assertEquals(responseHeaders["access-control-expose-headers"], "X-Total-Count, X-Request-Id");
		assertEquals(responseHeaders["access-control-allow-origin"], "https://example.com");
		assertEquals(responseHeaders["x-total-count"], "42");
	});

	Deno.test("cors: 06_cors_preflight_method_not_allowed", async () => {
		const app = createAppCors06CorsPreflightMethodNotAllowed();
		const client = new TestClient(app);

		const headers = {
			"Access-Control-Request-Headers": "Content-Type",
			Origin: "https://example.com",
			"Access-Control-Request-Method": "DELETE",
		};
		const response = await client.options("/api/data", { headers });

		assertEquals(response.statusCode, 403);
	});