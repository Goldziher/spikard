/**
 * E2E tests for rate_limit
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { describe, expect, test } from "vitest";
import {
	createAppRateLimitRateLimitBelowThresholdSucceeds,
	createAppRateLimitRateLimitExceededReturns429,
} from "../app/main.ts";

describe("rate_limit", () => {
	test("Rate limit below threshold succeeds", async () => {
		const app = createAppRateLimitRateLimitBelowThresholdSucceeds();
		const client = new TestClient(app);

		const response = await client.get("/rate-limit/basic");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("status");
		expect(responseData.status).toBe("ok");
		expect(responseData).toHaveProperty("request");
		expect(responseData.request).toBe("under-limit");
	});

	test("Rate limit exceeded returns 429", async () => {
		const app = createAppRateLimitRateLimitExceededReturns429();
		const client = new TestClient(app);

		for (let i = 0; i < 1; i += 1) {
			const warmupResponse = await client.get("/rate-limit/exceeded");
			expect(warmupResponse.statusCode).toBe(200);
			await new Promise((resolve) => setTimeout(resolve, 0));
		}

		const response = await client.get("/rate-limit/exceeded");

		expect(response.statusCode).toBe(429);
	});
});
