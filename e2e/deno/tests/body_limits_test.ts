/**
 * E2E tests for body_limits
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
import { createAppBodyLimitsBodyOverLimitReturns413, createAppBodyLimitsBodyUnderLimitSucceeds } from "../app/main.ts";

Deno.test("body_limits: Body under limit succeeds", async () => {
	const app = createAppBodyLimitsBodyUnderLimitSucceeds();
	const client = new TestClient(app);

	const json = { note: "small" };
	const response = await client.post("/body-limit/under", { json });

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "accepted"));
	assertEquals(responseData.accepted, true);
	assert(Object.hasOwn(responseData, "note"));
	assertEquals(responseData.note, "small");
});

Deno.test("body_limits: Body over limit returns 413", async () => {
	const app = createAppBodyLimitsBodyOverLimitReturns413();
	const client = new TestClient(app);

	const json = {
		note: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
	};
	const response = await client.post("/body-limit/over", { json });

	assertEquals(response.statusCode, 413);
});
