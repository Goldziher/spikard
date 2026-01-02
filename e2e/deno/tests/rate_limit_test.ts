/**
 * E2E tests for rate_limit
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
import { createAppRateLimitRateLimitBelowThresholdSucceeds, createAppRateLimitRateLimitExceededReturns429 } from "../app/main.ts";

	Deno.test("rate_limit: Rate limit below threshold succeeds", async () => {
		const app = createAppRateLimitRateLimitBelowThresholdSucceeds();
		const client = new TestClient(app);

		const response = await client.get("/rate-limit/basic");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "status"));
		assertEquals(responseData.status, "ok");
		assert(Object.hasOwn(responseData, "request"));
		assertEquals(responseData.request, "under-limit");
	});

	Deno.test("rate_limit: Rate limit exceeded returns 429", async () => {
		const app = createAppRateLimitRateLimitExceededReturns429();
		const client = new TestClient(app);

		for (let i = 0; i < 1; i += 1) {
			const warmupResponse = await client.get("/rate-limit/exceeded");
			assertEquals(warmupResponse.statusCode, 200);
			await new Promise((resolve) => setTimeout(resolve, 0));
		}

		const response = await client.get("/rate-limit/exceeded");

		assertEquals(response.statusCode, 429);
	});