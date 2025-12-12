/**
 * E2E tests for di
 * @generated
 */

import { TestClient } from "@spikard/node";
import { describe, expect, test } from "vitest";
import {
	createAppDiAsyncFactoryDependencySuccess,
	createAppDiCircularDependencyDetectionError,
	createAppDiDependencyInjectionInLifecycleHooksSuccess,
	createAppDiFactoryDependencySuccess,
	createAppDiMissingDependencyError,
	createAppDiMixedSingletonAndPerRequestCachingSuccess,
	createAppDiMultipleDependenciesWithCleanupSuccess,
	createAppDiNestedDependencies3LevelsSuccess,
	createAppDiNodeJsObjectDestructuringInjectionSuccess,
	createAppDiPerRequestDependencyCachingSuccess,
	createAppDiPythonParameterNameBasedInjectionSuccess,
	createAppDiPythonTypeAnnotationBasedInjectionSuccess,
	createAppDiResourceCleanupAfterRequestSuccess,
	createAppDiRouteLevelDependencyOverrideSuccess,
	createAppDiRubyKeywordArgumentInjectionSuccess,
	createAppDiSingletonDependencyCachingSuccess,
	createAppDiTypeMismatchInDependencyResolutionError,
	createAppDiValueDependencyInjectionSuccess,
} from "../app/main.ts";

