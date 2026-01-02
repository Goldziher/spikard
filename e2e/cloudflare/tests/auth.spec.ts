/**
 * E2E tests for auth
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { describe, expect, test } from "vitest";
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

describe("auth", () => {
	test("JWT malformed token format", async () => {
		const app = createAppAuthJwtMalformedTokenFormat();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer invalid.token",
		};
		const response = await client.get("/api/protected", headers);

		expect(response.statusCode).toBe(401);
	});

	test("Bearer token without prefix", async () => {
		const app = createAppAuthBearerTokenWithoutPrefix();
		const client = new TestClient(app);

		const headers = {
			Authorization:
				"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDZ9.8yXqZ9jKCR0BwqJc7pN_QvD3mYLxHfWzUeIaGkTnOsA",
		};
		const response = await client.get("/api/protected", headers);

		expect(response.statusCode).toBe(401);
	});

	test("JWT authentication - valid token", async () => {
		const app = createAppAuthJwtAuthenticationValidToken();
		const client = new TestClient(app);

		const headers = {
			Authorization:
				"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg",
		};
		const response = await client.get("/protected/user", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Access granted");
		expect(responseData).toHaveProperty("user_id");
		expect(responseData.user_id).toBe("user123");
	});

	test("API key rotation - old key still valid", async () => {
		const app = createAppAuthApiKeyRotationOldKeyStillValid();
		const client = new TestClient(app);

		const headers = {
			"X-API-Key": "sk_test_old_123456",
		};
		const response = await client.get("/api/data", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Access granted");
		expect(responseData).toHaveProperty("data");
		expect(responseData.data).toBe("sensitive information");
		const responseHeaders = response.headers();
		expect(responseHeaders["x-api-key-deprecated"]).toBe("true");
	});

	test("JWT invalid issuer", async () => {
		const app = createAppAuthJwtInvalidIssuer();
		const client = new TestClient(app);

		const headers = {
			Authorization:
				"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2V2aWwuY29tIn0.mbL5L04_hpaaiz0SPABap6ZWfBLu18aiexBjzwQ1nnA",
		};
		const response = await client.get("/api/protected", headers);

		expect(response.statusCode).toBe(401);
	});

	test("JWT with multiple audiences", async () => {
		const app = createAppAuthJwtWithMultipleAudiences();
		const client = new TestClient(app);

		const headers = {
			Authorization:
				"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSIsImh0dHBzOi8vYWRtaW4uZXhhbXBsZS5jb20iXSwiaXNzIjoiaHR0cHM6Ly9hdXRoLmV4YW1wbGUuY29tIn0.9MBL_XccGXfu9cDUnCpQruDMOl2hHYydzeGn-20dQOs",
		};
		const response = await client.get("/api/protected", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Access granted");
		expect(responseData).toHaveProperty("user_id");
		expect(responseData.user_id).toBe("user123");
	});

	test("API key in query parameter", async () => {
		const app = createAppAuthApiKeyInQueryParameter();
		const client = new TestClient(app);

		const response = await client.get("/api/data?api_key=sk_test_123456");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Access granted");
		expect(responseData).toHaveProperty("data");
		expect(responseData.data).toBe("sensitive information");
	});

	test("JWT authentication - expired token", async () => {
		const app = createAppAuthJwtAuthenticationExpiredToken();
		const client = new TestClient(app);

		const headers = {
			Authorization:
				"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoxNjAwMDAwMDAwLCJpYXQiOjE1OTAwMDAwMDB9.n4oBw9XuO2aAJWi1e4Bz9Y_m2iEyJHGAODcetNuwYFo",
		};
		const response = await client.get("/protected/user", headers);

		expect(response.statusCode).toBe(401);
	});

	test("API key authentication - invalid key", async () => {
		const app = createAppAuthApiKeyAuthenticationInvalidKey();
		const client = new TestClient(app);

		const headers = {
			"X-API-Key": "invalid_key_12345",
		};
		const response = await client.get("/api/data", headers);

		expect(response.statusCode).toBe(401);
	});

	test("JWT not before claim in future", async () => {
		const app = createAppAuthJwtNotBeforeClaimInFuture();
		const client = new TestClient(app);

		const headers = {
			Authorization:
				"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsIm5iZiI6MjYyNjc4Mzk0NiwiYXVkIjpbImh0dHBzOi8vYXBpLmV4YW1wbGUuY29tIl0sImlzcyI6Imh0dHBzOi8vYXV0aC5leGFtcGxlLmNvbSJ9.hG4I76_3kJfsbJ_jmxoP1NSYnkcqdyBFcPpdo-jYU4E",
		};
		const response = await client.get("/api/protected", headers);

		expect(response.statusCode).toBe(401);
	});

	test("Multiple authentication schemes - JWT precedence", async () => {
		const app = createAppAuthMultipleAuthenticationSchemesJwtPrecedence();
		const client = new TestClient(app);

		const headers = {
			"X-API-Key": "sk_test_123456",
			Authorization:
				"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg",
		};
		const response = await client.get("/api/data", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Access granted");
		expect(responseData).toHaveProperty("user_id");
		expect(responseData.user_id).toBe("user123");
		expect(responseData).toHaveProperty("auth_method");
		expect(responseData.auth_method).toBe("jwt");
	});

	test("JWT missing required custom claims", async () => {
		const app = createAppAuthJwtMissingRequiredCustomClaims();
		const client = new TestClient(app);

		const headers = {
			Authorization:
				"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg",
		};
		const response = await client.get("/api/admin", headers);

		expect(response.statusCode).toBe(403);
	});

	test("API key authentication - valid key", async () => {
		const app = createAppAuthApiKeyAuthenticationValidKey();
		const client = new TestClient(app);

		const headers = {
			"X-API-Key": "sk_test_123456",
		};
		const response = await client.get("/api/data", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Access granted");
		expect(responseData).toHaveProperty("data");
		expect(responseData.data).toBe("sensitive information");
	});

	test("API key with custom header name", async () => {
		const app = createAppAuthApiKeyWithCustomHeaderName();
		const client = new TestClient(app);

		const headers = {
			"X-API-Token": "sk_test_123456",
		};
		const response = await client.get("/api/data", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Access granted");
		expect(responseData).toHaveProperty("data");
		expect(responseData.data).toBe("sensitive information");
	});

	test("API key authentication - missing header", async () => {
		const app = createAppAuthApiKeyAuthenticationMissingHeader();
		const client = new TestClient(app);

		const response = await client.get("/api/data");

		expect(response.statusCode).toBe(401);
	});

	test("JWT authentication - invalid signature", async () => {
		const app = createAppAuthJwtAuthenticationInvalidSignature();
		const client = new TestClient(app);

		const headers = {
			Authorization:
				"Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTksImlhdCI6MTczMTI1MjAwMH0.invalid_signature_here",
		};
		const response = await client.get("/protected/user", headers);

		expect(response.statusCode).toBe(401);
	});

	test("JWT authentication - missing Authorization header", async () => {
		const app = createAppAuthJwtAuthenticationMissingAuthorizationHeader();
		const client = new TestClient(app);

		const response = await client.get("/protected/user");

		expect(response.statusCode).toBe(401);
	});

	test("JWT authentication - invalid audience", async () => {
		const app = createAppAuthJwtAuthenticationInvalidAudience();
		const client = new TestClient(app);

		const headers = {
			Authorization:
				"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTk5LCJpYXQiOjE3MzEyNTIwMDAsImF1ZCI6WyJodHRwczovL3dyb25nLXNlcnZpY2UuY29tIl19.YR2a9fSJjhen7ksYFI2djSBSC7Pc29FDCloBGhkj3kU",
		};
		const response = await client.get("/protected/user", headers);

		expect(response.statusCode).toBe(401);
	});
});
