/**
 * E2E tests for lifecycle_hooks
 * @generated
 */

import { TestClient } from "@spikard/node";
import { describe, expect, test } from "vitest";
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

describe("lifecycle_hooks", () => {
	test("onResponse - Security Headers", async () => {
		const app = createAppLifecycleHooksOnresponseSecurityHeaders();
		const client = new TestClient(app);

		const response = await client.get("/api/test-security-headers");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Response with security headers");
		const responseHeaders = response.headers();
		expect(responseHeaders["x-content-type-options"]).toBe("nosniff");
		expect(responseHeaders["strict-transport-security"]).toBe("max-age=31536000; includeSubDomains");
		expect(responseHeaders["x-xss-protection"]).toBe("1; mode=block");
		expect(responseHeaders["x-frame-options"]).toBe("DENY");
	});

	test("preHandler - Authentication Failed Short Circuit", async () => {
		const app = createAppLifecycleHooksPrehandlerAuthenticationFailedShortCircuit();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer invalid-token",
		};
		const response = await client.get("/api/protected-resource-fail", headers);

		expect(response.statusCode).toBe(401);
	});

	test("preHandler - Authorization Check", async () => {
		const app = createAppLifecycleHooksPrehandlerAuthorizationCheck();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer admin-token-67890",
		};
		const response = await client.get("/api/admin-only", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Admin access granted");
		expect(responseData).toHaveProperty("role");
		expect(responseData.role).toBe("admin");
		expect(responseData).toHaveProperty("user_id");
		expect(responseData.user_id).toBe("admin-456");
	});

	test("preHandler - Authentication Success", async () => {
		const app = createAppLifecycleHooksPrehandlerAuthenticationSuccess();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer valid-token-12345",
		};
		const response = await client.get("/api/protected-resource", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("authenticated");
		expect(responseData.authenticated).toBe(true);
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Access granted");
		expect(responseData).toHaveProperty("user_id");
		expect(responseData.user_id).toBe("user-123");
	});

	test("preValidation - Rate Limit Exceeded Short Circuit", async () => {
		const app = createAppLifecycleHooksPrevalidationRateLimitExceededShortCircuit();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { data: "test" };
		const response = await client.post("/api/test-rate-limit-exceeded", { headers, json });

		expect(response.statusCode).toBe(429);
		const responseHeaders = response.headers();
		expect(responseHeaders["retry-after"]).toBe("60");
	});

	test("onError - Error Logging", async () => {
		const app = createAppLifecycleHooksOnerrorErrorLogging();
		const client = new TestClient(app);

		const response = await client.get("/api/test-error");

		expect(response.statusCode).toBe(500);
		const responseHeaders = response.headers();
		expect(responseHeaders["content-type"]).toBe("application/json");
	});

	test("Multiple Hooks - All Phases", async () => {
		const app = createAppLifecycleHooksMultipleHooksAllPhases();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
			Authorization: "Bearer valid-token-12345",
		};
		const json = { action: "update_profile", user_id: "user-123" };
		const response = await client.post("/api/full-lifecycle", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("action");
		expect(responseData.action).toBe("update_profile");
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Action completed successfully");
		expect(responseData).toHaveProperty("request_id");
		expect(responseData.request_id).toBe(".*");
		expect(responseData).toHaveProperty("user_id");
		expect(responseData.user_id).toBe("user-123");
		const responseHeaders = response.headers();
		expect(responseHeaders["x-response-time"]).toBe(".*ms");
		expect(responseHeaders["x-frame-options"]).toBe("DENY");
		expect(responseHeaders["x-content-type-options"]).toBe("nosniff");
		expect(responseHeaders["x-request-id"]).toBe(".*");
	});

	test("Hook Execution Order", async () => {
		const app = createAppLifecycleHooksHookExecutionOrder();
		const client = new TestClient(app);

		const response = await client.get("/api/test-hook-order");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("execution_order");
		expect(responseData.execution_order.length).toBe(3);
		expect(responseData.execution_order[0]).toBe("first_hook");
		expect(responseData.execution_order[1]).toBe("second_hook");
		expect(responseData.execution_order[2]).toBe("third_hook");
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Hooks executed in order");
	});

	test("onResponse - Response Timing", async () => {
		const app = createAppLifecycleHooksOnresponseResponseTiming();
		const client = new TestClient(app);

		const response = await client.get("/api/test-timing");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Response with timing info");
		const responseHeaders = response.headers();
		expect(responseHeaders["x-response-time"]).toBe(".*ms");
	});

	test("preHandler - Authorization Forbidden Short Circuit", async () => {
		const app = createAppLifecycleHooksPrehandlerAuthorizationForbiddenShortCircuit();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer user-token-11111",
		};
		const response = await client.get("/api/admin-only-forbidden", headers);

		expect(response.statusCode).toBe(403);
	});

	test("onRequest - Request Logging", async () => {
		const app = createAppLifecycleHooksOnrequestRequestLogging();
		const client = new TestClient(app);

		const response = await client.get("/api/test-on-request");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("has_request_id");
		expect(responseData.has_request_id).toBe(true);
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("onRequest hooks executed");
		expect(responseData).toHaveProperty("request_logged");
		expect(responseData.request_logged).toBe(true);
		const responseHeaders = response.headers();
		expect(responseHeaders["x-request-id"]).toBe(".*");
	});

	test("preValidation - Rate Limiting", async () => {
		const app = createAppLifecycleHooksPrevalidationRateLimiting();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { data: "test" };
		const response = await client.post("/api/test-rate-limit", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Request accepted");
		expect(responseData).toHaveProperty("rate_limit_checked");
		expect(responseData.rate_limit_checked).toBe(true);
	});
});
