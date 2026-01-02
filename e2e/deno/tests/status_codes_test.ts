/**
 * E2E tests for status_codes
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
import {
	createAppStatusCodes19413PayloadTooLarge,
	createAppStatusCodes200OkSuccess,
	createAppStatusCodes201CreatedResourceCreated,
	createAppStatusCodes202AcceptedRequestAcceptedForProcessing,
	createAppStatusCodes20414UriTooLong,
	createAppStatusCodes204NoContentSuccessWithNoBody,
	createAppStatusCodes206PartialContent,
	createAppStatusCodes21431RequestHeaderFieldsTooLarge,
	createAppStatusCodes22501NotImplemented,
	createAppStatusCodes23503ServiceUnavailable,
	createAppStatusCodes301MovedPermanentlyPermanentRedirect,
	createAppStatusCodes302FoundTemporaryRedirect,
	createAppStatusCodes304NotModifiedCachedContentValid,
	createAppStatusCodes307TemporaryRedirectMethodPreserved,
	createAppStatusCodes400BadRequestInvalidRequest,
	createAppStatusCodes401UnauthorizedMissingAuthentication,
	createAppStatusCodes403ForbiddenInsufficientPermissions,
	createAppStatusCodes404NotFoundResourceNotFound,
	createAppStatusCodes408RequestTimeout,
	createAppStatusCodes422UnprocessableEntityValidationError,
	createAppStatusCodes429TooManyRequests,
	createAppStatusCodes500InternalServerErrorServerError,
	createAppStatusCodes503ServiceUnavailableServerOverload,
} from "../app/main.ts";

	Deno.test("status_codes: 408 Request Timeout", async () => {
		const app = createAppStatusCodes408RequestTimeout();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { data: "large_data" };
		const response = await client.post("/slow-endpoint", { headers, json });

		assertEquals(response.statusCode, 408);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders.connection, "close");
	});

	Deno.test("status_codes: 404 Not Found - Resource not found", async () => {
		const app = createAppStatusCodes404NotFoundResourceNotFound();
		const client = new TestClient(app);

		const response = await client.get("/status-test/404");

		assertEquals(response.statusCode, 404);
	});

	Deno.test("status_codes: 503 Service Unavailable - Server overload", async () => {
		const app = createAppStatusCodes503ServiceUnavailableServerOverload();
		const client = new TestClient(app);

		const response = await client.get("/health");

		assertEquals(response.statusCode, 503);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["retry-after"], "120");
	});

	Deno.test("status_codes: 422 Unprocessable Entity - Validation error", async () => {
		const app = createAppStatusCodes422UnprocessableEntityValidationError();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { price: "not a number" };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("status_codes: 302 Found - Temporary redirect", async () => {
		const app = createAppStatusCodes302FoundTemporaryRedirect();
		const client = new TestClient(app);

		const response = await client.get("/temp-redirect");

		assertEquals(response.statusCode, 302);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders.location, "/target-path");
	});

	Deno.test("status_codes: 304 Not Modified - Cached content valid", async () => {
		const app = createAppStatusCodes304NotModifiedCachedContentValid();
		const client = new TestClient(app);

		const headers = {
			"If-None-Match": "\"abc123\"",
		};
		const response = await client.get("/status-test/304", headers);

		assertEquals(response.statusCode, 304);
	});

	Deno.test("status_codes: 400 Bad Request - Invalid request", async () => {
		const app = createAppStatusCodes400BadRequestInvalidRequest();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = "not valid json";
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 400);
	});

	Deno.test("status_codes: 22_501_not_implemented", async () => {
		const app = createAppStatusCodes22501NotImplemented();
		const client = new TestClient(app);

		const response = await client.trace("/data");

		assertEquals(response.statusCode, 405);
	});

	Deno.test("status_codes: 204 No Content - Success with no body", async () => {
		const app = createAppStatusCodes204NoContentSuccessWithNoBody();
		const client = new TestClient(app);

		const response = await client.delete("/status-test/204");

		assertEquals(response.statusCode, 204);
	});

	Deno.test("status_codes: 301 Moved Permanently - Permanent redirect", async () => {
		const app = createAppStatusCodes301MovedPermanentlyPermanentRedirect();
		const client = new TestClient(app);

		const response = await client.get("/old-path");

		assertEquals(response.statusCode, 301);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders.location, "/new-path");
	});

	Deno.test("status_codes: 201 Created - Resource created", async () => {
		const app = createAppStatusCodes201CreatedResourceCreated();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "New Item" };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 201);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "id"));
		assertEquals(responseData.id, 1);
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "New Item");
	});

	Deno.test("status_codes: 202 Accepted - Request accepted for processing", async () => {
		const app = createAppStatusCodes202AcceptedRequestAcceptedForProcessing();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { task: "process_data" };
		const response = await client.post("/tasks/", { headers, json });

		assertEquals(response.statusCode, 202);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Task accepted for processing");
		assert(Object.hasOwn(responseData, "task_id"));
		assertEquals(responseData.task_id, "abc123");
	});

	Deno.test("status_codes: 307 Temporary Redirect - Method preserved", async () => {
		const app = createAppStatusCodes307TemporaryRedirectMethodPreserved();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {  };
		const response = await client.post("/redirect-post", { headers, json });

		assertEquals(response.statusCode, 307);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders.location, "/target-post");
	});

	Deno.test("status_codes: 500 Internal Server Error - Server error", async () => {
		const app = createAppStatusCodes500InternalServerErrorServerError();
		const client = new TestClient(app);

		const response = await client.get("/error");

		assertEquals(response.statusCode, 500);
	});

	Deno.test("status_codes: 20_414_uri_too_long", async () => {
		const app = createAppStatusCodes20414UriTooLong();
		const client = new TestClient(app);

		const response = await client.get("/data?skip_template_expansion=true");

		assertEquals(response.statusCode, 200);
	});

	Deno.test("status_codes: 401 Unauthorized - Missing authentication", async () => {
		const app = createAppStatusCodes401UnauthorizedMissingAuthentication();
		const client = new TestClient(app);

		const response = await client.get("/users/me");

		assertEquals(response.statusCode, 401);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["www-authenticate"], "Bearer");
	});

	Deno.test("status_codes: 23_503_service_unavailable", async () => {
		const app = createAppStatusCodes23503ServiceUnavailable();
		const client = new TestClient(app);

		const response = await client.get("/data");

		assertEquals(response.statusCode, 503);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["retry-after"], "60");
	});

	Deno.test("status_codes: 19_413_payload_too_large", async () => {
		const app = createAppStatusCodes19413PayloadTooLarge();
		const client = new TestClient(app);

		const json = { data: "{{ repeat 'x' 2000 times }}" };
		const response = await client.post("/upload", { json });

		assertEquals(response.statusCode, 413);
	});

	Deno.test("status_codes: 403 Forbidden - Insufficient permissions", async () => {
		const app = createAppStatusCodes403ForbiddenInsufficientPermissions();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer valid_token",
		};
		const response = await client.get("/admin/users", headers);

		assertEquals(response.statusCode, 403);
	});

	Deno.test("status_codes: 21_431_request_header_fields_too_large", async () => {
		const app = createAppStatusCodes21431RequestHeaderFieldsTooLarge();
		const client = new TestClient(app);

		const headers = {
			"X-Large-Header": "{{ repeat 'x' 10000 times }}",
		};
		const response = await client.get("/data", headers);

		assertEquals(response.statusCode, 431);
	});

	Deno.test("status_codes: 429 Too Many Requests", async () => {
		const app = createAppStatusCodes429TooManyRequests();
		const client = new TestClient(app);

		const response = await client.get("/api/resource");

		assertEquals(response.statusCode, 429);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["x-ratelimit-reset"], "1609459200");
		assertEquals(responseHeaders["x-ratelimit-limit"], "100");
		assertEquals(responseHeaders["x-ratelimit-remaining"], "0");
		assertEquals(responseHeaders["retry-after"], "60");
	});

	Deno.test("status_codes: 200 OK - Success", async () => {
		const app = createAppStatusCodes200OkSuccess();
		const client = new TestClient(app);

		const response = await client.get("/status-test/200");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "id"));
		assertEquals(responseData.id, 1);
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Item 1");
	});

	Deno.test("status_codes: 206 Partial Content", async () => {
		const app = createAppStatusCodes206PartialContent();
		const client = new TestClient(app);

		const headers = {
			Range: "bytes=0-1023",
		};
		const response = await client.get("/files/document.pdf", headers);

		assertEquals(response.statusCode, 206);
		const bodyBytes = response.bytes();
		assertEquals(bodyBytes.length, 1024);
		assertEquals(bodyBytes.toString("utf-8").startsWith("binary_data_1024_bytes"), true);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["content-range"], "bytes 0-1023/5000");
		assertEquals(responseHeaders["content-length"], "1024");
		assertEquals(responseHeaders["content-type"], "application/pdf");
		assertEquals(responseHeaders["accept-ranges"], "bytes");
	});