/**
 * E2E tests for auth
 * @generated
 */

import { TestClient } from "@spikard/node";
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
} from "../app/main.js";

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
		expect(responseData).toHaveProperty("data");
		expect(responseData.data).toBe("sensitive information");
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Access granted");
	});

	test("JWT invalid issuer", async () => {
		const app = createAppAuthJwtInvalidIssuer();
		const client = new TestClient(app);

		const headers = {
			Authorization:
				"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImlzcyI6Imh0dHBzOi8vZXZpbC5jb20ifQ.O3gVwqYHqJQPL2PtgWmBN0sQd5_HvYKKjZGhPkXqM_w",
		};
		const response = await client.get("/api/protected", headers);

		expect(response.statusCode).toBe(401);
	});

	test("JWT with multiple audiences", async () => {
		const app = createAppAuthJwtWithMultipleAudiences();
		const client = new TestClient(app);

		const headers = {
			Authorization:
				"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSIsImh0dHBzOi8vYWRtaW4uZXhhbXBsZS5jb20iXSwiaXNzIjoiaHR0cHM6Ly9hdXRoLmV4YW1wbGUuY29tIn0.qVfBpQYPcX9wWZJhULmN7KR8vT3DxGbH2jSaIoFnYwE",
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
		expect(responseData).toHaveProperty("data");
		expect(responseData.data).toBe("sensitive information");
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Access granted");
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
				"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsIm5iZiI6MjYyNjc4Mzk0Nn0.8yXqZ9jKCR0BwqJc7pN_QvD3mYLxHfWzUeIaGkTnOsA",
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
		expect(responseData).toHaveProperty("auth_method");
		expect(responseData.auth_method).toBe("jwt");
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Access granted");
		expect(responseData).toHaveProperty("user_id");
		expect(responseData.user_id).toBe("user123");
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
		expect(responseData).toHaveProperty("data");
		expect(responseData.data).toBe("sensitive information");
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Access granted");
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
		expect(responseData).toHaveProperty("data");
		expect(responseData.data).toBe("sensitive information");
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Access granted");
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
