/**
 * E2E tests for headers
 * @generated
 */

import { TestClient } from "@spikard/node";
import { describe, expect, test } from "vitest";
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

describe("headers", () => {
	test("Header regex validation - success", async () => {
		const app = createAppHeadersHeaderRegexValidationSuccess();
		const client = new TestClient(app);

		const headers = {
			"X-Request-Id": "12345",
		};
		const response = await client.get("/headers/pattern", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("x_request_id");
		expect(responseData.x_request_id).toBe("12345");
	});

	test("33_api_key_header_valid", async () => {
		const app = createAppHeaders33ApiKeyHeaderValid();
		const client = new TestClient(app);

		const headers = {
			"X-API-Key": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4",
		};
		const response = await client.get("/api/data", headers);

		expect(response.statusCode).toBe(200);
	});

	test("Content-Type header - application json", async () => {
		const app = createAppHeadersContentTypeHeaderApplicationJson();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const response = await client.get("/headers/content-type", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("content_type");
		expect(responseData.content_type).toBe("application/json");
	});

	test("Accept-Language header", async () => {
		const app = createAppHeadersAcceptLanguageHeader();
		const client = new TestClient(app);

		const headers = {
			"Accept-Language": "en-US,en;q=0.9",
		};
		const response = await client.get("/headers/accept-language", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("accept_language");
		expect(responseData.accept_language).toBe("en-US,en;q=0.9");
	});

	test("X-API-Key required header - success", async () => {
		const app = createAppHeadersXApiKeyRequiredHeaderSuccess();
		const client = new TestClient(app);

		const headers = {
			key: "secret",
		};
		const response = await client.get("/users/me", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("username");
		expect(responseData.username).toBe("secret");
	});

	test("Header validation - max_length constraint fail", async () => {
		const app = createAppHeadersHeaderValidationMaxLengthConstraintFail();
		const client = new TestClient(app);

		const headers = {
			"X-Session-Id": "this_is_way_too_long_for_validation",
		};
		const response = await client.get("/headers/max-length", headers);

		expect(response.statusCode).toBe(422);
	});

	test("X-API-Key required header - missing", async () => {
		const app = createAppHeadersXApiKeyRequiredHeaderMissing();
		const client = new TestClient(app);

		const response = await client.get("/users/me");

		expect(response.statusCode).toBe(422);
	});

	test("Origin header", async () => {
		const app = createAppHeadersOriginHeader();
		const client = new TestClient(app);

		const headers = {
			Origin: "https://example.com",
		};
		const response = await client.get("/headers/origin", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("origin");
		expect(responseData.origin).toBe("https://example.com");
	});

	test("User-Agent header - default value", async () => {
		const app = createAppHeadersUserAgentHeaderDefaultValue();
		const client = new TestClient(app);

		const response = await client.get("/items/");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("User-Agent");
		expect(responseData["User-Agent"]).toBe("testclient");
	});

	test("32_bearer_token_missing_prefix", async () => {
		const app = createAppHeaders32BearerTokenMissingPrefix();
		const client = new TestClient(app);

		const headers = {
			Authorization: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
		};
		const response = await client.get("/protected", headers);

		expect(response.statusCode).toBe(422);
	});

	test("Optional header with None default - missing", async () => {
		const app = createAppHeadersOptionalHeaderWithNoneDefaultMissing();
		const client = new TestClient(app);

		const response = await client.get("/items/");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("strange_header");
		expect(responseData.strange_header).toBe(null);
	});

	test("Header regex validation - fail", async () => {
		const app = createAppHeadersHeaderRegexValidationFail();
		const client = new TestClient(app);

		const headers = {
			"X-Request-Id": "invalid-format",
		};
		const response = await client.get("/headers/pattern", headers);

		expect(response.statusCode).toBe(422);
	});

	test("31_bearer_token_format_invalid", async () => {
		const app = createAppHeaders31BearerTokenFormatInvalid();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer invalid token with spaces",
		};
		const response = await client.get("/protected", headers);

		expect(response.statusCode).toBe(422);
	});

	test("X-API-Key optional header - success", async () => {
		const app = createAppHeadersXApiKeyOptionalHeaderSuccess();
		const client = new TestClient(app);

		const headers = {
			key: "secret",
		};
		const response = await client.get("/users/me", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("msg");
		expect(responseData.msg).toBe("Hello secret");
	});

	test("Authorization header - success", async () => {
		const app = createAppHeadersAuthorizationHeaderSuccess();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Digest foobar",
		};
		const response = await client.get("/users/me", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("scheme");
		expect(responseData.scheme).toBe("Digest");
		expect(responseData).toHaveProperty("credentials");
		expect(responseData.credentials).toBe("foobar");
	});

	test("30_bearer_token_format_valid", async () => {
		const app = createAppHeaders30BearerTokenFormatValid();
		const client = new TestClient(app);

		const headers = {
			Authorization:
				"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U",
		};
		const response = await client.get("/protected", headers);

		expect(response.statusCode).toBe(200);
	});

	test("Authorization header - missing", async () => {
		const app = createAppHeadersAuthorizationHeaderMissing();
		const client = new TestClient(app);

		const response = await client.get("/users/me");

		expect(response.statusCode).toBe(422);
	});

	test("Accept header - JSON", async () => {
		const app = createAppHeadersAcceptHeaderJson();
		const client = new TestClient(app);

		const headers = {
			Accept: "application/json",
		};
		const response = await client.get("/headers/accept", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("accept");
		expect(responseData.accept).toBe("application/json");
	});

	test("Accept-Encoding header", async () => {
		const app = createAppHeadersAcceptEncodingHeader();
		const client = new TestClient(app);

		const headers = {
			"Accept-Encoding": "gzip, deflate, br",
		};
		const response = await client.get("/headers/accept-encoding", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("accept_encoding");
		expect(responseData.accept_encoding).toBe("gzip, deflate, br");
	});

	test("Authorization header - wrong scheme", async () => {
		const app = createAppHeadersAuthorizationHeaderWrongScheme();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Other invalidauthorization",
		};
		const response = await client.get("/users/me", headers);

		expect(response.statusCode).toBe(422);
	});

	test("Header validation - min_length constraint", async () => {
		const app = createAppHeadersHeaderValidationMinLengthConstraint();
		const client = new TestClient(app);

		const headers = {
			"X-Token": "ab",
		};
		const response = await client.get("/headers/validated", headers);

		expect(response.statusCode).toBe(422);
	});

	test("Basic authentication - success", async () => {
		const app = createAppHeadersBasicAuthenticationSuccess();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Basic dXNlcm5hbWU6cGFzc3dvcmQ=",
		};
		const response = await client.get("/headers/basic-auth", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("username");
		expect(responseData.username).toBe("username");
		expect(responseData).toHaveProperty("password");
		expect(responseData.password).toBe("password");
	});

	test("Bearer token authentication - missing", async () => {
		const app = createAppHeadersBearerTokenAuthenticationMissing();
		const client = new TestClient(app);

		const response = await client.get("/headers/bearer-auth");

		expect(response.statusCode).toBe(422);
	});

	test("X-API-Key optional header - missing", async () => {
		const app = createAppHeadersXApiKeyOptionalHeaderMissing();
		const client = new TestClient(app);

		const response = await client.get("/users/me");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("msg");
		expect(responseData.msg).toBe("Hello World");
	});

	test("Multiple header values - X-Token", async () => {
		const app = createAppHeadersMultipleHeaderValuesXToken();
		const client = new TestClient(app);

		const headers = {
			"x-token": "foo, bar",
		};
		const response = await client.get("/items/", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("X-Token values");
		expect(responseData["X-Token values"].length).toBe(2);
		expect(responseData["X-Token values"][0]).toBe("foo");
		expect(responseData["X-Token values"][1]).toBe("bar");
	});

	test("Multiple custom headers", async () => {
		const app = createAppHeadersMultipleCustomHeaders();
		const client = new TestClient(app);

		const headers = {
			"X-Request-Id": "req-12345",
			"X-Trace-Id": "trace-abc",
			"X-Client-Version": "1.2.3",
		};
		const response = await client.get("/headers/multiple", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("x_request_id");
		expect(responseData.x_request_id).toBe("req-12345");
		expect(responseData).toHaveProperty("x_client_version");
		expect(responseData.x_client_version).toBe("1.2.3");
		expect(responseData).toHaveProperty("x_trace_id");
		expect(responseData.x_trace_id).toBe("trace-abc");
	});

	test("34_api_key_header_invalid", async () => {
		const app = createAppHeaders34ApiKeyHeaderInvalid();
		const client = new TestClient(app);

		const headers = {
			"X-API-Key": "invalid-key",
		};
		const response = await client.get("/api/data", headers);

		expect(response.statusCode).toBe(422);
	});

	test("Bearer token authentication - success", async () => {
		const app = createAppHeadersBearerTokenAuthenticationSuccess();
		const client = new TestClient(app);

		const headers = {
			Authorization: "Bearer valid_token_123",
		};
		const response = await client.get("/headers/bearer-auth", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("token");
		expect(responseData.token).toBe("valid_token_123");
	});

	test("Host header", async () => {
		const app = createAppHeadersHostHeader();
		const client = new TestClient(app);

		const headers = {
			Host: "example.com:8080",
		};
		const response = await client.get("/headers/host", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("host");
		expect(responseData.host).toBe("example.com:8080");
	});

	test("Referer header", async () => {
		const app = createAppHeadersRefererHeader();
		const client = new TestClient(app);

		const headers = {
			Referer: "https://example.com/page",
		};
		const response = await client.get("/headers/referer", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("referer");
		expect(responseData.referer).toBe("https://example.com/page");
	});

	test("Header with underscore conversion - explicit", async () => {
		const app = createAppHeadersHeaderWithUnderscoreConversionExplicit();
		const client = new TestClient(app);

		const headers = {
			"X-Token": "secret123",
		};
		const response = await client.get("/headers/underscore", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("x_token");
		expect(responseData.x_token).toBe("secret123");
	});

	test("Header case insensitivity - access", async () => {
		const app = createAppHeadersHeaderCaseInsensitivityAccess();
		const client = new TestClient(app);

		const headers = {
			"content-type": "application/json",
		};
		const json = { test: "data" };
		const response = await client.post("/echo", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("content_type_lower");
		expect(responseData.content_type_lower).toBe("application/json");
		expect(responseData).toHaveProperty("content_type_upper");
		expect(responseData.content_type_upper).toBe("application/json");
		expect(responseData).toHaveProperty("content_type_mixed");
		expect(responseData.content_type_mixed).toBe("application/json");
	});

	test("User-Agent header - custom value", async () => {
		const app = createAppHeadersUserAgentHeaderCustomValue();
		const client = new TestClient(app);

		const headers = {
			"User-Agent": "Mozilla/5.0 Custom Browser",
		};
		const response = await client.get("/items/", headers);

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("User-Agent");
		expect(responseData["User-Agent"]).toBe("Mozilla/5.0 Custom Browser");
	});
});
