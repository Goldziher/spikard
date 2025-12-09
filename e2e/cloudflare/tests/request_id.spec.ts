/**
 * E2E tests for request_id
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { describe, expect, test } from "vitest";
import {
	createAppRequestIdRequestIdHeaderIsPreserved,
	createAppRequestIdRequestIdIsGeneratedWhenNotProvided,
	createAppRequestIdRequestIdMiddlewareCanBeDisabled,
} from "../app/main.ts";

describe("request_id", () => {
	test("Request ID header is preserved", async () => {
		const app = createAppRequestIdRequestIdHeaderIsPreserved();
		const client = new TestClient(app);

		const headers = {
			"X-Request-ID": "trace-123",
		};
		const response = await client.get("/request-id/preserved", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("echo");
		expect(responseData.echo).toBe("trace-123");
		expect(responseData).toHaveProperty("status");
		expect(responseData.status).toBe("preserved");
		const responseHeaders = response.headers();
		expect(responseHeaders["x-request-id"]).toBe("trace-123");
	});

	test("Request ID middleware can be disabled", async () => {
		const app = createAppRequestIdRequestIdMiddlewareCanBeDisabled();
		const client = new TestClient(app);

		const headers = {
			"X-Request-ID": "external-id",
		};
		const response = await client.get("/request-id/disabled", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("status");
		expect(responseData.status).toBe("no-request-id");
		const responseHeaders = response.headers();
		expect(responseHeaders["x-request-id"]).toBeUndefined();
	});

	test("Request ID is generated when not provided", async () => {
		const app = createAppRequestIdRequestIdIsGeneratedWhenNotProvided();
		const client = new TestClient(app);

		const response = await client.get("/request-id/generated");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("status");
		expect(responseData.status).toBe("generated");
		const responseHeaders = response.headers();
		expect(responseHeaders["x-request-id"]).toMatch(/^[0-9a-fA-F-]{36}$/);
	});
});
