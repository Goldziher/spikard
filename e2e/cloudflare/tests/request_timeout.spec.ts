/**
 * E2E tests for request_timeout
 * @generated
 */

import { describe, expect, test } from "vitest";
import { TestClient } from "../../packages/wasm/src/index.ts";
import {
	createAppRequestTimeoutRequestCompletesBeforeTimeout,
	createAppRequestTimeoutRequestExceedsTimeout,
} from "../app/main.ts";

describe("request_timeout", () => {
	test("Request exceeds timeout", async () => {
		const app = createAppRequestTimeoutRequestExceedsTimeout();
		const client = new TestClient(app);

		const response = await client.get("/timeouts/slow");

		expect(response.statusCode).toBe(408);
	});

	test("Request completes before timeout", async () => {
		const app = createAppRequestTimeoutRequestCompletesBeforeTimeout();
		const client = new TestClient(app);

		const response = await client.get("/timeouts/fast");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("duration");
		expect(responseData.duration).toBe("fast");
		expect(responseData).toHaveProperty("status");
		expect(responseData.status).toBe("ok");
	});
});
