/**
 * E2E tests for cookies
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
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

	Deno.test("cookies: 25_cookie_samesite_lax", async () => {
		const app = createAppCookies25CookieSamesiteLax();
		const client = new TestClient(app);

		const headers = {
			Cookie: "tracking=track123",
		};
		const response = await client.get("/data", headers);

		assertEquals(response.statusCode, 200);
	});

	Deno.test("cookies: Optional cookie parameter - success", async () => {
		const app = createAppCookiesOptionalCookieParameterSuccess();
		const client = new TestClient(app);

		const headers = {
			Cookie: "ads_id=abc123",
		};
		const response = await client.get("/items/", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "ads_id"));
		assertEquals(responseData.ads_id, "abc123");
	});

	Deno.test("cookies: Cookie regex pattern validation - fail", async () => {
		const app = createAppCookiesCookieRegexPatternValidationFail();
		const client = new TestClient(app);

		const headers = {
			Cookie: "tracking_id=invalid-format",
		};
		const response = await client.get("/cookies/pattern", headers);

		assertEquals(response.statusCode, 422);
	});

	Deno.test("cookies: Response - session cookie no max_age", async () => {
		const app = createAppCookiesResponseSessionCookieNoMaxAge();
		const client = new TestClient(app);

		const json = { value: "session_abc123" };
		const response = await client.post("/cookies/session", { json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Session cookie set");
	});

	Deno.test("cookies: 27_cookie_httponly_flag", async () => {
		const app = createAppCookies27CookieHttponlyFlag();
		const client = new TestClient(app);

		const headers = {
			Cookie: "session=session_abc123",
		};
		const response = await client.get("/secure", headers);

		assertEquals(response.statusCode, 200);
	});

	Deno.test("cookies: Response cookie with attributes", async () => {
		const app = createAppCookiesResponseCookieWithAttributes();
		const client = new TestClient(app);

		const response = await client.get("/cookie/set");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Cookie set");
	});

	Deno.test("cookies: 24_cookie_samesite_strict", async () => {
		const app = createAppCookies24CookieSamesiteStrict();
		const client = new TestClient(app);

		const headers = {
			Cookie: "session_id=abc123xyz789",
		};
		const response = await client.get("/secure", headers);

		assertEquals(response.statusCode, 200);
	});

	Deno.test("cookies: APIKey cookie authentication - success", async () => {
		const app = createAppCookiesApikeyCookieAuthenticationSuccess();
		const client = new TestClient(app);

		const headers = {
			Cookie: "key=secret",
		};
		const response = await client.get("/users/me", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "username"));
		assertEquals(responseData.username, "secret");
	});

	Deno.test("cookies: Cookie validation - min_length constraint success", async () => {
		const app = createAppCookiesCookieValidationMinLengthConstraintSuccess();
		const client = new TestClient(app);

		const headers = {
			Cookie: "token=abc",
		};
		const response = await client.get("/cookies/min-length", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "token"));
		assertEquals(responseData.token, "abc");
	});

	Deno.test("cookies: Cookie validation - min_length failure", async () => {
		const app = createAppCookiesCookieValidationMinLengthFailure();
		const client = new TestClient(app);

		const headers = {
			Cookie: "tracking_id=ab",
		};
		const response = await client.get("/items/", headers);

		assertEquals(response.statusCode, 422);
	});

	Deno.test("cookies: Cookie validation - max_length constraint fail", async () => {
		const app = createAppCookiesCookieValidationMaxLengthConstraintFail();
		const client = new TestClient(app);

		const headers = {
			Cookie: "session_id=this_cookie_value_is_way_too_long",
		};
		const response = await client.get("/cookies/validated", headers);

		assertEquals(response.statusCode, 422);
	});

	Deno.test("cookies: Required cookie - missing", async () => {
		const app = createAppCookiesRequiredCookieMissing();
		const client = new TestClient(app);

		const headers = {
			Cookie: "fatebook_tracker=tracker456",
		};
		const response = await client.get("/items/cookies", headers);

		assertEquals(response.statusCode, 422);
	});

	Deno.test("cookies: Optional cookie parameter - missing", async () => {
		const app = createAppCookiesOptionalCookieParameterMissing();
		const client = new TestClient(app);

		const response = await client.get("/items/");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "ads_id"));
		assertEquals(responseData.ads_id, null);
	});

	Deno.test("cookies: APIKey cookie authentication - missing", async () => {
		const app = createAppCookiesApikeyCookieAuthenticationMissing();
		const client = new TestClient(app);

		const response = await client.get("/users/me/auth");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("cookies: Response - multiple cookies", async () => {
		const app = createAppCookiesResponseMultipleCookies();
		const client = new TestClient(app);

		const json = { session: "session123", user: "john" };
		const response = await client.post("/cookies/multiple", { json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Multiple cookies set");
	});

	Deno.test("cookies: Response cookie with SameSite Lax", async () => {
		const app = createAppCookiesResponseCookieWithSamesiteLax();
		const client = new TestClient(app);

		const json = { value: "lax_cookie" };
		const response = await client.post("/cookies/samesite-lax", { json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Cookie set with SameSite=Lax");
	});

	Deno.test("cookies: Response - delete cookie", async () => {
		const app = createAppCookiesResponseDeleteCookie();
		const client = new TestClient(app);

		const headers = {
			Cookie: "session=old_session_123",
		};
		const response = await client.post("/cookies/delete", { headers });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Cookie deleted");
	});

	Deno.test("cookies: Response cookie with path attribute", async () => {
		const app = createAppCookiesResponseCookieWithPathAttribute();
		const client = new TestClient(app);

		const json = { value: "path_test" };
		const response = await client.post("/cookies/set-with-path", { json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Cookie set with path");
	});

	Deno.test("cookies: Optional APIKey cookie - missing", async () => {
		const app = createAppCookiesOptionalApikeyCookieMissing();
		const client = new TestClient(app);

		const response = await client.get("/users/me");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "msg"));
		assertEquals(responseData.msg, "Create an account first");
	});

	Deno.test("cookies: Response cookie with SameSite Strict", async () => {
		const app = createAppCookiesResponseCookieWithSamesiteStrict();
		const client = new TestClient(app);

		const json = { value: "strict_cookie" };
		const response = await client.post("/cookies/samesite-strict", { json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Cookie set with SameSite=Strict");
	});

	Deno.test("cookies: Response cookie with SameSite None", async () => {
		const app = createAppCookiesResponseCookieWithSamesiteNone();
		const client = new TestClient(app);

		const json = { value: "none_cookie" };
		const response = await client.post("/cookies/samesite-none", { json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Cookie set with SameSite=None");
	});

	Deno.test("cookies: Cookie regex pattern validation - success", async () => {
		const app = createAppCookiesCookieRegexPatternValidationSuccess();
		const client = new TestClient(app);

		const headers = {
			Cookie: "tracking_id=ABC12345",
		};
		const response = await client.get("/cookies/pattern", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "tracking_id"));
		assertEquals(responseData.tracking_id, "ABC12345");
	});

	Deno.test("cookies: Response set cookie - basic", async () => {
		const app = createAppCookiesResponseSetCookieBasic();
		const client = new TestClient(app);

		const response = await client.post("/cookie/");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Come to the dark side, we have cookies");
	});

	Deno.test("cookies: Multiple cookies - success", async () => {
		const app = createAppCookiesMultipleCookiesSuccess();
		const client = new TestClient(app);

		const headers = {
			Cookie: "session_id=session123; fatebook_tracker=tracker456; googall_tracker=ga789",
		};
		const response = await client.get("/items/", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "fatebook_tracker"));
		assertEquals(responseData.fatebook_tracker, "tracker456");
		assert(Object.hasOwn(responseData, "googall_tracker"));
		assertEquals(responseData.googall_tracker, "ga789");
		assert(Object.hasOwn(responseData, "session_id"));
		assertEquals(responseData.session_id, "session123");
	});

	Deno.test("cookies: 26_cookie_secure_flag", async () => {
		const app = createAppCookies26CookieSecureFlag();
		const client = new TestClient(app);

		const headers = {
			Cookie: "auth_token=secure_token_xyz",
		};
		const response = await client.get("/secure", headers);

		assertEquals(response.statusCode, 200);
	});

	Deno.test("cookies: Response cookie with domain attribute", async () => {
		const app = createAppCookiesResponseCookieWithDomainAttribute();
		const client = new TestClient(app);

		const json = { value: "domain_test" };
		const response = await client.post("/cookies/set-with-domain", { json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Cookie set with domain");
	});