describe("di", () => {
	test("Route-level dependency override - success", async () => {
		const app = createAppDiRouteLevelDependencyOverrideSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/override-test");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("mode");
		expect(responseData.mode).toBe("test");
		expect(responseData).toHaveProperty("strict");
		expect(responseData.strict).toBe(false);
	});

	test("Circular dependency detection - error", async () => {
		const app = createAppDiCircularDependencyDetectionError();
		const client = new TestClient(app);

		const response = await client.get("/api/circular");

		expect(response.statusCode).toBe(500);
	});

	test("Factory dependency - success", async () => {
		const app = createAppDiFactoryDependencySuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/timestamp");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("timestamp");
		expect(responseData.timestamp).toBe("<<present>>");
	});

	test("Value dependency injection - success", async () => {
		const app = createAppDiValueDependencyInjectionSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/config");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("app_name");
		expect(responseData.app_name).toBe("SpikardApp");
		expect(responseData).toHaveProperty("max_connections");
		expect(responseData.max_connections).toBe(100);
		expect(responseData).toHaveProperty("version");
		expect(responseData.version).toBe("1.0.0");
	});

	test("Node js object destructuring injection - success", async () => {
		const app = createAppDiNodeJsObjectDestructuringInjectionSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/node-destructure");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("db_name");
		expect(responseData.db_name).toBe("PostgreSQL");
		expect(responseData).toHaveProperty("log_level");
		expect(responseData.log_level).toBe("info");
	});

	test("Nested dependencies 3 levels - success", async () => {
		const app = createAppDiNestedDependencies3LevelsSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/auth-status");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("auth_enabled");
		expect(responseData.auth_enabled).toBe(true);
		expect(responseData).toHaveProperty("has_cache");
		expect(responseData.has_cache).toBe(true);
		expect(responseData).toHaveProperty("has_db");
		expect(responseData.has_db).toBe(true);
	});

	test("Type mismatch in dependency resolution - error", async () => {
		const app = createAppDiTypeMismatchInDependencyResolutionError();
		const client = new TestClient(app);

		const response = await client.get("/api/type-mismatch");

		expect(response.statusCode).toBe(500);
	});

	test("Missing dependency - error", async () => {
		const app = createAppDiMissingDependencyError();
		const client = new TestClient(app);

		const response = await client.get("/api/missing-dep");

		expect(response.statusCode).toBe(500);
	});

	test("Python parameter name-based injection - success", async () => {
		const app = createAppDiPythonParameterNameBasedInjectionSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/python-name-inject");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("cache_status");
		expect(responseData.cache_status).toBe("ready");
		expect(responseData).toHaveProperty("db_status");
		expect(responseData.db_status).toBe("connected");
	});

	test("Dependency injection in lifecycle hooks - success", async () => {
		const app = createAppDiDependencyInjectionInLifecycleHooksSuccess();
		const client = new TestClient(app);

		const headers = {
			authorization: "Bearer valid_token",
		};
		const response = await client.get("/api/hook-di-test", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("authenticated");
		expect(responseData.authenticated).toBe(true);
		expect(responseData).toHaveProperty("logged");
		expect(responseData.logged).toBe(true);
		const responseHeaders = response.headers();
		expect(responseHeaders["x-log-level"]).toBe("debug");
		expect(responseHeaders["x-auth-mode"]).toBe("strict");
	});

	test("Ruby keyword argument injection - success", async () => {
		const app = createAppDiRubyKeywordArgumentInjectionSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/ruby-kwargs");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("adapter");
		expect(responseData.adapter).toBe("postgresql");
		expect(responseData).toHaveProperty("user_id");
		expect(responseData.user_id).toBe(42);
	});

	test("Multiple dependencies with cleanup - success", async () => {
		const app = createAppDiMultipleDependenciesWithCleanupSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/multi-cleanup-test");

		expect(response.statusCode).toBe(200);
		const stateResponse = await client.get("/api/multi-cleanup-state");
		expect(stateResponse.statusCode).toBe(200);
		expect(stateResponse.json()).toStrictEqual({
			cleanup_order: ["db_opened", "cache_opened", "session_opened", "session_closed", "cache_closed", "db_closed"],
		});
	});

	test("Mixed singleton and per-request caching - success", async () => {
		const app = createAppDiMixedSingletonAndPerRequestCachingSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/mixed-caching");

		expect(response.statusCode).toBe(200);

		const response2 = await client.get("/api/mixed-caching");
		expect(response2.statusCode).toBe(200);
		const data1 = response.json();
		const data2 = response2.json();

		expect(data1.id).toBeDefined();
		expect(data2.id).toBeDefined();
		expect(data1.id).toBe(data2.id);
		if (data1.count !== undefined && data2.count !== undefined) {
			expect(data2.count).toBeGreaterThan(data1.count);
		}
	});

	test("Resource cleanup after request - success", async () => {
		const app = createAppDiResourceCleanupAfterRequestSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/cleanup-test");

		expect(response.statusCode).toBe(200);
		const stateResponse = await client.get("/api/cleanup-state");
		expect(stateResponse.statusCode).toBe(200);
		expect(stateResponse.json()).toStrictEqual({ cleanup_events: ["session_opened", "session_closed"] });
	});

	test("Python type annotation-based injection - success", async () => {
		const app = createAppDiPythonTypeAnnotationBasedInjectionSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/python-type-inject");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("cache_type");
		expect(responseData.cache_type).toBe("Redis");
		expect(responseData).toHaveProperty("pool_type");
		expect(responseData.pool_type).toBe("PostgreSQL");
	});

	test("Per-request dependency caching - success", async () => {
		const app = createAppDiPerRequestDependencyCachingSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/request-id");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("first_id");
		expect(responseData.first_id).toBe("<<uuid>>");
		expect(responseData).toHaveProperty("second_id");
		expect(responseData.second_id).toBe("<<same_as:first_id>>");
	});

	test("Singleton dependency caching - success", async () => {
		const app = createAppDiSingletonDependencyCachingSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/app-counter");

		expect(response.statusCode).toBe(200);

		const response2 = await client.get("/api/app-counter");
		expect(response2.statusCode).toBe(200);
		const data1 = response.json();
		const data2 = response2.json();

		expect(data1.id).toBeDefined();
		expect(data2.id).toBeDefined();
		expect(data1.id).toBe(data2.id);
		if (data1.count !== undefined && data2.count !== undefined) {
			expect(data2.count).toBeGreaterThan(data1.count);
		}
	});

	test("Async factory dependency - success", async () => {
		const app = createAppDiAsyncFactoryDependencySuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/db-status");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("max_size");
		expect(responseData.max_size).toBe(10);
		expect(responseData).toHaveProperty("pool_status");
		expect(responseData.pool_status).toBe("connected");
	});
});
