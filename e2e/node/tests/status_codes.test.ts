/**
 * E2E tests for status_codes
 * @generated
 */

import { describe, test, expect } from "vitest";
import { TestClient } from "@spikard/node";
import { createAppStatusCodes408RequestTimeout, createAppStatusCodes404NotFoundResourceNotFound, createAppStatusCodes503ServiceUnavailableServerOverload, createAppStatusCodes422UnprocessableEntityValidationError, createAppStatusCodes302FoundTemporaryRedirect, createAppStatusCodes304NotModifiedCachedContentValid, createAppStatusCodes400BadRequestInvalidRequest, createAppStatusCodes22501NotImplemented, createAppStatusCodes204NoContentSuccessWithNoBody, createAppStatusCodes301MovedPermanentlyPermanentRedirect, createAppStatusCodes201CreatedResourceCreated, createAppStatusCodes202AcceptedRequestAcceptedForProcessing, createAppStatusCodes307TemporaryRedirectMethodPreserved, createAppStatusCodes500InternalServerErrorServerError, createAppStatusCodes20414UriTooLong, createAppStatusCodes401UnauthorizedMissingAuthentication, createAppStatusCodes23503ServiceUnavailable, createAppStatusCodes19413PayloadTooLarge, createAppStatusCodes403ForbiddenInsufficientPermissions, createAppStatusCodes21431RequestHeaderFieldsTooLarge, createAppStatusCodes429TooManyRequests, createAppStatusCodes200OkSuccess, createAppStatusCodes206PartialContent } from "../app/main.js";

