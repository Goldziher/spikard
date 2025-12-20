/**
 * E2E tests for cookies
 * @generated
 */

import { TestClient } from "@spikard/wasm/node";
import { describe, expect, test } from "vitest";
import {
	createAppCookies24CookieSamesiteStrict,
	createAppCookies25CookieSamesiteLax,
	createAppCookies26CookieSecureFlag,
	createAppCookies27CookieHttponlyFlag,
	createAppCookiesApikeyCookieAuthenticationMissing,
	createAppCookiesApikeyCookieAuthenticationSuccess,
	createAppCookiesCookieRegexPatternValidationFail,
	createAppCookiesCookieRegexPatternValidationSuccess,
	createAppCookiesCookieValidationMaxLengthConstraintFail,
	createAppCookiesCookieValidationMinLengthConstraintSuccess,
	createAppCookiesCookieValidationMinLengthFailure,
	createAppCookiesMultipleCookiesSuccess,
	createAppCookiesOptionalApikeyCookieMissing,
	createAppCookiesOptionalCookieParameterMissing,
	createAppCookiesOptionalCookieParameterSuccess,
	createAppCookiesRequiredCookieMissing,
	createAppCookiesResponseCookieWithAttributes,
	createAppCookiesResponseCookieWithDomainAttribute,
	createAppCookiesResponseCookieWithPathAttribute,
	createAppCookiesResponseCookieWithSamesiteLax,
	createAppCookiesResponseCookieWithSamesiteNone,
	createAppCookiesResponseCookieWithSamesiteStrict,
	createAppCookiesResponseDeleteCookie,
	createAppCookiesResponseMultipleCookies,
	createAppCookiesResponseSessionCookieNoMaxAge,
	createAppCookiesResponseSetCookieBasic,
} from "../app/main.ts";

