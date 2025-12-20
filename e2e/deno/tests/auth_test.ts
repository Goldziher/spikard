/**
 * E2E tests for auth
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
import {
	createAppAuthApiKeyAuthenticationInvalidKey,
	createAppAuthApiKeyAuthenticationMissingHeader,
	createAppAuthApiKeyAuthenticationValidKey,
	createAppAuthApiKeyInQueryParameter,
	createAppAuthApiKeyRotationOldKeyStillValid,
	createAppAuthApiKeyWithCustomHeaderName,
	createAppAuthBearerTokenWithoutPrefix,
	createAppAuthJwtAuthenticationExpiredToken,
	createAppAuthJwtAuthenticationInvalidAudience,
	createAppAuthJwtAuthenticationInvalidSignature,
	createAppAuthJwtAuthenticationMissingAuthorizationHeader,
	createAppAuthJwtAuthenticationValidToken,
	createAppAuthJwtInvalidIssuer,
	createAppAuthJwtMalformedTokenFormat,
	createAppAuthJwtMissingRequiredCustomClaims,
	createAppAuthJwtNotBeforeClaimInFuture,
	createAppAuthJwtWithMultipleAudiences,
	createAppAuthMultipleAuthenticationSchemesJwtPrecedence,
} from "../app/main.ts";

	Deno.test("auth: JWT malformed token format", async () => {
		const app = createAppAuthJwtMalformedTokenFormat();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer invalid.token",
		};
		const response = await client.get("/api/protected", headers);

		assertEquals(response.statusCode, 401);
	});

	Deno.test("auth: Bearer token without prefix", async () => {
		const app = createAppAuthBearerTokenWithoutPrefix();
		const client = new TestClient(app);

		const headers = {
			Authorization: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDZ9.8yXqZ9jKCR0BwqJc7pN_QvD3mYLxHfWzUeIaGkTnOsA",
		};
		const response = await client.get("/api/protected", headers);

		assertEquals(response.statusCode, 401);
	});

	Deno.test("auth: JWT authentication - valid token", async () => {
		const app = createAppAuthJwtAuthenticationValidToken();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg",
		};
		const response = await client.get("/protected/user", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Access granted");
		assert(Object.hasOwn(responseData, "user_id"));
		assertEquals(responseData.user_id, "user123");
	});

	Deno.test("auth: API key rotation - old key still valid", async () => {
		const app = createAppAuthApiKeyRotationOldKeyStillValid();
		const client = new TestClient(app);

		const headers = {
			"X-API-Key": "sk_test_old_123456",
		};
		const response = await client.get("/api/data", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "data"));
		assertEquals(responseData.data, "sensitive information");
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Access granted");
		const responseHeaders = response.headers();
		assertEquals(responseHeaders["x-api-key-deprecated"], "true");
	});

	Deno.test("auth: JWT invalid issuer", async () => {
		const app = createAppAuthJwtInvalidIssuer();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2V2aWwuY29tIn0.mbL5L04_hpaaiz0SPABap6ZWfBLu18aiexBjzwQ1nnA",
		};
		const response = await client.get("/api/protected", headers);

		assertEquals(response.statusCode, 401);
	});

	Deno.test("auth: JWT with multiple audiences", async () => {
		const app = createAppAuthJwtWithMultipleAudiences();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSIsImh0dHBzOi8vYWRtaW4uZXhhbXBsZS5jb20iXSwiaXNzIjoiaHR0cHM6Ly9hdXRoLmV4YW1wbGUuY29tIn0.9MBL_XccGXfu9cDUnCpQruDMOl2hHYydzeGn-20dQOs",
		};
		const response = await client.get("/api/protected", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Access granted");
		assert(Object.hasOwn(responseData, "user_id"));
		assertEquals(responseData.user_id, "user123");
	});

	Deno.test("auth: API key in query parameter", async () => {
		const app = createAppAuthApiKeyInQueryParameter();
		const client = new TestClient(app);

		const response = await client.get("/api/data?api_key=sk_test_123456");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "data"));
		assertEquals(responseData.data, "sensitive information");
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Access granted");
	});

	Deno.test("auth: JWT authentication - expired token", async () => {
		const app = createAppAuthJwtAuthenticationExpiredToken();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoxNjAwMDAwMDAwLCJpYXQiOjE1OTAwMDAwMDB9.n4oBw9XuO2aAJWi1e4Bz9Y_m2iEyJHGAODcetNuwYFo",
		};
		const response = await client.get("/protected/user", headers);

		assertEquals(response.statusCode, 401);
	});

	Deno.test("auth: API key authentication - invalid key", async () => {
		const app = createAppAuthApiKeyAuthenticationInvalidKey();
		const client = new TestClient(app);

		const headers = {
			"X-API-Key": "invalid_key_12345",
		};
		const response = await client.get("/api/data", headers);

		assertEquals(response.statusCode, 401);
	});

	Deno.test("auth: JWT not before claim in future", async () => {
		const app = createAppAuthJwtNotBeforeClaimInFuture();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsIm5iZiI6MjYyNjc4Mzk0NiwiYXVkIjpbImh0dHBzOi8vYXBpLmV4YW1wbGUuY29tIl0sImlzcyI6Imh0dHBzOi8vYXV0aC5leGFtcGxlLmNvbSJ9.hG4I76_3kJfsbJ_jmxoP1NSYnkcqdyBFcPpdo-jYU4E",
		};
		const response = await client.get("/api/protected", headers);

		assertEquals(response.statusCode, 401);
	});

	Deno.test("auth: Multiple authentication schemes - JWT precedence", async () => {
		const app = createAppAuthMultipleAuthenticationSchemesJwtPrecedence();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg",
			"X-API-Key": "sk_test_123456",
		};
		const response = await client.get("/api/data", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "auth_method"));
		assertEquals(responseData.auth_method, "jwt");
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Access granted");
		assert(Object.hasOwn(responseData, "user_id"));
		assertEquals(responseData.user_id, "user123");
	});

	Deno.test("auth: JWT missing required custom claims", async () => {
		const app = createAppAuthJwtMissingRequiredCustomClaims();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg",
		};
		const response = await client.get("/api/admin", headers);

		assertEquals(response.statusCode, 403);
	});

	Deno.test("auth: API key authentication - valid key", async () => {
		const app = createAppAuthApiKeyAuthenticationValidKey();
		const client = new TestClient(app);

		const headers = {
			"X-API-Key": "sk_test_123456",
		};
		const response = await client.get("/api/data", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "data"));
		assertEquals(responseData.data, "sensitive information");
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Access granted");
	});

	Deno.test("auth: API key with custom header name", async () => {
		const app = createAppAuthApiKeyWithCustomHeaderName();
		const client = new TestClient(app);

		const headers = {
			"X-API-Token": "sk_test_123456",
		};
		const response = await client.get("/api/data", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "data"));
		assertEquals(responseData.data, "sensitive information");
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Access granted");
	});

	Deno.test("auth: API key authentication - missing header", async () => {
		const app = createAppAuthApiKeyAuthenticationMissingHeader();
		const client = new TestClient(app);

		const response = await client.get("/api/data");

		assertEquals(response.statusCode, 401);
	});

	Deno.test("auth: JWT authentication - invalid signature", async () => {
		const app = createAppAuthJwtAuthenticationInvalidSignature();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTksImlhdCI6MTczMTI1MjAwMH0.invalid_signature_here",
		};
		const response = await client.get("/protected/user", headers);

		assertEquals(response.statusCode, 401);
	});

	Deno.test("auth: JWT authentication - missing Authorization header", async () => {
		const app = createAppAuthJwtAuthenticationMissingAuthorizationHeader();
		const client = new TestClient(app);

		const response = await client.get("/protected/user");

		assertEquals(response.statusCode, 401);
	});

	Deno.test("auth: JWT authentication - invalid audience", async () => {
		const app = createAppAuthJwtAuthenticationInvalidAudience();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTk5LCJpYXQiOjE3MzEyNTIwMDAsImF1ZCI6WyJodHRwczovL3dyb25nLXNlcnZpY2UuY29tIl19.YR2a9fSJjhen7ksYFI2djSBSC7Pc29FDCloBGhkj3kU",
		};
		const response = await client.get("/protected/user", headers);

		assertEquals(response.statusCode, 401);
	});