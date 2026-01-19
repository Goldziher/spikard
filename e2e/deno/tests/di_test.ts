/**
 * E2E tests for di
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
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

	Deno.test("di: Route-level dependency override - success", async () => {
		const app = createAppDiRouteLevelDependencyOverrideSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/override-test");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "mode"));
		assertEquals(responseData.mode, "test");
		assert(Object.hasOwn(responseData, "strict"));
		assertEquals(responseData.strict, false);
	});

	Deno.test("di: Circular dependency detection - error", async () => {
		const app = createAppDiCircularDependencyDetectionError();
		const client = new TestClient(app);

		const response = await client.get("/api/circular");

		assertEquals(response.statusCode, 500);
	});

	Deno.test("di: Factory dependency - success", async () => {
		const app = createAppDiFactoryDependencySuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/timestamp");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "timestamp"));
		assertEquals(responseData.timestamp, "<<present>>");
	});

	Deno.test("di: Value dependency injection - success", async () => {
		const app = createAppDiValueDependencyInjectionSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/config");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "app_name"));
		assertEquals(responseData.app_name, "SpikardApp");
		assert(Object.hasOwn(responseData, "version"));
		assertEquals(responseData.version, "1.0.0");
		assert(Object.hasOwn(responseData, "max_connections"));
		assertEquals(responseData.max_connections, 100);
	});

	Deno.test("di: Node js object destructuring injection - success", async () => {
		const app = createAppDiNodeJsObjectDestructuringInjectionSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/node-destructure");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "db_name"));
		assertEquals(responseData.db_name, "PostgreSQL");
		assert(Object.hasOwn(responseData, "log_level"));
		assertEquals(responseData.log_level, "info");
	});

	Deno.test("di: Nested dependencies 3 levels - success", async () => {
		const app = createAppDiNestedDependencies3LevelsSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/auth-status");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "auth_enabled"));
		assertEquals(responseData.auth_enabled, true);
		assert(Object.hasOwn(responseData, "has_db"));
		assertEquals(responseData.has_db, true);
		assert(Object.hasOwn(responseData, "has_cache"));
		assertEquals(responseData.has_cache, true);
	});

	Deno.test("di: Type mismatch in dependency resolution - error", async () => {
		const app = createAppDiTypeMismatchInDependencyResolutionError();
		const client = new TestClient(app);

		const response = await client.get("/api/type-mismatch");

		assertEquals(response.statusCode, 500);
	});

	Deno.test("di: Missing dependency - error", async () => {
		const app = createAppDiMissingDependencyError();
		const client = new TestClient(app);

		const response = await client.get("/api/missing-dep");

		assertEquals(response.statusCode, 500);
	});

	Deno.test("di: Python parameter name-based injection - success", async () => {
		const app = createAppDiPythonParameterNameBasedInjectionSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/python-name-inject");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "db_status"));
		assertEquals(responseData.db_status, "connected");
		assert(Object.hasOwn(responseData, "cache_status"));
		assertEquals(responseData.cache_status, "ready");
	});

	Deno.test("di: Dependency injection in lifecycle hooks - success", async () => {
		const app = createAppDiDependencyInjectionInLifecycleHooksSuccess();
		const client = new TestClient(app);

		const headers = {
			authorization: "Bearer valid_token",
		};
		const response = await client.get("/api/hook-di-test", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "authenticated"));
		assertEquals(responseData.authenticated, true);
		assert(Object.hasOwn(responseData, "logged"));
		assertEquals(responseData.logged, true);
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["x-auth-mode"], "strict");
		assertEquals(responseHeaders["x-log-level"], "debug");
	});

	Deno.test("di: Ruby keyword argument injection - success", async () => {
		const app = createAppDiRubyKeywordArgumentInjectionSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/ruby-kwargs");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "adapter"));
		assertEquals(responseData.adapter, "postgresql");
		assert(Object.hasOwn(responseData, "user_id"));
		assertEquals(responseData.user_id, 42);
	});

	Deno.test("di: Multiple dependencies with cleanup - success", async () => {
		const app = createAppDiMultipleDependenciesWithCleanupSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/multi-cleanup-test");

		assertEquals(response.statusCode, 200);
		const stateResponse = await client.get("/api/multi-cleanup-state");
		assertEquals(stateResponse.statusCode, 200);
		assertEquals(stateResponse.json(), { cleanup_order: ["db_opened", "cache_opened", "session_opened", "session_closed", "cache_closed", "db_closed"] });
	});

	Deno.test("di: Mixed singleton and per-request caching - success", async () => {
		const app = createAppDiMixedSingletonAndPerRequestCachingSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/mixed-caching");

		assertEquals(response.statusCode, 200);

		// Second request to verify caching behavior
		const response2 = await client.get("/api/mixed-caching");
		assertEquals(response2.statusCode, 200);
		const data1 = response.json();
		const data2 = response2.json();

		// pool_id is singleton; context_id is per-request
		assert(data1.pool_id !== undefined && data1.pool_id !== null);
		assert(data2.pool_id !== undefined && data2.pool_id !== null);
		assertEquals(data1.pool_id, data2.pool_id);
		assert(data1.context_id !== undefined && data1.context_id !== null);
		assert(data2.context_id !== undefined && data2.context_id !== null);
		assert(data1.context_id !== data2.context_id);
	});

	Deno.test("di: Resource cleanup after request - success", async () => {
		const app = createAppDiResourceCleanupAfterRequestSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/cleanup-test");

		assertEquals(response.statusCode, 200);
		const stateResponse = await client.get("/api/cleanup-state");
		assertEquals(stateResponse.statusCode, 200);
		assertEquals(stateResponse.json(), { cleanup_events: ["session_opened", "session_closed"] });
	});

	Deno.test("di: Python type annotation-based injection - success", async () => {
		const app = createAppDiPythonTypeAnnotationBasedInjectionSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/python-type-inject");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "pool_type"));
		assertEquals(responseData.pool_type, "PostgreSQL");
		assert(Object.hasOwn(responseData, "cache_type"));
		assertEquals(responseData.cache_type, "Redis");
	});

	Deno.test("di: Per-request dependency caching - success", async () => {
		const app = createAppDiPerRequestDependencyCachingSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/request-id");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "first_id"));
		assertEquals(responseData.first_id, "<<uuid>>");
		assert(Object.hasOwn(responseData, "second_id"));
		assertEquals(responseData.second_id, "<<same_as:first_id>>");
	});

	Deno.test("di: Singleton dependency caching - success", async () => {
		const app = createAppDiSingletonDependencyCachingSuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/app-counter");

		assertEquals(response.statusCode, 200);

		// Second request to verify caching behavior
		const response2 = await client.get("/api/app-counter");
		assertEquals(response2.statusCode, 200);
		const data1 = response.json();
		const data2 = response2.json();

		// Singleton counter should have stable counter_id and incremented count
		assert(data1.counter_id !== undefined && data1.counter_id !== null);
		assert(data2.counter_id !== undefined && data2.counter_id !== null);
		assertEquals(data1.counter_id, data2.counter_id);
		assert(data2.count > data1.count);
	});

	Deno.test("di: Async factory dependency - success", async () => {
		const app = createAppDiAsyncFactoryDependencySuccess();
		const client = new TestClient(app);

		const response = await client.get("/api/db-status");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "pool_status"));
		assertEquals(responseData.pool_status, "connected");
		assert(Object.hasOwn(responseData, "max_size"));
		assertEquals(responseData.max_size, 10);
	});