describe("cookies", () => {
	test("25_cookie_samesite_lax", async () => {
		const app = createAppCookies25CookieSamesiteLax();
		const client = new TestClient(app);

		const headers = {
			Cookie: "tracking=track123",
		};
		const response = await client.get("/data", headers);

		expect(response.statusCode).toBe(200);
	});

	test("Optional cookie parameter - success", async () => {
		const app = createAppCookiesOptionalCookieParameterSuccess();
		const client = new TestClient(app);

		const headers = {
			Cookie: "ads_id=abc123",
		};
		const response = await client.get("/items/", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("ads_id");
		expect(responseData.ads_id).toBe("abc123");
	});

	test("Cookie regex pattern validation - fail", async () => {
		const app = createAppCookiesCookieRegexPatternValidationFail();
		const client = new TestClient(app);

		const headers = {
			Cookie: "tracking_id=invalid-format",
		};
		const response = await client.get("/cookies/pattern", headers);

		expect(response.statusCode).toBe(422);
	});

	test("Response - session cookie no max_age", async () => {
		const app = createAppCookiesResponseSessionCookieNoMaxAge();
		const client = new TestClient(app);

		const json = { value: "session_abc123" };
		const response = await client.post("/cookies/session", { json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Session cookie set");
	});

	test("27_cookie_httponly_flag", async () => {
		const app = createAppCookies27CookieHttponlyFlag();
		const client = new TestClient(app);

		const headers = {
			Cookie: "session=session_abc123",
		};
		const response = await client.get("/secure", headers);

		expect(response.statusCode).toBe(200);
	});

	test("Response cookie with attributes", async () => {
		const app = createAppCookiesResponseCookieWithAttributes();
		const client = new TestClient(app);

		const response = await client.get("/cookie/set");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Cookie set");
	});

	test("24_cookie_samesite_strict", async () => {
		const app = createAppCookies24CookieSamesiteStrict();
		const client = new TestClient(app);

		const headers = {
			Cookie: "session_id=abc123xyz789",
		};
		const response = await client.get("/secure", headers);

		expect(response.statusCode).toBe(200);
	});

	test("APIKey cookie authentication - success", async () => {
		const app = createAppCookiesApikeyCookieAuthenticationSuccess();
		const client = new TestClient(app);

		const headers = {
			Cookie: "key=secret",
		};
		const response = await client.get("/users/me", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("username");
		expect(responseData.username).toBe("secret");
	});

	test("Cookie validation - min_length constraint success", async () => {
		const app = createAppCookiesCookieValidationMinLengthConstraintSuccess();
		const client = new TestClient(app);

		const headers = {
			Cookie: "token=abc",
		};
		const response = await client.get("/cookies/min-length", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("token");
		expect(responseData.token).toBe("abc");
	});

	test("Cookie validation - min_length failure", async () => {
		const app = createAppCookiesCookieValidationMinLengthFailure();
		const client = new TestClient(app);

		const headers = {
			Cookie: "tracking_id=ab",
		};
		const response = await client.get("/items/", headers);

		expect(response.statusCode).toBe(422);
	});

	test("Cookie validation - max_length constraint fail", async () => {
		const app = createAppCookiesCookieValidationMaxLengthConstraintFail();
		const client = new TestClient(app);

		const headers = {
			Cookie: "session_id=this_cookie_value_is_way_too_long",
		};
		const response = await client.get("/cookies/validated", headers);

		expect(response.statusCode).toBe(422);
	});

	test("Required cookie - missing", async () => {
		const app = createAppCookiesRequiredCookieMissing();
		const client = new TestClient(app);

		const headers = {
			Cookie: "fatebook_tracker=tracker456",
		};
		const response = await client.get("/items/cookies", headers);

		expect(response.statusCode).toBe(422);
	});

	test("Optional cookie parameter - missing", async () => {
		const app = createAppCookiesOptionalCookieParameterMissing();
		const client = new TestClient(app);

		const response = await client.get("/items/");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("ads_id");
		expect(responseData.ads_id).toBe(null);
	});

	test("APIKey cookie authentication - missing", async () => {
		const app = createAppCookiesApikeyCookieAuthenticationMissing();
		const client = new TestClient(app);

		const response = await client.get("/users/me/auth");

		expect(response.statusCode).toBe(422);
	});

	test("Response - multiple cookies", async () => {
		const app = createAppCookiesResponseMultipleCookies();
		const client = new TestClient(app);

		const json = { session: "session123", user: "john" };
		const response = await client.post("/cookies/multiple", { json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Multiple cookies set");
	});

	test("Response cookie with SameSite Lax", async () => {
		const app = createAppCookiesResponseCookieWithSamesiteLax();
		const client = new TestClient(app);

		const json = { value: "lax_cookie" };
		const response = await client.post("/cookies/samesite-lax", { json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Cookie set with SameSite=Lax");
	});

	test("Response - delete cookie", async () => {
		const app = createAppCookiesResponseDeleteCookie();
		const client = new TestClient(app);

		const headers = {
			Cookie: "session=old_session_123",
		};
		const response = await client.post("/cookies/delete", { headers });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Cookie deleted");
	});

	test("Response cookie with path attribute", async () => {
		const app = createAppCookiesResponseCookieWithPathAttribute();
		const client = new TestClient(app);

		const json = { value: "path_test" };
		const response = await client.post("/cookies/set-with-path", { json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Cookie set with path");
	});

	test("Optional APIKey cookie - missing", async () => {
		const app = createAppCookiesOptionalApikeyCookieMissing();
		const client = new TestClient(app);

		const response = await client.get("/users/me");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("msg");
		expect(responseData.msg).toBe("Create an account first");
	});

	test("Response cookie with SameSite Strict", async () => {
		const app = createAppCookiesResponseCookieWithSamesiteStrict();
		const client = new TestClient(app);

		const json = { value: "strict_cookie" };
		const response = await client.post("/cookies/samesite-strict", { json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Cookie set with SameSite=Strict");
	});

	test("Response cookie with SameSite None", async () => {
		const app = createAppCookiesResponseCookieWithSamesiteNone();
		const client = new TestClient(app);

		const json = { value: "none_cookie" };
		const response = await client.post("/cookies/samesite-none", { json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Cookie set with SameSite=None");
	});

	test("Cookie regex pattern validation - success", async () => {
		const app = createAppCookiesCookieRegexPatternValidationSuccess();
		const client = new TestClient(app);

		const headers = {
			Cookie: "tracking_id=ABC12345",
		};
		const response = await client.get("/cookies/pattern", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("tracking_id");
		expect(responseData.tracking_id).toBe("ABC12345");
	});

	test("Response set cookie - basic", async () => {
		const app = createAppCookiesResponseSetCookieBasic();
		const client = new TestClient(app);

		const response = await client.post("/cookie/");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Come to the dark side, we have cookies");
	});

	test("Multiple cookies - success", async () => {
		const app = createAppCookiesMultipleCookiesSuccess();
		const client = new TestClient(app);

		const headers = {
			Cookie: "fatebook_tracker=tracker456; session_id=session123; googall_tracker=ga789",
		};
		const response = await client.get("/items/", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("fatebook_tracker");
		expect(responseData.fatebook_tracker).toBe("tracker456");
		expect(responseData).toHaveProperty("googall_tracker");
		expect(responseData.googall_tracker).toBe("ga789");
		expect(responseData).toHaveProperty("session_id");
		expect(responseData.session_id).toBe("session123");
	});

	test("26_cookie_secure_flag", async () => {
		const app = createAppCookies26CookieSecureFlag();
		const client = new TestClient(app);

		const headers = {
			Cookie: "auth_token=secure_token_xyz",
		};
		const response = await client.get("/secure", headers);

		expect(response.statusCode).toBe(200);
	});

	test("Response cookie with domain attribute", async () => {
		const app = createAppCookiesResponseCookieWithDomainAttribute();
		const client = new TestClient(app);

		const json = { value: "domain_test" };
		const response = await client.post("/cookies/set-with-domain", { json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Cookie set with domain");
	});
});
