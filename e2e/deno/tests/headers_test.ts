/**
 * E2E tests for headers
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
import {
	createAppHeaders30BearerTokenFormatValid,
	createAppHeaders31BearerTokenFormatInvalid,
	createAppHeaders32BearerTokenMissingPrefix,
	createAppHeaders33ApiKeyHeaderValid,
	createAppHeaders34ApiKeyHeaderInvalid,
	createAppHeadersAcceptEncodingHeader,
	createAppHeadersAcceptHeaderJson,
	createAppHeadersAcceptLanguageHeader,
	createAppHeadersAuthorizationHeaderMissing,
	createAppHeadersAuthorizationHeaderSuccess,
	createAppHeadersAuthorizationHeaderWrongScheme,
	createAppHeadersBasicAuthenticationSuccess,
	createAppHeadersBearerTokenAuthenticationMissing,
	createAppHeadersBearerTokenAuthenticationSuccess,
	createAppHeadersContentTypeHeaderApplicationJson,
	createAppHeadersHeaderCaseInsensitivityAccess,
	createAppHeadersHeaderRegexValidationFail,
	createAppHeadersHeaderRegexValidationSuccess,
	createAppHeadersHeaderValidationMaxLengthConstraintFail,
	createAppHeadersHeaderValidationMinLengthConstraint,
	createAppHeadersHeaderWithUnderscoreConversionExplicit,
	createAppHeadersHostHeader,
	createAppHeadersMultipleCustomHeaders,
	createAppHeadersMultipleHeaderValuesXToken,
	createAppHeadersOptionalHeaderWithNoneDefaultMissing,
	createAppHeadersOriginHeader,
	createAppHeadersRefererHeader,
	createAppHeadersUserAgentHeaderCustomValue,
	createAppHeadersUserAgentHeaderDefaultValue,
	createAppHeadersXApiKeyOptionalHeaderMissing,
	createAppHeadersXApiKeyOptionalHeaderSuccess,
	createAppHeadersXApiKeyRequiredHeaderMissing,
	createAppHeadersXApiKeyRequiredHeaderSuccess,
} from "../app/main.ts";

	Deno.test("headers: Header regex validation - success", async () => {
		const app = createAppHeadersHeaderRegexValidationSuccess();
		const client = new TestClient(app);

		const headers = {
			"X-Request-Id": "12345",
		};
		const response = await client.get("/headers/pattern", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "x_request_id"));
		assertEquals(responseData.x_request_id, "12345");
	});

	Deno.test("headers: 33_api_key_header_valid", async () => {
		const app = createAppHeaders33ApiKeyHeaderValid();
		const client = new TestClient(app);

		const headers = {
			"X-API-Key": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4",
		};
		const response = await client.get("/api/data", headers);

		assertEquals(response.statusCode, 200);
	});

	Deno.test("headers: Content-Type header - application json", async () => {
		const app = createAppHeadersContentTypeHeaderApplicationJson();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const response = await client.get("/headers/content-type", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "content_type"));
		assertEquals(responseData.content_type, "application/json");
	});

	Deno.test("headers: Accept-Language header", async () => {
		const app = createAppHeadersAcceptLanguageHeader();
		const client = new TestClient(app);

		const headers = {
			"Accept-Language": "en-US,en;q=0.9",
		};
		const response = await client.get("/headers/accept-language", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "accept_language"));
		assertEquals(responseData.accept_language, "en-US,en;q=0.9");
	});

	Deno.test("headers: X-API-Key required header - success", async () => {
		const app = createAppHeadersXApiKeyRequiredHeaderSuccess();
		const client = new TestClient(app);

		const headers = {
			key: "secret",
		};
		const response = await client.get("/users/me", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "username"));
		assertEquals(responseData.username, "secret");
	});

	Deno.test("headers: Header validation - max_length constraint fail", async () => {
		const app = createAppHeadersHeaderValidationMaxLengthConstraintFail();
		const client = new TestClient(app);

		const headers = {
			"X-Session-Id": "this_is_way_too_long_for_validation",
		};
		const response = await client.get("/headers/max-length", headers);

		assertEquals(response.statusCode, 422);
	});

	Deno.test("headers: X-API-Key required header - missing", async () => {
		const app = createAppHeadersXApiKeyRequiredHeaderMissing();
		const client = new TestClient(app);

		const response = await client.get("/users/me");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("headers: Origin header", async () => {
		const app = createAppHeadersOriginHeader();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://example.com",
		};
		const response = await client.get("/headers/origin", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "origin"));
		assertEquals(responseData.origin, "https://example.com");
	});

	Deno.test("headers: User-Agent header - default value", async () => {
		const app = createAppHeadersUserAgentHeaderDefaultValue();
		const client = new TestClient(app);

		const response = await client.get("/items/");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "User-Agent"));
		assertEquals(responseData["User-Agent"], "testclient");
	});

	Deno.test("headers: 32_bearer_token_missing_prefix", async () => {
		const app = createAppHeaders32BearerTokenMissingPrefix();
		const client = new TestClient(app);

		const headers = {
			Authorization: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
		};
		const response = await client.get("/protected", headers);

		assertEquals(response.statusCode, 422);
	});

	Deno.test("headers: Optional header with None default - missing", async () => {
		const app = createAppHeadersOptionalHeaderWithNoneDefaultMissing();
		const client = new TestClient(app);

		const response = await client.get("/items/");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "strange_header"));
		assertEquals(responseData.strange_header, null);
	});

	Deno.test("headers: Header regex validation - fail", async () => {
		const app = createAppHeadersHeaderRegexValidationFail();
		const client = new TestClient(app);

		const headers = {
			"X-Request-Id": "invalid-format",
		};
		const response = await client.get("/headers/pattern", headers);

		assertEquals(response.statusCode, 422);
	});

	Deno.test("headers: 31_bearer_token_format_invalid", async () => {
		const app = createAppHeaders31BearerTokenFormatInvalid();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer invalid token with spaces",
		};
		const response = await client.get("/protected", headers);

		assertEquals(response.statusCode, 422);
	});

	Deno.test("headers: X-API-Key optional header - success", async () => {
		const app = createAppHeadersXApiKeyOptionalHeaderSuccess();
		const client = new TestClient(app);

		const headers = {
			key: "secret",
		};
		const response = await client.get("/users/me", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "msg"));
		assertEquals(responseData.msg, "Hello secret");
	});

	Deno.test("headers: Authorization header - success", async () => {
		const app = createAppHeadersAuthorizationHeaderSuccess();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Digest foobar",
		};
		const response = await client.get("/users/me", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "credentials"));
		assertEquals(responseData.credentials, "foobar");
		assert(Object.hasOwn(responseData, "scheme"));
		assertEquals(responseData.scheme, "Digest");
	});

	Deno.test("headers: 30_bearer_token_format_valid", async () => {
		const app = createAppHeaders30BearerTokenFormatValid();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U",
		};
		const response = await client.get("/protected", headers);

		assertEquals(response.statusCode, 200);
	});

	Deno.test("headers: Authorization header - missing", async () => {
		const app = createAppHeadersAuthorizationHeaderMissing();
		const client = new TestClient(app);

		const response = await client.get("/users/me");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("headers: Accept header - JSON", async () => {
		const app = createAppHeadersAcceptHeaderJson();
		const client = new TestClient(app);

		const headers = {
			Accept: "application/json",
		};
		const response = await client.get("/headers/accept", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "accept"));
		assertEquals(responseData.accept, "application/json");
	});

	Deno.test("headers: Accept-Encoding header", async () => {
		const app = createAppHeadersAcceptEncodingHeader();
		const client = new TestClient(app);

		const headers = {
			"Accept-Encoding": "gzip, deflate, br",
		};
		const response = await client.get("/headers/accept-encoding", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "accept_encoding"));
		assertEquals(responseData.accept_encoding, "gzip, deflate, br");
	});

	Deno.test("headers: Authorization header - wrong scheme", async () => {
		const app = createAppHeadersAuthorizationHeaderWrongScheme();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Other invalidauthorization",
		};
		const response = await client.get("/users/me", headers);

		assertEquals(response.statusCode, 422);
	});

	Deno.test("headers: Header validation - min_length constraint", async () => {
		const app = createAppHeadersHeaderValidationMinLengthConstraint();
		const client = new TestClient(app);

		const headers = {
			"X-Token": "ab",
		};
		const response = await client.get("/headers/validated", headers);

		assertEquals(response.statusCode, 422);
	});

	Deno.test("headers: Basic authentication - success", async () => {
		const app = createAppHeadersBasicAuthenticationSuccess();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Basic dXNlcm5hbWU6cGFzc3dvcmQ=",
		};
		const response = await client.get("/headers/basic-auth", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "password"));
		assertEquals(responseData.password, "password");
		assert(Object.hasOwn(responseData, "username"));
		assertEquals(responseData.username, "username");
	});

	Deno.test("headers: Bearer token authentication - missing", async () => {
		const app = createAppHeadersBearerTokenAuthenticationMissing();
		const client = new TestClient(app);

		const response = await client.get("/headers/bearer-auth");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("headers: X-API-Key optional header - missing", async () => {
		const app = createAppHeadersXApiKeyOptionalHeaderMissing();
		const client = new TestClient(app);

		const response = await client.get("/users/me");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "msg"));
		assertEquals(responseData.msg, "Hello World");
	});

	Deno.test("headers: Multiple header values - X-Token", async () => {
		const app = createAppHeadersMultipleHeaderValuesXToken();
		const client = new TestClient(app);

		const headers = {
			"x-token": "foo, bar",
		};
		const response = await client.get("/items/", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "X-Token values"));
		assertEquals(responseData["X-Token values"].length, 2);
		assertEquals(responseData["X-Token values"][0], "foo");
		assertEquals(responseData["X-Token values"][1], "bar");
	});

	Deno.test("headers: Multiple custom headers", async () => {
		const app = createAppHeadersMultipleCustomHeaders();
		const client = new TestClient(app);

		const headers = {
			"X-Trace-Id": "trace-abc",
			"X-Client-Version": "1.2.3",
			"X-Request-Id": "req-12345",
		};
		const response = await client.get("/headers/multiple", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "x_client_version"));
		assertEquals(responseData.x_client_version, "1.2.3");
		assert(Object.hasOwn(responseData, "x_request_id"));
		assertEquals(responseData.x_request_id, "req-12345");
		assert(Object.hasOwn(responseData, "x_trace_id"));
		assertEquals(responseData.x_trace_id, "trace-abc");
	});

	Deno.test("headers: 34_api_key_header_invalid", async () => {
		const app = createAppHeaders34ApiKeyHeaderInvalid();
		const client = new TestClient(app);

		const headers = {
			"X-API-Key": "invalid-key",
		};
		const response = await client.get("/api/data", headers);

		assertEquals(response.statusCode, 422);
	});

	Deno.test("headers: Bearer token authentication - success", async () => {
		const app = createAppHeadersBearerTokenAuthenticationSuccess();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer valid_token_123",
		};
		const response = await client.get("/headers/bearer-auth", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "token"));
		assertEquals(responseData.token, "valid_token_123");
	});

	Deno.test("headers: Host header", async () => {
		const app = createAppHeadersHostHeader();
		const client = new TestClient(app);

		const headers = {
			Host: "example.com:8080",
		};
		const response = await client.get("/headers/host", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "host"));
		assertEquals(responseData.host, "example.com:8080");
	});

	Deno.test("headers: Referer header", async () => {
		const app = createAppHeadersRefererHeader();
		const client = new TestClient(app);

		const headers = {
			Referer: "https://example.com/page",
		};
		const response = await client.get("/headers/referer", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "referer"));
		assertEquals(responseData.referer, "https://example.com/page");
	});

	Deno.test("headers: Header with underscore conversion - explicit", async () => {
		const app = createAppHeadersHeaderWithUnderscoreConversionExplicit();
		const client = new TestClient(app);

		const headers = {
			"X-Token": "secret123",
		};
		const response = await client.get("/headers/underscore", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "x_token"));
		assertEquals(responseData.x_token, "secret123");
	});

	Deno.test("headers: Header case insensitivity - access", async () => {
		const app = createAppHeadersHeaderCaseInsensitivityAccess();
		const client = new TestClient(app);

		const headers = {
			"content-type": "application/json",
		};
		const json = { test: "data" };
		const response = await client.post("/echo", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "content_type_lower"));
		assertEquals(responseData.content_type_lower, "application/json");
		assert(Object.hasOwn(responseData, "content_type_mixed"));
		assertEquals(responseData.content_type_mixed, "application/json");
		assert(Object.hasOwn(responseData, "content_type_upper"));
		assertEquals(responseData.content_type_upper, "application/json");
	});

	Deno.test("headers: User-Agent header - custom value", async () => {
		const app = createAppHeadersUserAgentHeaderCustomValue();
		const client = new TestClient(app);

		const headers = {
			"User-Agent": "Mozilla/5.0 Custom Browser",
		};
		const response = await client.get("/items/", headers);

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "User-Agent"));
		assertEquals(responseData["User-Agent"], "Mozilla/5.0 Custom Browser");
	});