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
	createAppAuthJwtAuthenticationExpiredToken,
	createAppAuthJwtAuthenticationInvalidAudience,
	createAppAuthJwtAuthenticationInvalidSignature,
	createAppAuthJwtAuthenticationMissingAuthorizationHeader,
	createAppAuthJwtAuthenticationValidToken,
} from "../app/main.js";

describe("auth", () => {
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
				"Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTksImlhdCI6MTczMTI1MjAwMCwiYXVkIjpbImh0dHBzOi8vd3Jvbmctc2VydmljZS5jb20iXX0.xxxxxx",
		};
		const response = await client.get("/protected/user", headers);

		expect(response.statusCode).toBe(401);
	});
});
