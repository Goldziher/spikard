/**
 * E2E tests for request_timeout
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assertEquals } from "@std/assert";
import {
	createAppRequestTimeoutRequestCompletesBeforeTimeout,
	createAppRequestTimeoutRequestExceedsTimeout,
} from "../app/main.ts";
Deno.test("request_timeout: Request exceeds timeout", async () => {
	const app = createAppRequestTimeoutRequestExceedsTimeout();
	const client = new TestClient(app);

	const response = await client.get("/timeouts/slow");

	assertEquals(response.statusCode, 408);
});

Deno.test("request_timeout: Request completes before timeout", async () => {
	const app = createAppRequestTimeoutRequestCompletesBeforeTimeout();
	const client = new TestClient(app);

	const response = await client.get("/timeouts/fast");

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	expect(responseData).toHaveProperty("duration");
	expect(responseData.duration).toBe("fast");
	expect(responseData).toHaveProperty("status");
	expect(responseData.status).toBe("ok");
});