describe("status_codes", () => {
	test("408 Request Timeout", async () => {
		const app = createAppStatusCodes408RequestTimeout();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {"data":"large_data"};
		const response = await client.post("/slow-endpoint", {headers, json});

		expect(response.statusCode).toBe(408);
		const responseData = response.json();
		expect(responseData).toHaveProperty("detail");
		expect(responseData["detail"]).toBe("Request timeout");
	});

	test("404 Not Found - Resource not found", async () => {
		const app = createAppStatusCodes404NotFoundResourceNotFound();
		const client = new TestClient(app);

		const response = await client.get("/status-test/404");

		expect(response.statusCode).toBe(404);
		const responseData = response.json();
		expect(responseData).toHaveProperty("detail");
		expect(responseData["detail"]).toBe("Item not found");
	});

	test("503 Service Unavailable - Server overload", async () => {
		const app = createAppStatusCodes503ServiceUnavailableServerOverload();
		const client = new TestClient(app);

		const response = await client.get("/health");

		expect(response.statusCode).toBe(503);
		const responseData = response.json();
		expect(responseData).toHaveProperty("detail");
		expect(responseData["detail"]).toBe("Service temporarily unavailable");
	});

	test("422 Unprocessable Entity - Validation error", async () => {
		const app = createAppStatusCodes422UnprocessableEntityValidationError();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {"price":"not a number"};
		const response = await client.post("/items/", {headers, json});

		expect(response.statusCode).toBe(422);
		const responseData = response.json();
		// Validation should be done by framework, not handler
		expect(responseData).toHaveProperty("errors");
	});

	test("302 Found - Temporary redirect", async () => {
		const app = createAppStatusCodes302FoundTemporaryRedirect();
		const client = new TestClient(app);

		const response = await client.get("/temp-redirect");

		expect(response.statusCode).toBe(302);
	});

	test("304 Not Modified - Cached content valid", async () => {
		const app = createAppStatusCodes304NotModifiedCachedContentValid();
		const client = new TestClient(app);

		const headers = {
			"If-None-Match": "\"abc123\"",
		};
		const response = await client.get("/status-test/304", headers);

		expect(response.statusCode).toBe(304);
	});

	test("400 Bad Request - Invalid request", async () => {
		const app = createAppStatusCodes400BadRequestInvalidRequest();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = "not valid json";
		const response = await client.post("/items/", {headers, json});

		expect(response.statusCode).toBe(400);
		const responseData = response.json();
		expect(responseData).toHaveProperty("detail");
		expect(responseData["detail"]).toBe("Invalid request format");
	});

	test("22_501_not_implemented", async () => {
		const app = createAppStatusCodes22501NotImplemented();
		const client = new TestClient(app);

		const response = await client.trace("/data", {});

		expect(response.statusCode).toBe(405);
	});

	test("204 No Content - Success with no body", async () => {
		const app = createAppStatusCodes204NoContentSuccessWithNoBody();
		const client = new TestClient(app);

		const response = await client.delete("/status-test/204");

		expect(response.statusCode).toBe(204);
	});

	test("301 Moved Permanently - Permanent redirect", async () => {
		const app = createAppStatusCodes301MovedPermanentlyPermanentRedirect();
		const client = new TestClient(app);

		const response = await client.get("/old-path");

		expect(response.statusCode).toBe(301);
	});

	test("201 Created - Resource created", async () => {
		const app = createAppStatusCodes201CreatedResourceCreated();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {"name":"New Item"};
		const response = await client.post("/items/", {headers, json});

		expect(response.statusCode).toBe(201);
		const responseData = response.json();
		expect(responseData).toHaveProperty("id");
		expect(responseData["id"]).toBe(1);
		expect(responseData).toHaveProperty("name");
		expect(responseData["name"]).toBe("New Item");
	});

	test("202 Accepted - Request accepted for processing", async () => {
		const app = createAppStatusCodes202AcceptedRequestAcceptedForProcessing();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {"task":"process_data"};
		const response = await client.post("/tasks/", {headers, json});

		expect(response.statusCode).toBe(202);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData["message"]).toBe("Task accepted for processing");
		expect(responseData).toHaveProperty("task_id");
		expect(responseData["task_id"]).toBe("abc123");
	});

	test("307 Temporary Redirect - Method preserved", async () => {
		const app = createAppStatusCodes307TemporaryRedirectMethodPreserved();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {};
		const response = await client.post("/redirect-post", {headers, json});

		expect(response.statusCode).toBe(307);
		const responseData = response.json();
	});

	test("500 Internal Server Error - Server error", async () => {
		const app = createAppStatusCodes500InternalServerErrorServerError();
		const client = new TestClient(app);

		const response = await client.get("/error");

		expect(response.statusCode).toBe(500);
		const responseData = response.json();
		expect(responseData).toHaveProperty("detail");
		expect(responseData["detail"]).toBe("Internal server error");
		expect(responseData).toHaveProperty("status");
		expect(responseData["status"]).toBe(500);
		expect(responseData).toHaveProperty("title");
		expect(responseData["title"]).toBe("Internal Server Error");
		expect(responseData).toHaveProperty("type");
		expect(responseData["type"]).toBe("https://spikard.dev/errors/internal-server-error");
	});

	test("20_414_uri_too_long", async () => {
		const app = createAppStatusCodes20414UriTooLong();
		const client = new TestClient(app);

		const response = await client.get("/data?skip_template_expansion=true");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
	});

	test("401 Unauthorized - Missing authentication", async () => {
		const app = createAppStatusCodes401UnauthorizedMissingAuthentication();
		const client = new TestClient(app);

		const response = await client.get("/users/me");

		expect(response.statusCode).toBe(401);
		const responseData = response.json();
		expect(responseData).toHaveProperty("detail");
		expect(responseData["detail"]).toBe("Not authenticated");
	});

	test("23_503_service_unavailable", async () => {
		const app = createAppStatusCodes23503ServiceUnavailable();
		const client = new TestClient(app);

		const response = await client.get("/data");

		expect(response.statusCode).toBe(503);
		const responseData = response.json();
		expect(responseData).toHaveProperty("error");
		expect(responseData["error"]).toBe("Service Unavailable");
		expect(responseData).toHaveProperty("message");
		expect(responseData["message"]).toBe("The service is temporarily unavailable. Please try again later.");
	});

	test("19_413_payload_too_large", async () => {
		const app = createAppStatusCodes19413PayloadTooLarge();
		const client = new TestClient(app);

		const json = {"data":"{{ repeat 'x' 2000 times }}"};
		const response = await client.post("/upload", {json});

		expect(response.statusCode).toBe(413);
		const responseData = response.json();
		expect(responseData).toHaveProperty("error");
		expect(responseData["error"]).toBe("Payload Too Large");
		expect(responseData).toHaveProperty("message");
		expect(responseData["message"]).toBe("Request body size exceeds maximum allowed size of 1024 bytes");
	});

	test("403 Forbidden - Insufficient permissions", async () => {
		const app = createAppStatusCodes403ForbiddenInsufficientPermissions();
		const client = new TestClient(app);

		const headers = {
			"Authorization": "Bearer valid_token",
		};
		const response = await client.get("/admin/users", headers);

		expect(response.statusCode).toBe(403);
		const responseData = response.json();
		expect(responseData).toHaveProperty("detail");
		expect(responseData["detail"]).toBe("Not enough permissions");
	});

	test("21_431_request_header_fields_too_large", async () => {
		const app = createAppStatusCodes21431RequestHeaderFieldsTooLarge();
		const client = new TestClient(app);

		const headers = {
			"X-Large-Header": "{{ repeat 'x' 10000 times }}",
		};
		const response = await client.get("/data", headers);

		expect(response.statusCode).toBe(431);
		const responseData = response.json();
		expect(responseData).toHaveProperty("error");
		expect(responseData["error"]).toBe("Request Header Fields Too Large");
		expect(responseData).toHaveProperty("message");
		expect(responseData["message"]).toBe("Request headers exceed maximum allowed size of 8192 bytes");
	});

	test("429 Too Many Requests", async () => {
		const app = createAppStatusCodes429TooManyRequests();
		const client = new TestClient(app);

		const response = await client.get("/api/resource");

		expect(response.statusCode).toBe(429);
		const responseData = response.json();
		expect(responseData).toHaveProperty("detail");
		expect(responseData["detail"]).toBe("Rate limit exceeded. Try again in 60 seconds.");
	});

	test("200 OK - Success", async () => {
		const app = createAppStatusCodes200OkSuccess();
		const client = new TestClient(app);

		const response = await client.get("/status-test/200");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("id");
		expect(responseData["id"]).toBe(1);
		expect(responseData).toHaveProperty("name");
		expect(responseData["name"]).toBe("Item 1");
	});

	test("206 Partial Content", async () => {
		const app = createAppStatusCodes206PartialContent();
		const client = new TestClient(app);

		const headers = {
			"Range": "bytes=0-1023",
		};
		const response = await client.get("/files/document.pdf", headers);

		expect(response.statusCode).toBe(206);
		const responseData = response.json();
		expect(responseData).toBe("binary_data_1024_bytes");
	});

});
