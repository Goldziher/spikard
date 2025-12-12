/**
 * E2E tests for lifecycle_hooks
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
import {
	createAppLifecycleHooksHookExecutionOrder,
	createAppLifecycleHooksMultipleHooksAllPhases,
	createAppLifecycleHooksOnerrorErrorLogging,
	createAppLifecycleHooksOnrequestRequestLogging,
	createAppLifecycleHooksOnresponseResponseTiming,
	createAppLifecycleHooksOnresponseSecurityHeaders,
	createAppLifecycleHooksPrehandlerAuthenticationFailedShortCircuit,
	createAppLifecycleHooksPrehandlerAuthenticationSuccess,
	createAppLifecycleHooksPrehandlerAuthorizationCheck,
	createAppLifecycleHooksPrehandlerAuthorizationForbiddenShortCircuit,
	createAppLifecycleHooksPrevalidationRateLimitExceededShortCircuit,
	createAppLifecycleHooksPrevalidationRateLimiting,
} from "../app/main.ts";

Deno.test("lifecycle_hooks: onResponse - Security Headers", async () => {
	const app = createAppLifecycleHooksOnresponseSecurityHeaders();
	const client = new TestClient(app);

	const response = await client.get("/api/test-security-headers");

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "message"));
	assertEquals(responseData.message, "Response with security headers");
	const responseHeaders = response.headers();
	assertEquals(responseHeaders["x-xss-protection"], "1; mode=block");
	assertEquals(responseHeaders["x-frame-options"], "DENY");
	assertEquals(responseHeaders["x-content-type-options"], "nosniff");
	assertEquals(responseHeaders["strict-transport-security"], "max-age=31536000; includeSubDomains");
});

Deno.test("lifecycle_hooks: preHandler - Authentication Failed Short Circuit", async () => {
	const app = createAppLifecycleHooksPrehandlerAuthenticationFailedShortCircuit();
	const client = new TestClient(app);

	const headers = {
		Authorization: "Bearer invalid-token",
	};
	const response = await client.get("/api/protected-resource-fail", headers);

	assertEquals(response.statusCode, 401);
});

Deno.test("lifecycle_hooks: preHandler - Authorization Check", async () => {
	const app = createAppLifecycleHooksPrehandlerAuthorizationCheck();
	const client = new TestClient(app);

	const headers = {
		Authorization: "Bearer admin-token-67890",
	};
	const response = await client.get("/api/admin-only", headers);

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "message"));
	assertEquals(responseData.message, "Admin access granted");
	assert(Object.hasOwn(responseData, "role"));
	assertEquals(responseData.role, "admin");
	assert(Object.hasOwn(responseData, "user_id"));
	assertEquals(responseData.user_id, "admin-456");
});

Deno.test("lifecycle_hooks: preHandler - Authentication Success", async () => {
	const app = createAppLifecycleHooksPrehandlerAuthenticationSuccess();
	const client = new TestClient(app);

	const headers = {
		Authorization: "Bearer valid-token-12345",
	};
	const response = await client.get("/api/protected-resource", headers);

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "authenticated"));
	assertEquals(responseData.authenticated, true);
	assert(Object.hasOwn(responseData, "message"));
	assertEquals(responseData.message, "Access granted");
	assert(Object.hasOwn(responseData, "user_id"));
	assertEquals(responseData.user_id, "user-123");
});

Deno.test("lifecycle_hooks: preValidation - Rate Limit Exceeded Short Circuit", async () => {
	const app = createAppLifecycleHooksPrevalidationRateLimitExceededShortCircuit();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { data: "test" };
	const response = await client.post("/api/test-rate-limit-exceeded", { headers, json });

	assertEquals(response.statusCode, 429);
	const responseHeaders = response.headers();
	assertEquals(responseHeaders["retry-after"], "60");
});

Deno.test("lifecycle_hooks: onError - Error Logging", async () => {
	const app = createAppLifecycleHooksOnerrorErrorLogging();
	const client = new TestClient(app);

	const response = await client.get("/api/test-error");

	assertEquals(response.statusCode, 500);
	const responseHeaders = response.headers();
	assertEquals(responseHeaders["content-type"], "application/json");
});

Deno.test("lifecycle_hooks: Multiple Hooks - All Phases", async () => {
	const app = createAppLifecycleHooksMultipleHooksAllPhases();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
		Authorization: "Bearer valid-token-12345",
	};
	const json = { action: "update_profile", user_id: "user-123" };
	const response = await client.post("/api/full-lifecycle", { headers, json });

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "action"));
	assertEquals(responseData.action, "update_profile");
	assert(Object.hasOwn(responseData, "message"));
	assertEquals(responseData.message, "Action completed successfully");
	assert(Object.hasOwn(responseData, "request_id"));
	assertEquals(responseData.request_id, ".*");
	assert(Object.hasOwn(responseData, "user_id"));
	assertEquals(responseData.user_id, "user-123");
	const responseHeaders = response.headers();
	assertEquals(responseHeaders["x-frame-options"], "DENY");
	assertEquals(responseHeaders["x-request-id"], ".*");
	assertEquals(responseHeaders["x-content-type-options"], "nosniff");
	assertEquals(responseHeaders["x-response-time"], ".*ms");
});

Deno.test("lifecycle_hooks: Hook Execution Order", async () => {
	const app = createAppLifecycleHooksHookExecutionOrder();
	const client = new TestClient(app);

	const response = await client.get("/api/test-hook-order");

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "execution_order"));
	assertEquals(responseData.execution_order.length, 3);
	assertEquals(responseData.execution_order[0], "first_hook");
	assertEquals(responseData.execution_order[1], "second_hook");
	assertEquals(responseData.execution_order[2], "third_hook");
	assert(Object.hasOwn(responseData, "message"));
	assertEquals(responseData.message, "Hooks executed in order");
});

Deno.test("lifecycle_hooks: onResponse - Response Timing", async () => {
	const app = createAppLifecycleHooksOnresponseResponseTiming();
	const client = new TestClient(app);

	const response = await client.get("/api/test-timing");

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "message"));
	assertEquals(responseData.message, "Response with timing info");
	const responseHeaders = response.headers();
	assertEquals(responseHeaders["x-response-time"], ".*ms");
});

Deno.test("lifecycle_hooks: preHandler - Authorization Forbidden Short Circuit", async () => {
	const app = createAppLifecycleHooksPrehandlerAuthorizationForbiddenShortCircuit();
	const client = new TestClient(app);

	const headers = {
		Authorization: "Bearer user-token-11111",
	};
	const response = await client.get("/api/admin-only-forbidden", headers);

	assertEquals(response.statusCode, 403);
});

Deno.test("lifecycle_hooks: onRequest - Request Logging", async () => {
	const app = createAppLifecycleHooksOnrequestRequestLogging();
	const client = new TestClient(app);

	const response = await client.get("/api/test-on-request");

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "has_request_id"));
	assertEquals(responseData.has_request_id, true);
	assert(Object.hasOwn(responseData, "message"));
	assertEquals(responseData.message, "onRequest hooks executed");
	assert(Object.hasOwn(responseData, "request_logged"));
	assertEquals(responseData.request_logged, true);
	const responseHeaders = response.headers();
	assertEquals(responseHeaders["x-request-id"], ".*");
});

Deno.test("lifecycle_hooks: preValidation - Rate Limiting", async () => {
	const app = createAppLifecycleHooksPrevalidationRateLimiting();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { data: "test" };
	const response = await client.post("/api/test-rate-limit", { headers, json });

	assertEquals(response.statusCode, 200);
	const responseData = response.json();
	assert(Object.hasOwn(responseData, "message"));
	assertEquals(responseData.message, "Request accepted");
	assert(Object.hasOwn(responseData, "rate_limit_checked"));
	assertEquals(responseData.rate_limit_checked, true);
});
