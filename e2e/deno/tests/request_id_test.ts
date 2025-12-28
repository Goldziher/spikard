/**
 * E2E tests for request_id
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
import {
	createAppRequestIdRequestIdHeaderIsPreserved,
	createAppRequestIdRequestIdIsGeneratedWhenNotProvided,
	createAppRequestIdRequestIdMiddlewareCanBeDisabled,
} from "../app/main.ts";

	Deno.test("request_id: Request ID header is preserved", async () => {
		const app = createAppRequestIdRequestIdHeaderIsPreserved();
		const client = new TestClient(app);

		const headers = {
			"X-Request-ID": "trace-123",
		};
		const response = await client.get("/request-id/preserved", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "echo"));
		assertEquals(responseData.echo, "trace-123");
		assert(Object.hasOwn(responseData, "status"));
		assertEquals(responseData.status, "preserved");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["x-request-id"], "trace-123");
	});

	Deno.test("request_id: Request ID middleware can be disabled", async () => {
		const app = createAppRequestIdRequestIdMiddlewareCanBeDisabled();
		const client = new TestClient(app);

		const headers = {
			"X-Request-ID": "external-id",
		};
		const response = await client.get("/request-id/disabled", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "status"));
		assertEquals(responseData.status, "no-request-id");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["x-request-id"], undefined);
	});

	Deno.test("request_id: Request ID is generated when not provided", async () => {
		const app = createAppRequestIdRequestIdIsGeneratedWhenNotProvided();
		const client = new TestClient(app);

		const response = await client.get("/request-id/generated");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "status"));
		assertEquals(responseData.status, "generated");
		const responseHeaders = response.headers();
		assert(/^[0-9a-fA-F-]{36}$/.test(responseHeaders["x-request-id"]));
	});