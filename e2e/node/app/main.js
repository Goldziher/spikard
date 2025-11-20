import { Buffer } from "node:buffer";
import { background, StreamingResponse } from "@spikard/node";
import { z } from "zod";

function normalizeWebsocketPayload(message) {
	if (Array.isArray(message)) {
		if (message.length === 1) {
			return normalizeWebsocketPayload(message[0]);
		}
		return message.map((entry) => normalizeWebsocketPayload(entry));
	}
	if (typeof message === "string") {
		try {
			return JSON.parse(message);
		} catch {
			return message;
		}
	}
	if (Buffer?.isBuffer(message)) {
		return JSON.parse(message.toString("utf-8"));
	}
	if (message instanceof ArrayBuffer) {
		return JSON.parse(Buffer.from(message).toString("utf-8"));
	}
	if (message && typeof message === "object" && ArrayBuffer.isView(message)) {
		const view = message;
		const buffer = Buffer.from(view.buffer, view.byteOffset, view.byteLength);
		return JSON.parse(buffer.toString("utf-8"));
	}
	return message;
}
const UserNotificationMessageSchema = z.object({
	body: z.string(),
	priority: z.union([z.literal("low"), z.literal("normal"), z.literal("high"), z.literal("urgent")]).optional(),
	timestamp: z.string(),
	title: z.string(),
	type: z.literal("user_notification"),
	userId: z.string(),
});
const ChatAckMessageSchema = z.object({
	messageId: z.string(),
	status: z.union([z.literal("queued"), z.literal("delivered"), z.literal("rejected")]),
	timestamp: z.string(),
	type: z.literal("chatAck"),
});
const SystemAlertMessageSchema = z.object({
	level: z.union([z.literal("info"), z.literal("warning"), z.literal("error"), z.literal("critical")]),
	message: z.string(),
	source: z.string().optional(),
	timestamp: z.string(),
	type: z.literal("system_alert"),
});
const ChatMessageMessageSchema = z.object({
	text: z.string(),
	timestamp: z.string(),
	type: z.literal("message"),
	user: z.string(),
});
const UserJoinedMessageSchema = z.object({
	timestamp: z.string(),
	type: z.literal("userJoined"),
	user: z.string(),
});
const UserLeftMessageSchema = z.object({
	timestamp: z.string(),
	type: z.literal("userLeft"),
	user: z.string(),
});
const NotificationBatchMessageSchema = z.array(
	z.object({
		message: z.string(),
		timestamp: z.string(),
		type: z.string(),
	}),
);
const StatusUpdateMessageSchema = z.object({
	message: z.string().optional(),
	metadata: z.record(z.string(), z.unknown()).optional(),
	service: z.string(),
	status: z.union([z.literal("operational"), z.literal("degraded"), z.literal("outage"), z.literal("maintenance")]),
	timestamp: z.string(),
	type: z.literal("status_update"),
});
const BACKGROUND_STATE = {};
async function authJwtMalformedTokenFormat(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 401 };
	const responseBody = {
		detail: "Malformed JWT token: expected 3 parts separated by dots, found 2",
		status: 401,
		title: "Malformed JWT token",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthJwtMalformedTokenFormat() {
	const config = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
		},
	};
	const route = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_jwt_malformed_token_format",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_jwt_malformed_token_format: authJwtMalformedTokenFormat,
		},
		config,
	};
}
async function authBearerTokenWithoutPrefix(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 401 };
	const responseBody = {
		detail: "Authorization header must use Bearer scheme: 'Bearer <token>'",
		status: 401,
		title: "Invalid Authorization header format",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthBearerTokenWithoutPrefix() {
	const config = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
		},
	};
	const route = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_bearer_token_without_prefix",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_bearer_token_without_prefix: authBearerTokenWithoutPrefix,
		},
		config,
	};
}
async function authJwtAuthenticationValidToken(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Access granted", user_id: "user123" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthJwtAuthenticationValidToken() {
	const config = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
			audience: ["https://api.example.com"],
			issuer: "https://auth.example.com",
		},
	};
	const route = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_valid_token",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_jwt_authentication_valid_token: authJwtAuthenticationValidToken,
		},
		config,
	};
}
async function authApiKeyRotationOldKeyStillValid(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "x-api-key-deprecated": "true" };
	const responseBody = { data: "sensitive information", message: "Access granted" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthApiKeyRotationOldKeyStillValid() {
	const config = {
		apiKeyAuth: {
			keys: ["sk_test_old_123456", "sk_test_new_789012"],
			headerName: "X-API-Key",
		},
	};
	const route = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_rotation_old_key_still_valid",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "X-API-Key": { description: "API key for authentication", source: "header", type: "string" } },
			required: ["X-API-Key"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_api_key_rotation_old_key_still_valid: authApiKeyRotationOldKeyStillValid,
		},
		config,
	};
}
async function authJwtInvalidIssuer(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 401 };
	const responseBody = {
		detail: "Token issuer is invalid, expected 'https://auth.example.com'",
		status: 401,
		title: "JWT validation failed",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthJwtInvalidIssuer() {
	const config = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
			issuer: "https://auth.example.com",
			leeway: 0,
		},
	};
	const route = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_jwt_invalid_issuer",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_jwt_invalid_issuer: authJwtInvalidIssuer,
		},
		config,
	};
}
async function authJwtWithMultipleAudiences(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Access granted", user_id: "user123" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthJwtWithMultipleAudiences() {
	const config = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
			audience: ["https://api.example.com"],
			issuer: "https://auth.example.com",
		},
	};
	const route = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_jwt_with_multiple_audiences",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_jwt_with_multiple_audiences: authJwtWithMultipleAudiences,
		},
		config,
	};
}
async function authApiKeyInQueryParameter(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { data: "sensitive information", message: "Access granted" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthApiKeyInQueryParameter() {
	const config = {
		apiKeyAuth: {
			keys: ["sk_test_123456", "sk_test_789012"],
			headerName: "X-API-Key",
		},
	};
	const route = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_in_query_parameter",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_api_key_in_query_parameter: authApiKeyInQueryParameter,
		},
		config,
	};
}
async function authJwtAuthenticationExpiredToken(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 401 };
	const responseBody = {
		detail: "Token has expired",
		status: 401,
		title: "JWT validation failed",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthJwtAuthenticationExpiredToken() {
	const config = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
		},
	};
	const route = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_expired_token",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_jwt_authentication_expired_token: authJwtAuthenticationExpiredToken,
		},
		config,
	};
}
async function authApiKeyAuthenticationInvalidKey(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 401 };
	const responseBody = {
		detail: "The provided API key is not valid",
		status: 401,
		title: "Invalid API key",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthApiKeyAuthenticationInvalidKey() {
	const config = {
		apiKeyAuth: {
			keys: ["sk_test_123456", "sk_test_789012"],
			headerName: "X-API-Key",
		},
	};
	const route = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_authentication_invalid_key",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "X-API-Key": { source: "header", type: "string" } },
			required: ["X-API-Key"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_api_key_authentication_invalid_key: authApiKeyAuthenticationInvalidKey,
		},
		config,
	};
}
async function authJwtNotBeforeClaimInFuture(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 401 };
	const responseBody = {
		detail: "JWT not valid yet, not before claim is in the future",
		status: 401,
		title: "JWT validation failed",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthJwtNotBeforeClaimInFuture() {
	const config = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
			leeway: 0,
		},
	};
	const route = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_jwt_not_before_claim_in_future",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_jwt_not_before_claim_in_future: authJwtNotBeforeClaimInFuture,
		},
		config,
	};
}
async function authMultipleAuthenticationSchemesJwtPrecedence(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { auth_method: "jwt", message: "Access granted", user_id: "user123" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthMultipleAuthenticationSchemesJwtPrecedence() {
	const config = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
			audience: ["https://api.example.com"],
			issuer: "https://auth.example.com",
		},
		apiKeyAuth: {
			keys: ["sk_test_123456", "sk_test_789012"],
			headerName: "X-API-Key",
		},
	};
	const route = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_multiple_authentication_schemes_jwt_precedence",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" },
				"X-API-Key": { description: "API key for authentication", source: "header", type: "string" },
			},
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_multiple_authentication_schemes_jwt_precedence: authMultipleAuthenticationSchemesJwtPrecedence,
		},
		config,
	};
}
async function authJwtMissingRequiredCustomClaims(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 403 };
	const responseBody = {
		detail: "Required claims 'role' and 'permissions' missing from JWT",
		status: 403,
		title: "Forbidden",
		type: "https://spikard.dev/errors/forbidden",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthJwtMissingRequiredCustomClaims() {
	const config = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
			audience: ["https://api.example.com"],
			issuer: "https://auth.example.com",
		},
	};
	const route = {
		method: "GET",
		path: "/api/admin",
		handler_name: "auth_jwt_missing_required_custom_claims",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_jwt_missing_required_custom_claims: authJwtMissingRequiredCustomClaims,
		},
		config,
	};
}
async function authApiKeyAuthenticationValidKey(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { data: "sensitive information", message: "Access granted" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthApiKeyAuthenticationValidKey() {
	const config = {
		apiKeyAuth: {
			keys: ["sk_test_123456", "sk_test_789012"],
			headerName: "X-API-Key",
		},
	};
	const route = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_authentication_valid_key",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "X-API-Key": { description: "API key for authentication", source: "header", type: "string" } },
			required: ["X-API-Key"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_api_key_authentication_valid_key: authApiKeyAuthenticationValidKey,
		},
		config,
	};
}
async function authApiKeyWithCustomHeaderName(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { data: "sensitive information", message: "Access granted" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthApiKeyWithCustomHeaderName() {
	const config = {
		apiKeyAuth: {
			keys: ["sk_test_123456", "sk_test_789012"],
			headerName: "X-API-Token",
		},
	};
	const route = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_with_custom_header_name",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "X-API-Token": { description: "API token for authentication", source: "header", type: "string" } },
			required: ["X-API-Token"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_api_key_with_custom_header_name: authApiKeyWithCustomHeaderName,
		},
		config,
	};
}
async function authApiKeyAuthenticationMissingHeader(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 401 };
	const responseBody = {
		detail: "Expected 'X-API-Key' header or 'api_key' query parameter with valid API key",
		status: 401,
		title: "Missing API key",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthApiKeyAuthenticationMissingHeader() {
	const config = {
		apiKeyAuth: {
			keys: ["sk_test_123456", "sk_test_789012"],
			headerName: "X-API-Key",
		},
	};
	const route = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_authentication_missing_header",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: {}, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_api_key_authentication_missing_header: authApiKeyAuthenticationMissingHeader,
		},
		config,
	};
}
async function authJwtAuthenticationInvalidSignature(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 401 };
	const responseBody = {
		detail: "Token signature is invalid",
		status: 401,
		title: "JWT validation failed",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthJwtAuthenticationInvalidSignature() {
	const config = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
		},
	};
	const route = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_invalid_signature",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_jwt_authentication_invalid_signature: authJwtAuthenticationInvalidSignature,
		},
		config,
	};
}
async function authJwtAuthenticationMissingAuthorizationHeader(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 401 };
	const responseBody = {
		detail: "Expected 'Authorization: Bearer <token>'",
		status: 401,
		title: "Missing or invalid Authorization header",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthJwtAuthenticationMissingAuthorizationHeader() {
	const config = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
		},
	};
	const route = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_missing_authorization_header",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: {}, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_jwt_authentication_missing_authorization_header: authJwtAuthenticationMissingAuthorizationHeader,
		},
		config,
	};
}
async function authJwtAuthenticationInvalidAudience(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 401 };
	const responseBody = {
		detail: "Token audience is invalid",
		status: 401,
		title: "JWT validation failed",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppAuthJwtAuthenticationInvalidAudience() {
	const config = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
			audience: ["https://api.example.com"],
		},
	};
	const route = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_invalid_audience",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			auth_jwt_authentication_invalid_audience: authJwtAuthenticationInvalidAudience,
		},
		config,
	};
}
async function requestTimeoutRequestExceedsTimeout(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 408 };
	await new Promise((resolve) => setTimeout(resolve, 1500));
	response.body = null;
	return JSON.stringify(response);
}
function createAppRequestTimeoutRequestExceedsTimeout() {
	const config = {
		requestTimeout: 1,
	};
	const route = {
		method: "GET",
		path: "/timeouts/slow",
		handler_name: "request_timeout_request_exceeds_timeout",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			request_timeout_request_exceeds_timeout: requestTimeoutRequestExceedsTimeout,
		},
		config,
	};
}
async function requestTimeoutRequestCompletesBeforeTimeout(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	await new Promise((resolve) => setTimeout(resolve, 100));
	const responseBody = { duration: "fast", status: "ok" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppRequestTimeoutRequestCompletesBeforeTimeout() {
	const config = {
		requestTimeout: 2,
	};
	const route = {
		method: "GET",
		path: "/timeouts/fast",
		handler_name: "request_timeout_request_completes_before_timeout",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			request_timeout_request_completes_before_timeout: requestTimeoutRequestCompletesBeforeTimeout,
		},
		config,
	};
}
async function httpMethodsOptionsCorsPreflightRequest(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = {
		"access-control-allow-headers": "Content-Type",
		"access-control-allow-methods": "GET, POST, PUT, DELETE, OPTIONS",
		"access-control-allow-origin": "https://example.com",
		"access-control-max-age": "86400",
	};
	const result = {};
	response.body = result;
	return JSON.stringify(response);
}
function createAppHttpMethodsOptionsCorsPreflightRequest() {
	const route = {
		method: "OPTIONS",
		path: "/items/",
		handler_name: "http_methods_options_cors_preflight_request",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			http_methods_options_cors_preflight_request: httpMethodsOptionsCorsPreflightRequest,
		},
	};
}
async function httpMethodsDeleteRemoveResource(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppHttpMethodsDeleteRemoveResource() {
	const route = {
		method: "DELETE",
		path: "/items/{id}",
		handler_name: "http_methods_delete_remove_resource",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			http_methods_delete_remove_resource: httpMethodsDeleteRemoveResource,
		},
	};
}
async function httpMethodsPutCreateResourceIfDoesnTExist(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { id: 999, name: "New Item", price: 49.99 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHttpMethodsPutCreateResourceIfDoesnTExist() {
	const route = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_create_resource_if_doesn_t_exist",
		request_schema: {
			properties: { id: { type: "integer" }, name: { type: "string" }, price: { type: "number" } },
			required: ["id", "name", "price"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			http_methods_put_create_resource_if_doesn_t_exist: httpMethodsPutCreateResourceIfDoesnTExist,
		},
	};
}
async function httpMethodsPatchUpdateMultipleFields(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { id: 1, in_stock: false, name: "Updated Name", price: 89.99 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHttpMethodsPatchUpdateMultipleFields() {
	const route = {
		method: "PATCH",
		path: "/items/{id}",
		handler_name: "http_methods_patch_update_multiple_fields",
		request_schema: {
			properties: { in_stock: { type: "boolean" }, name: { type: "string" }, price: { type: "number" } },
			required: ["in_stock", "name", "price"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			http_methods_patch_update_multiple_fields: httpMethodsPatchUpdateMultipleFields,
		},
	};
}
async function httpMethodsPutValidationError(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const id = params.id;
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	if (id !== null && id !== void 0) {
		result.id = id;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHttpMethodsPutValidationError() {
	const route = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_validation_error",
		request_schema: {
			$schema: "https://json-schema.org/draft/2020-12/schema",
			properties: {
				id: { type: "integer" },
				name: { minLength: 3, type: "string" },
				price: { exclusiveMinimum: 0, type: "number" },
			},
			required: ["id", "name", "price"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			http_methods_put_validation_error: httpMethodsPutValidationError,
		},
	};
}
async function httpMethodsHeadGetMetadataWithoutBody(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "content-length": "85", "content-type": "application/json" };
	const result = {};
	const id = params.id;
	if (id !== null && id !== void 0) {
		result.id = id;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHttpMethodsHeadGetMetadataWithoutBody() {
	const route = {
		method: "HEAD",
		path: "/items/{id}",
		handler_name: "http_methods_head_get_metadata_without_body",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			http_methods_head_get_metadata_without_body: httpMethodsHeadGetMetadataWithoutBody,
		},
	};
}
async function httpMethodsDeleteWithResponseBody(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { id: 1, message: "Item deleted successfully", name: "Deleted Item" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHttpMethodsDeleteWithResponseBody() {
	const route = {
		method: "DELETE",
		path: "/items/{id}",
		handler_name: "http_methods_delete_with_response_body",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			http_methods_delete_with_response_body: httpMethodsDeleteWithResponseBody,
		},
	};
}
async function httpMethodsPutMissingRequiredField(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const id = params.id;
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	if (id !== null && id !== void 0) {
		result.id = id;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHttpMethodsPutMissingRequiredField() {
	const route = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_missing_required_field",
		request_schema: {
			properties: { id: { type: "integer" }, name: { type: "string" }, price: { type: "string" } },
			required: ["price"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			http_methods_put_missing_required_field: httpMethodsPutMissingRequiredField,
		},
	};
}
async function httpMethodsPatchPartialUpdate(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { id: 1, in_stock: true, name: "Existing Item", price: 79.99 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHttpMethodsPatchPartialUpdate() {
	const route = {
		method: "PATCH",
		path: "/items/{id}",
		handler_name: "http_methods_patch_partial_update",
		request_schema: { properties: { price: { type: "number" } }, required: ["price"], type: "object" },
		response_schema: void 0,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			http_methods_patch_partial_update: httpMethodsPatchPartialUpdate,
		},
	};
}
async function httpMethodsDeleteResourceNotFound(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppHttpMethodsDeleteResourceNotFound() {
	const route = {
		method: "DELETE",
		path: "/items/{id}",
		handler_name: "http_methods_delete_resource_not_found",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			http_methods_delete_resource_not_found: httpMethodsDeleteResourceNotFound,
		},
	};
}
async function httpMethodsPutIdempotentOperation(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { id: 1, name: "Fixed Name", price: 50 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHttpMethodsPutIdempotentOperation() {
	const route = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_idempotent_operation",
		request_schema: {
			properties: { id: { type: "integer" }, name: { type: "string" }, price: { type: "number" } },
			required: ["id", "name", "price"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			http_methods_put_idempotent_operation: httpMethodsPutIdempotentOperation,
		},
	};
}
async function httpMethodsPutCompleteResourceReplacement(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		description: "Completely replaced",
		id: 1,
		in_stock: true,
		name: "Updated Item",
		price: 99.99,
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHttpMethodsPutCompleteResourceReplacement() {
	const route = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_complete_resource_replacement",
		request_schema: {
			properties: {
				description: { type: "string" },
				id: { type: "integer" },
				in_stock: { type: "boolean" },
				name: { type: "string" },
				price: { type: "number" },
			},
			required: ["description", "id", "in_stock", "name", "price"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			http_methods_put_complete_resource_replacement: httpMethodsPutCompleteResourceReplacement,
		},
	};
}
async function pathParamsBooleanPathParameterTrue(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item_id: true };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsBooleanPathParameterTrue() {
	const route = {
		method: "GET",
		path: "/path/bool/{item_id}",
		handler_name: "path_params_boolean_path_parameter_true",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { source: "path", type: "boolean" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_boolean_path_parameter_true: pathParamsBooleanPathParameterTrue,
		},
	};
}
async function pathParams29DecimalPathParamSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { amount: "19.99" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParams29DecimalPathParamSuccess() {
	const route = {
		method: "GET",
		path: "/prices/{amount}",
		handler_name: "path_params_29_decimal_path_param_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { amount: { format: "decimal", source: "path", type: "string" } },
			required: ["amount"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_29_decimal_path_param_success: pathParams29DecimalPathParamSuccess,
		},
	};
}
async function pathParamsIntegerPathParameterWithCombinedLtAndGtConstraintsSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item_id: 2 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsIntegerPathParameterWithCombinedLtAndGtConstraintsSuccess() {
	const route = {
		method: "GET",
		path: "/path/param-lt-gt/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { exclusiveMaximum: 3, exclusiveMinimum: 1, source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success:
				pathParamsIntegerPathParameterWithCombinedLtAndGtConstraintsSuccess,
		},
	};
}
async function pathParams33StringPatternPathSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { owner: "spikard-labs", repo: "spikard-http" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParams33StringPatternPathSuccess() {
	const route = {
		method: "GET",
		path: "/repos/{owner}/{repo}",
		handler_name: "path_params_33_string_pattern_path_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				owner: { pattern: "^[a-zA-Z0-9-]+$", source: "path", type: "string" },
				repo: { pattern: "^[a-zA-Z0-9-_]+$", source: "path", type: "string" },
			},
			required: ["owner", "repo"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_33_string_pattern_path_success: pathParams33StringPatternPathSuccess,
		},
	};
}
async function pathParams31StringMinlengthPathFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const username = params.username;
	if (username !== null && username !== void 0) {
		result.username = username;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppPathParams31StringMinlengthPathFailure() {
	const route = {
		method: "GET",
		path: "/users/{username}",
		handler_name: "path_params_31_string_minlength_path_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { username: { minLength: 3, source: "path", type: "string" } },
			required: ["username"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_31_string_minlength_path_failure: pathParams31StringMinlengthPathFailure,
		},
	};
}
async function pathParams35NegativeIntegerPathParam(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { value: -100 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParams35NegativeIntegerPathParam() {
	const route = {
		method: "GET",
		path: "/offset/{value}",
		handler_name: "path_params_35_negative_integer_path_param",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { value: { source: "path", type: "integer" } },
			required: ["value"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_35_negative_integer_path_param: pathParams35NegativeIntegerPathParam,
		},
	};
}
async function pathParamsEnumPathParameterInvalidValue(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const modelName = params.model_name;
	if (modelName !== null && modelName !== void 0) {
		result.model_name = modelName;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppPathParamsEnumPathParameterInvalidValue() {
	const route = {
		method: "GET",
		path: "/models/{model_name}",
		handler_name: "path_params_enum_path_parameter_invalid_value",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { model_name: { enum: ["alexnet", "resnet", "lenet"], source: "path", type: "string" } },
			required: ["model_name"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_enum_path_parameter_invalid_value: pathParamsEnumPathParameterInvalidValue,
		},
	};
}
async function pathParams27DatetimeFormatPathParamSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { timestamp: "2025-10-30T14:30:00Z" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParams27DatetimeFormatPathParamSuccess() {
	const route = {
		method: "GET",
		path: "/bookings/{timestamp}",
		handler_name: "path_params_27_datetime_format_path_param_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { timestamp: { format: "date-time", source: "path", type: "string" } },
			required: ["timestamp"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_27_datetime_format_path_param_success: pathParams27DatetimeFormatPathParamSuccess,
		},
	};
}
async function pathParams25DateFormatInvalidFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const date = params.date;
	if (date !== null && date !== void 0) {
		result.date = date.toISOString();
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppPathParams25DateFormatInvalidFailure() {
	const route = {
		method: "GET",
		path: "/events/{date}",
		handler_name: "path_params_25_date_format_invalid_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { date: { format: "date", source: "path", type: "string" } },
			required: ["date"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_25_date_format_invalid_failure: pathParams25DateFormatInvalidFailure,
		},
	};
}
async function pathParamsIntegerPathParameterWithLtConstraintSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item_id: 2 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsIntegerPathParameterWithLtConstraintSuccess() {
	const route = {
		method: "GET",
		path: "/path/param-lt/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_lt_constraint_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { exclusiveMaximum: 3, source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_lt_constraint_success:
				pathParamsIntegerPathParameterWithLtConstraintSuccess,
		},
	};
}
async function pathParamsIntegerPathParameterWithGtConstraintSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item_id: 42 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsIntegerPathParameterWithGtConstraintSuccess() {
	const route = {
		method: "GET",
		path: "/path/param-gt/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_gt_constraint_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { exclusiveMinimum: 3, source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_gt_constraint_success:
				pathParamsIntegerPathParameterWithGtConstraintSuccess,
		},
	};
}
async function pathParams28DurationFormatPathParamSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { duration: "P1DT2H30M" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParams28DurationFormatPathParamSuccess() {
	const route = {
		method: "GET",
		path: "/delays/{duration}",
		handler_name: "path_params_28_duration_format_path_param_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { duration: { format: "duration", source: "path", type: "string" } },
			required: ["duration"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_28_duration_format_path_param_success: pathParams28DurationFormatPathParamSuccess,
		},
	};
}
async function pathParamsPathParameterTypeSyntaxWithOverride(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { count: "50" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsPathParameterTypeSyntaxWithOverride() {
	const route = {
		method: "GET",
		path: "/type-syntax/items-count/{count:int}",
		handler_name: "path_params_path_parameter_type_syntax_with_override",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { count: { maximum: 100, minimum: 1, source: "path", type: "integer" } },
			required: ["count"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_path_parameter_type_syntax_with_override: pathParamsPathParameterTypeSyntaxWithOverride,
		},
	};
}
async function pathParams20UuidV3PathParamSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { id: "e8b5a51d-11c8-3310-a6ab-367563f20686" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParams20UuidV3PathParamSuccess() {
	const route = {
		method: "GET",
		path: "/items/{id}",
		handler_name: "path_params_20_uuid_v3_path_param_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { id: { format: "uuid", source: "path", type: "string", uuidVersion: "3" } },
			required: ["id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_20_uuid_v3_path_param_success: pathParams20UuidV3PathParamSuccess,
		},
	};
}
async function pathParamsIntegerPathParameterInvalidString(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const itemId = params.item_id;
	if (itemId !== null && itemId !== void 0) {
		result.item_id = itemId;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppPathParamsIntegerPathParameterInvalidString() {
	const route = {
		method: "GET",
		path: "/path/int/{item_id}",
		handler_name: "path_params_integer_path_parameter_invalid_string",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_invalid_string: pathParamsIntegerPathParameterInvalidString,
		},
	};
}
async function pathParams30StringMinlengthPathSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { username: "alice" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParams30StringMinlengthPathSuccess() {
	const route = {
		method: "GET",
		path: "/users/{username}",
		handler_name: "path_params_30_string_minlength_path_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { username: { minLength: 3, source: "path", type: "string" } },
			required: ["username"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_30_string_minlength_path_success: pathParams30StringMinlengthPathSuccess,
		},
	};
}
async function pathParamsIntegerPathParameterWithLeConstraintSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item_id: 3 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsIntegerPathParameterWithLeConstraintSuccess() {
	const route = {
		method: "GET",
		path: "/path/param-le/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_le_constraint_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { maximum: 3, source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_le_constraint_success:
				pathParamsIntegerPathParameterWithLeConstraintSuccess,
		},
	};
}
async function pathParamsPathParameterTypeSyntaxInvalidUuid(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	response.body = result;
	return JSON.stringify(response);
}
function createAppPathParamsPathParameterTypeSyntaxInvalidUuid() {
	const route = {
		method: "GET",
		path: "/type-syntax/items/{id:uuid}",
		handler_name: "path_params_path_parameter_type_syntax_invalid_uuid",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_path_parameter_type_syntax_invalid_uuid: pathParamsPathParameterTypeSyntaxInvalidUuid,
		},
	};
}
async function pathParamsPathTypeParameterFilePath(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { file_path: "home/johndoe/myfile.txt" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsPathTypeParameterFilePath() {
	const route = {
		method: "GET",
		path: "/files/{file_path:path}",
		handler_name: "path_params_path_type_parameter_file_path",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { file_path: { source: "path", type: "string" } },
			required: ["file_path"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_path_type_parameter_file_path: pathParamsPathTypeParameterFilePath,
		},
	};
}
async function pathParamsPathParameterWithTypeSyntaxUuid(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { id: "550e8400-e29b-41d4-a716-446655440000" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsPathParameterWithTypeSyntaxUuid() {
	const route = {
		method: "GET",
		path: "/type-syntax/items/{id:uuid}",
		handler_name: "path_params_path_parameter_with_type_syntax_uuid",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_path_parameter_with_type_syntax_uuid: pathParamsPathParameterWithTypeSyntaxUuid,
		},
	};
}
async function pathParams32StringMaxlengthPathFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const username = params.username;
	if (username !== null && username !== void 0) {
		result.username = username;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppPathParams32StringMaxlengthPathFailure() {
	const route = {
		method: "GET",
		path: "/users/{username}",
		handler_name: "path_params_32_string_maxlength_path_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { username: { maxLength: 20, source: "path", type: "string" } },
			required: ["username"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_32_string_maxlength_path_failure: pathParams32StringMaxlengthPathFailure,
		},
	};
}
async function pathParamsIntegerPathParameterSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item_id: 42 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsIntegerPathParameterSuccess() {
	const route = {
		method: "GET",
		path: "/path/int/{item_id}",
		handler_name: "path_params_integer_path_parameter_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_success: pathParamsIntegerPathParameterSuccess,
		},
	};
}
async function pathParams34StringPatternPathFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const owner = params.owner;
	if (owner !== null && owner !== void 0) {
		result.owner = owner;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppPathParams34StringPatternPathFailure() {
	const route = {
		method: "GET",
		path: "/repos/{owner}",
		handler_name: "path_params_34_string_pattern_path_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { owner: { pattern: "^[a-zA-Z0-9-]+$", source: "path", type: "string" } },
			required: ["owner"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_34_string_pattern_path_failure: pathParams34StringPatternPathFailure,
		},
	};
}
async function pathParams21UuidV5PathParamSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { id: "630eb68f-e0fa-5ecc-887a-7c7a62614681" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParams21UuidV5PathParamSuccess() {
	const route = {
		method: "GET",
		path: "/items/{id}",
		handler_name: "path_params_21_uuid_v5_path_param_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { id: { format: "uuid", source: "path", type: "string", uuidVersion: "5" } },
			required: ["id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_21_uuid_v5_path_param_success: pathParams21UuidV5PathParamSuccess,
		},
	};
}
async function pathParamsStringPathParameterWithMaxLengthFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const itemId = params.item_id;
	if (itemId !== null && itemId !== void 0) {
		result.item_id = itemId;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppPathParamsStringPathParameterWithMaxLengthFailure() {
	const route = {
		method: "GET",
		path: "/path/param-maxlength/{item_id}",
		handler_name: "path_params_string_path_parameter_with_max_length_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { maxLength: 3, source: "path", type: "string" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_string_path_parameter_with_max_length_failure: pathParamsStringPathParameterWithMaxLengthFailure,
		},
	};
}
async function pathParamsStringPathParameterWithMinLengthFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const itemId = params.item_id;
	if (itemId !== null && itemId !== void 0) {
		result.item_id = itemId;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppPathParamsStringPathParameterWithMinLengthFailure() {
	const route = {
		method: "GET",
		path: "/path/param-minlength/{item_id}",
		handler_name: "path_params_string_path_parameter_with_min_length_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { minLength: 3, source: "path", type: "string" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_string_path_parameter_with_min_length_failure: pathParamsStringPathParameterWithMinLengthFailure,
		},
	};
}
async function pathParamsMultiplePathParametersSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		order_id: "c892496f-b1fd-4b91-bdb8-b46f92df1716",
		service_id: 1,
		user_id: "abc",
		version: 1,
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsMultiplePathParametersSuccess() {
	const route = {
		method: "GET",
		path: "/{version}/{service_id}/{user_id}/{order_id}",
		handler_name: "path_params_multiple_path_parameters_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				order_id: { format: "uuid", source: "path", type: "string" },
				service_id: { source: "path", type: "integer" },
				user_id: { source: "path", type: "string" },
				version: { source: "path", type: "number" },
			},
			required: ["order_id", "service_id", "user_id", "version"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_multiple_path_parameters_success: pathParamsMultiplePathParametersSuccess,
		},
	};
}
async function pathParamsDatePathParameterSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { date_param: "2023-07-15" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsDatePathParameterSuccess() {
	const route = {
		method: "GET",
		path: "/date/{date_param}",
		handler_name: "path_params_date_path_parameter_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { date_param: { format: "date", source: "path", type: "string" } },
			required: ["date_param"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_date_path_parameter_success: pathParamsDatePathParameterSuccess,
		},
	};
}
async function pathParamsIntegerPathParameterWithGtConstraintFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const itemId = params.item_id;
	if (itemId !== null && itemId !== void 0) {
		result.item_id = itemId;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppPathParamsIntegerPathParameterWithGtConstraintFailure() {
	const route = {
		method: "GET",
		path: "/path/param-gt/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_gt_constraint_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { exclusiveMinimum: 3, source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_gt_constraint_failure:
				pathParamsIntegerPathParameterWithGtConstraintFailure,
		},
	};
}
async function pathParams24DateFormatPathParamSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { date: "2025-10-30" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParams24DateFormatPathParamSuccess() {
	const route = {
		method: "GET",
		path: "/events/{date}",
		handler_name: "path_params_24_date_format_path_param_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { date: { format: "date", source: "path", type: "string" } },
			required: ["date"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_24_date_format_path_param_success: pathParams24DateFormatPathParamSuccess,
		},
	};
}
async function pathParamsFloatPathParameterSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item_id: 42.5 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsFloatPathParameterSuccess() {
	const route = {
		method: "GET",
		path: "/path/float/{item_id}",
		handler_name: "path_params_float_path_parameter_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { source: "path", type: "number" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_float_path_parameter_success: pathParamsFloatPathParameterSuccess,
		},
	};
}
async function pathParamsPathParameterWithTypeSyntaxInteger(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { user_id: "42" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsPathParameterWithTypeSyntaxInteger() {
	const route = {
		method: "GET",
		path: "/type-syntax/users/{user_id:int}",
		handler_name: "path_params_path_parameter_with_type_syntax_integer",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_path_parameter_with_type_syntax_integer: pathParamsPathParameterWithTypeSyntaxInteger,
		},
	};
}
async function pathParamsStringPathParameterSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item_id: "foobar" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsStringPathParameterSuccess() {
	const route = {
		method: "GET",
		path: "/path/str/{item_id}",
		handler_name: "path_params_string_path_parameter_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { source: "path", type: "string" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_string_path_parameter_success: pathParamsStringPathParameterSuccess,
		},
	};
}
async function pathParamsUuidPathParameterSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item_id: "ec38df32-ceda-4cfa-9b4a-1aeb94ad551a" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsUuidPathParameterSuccess() {
	const route = {
		method: "GET",
		path: "/items/{item_id}",
		handler_name: "path_params_uuid_path_parameter_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { format: "uuid", source: "path", type: "string" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_uuid_path_parameter_success: pathParamsUuidPathParameterSuccess,
		},
	};
}
async function pathParamsIntegerPathParameterWithGeConstraintSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item_id: 3 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsIntegerPathParameterWithGeConstraintSuccess() {
	const route = {
		method: "GET",
		path: "/path/param-ge/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_ge_constraint_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { minimum: 3, source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_ge_constraint_success:
				pathParamsIntegerPathParameterWithGeConstraintSuccess,
		},
	};
}
async function pathParamsEnumPathParameterSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { model_name: "alexnet" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsEnumPathParameterSuccess() {
	const route = {
		method: "GET",
		path: "/models/{model_name}",
		handler_name: "path_params_enum_path_parameter_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { model_name: { enum: ["alexnet", "lenet", "resnet"], source: "path", type: "string" } },
			required: ["model_name"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_enum_path_parameter_success: pathParamsEnumPathParameterSuccess,
		},
	};
}
async function pathParamsBooleanPathParameterNumeric1(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item_id: true };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppPathParamsBooleanPathParameterNumeric1() {
	const route = {
		method: "GET",
		path: "/path/bool/{item_id}",
		handler_name: "path_params_boolean_path_parameter_numeric_1",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { source: "path", type: "boolean" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			path_params_boolean_path_parameter_numeric_1: pathParamsBooleanPathParameterNumeric1,
		},
	};
}
function createAppStaticFilesStaticFileServerReturnsTextFile() {
	const config = {
		staticFiles: [
			{
				directory: new URL(
					"./static_assets/static_files_static_file_server_returns_text_file/public_0",
					import.meta.url,
				).pathname,
				routePrefix: "/public",
				cacheControl: "public, max-age=60",
			},
		],
	};
	return {
		routes: [],
		handlers: {},
		config,
	};
}
function createAppStaticFilesStaticServerReturnsIndexHtmlForDirectory() {
	const config = {
		staticFiles: [
			{
				directory: new URL(
					"./static_assets/static_files_static_server_returns_index_html_for_directory/app_0",
					import.meta.url,
				).pathname,
				routePrefix: "/app",
			},
		],
	};
	return {
		routes: [],
		handlers: {},
		config,
	};
}
async function backgroundBackgroundEventLoggingSecondPayload(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 202 };
	response.headers = { "content-type": "application/json" };
	BACKGROUND_STATE.background_background_event_logging_second_payload =
		BACKGROUND_STATE.background_background_event_logging_second_payload ?? [];
	const state = BACKGROUND_STATE.background_background_event_logging_second_payload;
	const value = body && typeof body === "object" ? body.event : void 0;
	if (value === void 0 || value === null) {
		throw new Error("background task requires request body value");
	}
	background.run(async () => {
		state.push(value);
	});
	response.body = null;
	return JSON.stringify(response);
}
function _background_background_event_logging_second_payload_background_state() {
	const state = BACKGROUND_STATE.background_background_event_logging_second_payload ?? [];
	return { events: state };
}
function createAppBackgroundBackgroundEventLoggingSecondPayload() {
	const route = {
		method: "POST",
		path: "/background/events",
		handler_name: "background_background_event_logging_second_payload",
		request_schema: {
			additionalProperties: false,
			properties: { event: { type: "string" } },
			required: ["event"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	const backgroundRoute = {
		method: "GET",
		path: "/background/events",
		handler_name: "background_background_event_logging_second_payload_background_state",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route, backgroundRoute],
		handlers: {
			background_background_event_logging_second_payload: backgroundBackgroundEventLoggingSecondPayload,
			background_background_event_logging_second_payload_background_state:
				backgroundBackgroundEventLoggingSecondPayloadBackgroundState,
		},
	};
}
async function backgroundBackgroundEventLogging(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 202 };
	response.headers = { "content-type": "application/json" };
	BACKGROUND_STATE.background_background_event_logging = BACKGROUND_STATE.background_background_event_logging ?? [];
	const state = BACKGROUND_STATE.background_background_event_logging;
	const value = body && typeof body === "object" ? body.event : void 0;
	if (value === void 0 || value === null) {
		throw new Error("background task requires request body value");
	}
	background.run(async () => {
		state.push(value);
	});
	response.body = null;
	return JSON.stringify(response);
}
function _background_background_event_logging_background_state() {
	const state = BACKGROUND_STATE.background_background_event_logging ?? [];
	return { events: state };
}
function createAppBackgroundBackgroundEventLogging() {
	const route = {
		method: "POST",
		path: "/background/events",
		handler_name: "background_background_event_logging",
		request_schema: {
			additionalProperties: false,
			properties: { event: { type: "string" } },
			required: ["event"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	const backgroundRoute = {
		method: "GET",
		path: "/background/events",
		handler_name: "background_background_event_logging_background_state",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route, backgroundRoute],
		handlers: {
			background_background_event_logging: backgroundBackgroundEventLogging,
			background_background_event_logging_background_state: backgroundBackgroundEventLoggingBackgroundState,
		},
	};
}
async function requestIdRequestIdHeaderIsPreserved(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "x-request-id": "trace-123" };
	const responseBody = { echo: "trace-123", status: "preserved" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppRequestIdRequestIdHeaderIsPreserved() {
	const route = {
		method: "GET",
		path: "/request-id/preserved",
		handler_name: "request_id_request_id_header_is_preserved",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			request_id_request_id_header_is_preserved: requestIdRequestIdHeaderIsPreserved,
		},
	};
}
async function requestIdRequestIdMiddlewareCanBeDisabled(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { status: "no-request-id" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppRequestIdRequestIdMiddlewareCanBeDisabled() {
	const config = {
		enableRequestId: false,
	};
	const route = {
		method: "GET",
		path: "/request-id/disabled",
		handler_name: "request_id_request_id_middleware_can_be_disabled",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			request_id_request_id_middleware_can_be_disabled: requestIdRequestIdMiddlewareCanBeDisabled,
		},
		config,
	};
}
async function requestIdRequestIdIsGeneratedWhenNotProvided(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "x-request-id": "00000000-0000-4000-8000-000000000000" };
	const responseBody = { status: "generated" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppRequestIdRequestIdIsGeneratedWhenNotProvided() {
	const config = {
		enableRequestId: true,
	};
	const route = {
		method: "GET",
		path: "/request-id/generated",
		handler_name: "request_id_request_id_is_generated_when_not_provided",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			request_id_request_id_is_generated_when_not_provided: requestIdRequestIdIsGeneratedWhenNotProvided,
		},
		config,
	};
}
async function queryParamsStringValidationWithRegexSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item_query: "fixedquery" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsStringValidationWithRegexSuccess() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "query_params_string_validation_with_regex_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_query: { annotation: "str", pattern: "^fixedquery$", source: "query", type: "string" } },
			required: ["item_query"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_string_validation_with_regex_success: queryParamsStringValidationWithRegexSuccess,
		},
	};
}
async function queryParams49IntegerGtConstraintSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { limit: 5 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams49IntegerGtConstraintSuccess() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_49_integer_gt_constraint_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { limit: { exclusiveMinimum: 0, source: "query", type: "integer" } },
			required: ["limit"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_49_integer_gt_constraint_success: queryParams49IntegerGtConstraintSuccess,
		},
	};
}
async function queryParamsEnumQueryParameterInvalidValue(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const model = params.model;
	if (model !== null && model !== void 0) {
		result.model = model;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParamsEnumQueryParameterInvalidValue() {
	const route = {
		method: "GET",
		path: "/query/enum",
		handler_name: "query_params_enum_query_parameter_invalid_value",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				model: { annotation: "str", enum: ["alexnet", "resnet", "lenet"], source: "query", type: "string" },
			},
			required: ["model"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_enum_query_parameter_invalid_value: queryParamsEnumQueryParameterInvalidValue,
		},
	};
}
async function queryParams68ArrayUniqueitemsSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { ids: [1, 2, 3, 4] };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams68ArrayUniqueitemsSuccess() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_68_array_uniqueitems_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { ids: { items: { type: "integer" }, source: "query", type: "array", uniqueItems: true } },
			required: ["ids"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_68_array_uniqueitems_success: queryParams68ArrayUniqueitemsSuccess,
		},
	};
}
async function queryParams47PatternValidationEmailSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { email: "user@example.com" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams47PatternValidationEmailSuccess() {
	const route = {
		method: "GET",
		path: "/subscribe",
		handler_name: "query_params_47_pattern_validation_email_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				email: { pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$", source: "query", type: "string" },
			},
			required: ["email"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_47_pattern_validation_email_success: queryParams47PatternValidationEmailSuccess,
		},
	};
}
async function queryParamsRequiredIntegerQueryParameterSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = "foo bar 42";
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsRequiredIntegerQueryParameterSuccess() {
	const route = {
		method: "GET",
		path: "/query/int",
		handler_name: "query_params_required_integer_query_parameter_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { query: { annotation: "int", source: "query", type: "integer" } },
			required: ["query"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_success: queryParamsRequiredIntegerQueryParameterSuccess,
		},
	};
}
async function queryParamsRequiredStringQueryParameterMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const query = params.query;
	if (query !== null && query !== void 0) {
		result.query = query;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParamsRequiredStringQueryParameterMissing() {
	const route = {
		method: "GET",
		path: "/query",
		handler_name: "query_params_required_string_query_parameter_missing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { query: { annotation: "str", source: "query", type: "string" } },
			required: ["query"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_required_string_query_parameter_missing: queryParamsRequiredStringQueryParameterMissing,
		},
	};
}
async function queryParams57BooleanEmptyStringCoercion(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { active: false };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams57BooleanEmptyStringCoercion() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_57_boolean_empty_string_coercion",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { active: { source: "query", type: "boolean" } },
			required: ["active"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_57_boolean_empty_string_coercion: queryParams57BooleanEmptyStringCoercion,
		},
	};
}
async function queryParams52IntegerLeConstraintBoundary(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { limit: 100 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams52IntegerLeConstraintBoundary() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_52_integer_le_constraint_boundary",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { limit: { maximum: 100, source: "query", type: "integer" } },
			required: ["limit"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_52_integer_le_constraint_boundary: queryParams52IntegerLeConstraintBoundary,
		},
	};
}
async function queryParamsListWithDefaultEmptyArrayNoValuesProvided(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppQueryParamsListWithDefaultEmptyArrayNoValuesProvided() {
	const route = {
		method: "GET",
		path: "/query/list-default",
		handler_name: "query_params_list_with_default_empty_array_no_values_provided",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				tags: { annotation: "list[str]", default: [], items: { type: "string" }, source: "query", type: "array" },
			},
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_list_with_default_empty_array_no_values_provided:
				queryParamsListWithDefaultEmptyArrayNoValuesProvided,
		},
	};
}
async function queryParamsDateQueryParameterSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { event_date: "2024-01-15" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsDateQueryParameterSuccess() {
	const route = {
		method: "GET",
		path: "/query/date",
		handler_name: "query_params_date_query_parameter_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { event_date: { annotation: "str", format: "date", source: "query", type: "string" } },
			required: ["event_date"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_date_query_parameter_success: queryParamsDateQueryParameterSuccess,
		},
	};
}
async function queryParamsStringQueryParamWithMaxLengthConstraintFail(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const name = params.name;
	if (name !== null && name !== void 0) {
		result.name = name;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParamsStringQueryParamWithMaxLengthConstraintFail() {
	const route = {
		method: "GET",
		path: "/query/str-max-length",
		handler_name: "query_params_string_query_param_with_max_length_constraint_fail",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { name: { annotation: "str", maxLength: 10, source: "query", type: "string" } },
			required: ["name"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_string_query_param_with_max_length_constraint_fail:
				queryParamsStringQueryParamWithMaxLengthConstraintFail,
		},
	};
}
async function queryParams45StringMinlengthValidationFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const term = params.term;
	if (term !== null && term !== void 0) {
		result.term = term;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParams45StringMinlengthValidationFailure() {
	const route = {
		method: "GET",
		path: "/search",
		handler_name: "query_params_45_string_minlength_validation_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { term: { minLength: 3, source: "query", type: "string" } },
			required: ["term"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_45_string_minlength_validation_failure: queryParams45StringMinlengthValidationFailure,
		},
	};
}
async function queryParamsIntegerWithDefaultValueOverride(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = "foo bar 50";
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsIntegerWithDefaultValueOverride() {
	const route = {
		method: "GET",
		path: "/query/int/default",
		handler_name: "query_params_integer_with_default_value_override",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { query: { annotation: "int", default: 10, source: "query", type: "integer" } },
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_integer_with_default_value_override: queryParamsIntegerWithDefaultValueOverride,
		},
	};
}
async function queryParams67MultipleofConstraintFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const quantity = params.quantity;
	if (quantity !== null && quantity !== void 0) {
		result.quantity = quantity;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParams67MultipleofConstraintFailure() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_67_multipleof_constraint_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { quantity: { multipleOf: 5, source: "query", type: "integer" } },
			required: ["quantity"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_67_multipleof_constraint_failure: queryParams67MultipleofConstraintFailure,
		},
	};
}
async function queryParams58FormatEmailSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { email: "user@example.com" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams58FormatEmailSuccess() {
	const route = {
		method: "GET",
		path: "/subscribe",
		handler_name: "query_params_58_format_email_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { email: { format: "email", source: "query", type: "string" } },
			required: ["email"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_58_format_email_success: queryParams58FormatEmailSuccess,
		},
	};
}
async function queryParamsIntegerQueryParamWithGeConstraintBoundary(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { value: 10 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsIntegerQueryParamWithGeConstraintBoundary() {
	const route = {
		method: "GET",
		path: "/query/int-ge",
		handler_name: "query_params_integer_query_param_with_ge_constraint_boundary",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { value: { annotation: "int", minimum: 10, source: "query", type: "integer" } },
			required: ["value"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_integer_query_param_with_ge_constraint_boundary:
				queryParamsIntegerQueryParamWithGeConstraintBoundary,
		},
	};
}
async function queryParamsIntegerQueryParamWithGtConstraintValid(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { value: 1 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsIntegerQueryParamWithGtConstraintValid() {
	const route = {
		method: "GET",
		path: "/query/int-gt",
		handler_name: "query_params_integer_query_param_with_gt_constraint_valid",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { value: { annotation: "int", exclusiveMinimum: 0, source: "query", type: "integer" } },
			required: ["value"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_integer_query_param_with_gt_constraint_valid: queryParamsIntegerQueryParamWithGtConstraintValid,
		},
	};
}
async function queryParamsRequiredIntegerQueryParameterInvalidType(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const query = params.query;
	if (query !== null && query !== void 0) {
		result.query = query;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParamsRequiredIntegerQueryParameterInvalidType() {
	const route = {
		method: "GET",
		path: "/query/int",
		handler_name: "query_params_required_integer_query_parameter_invalid_type",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { query: { annotation: "int", source: "query", type: "integer" } },
			required: ["query"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_invalid_type: queryParamsRequiredIntegerQueryParameterInvalidType,
		},
	};
}
async function queryParamsRequiredIntegerQueryParameterFloatValue(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const query = params.query;
	if (query !== null && query !== void 0) {
		result.query = query;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParamsRequiredIntegerQueryParameterFloatValue() {
	const route = {
		method: "GET",
		path: "/query/int",
		handler_name: "query_params_required_integer_query_parameter_float_value",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { query: { annotation: "int", source: "query", type: "integer" } },
			required: ["query"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_float_value: queryParamsRequiredIntegerQueryParameterFloatValue,
		},
	};
}
async function queryParamsQueryParameterWithUrlEncodedSpecialCharacters(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { name: "test&value=123" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsQueryParameterWithUrlEncodedSpecialCharacters() {
	const route = {
		method: "GET",
		path: "/query/basic",
		handler_name: "query_params_query_parameter_with_url_encoded_special_characters",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { name: { annotation: "str", source: "query", type: "string" } },
			required: ["name"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_query_parameter_with_url_encoded_special_characters:
				queryParamsQueryParameterWithUrlEncodedSpecialCharacters,
		},
	};
}
async function queryParams59FormatEmailFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const email = params.email;
	if (email !== null && email !== void 0) {
		result.email = email;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParams59FormatEmailFailure() {
	const route = {
		method: "GET",
		path: "/subscribe",
		handler_name: "query_params_59_format_email_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { email: { format: "email", source: "query", type: "string" } },
			required: ["email"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_59_format_email_failure: queryParams59FormatEmailFailure,
		},
	};
}
async function queryParams43ScientificNotationFloat(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { threshold: 15e-4 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams43ScientificNotationFloat() {
	const route = {
		method: "GET",
		path: "/stats",
		handler_name: "query_params_43_scientific_notation_float",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { threshold: { annotation: "float", source: "query", type: "number" } },
			required: ["threshold"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_43_scientific_notation_float: queryParams43ScientificNotationFloat,
		},
	};
}
async function queryParams63FormatUriSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { url: "https://example.com/path?query=value" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams63FormatUriSuccess() {
	const route = {
		method: "GET",
		path: "/redirect",
		handler_name: "query_params_63_format_uri_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { url: { format: "uri", source: "query", type: "string" } },
			required: ["url"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_63_format_uri_success: queryParams63FormatUriSuccess,
		},
	};
}
async function queryParamsBooleanQueryParameterNumeric1(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { flag: true };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsBooleanQueryParameterNumeric1() {
	const route = {
		method: "GET",
		path: "/query/bool",
		handler_name: "query_params_boolean_query_parameter_numeric_1",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { flag: { annotation: "bool", source: "query", type: "boolean" } },
			required: ["flag"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_boolean_query_parameter_numeric_1: queryParamsBooleanQueryParameterNumeric1,
		},
	};
}
async function queryParamsStringQueryParamWithMinLengthConstraintFail(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const name = params.name;
	if (name !== null && name !== void 0) {
		result.name = name;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParamsStringQueryParamWithMinLengthConstraintFail() {
	const route = {
		method: "GET",
		path: "/query/str-min-length",
		handler_name: "query_params_string_query_param_with_min_length_constraint_fail",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { name: { annotation: "str", minLength: 3, source: "query", type: "string" } },
			required: ["name"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_string_query_param_with_min_length_constraint_fail:
				queryParamsStringQueryParamWithMinLengthConstraintFail,
		},
	};
}
async function queryParamsOptionalStringQueryParameterProvided(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = "foo bar baz";
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsOptionalStringQueryParameterProvided() {
	const route = {
		method: "GET",
		path: "/query/optional",
		handler_name: "query_params_optional_string_query_parameter_provided",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { query: { annotation: "str", source: "query", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_optional_string_query_parameter_provided: queryParamsOptionalStringQueryParameterProvided,
		},
	};
}
async function queryParamsListOfIntegersMultipleValues(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = [1, 2];
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsListOfIntegersMultipleValues() {
	const route = {
		method: "GET",
		path: "/query/list",
		handler_name: "query_params_list_of_integers_multiple_values",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				device_ids: { annotation: "list[int]", items: { type: "integer" }, source: "query", type: "array" },
			},
			required: ["device_ids"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_list_of_integers_multiple_values: queryParamsListOfIntegersMultipleValues,
		},
	};
}
async function queryParamsIntegerQueryParamWithLtConstraintValid(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { value: 49 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsIntegerQueryParamWithLtConstraintValid() {
	const route = {
		method: "GET",
		path: "/query/int-lt",
		handler_name: "query_params_integer_query_param_with_lt_constraint_valid",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { value: { annotation: "int", exclusiveMaximum: 50, source: "query", type: "integer" } },
			required: ["value"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_integer_query_param_with_lt_constraint_valid: queryParamsIntegerQueryParamWithLtConstraintValid,
		},
	};
}
async function queryParams42NegativeIntegerQueryParam(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { offset: -10 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams42NegativeIntegerQueryParam() {
	const route = {
		method: "GET",
		path: "/items/negative",
		handler_name: "query_params_42_negative_integer_query_param",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { offset: { annotation: "int", source: "query", type: "integer" } },
			required: ["offset"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_42_negative_integer_query_param: queryParams42NegativeIntegerQueryParam,
		},
	};
}
async function queryParams46StringMaxlengthValidationFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const term = params.term;
	if (term !== null && term !== void 0) {
		result.term = term;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParams46StringMaxlengthValidationFailure() {
	const route = {
		method: "GET",
		path: "/search",
		handler_name: "query_params_46_string_maxlength_validation_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { term: { maxLength: 10, source: "query", type: "string" } },
			required: ["term"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_46_string_maxlength_validation_failure: queryParams46StringMaxlengthValidationFailure,
		},
	};
}
async function queryParams56ArrayMaxitemsConstraintFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const tags = params.tags;
	if (tags !== null && tags !== void 0) {
		result.tags = tags;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParams56ArrayMaxitemsConstraintFailure() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_56_array_maxitems_constraint_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { tags: { items: { type: "string" }, maxItems: 5, source: "query", type: "array" } },
			required: ["tags"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_56_array_maxitems_constraint_failure: queryParams56ArrayMaxitemsConstraintFailure,
		},
	};
}
async function queryParamsStringQueryParamWithRegexPatternFail(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const code = params.code;
	if (code !== null && code !== void 0) {
		result.code = code;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParamsStringQueryParamWithRegexPatternFail() {
	const route = {
		method: "GET",
		path: "/query/pattern",
		handler_name: "query_params_string_query_param_with_regex_pattern_fail",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { code: { annotation: "str", pattern: "^[0-9]{3,}$", source: "query", type: "string" } },
			required: ["code"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_string_query_param_with_regex_pattern_fail: queryParamsStringQueryParamWithRegexPatternFail,
		},
	};
}
async function queryParams44StringMinlengthValidationSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { term: "foo" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams44StringMinlengthValidationSuccess() {
	const route = {
		method: "GET",
		path: "/search",
		handler_name: "query_params_44_string_minlength_validation_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { term: { minLength: 3, source: "query", type: "string" } },
			required: ["term"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_44_string_minlength_validation_success: queryParams44StringMinlengthValidationSuccess,
		},
	};
}
async function queryParams61FormatIpv4Failure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const ip = params.ip;
	if (ip !== null && ip !== void 0) {
		result.ip = ip;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParams61FormatIpv4Failure() {
	const route = {
		method: "GET",
		path: "/network",
		handler_name: "query_params_61_format_ipv4_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { ip: { format: "ipv4", source: "query", type: "string" } },
			required: ["ip"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_61_format_ipv4_failure: queryParams61FormatIpv4Failure,
		},
	};
}
async function queryParams48PatternValidationEmailFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const email = params.email;
	if (email !== null && email !== void 0) {
		result.email = email;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParams48PatternValidationEmailFailure() {
	const route = {
		method: "GET",
		path: "/subscribe",
		handler_name: "query_params_48_pattern_validation_email_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				email: { pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$", source: "query", type: "string" },
			},
			required: ["email"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_48_pattern_validation_email_failure: queryParams48PatternValidationEmailFailure,
		},
	};
}
async function queryParamsRequiredIntegerQueryParameterMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const query = params.query;
	if (query !== null && query !== void 0) {
		result.query = query;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParamsRequiredIntegerQueryParameterMissing() {
	const route = {
		method: "GET",
		path: "/query/int",
		handler_name: "query_params_required_integer_query_parameter_missing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { query: { annotation: "int", source: "query", type: "integer" } },
			required: ["query"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_missing: queryParamsRequiredIntegerQueryParameterMissing,
		},
	};
}
async function queryParamsQueryParameterWithSpecialCharactersUrlEncoding(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { email: "x@test.com", special: "&@A.ac" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsQueryParameterWithSpecialCharactersUrlEncoding() {
	const route = {
		method: "GET",
		path: "/test",
		handler_name: "query_params_query_parameter_with_special_characters_url_encoding",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				email: { annotation: "str", source: "query", type: "string" },
				special: { annotation: "str", source: "query", type: "string" },
			},
			required: ["email", "special"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_query_parameter_with_special_characters_url_encoding:
				queryParamsQueryParameterWithSpecialCharactersUrlEncoding,
		},
	};
}
async function queryParamsListQueryParameterRequiredButMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const deviceIds = params.device_ids;
	if (deviceIds !== null && deviceIds !== void 0) {
		result.device_ids = deviceIds;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParamsListQueryParameterRequiredButMissing() {
	const route = {
		method: "GET",
		path: "/query/list",
		handler_name: "query_params_list_query_parameter_required_but_missing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				device_ids: { annotation: "list[int]", items: { type: "integer" }, source: "query", type: "array" },
			},
			required: ["device_ids"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_list_query_parameter_required_but_missing: queryParamsListQueryParameterRequiredButMissing,
		},
	};
}
async function queryParamsRequiredStringQueryParameterSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = "foo bar baz";
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsRequiredStringQueryParameterSuccess() {
	const route = {
		method: "GET",
		path: "/query",
		handler_name: "query_params_required_string_query_parameter_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { query: { annotation: "str", source: "query", type: "string" } },
			required: ["query"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_required_string_query_parameter_success: queryParamsRequiredStringQueryParameterSuccess,
		},
	};
}
async function queryParams66MultipleofConstraintSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { quantity: 15 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams66MultipleofConstraintSuccess() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_66_multipleof_constraint_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { quantity: { multipleOf: 5, source: "query", type: "integer" } },
			required: ["quantity"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_66_multipleof_constraint_success: queryParams66MultipleofConstraintSuccess,
		},
	};
}
async function queryParams53IntegerLeConstraintFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const limit = params.limit;
	if (limit !== null && limit !== void 0) {
		result.limit = limit;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParams53IntegerLeConstraintFailure() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_53_integer_le_constraint_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { limit: { maximum: 100, source: "query", type: "integer" } },
			required: ["limit"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_53_integer_le_constraint_failure: queryParams53IntegerLeConstraintFailure,
		},
	};
}
async function queryParamsMultipleQueryParametersWithDifferentTypes(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { active: true, age: 30, name: "john", score: 95.5 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsMultipleQueryParametersWithDifferentTypes() {
	const route = {
		method: "GET",
		path: "/query/multi-type",
		handler_name: "query_params_multiple_query_parameters_with_different_types",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				active: { annotation: "bool", source: "query", type: "boolean" },
				age: { annotation: "int", source: "query", type: "integer" },
				name: { annotation: "str", source: "query", type: "string" },
				score: { annotation: "float", source: "query", type: "number" },
			},
			required: ["active", "age", "name", "score"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_multiple_query_parameters_with_different_types: queryParamsMultipleQueryParametersWithDifferentTypes,
		},
	};
}
async function queryParams71ArraySeparatorSemicolon(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { colors: ["red", "green", "blue"] };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams71ArraySeparatorSemicolon() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_71_array_separator_semicolon",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { colors: { items: { type: "string" }, separator: ";", source: "query", type: "array" } },
			required: ["colors"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_71_array_separator_semicolon: queryParams71ArraySeparatorSemicolon,
		},
	};
}
async function queryParams70ArraySeparatorPipe(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { tags: ["python", "rust", "typescript"] };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams70ArraySeparatorPipe() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_70_array_separator_pipe",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { tags: { items: { type: "string" }, separator: "|", source: "query", type: "array" } },
			required: ["tags"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_70_array_separator_pipe: queryParams70ArraySeparatorPipe,
		},
	};
}
async function queryParamsIntegerWithDefaultValueNotProvided(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = "foo bar 10";
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsIntegerWithDefaultValueNotProvided() {
	const route = {
		method: "GET",
		path: "/query/int/default",
		handler_name: "query_params_integer_with_default_value_not_provided",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { query: { annotation: "int", default: 10, source: "query", type: "integer" } },
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_integer_with_default_value_not_provided: queryParamsIntegerWithDefaultValueNotProvided,
		},
	};
}
async function queryParamsBooleanQueryParameterTrue(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { flag: true };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsBooleanQueryParameterTrue() {
	const route = {
		method: "GET",
		path: "/query/bool",
		handler_name: "query_params_boolean_query_parameter_true",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { flag: { annotation: "bool", source: "query", type: "boolean" } },
			required: ["flag"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_boolean_query_parameter_true: queryParamsBooleanQueryParameterTrue,
		},
	};
}
async function queryParamsIntegerQueryParamWithLeConstraintBoundary(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { value: 100 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsIntegerQueryParamWithLeConstraintBoundary() {
	const route = {
		method: "GET",
		path: "/query/int-le",
		handler_name: "query_params_integer_query_param_with_le_constraint_boundary",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { value: { annotation: "int", maximum: 100, source: "query", type: "integer" } },
			required: ["value"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_integer_query_param_with_le_constraint_boundary:
				queryParamsIntegerQueryParamWithLeConstraintBoundary,
		},
	};
}
async function queryParamsFloatQueryParamWithGeConstraintSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { price: 0.01 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsFloatQueryParamWithGeConstraintSuccess() {
	const route = {
		method: "GET",
		path: "/query/float-ge",
		handler_name: "query_params_float_query_param_with_ge_constraint_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { price: { annotation: "float", minimum: 0.01, source: "query", type: "number" } },
			required: ["price"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_float_query_param_with_ge_constraint_success: queryParamsFloatQueryParamWithGeConstraintSuccess,
		},
	};
}
async function queryParams51IntegerGeConstraintBoundary(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { offset: 0 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams51IntegerGeConstraintBoundary() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_51_integer_ge_constraint_boundary",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { offset: { minimum: 0, source: "query", type: "integer" } },
			required: ["offset"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_51_integer_ge_constraint_boundary: queryParams51IntegerGeConstraintBoundary,
		},
	};
}
async function queryParamsOptionalIntegerQueryParameterMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = "foo bar None";
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsOptionalIntegerQueryParameterMissing() {
	const route = {
		method: "GET",
		path: "/query/int/optional",
		handler_name: "query_params_optional_integer_query_parameter_missing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { query: { annotation: "int", source: "query", type: "integer" } },
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_optional_integer_query_parameter_missing: queryParamsOptionalIntegerQueryParameterMissing,
		},
	};
}
async function queryParams69ArrayUniqueitemsFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const ids = params.ids;
	if (ids !== null && ids !== void 0) {
		result.ids = ids;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParams69ArrayUniqueitemsFailure() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_69_array_uniqueitems_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { ids: { items: { type: "integer" }, source: "query", type: "array", uniqueItems: true } },
			required: ["ids"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_69_array_uniqueitems_failure: queryParams69ArrayUniqueitemsFailure,
		},
	};
}
async function queryParams72ArraySeparatorSpace(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { keywords: ["rust", "web", "framework"] };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams72ArraySeparatorSpace() {
	const route = {
		method: "GET",
		path: "/search",
		handler_name: "query_params_72_array_separator_space",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { keywords: { items: { type: "string" }, separator: " ", source: "query", type: "array" } },
			required: ["keywords"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_72_array_separator_space: queryParams72ArraySeparatorSpace,
		},
	};
}
async function queryParamsStringValidationWithRegexFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const itemQuery = params.item_query;
	if (itemQuery !== null && itemQuery !== void 0) {
		result.item_query = itemQuery;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParamsStringValidationWithRegexFailure() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "query_params_string_validation_with_regex_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_query: { annotation: "str", pattern: "^fixedquery$", source: "query", type: "string" } },
			required: ["item_query"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_string_validation_with_regex_failure: queryParamsStringValidationWithRegexFailure,
		},
	};
}
async function queryParams65FormatHostnameSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { host: "api.example.com" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams65FormatHostnameSuccess() {
	const route = {
		method: "GET",
		path: "/dns",
		handler_name: "query_params_65_format_hostname_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { host: { format: "hostname", source: "query", type: "string" } },
			required: ["host"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_65_format_hostname_success: queryParams65FormatHostnameSuccess,
		},
	};
}
async function queryParamsQueryParameterWithUrlEncodedSpace(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { name: "hello world" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsQueryParameterWithUrlEncodedSpace() {
	const route = {
		method: "GET",
		path: "/query/basic",
		handler_name: "query_params_query_parameter_with_url_encoded_space",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { name: { annotation: "str", source: "query", type: "string" } },
			required: ["name"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_query_parameter_with_url_encoded_space: queryParamsQueryParameterWithUrlEncodedSpace,
		},
	};
}
async function queryParamsListOfStringsMultipleValues(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { q: ["foo", "bar"] };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsListOfStringsMultipleValues() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "query_params_list_of_strings_multiple_values",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { q: { annotation: "list[str]", items: { type: "string" }, source: "query", type: "array" } },
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_list_of_strings_multiple_values: queryParamsListOfStringsMultipleValues,
		},
	};
}
async function queryParamsOptionalQueryParameterWithDefaultValue(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { limit: 10 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsOptionalQueryParameterWithDefaultValue() {
	const route = {
		method: "GET",
		path: "/query/optional-default",
		handler_name: "query_params_optional_query_parameter_with_default_value",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { limit: { annotation: "int", default: 10, source: "query", type: "integer" } },
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_optional_query_parameter_with_default_value: queryParamsOptionalQueryParameterWithDefaultValue,
		},
	};
}
async function queryParams62FormatIpv6Success(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { ip: "2001:0db8:85a3:0000:0000:8a2e:0370:7334" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams62FormatIpv6Success() {
	const route = {
		method: "GET",
		path: "/network/ipv6",
		handler_name: "query_params_62_format_ipv6_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { ip: { format: "ipv6", source: "query", type: "string" } },
			required: ["ip"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_62_format_ipv6_success: queryParams62FormatIpv6Success,
		},
	};
}
async function queryParamsArrayQueryParameterSingleValue(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = ["apple"];
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsArrayQueryParameterSingleValue() {
	const route = {
		method: "GET",
		path: "/query/list-default",
		handler_name: "query_params_array_query_parameter_single_value",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				tags: { annotation: "list[str]", default: [], items: { type: "string" }, source: "query", type: "array" },
			},
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_array_query_parameter_single_value: queryParamsArrayQueryParameterSingleValue,
		},
	};
}
async function queryParamsOptionalStringQueryParameterMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = "foo bar None";
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsOptionalStringQueryParameterMissing() {
	const route = {
		method: "GET",
		path: "/query/optional",
		handler_name: "query_params_optional_string_query_parameter_missing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { query: { annotation: "str", source: "query", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_optional_string_query_parameter_missing: queryParamsOptionalStringQueryParameterMissing,
		},
	};
}
async function queryParamsDatetimeQueryParameterSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { timestamp: "2024-01-15T10:30:00Z" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsDatetimeQueryParameterSuccess() {
	const route = {
		method: "GET",
		path: "/query/datetime",
		handler_name: "query_params_datetime_query_parameter_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { timestamp: { annotation: "str", format: "date-time", source: "query", type: "string" } },
			required: ["timestamp"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_datetime_query_parameter_success: queryParamsDatetimeQueryParameterSuccess,
		},
	};
}
async function queryParamsUuidQueryParameterInvalidFormat(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const itemId = params.item_id;
	if (itemId !== null && itemId !== void 0) {
		result.item_id = itemId;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParamsUuidQueryParameterInvalidFormat() {
	const route = {
		method: "GET",
		path: "/query/uuid",
		handler_name: "query_params_uuid_query_parameter_invalid_format",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { annotation: "str", format: "uuid", source: "query", type: "string" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_uuid_query_parameter_invalid_format: queryParamsUuidQueryParameterInvalidFormat,
		},
	};
}
async function queryParamsArrayQueryParameterEmptyArray(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppQueryParamsArrayQueryParameterEmptyArray() {
	const route = {
		method: "GET",
		path: "/query/list-default",
		handler_name: "query_params_array_query_parameter_empty_array",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				tags: { annotation: "list[str]", default: [], items: { type: "string" }, source: "query", type: "array" },
			},
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_array_query_parameter_empty_array: queryParamsArrayQueryParameterEmptyArray,
		},
	};
}
async function queryParamsEnumQueryParameterSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { model: "alexnet" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsEnumQueryParameterSuccess() {
	const route = {
		method: "GET",
		path: "/query/enum",
		handler_name: "query_params_enum_query_parameter_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				model: { annotation: "str", enum: ["alexnet", "resnet", "lenet"], source: "query", type: "string" },
			},
			required: ["model"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_enum_query_parameter_success: queryParamsEnumQueryParameterSuccess,
		},
	};
}
async function queryParamsUuidQueryParameterSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item_id: "c892496f-b1fd-4b91-bdb8-b46f92df1716" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParamsUuidQueryParameterSuccess() {
	const route = {
		method: "GET",
		path: "/query/uuid",
		handler_name: "query_params_uuid_query_parameter_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { annotation: "str", format: "uuid", source: "query", type: "string" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_uuid_query_parameter_success: queryParamsUuidQueryParameterSuccess,
		},
	};
}
async function queryParams50IntegerGtConstraintFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const limit = params.limit;
	if (limit !== null && limit !== void 0) {
		result.limit = limit;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParams50IntegerGtConstraintFailure() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_50_integer_gt_constraint_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { limit: { exclusiveMinimum: 0, source: "query", type: "integer" } },
			required: ["limit"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_50_integer_gt_constraint_failure: queryParams50IntegerGtConstraintFailure,
		},
	};
}
async function queryParams64FormatUriFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const url = params.url;
	if (url !== null && url !== void 0) {
		result.url = url;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParams64FormatUriFailure() {
	const route = {
		method: "GET",
		path: "/redirect",
		handler_name: "query_params_64_format_uri_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { url: { format: "uri", source: "query", type: "string" } },
			required: ["url"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_64_format_uri_failure: queryParams64FormatUriFailure,
		},
	};
}
async function queryParams54ArrayMinitemsConstraintSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { ids: [1, 2, 3] };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams54ArrayMinitemsConstraintSuccess() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_54_array_minitems_constraint_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { ids: { items: { type: "integer" }, minItems: 2, source: "query", type: "array" } },
			required: ["ids"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_54_array_minitems_constraint_success: queryParams54ArrayMinitemsConstraintSuccess,
		},
	};
}
async function queryParams55ArrayMinitemsConstraintFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const ids = params.ids;
	if (ids !== null && ids !== void 0) {
		result.ids = ids;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppQueryParams55ArrayMinitemsConstraintFailure() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_55_array_minitems_constraint_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { ids: { items: { type: "integer" }, minItems: 2, source: "query", type: "array" } },
			required: ["ids"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_55_array_minitems_constraint_failure: queryParams55ArrayMinitemsConstraintFailure,
		},
	};
}
async function queryParams60FormatIpv4Success(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { ip: "192.168.1.1" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppQueryParams60FormatIpv4Success() {
	const route = {
		method: "GET",
		path: "/network",
		handler_name: "query_params_60_format_ipv4_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { ip: { format: "ipv4", source: "query", type: "string" } },
			required: ["ip"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			query_params_60_format_ipv4_success: queryParams60FormatIpv4Success,
		},
	};
}
async function headersHeaderRegexValidationSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { x_request_id: "12345" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersHeaderRegexValidationSuccess() {
	const route = {
		method: "GET",
		path: "/headers/pattern",
		handler_name: "headers_header_regex_validation_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "X-Request-Id": { annotation: "str", pattern: "^[0-9]{3,}$", source: "header", type: "string" } },
			required: ["X-Request-Id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_header_regex_validation_success: headersHeaderRegexValidationSuccess,
		},
	};
}
async function headers33ApiKeyHeaderValid(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 200 };
	const result = {};
	const xAPIKey = params["X-API-Key"];
	if (xAPIKey !== null && xAPIKey !== void 0) {
		result["X-API-Key"] = xAPIKey;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHeaders33ApiKeyHeaderValid() {
	const route = {
		method: "GET",
		path: "/api/data",
		handler_name: "headers_33_api_key_header_valid",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "X-API-Key": { pattern: "^[a-f0-9]{32}$", source: "header", type: "string" } },
			required: ["X-API-Key"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_33_api_key_header_valid: headers33ApiKeyHeaderValid,
		},
	};
}
async function headersContentTypeHeaderApplicationJson(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { content_type: "application/json" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersContentTypeHeaderApplicationJson() {
	const route = {
		method: "GET",
		path: "/headers/content-type",
		handler_name: "headers_content_type_header_application_json",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "Content-Type": { annotation: "str", source: "header", type: "string" } },
			required: ["Content-Type"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_content_type_header_application_json: headersContentTypeHeaderApplicationJson,
		},
	};
}
async function headersAcceptLanguageHeader(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { accept_language: "en-US,en;q=0.9" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersAcceptLanguageHeader() {
	const route = {
		method: "GET",
		path: "/headers/accept-language",
		handler_name: "headers_accept_language_header",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "Accept-Language": { annotation: "str", source: "header", type: "string" } },
			required: ["Accept-Language"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_accept_language_header: headersAcceptLanguageHeader,
		},
	};
}
async function headersXApiKeyRequiredHeaderSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { username: "secret" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersXApiKeyRequiredHeaderSuccess() {
	const route = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_x_api_key_required_header_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { key: { annotation: "str", source: "header", type: "string" } },
			required: ["key"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_x_api_key_required_header_success: headersXApiKeyRequiredHeaderSuccess,
		},
	};
}
async function headersHeaderValidationMaxLengthConstraintFail(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const xSessionId = params["X-Session-Id"];
	if (xSessionId !== null && xSessionId !== void 0) {
		result["X-Session-Id"] = xSessionId;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHeadersHeaderValidationMaxLengthConstraintFail() {
	const route = {
		method: "GET",
		path: "/headers/max-length",
		handler_name: "headers_header_validation_max_length_constraint_fail",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "X-Session-Id": { annotation: "str", maxLength: 20, source: "header", type: "string" } },
			required: ["X-Session-Id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_header_validation_max_length_constraint_fail: headersHeaderValidationMaxLengthConstraintFail,
		},
	};
}
async function headersXApiKeyRequiredHeaderMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const xAPIKey = params["X-API-Key"];
	if (xAPIKey !== null && xAPIKey !== void 0) {
		result["X-API-Key"] = xAPIKey;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHeadersXApiKeyRequiredHeaderMissing() {
	const route = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_x_api_key_required_header_missing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "X-API-Key": { annotation: "str", source: "header", type: "string" } },
			required: ["X-API-Key"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_x_api_key_required_header_missing: headersXApiKeyRequiredHeaderMissing,
		},
	};
}
async function headersOriginHeader(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { origin: "https://example.com" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersOriginHeader() {
	const route = {
		method: "GET",
		path: "/headers/origin",
		handler_name: "headers_origin_header",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Origin: { annotation: "str", source: "header", type: "string" } },
			required: ["Origin"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_origin_header: headersOriginHeader,
		},
	};
}
async function headersUserAgentHeaderDefaultValue(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { "User-Agent": "testclient" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersUserAgentHeaderDefaultValue() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "headers_user_agent_header_default_value",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "User-Agent": { annotation: "str", default: "testclient", source: "header", type: "string" } },
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_user_agent_header_default_value: headersUserAgentHeaderDefaultValue,
		},
	};
}
async function headers32BearerTokenMissingPrefix(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const authorization = params.Authorization;
	if (authorization !== null && authorization !== void 0) {
		result.Authorization = authorization;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHeaders32BearerTokenMissingPrefix() {
	const route = {
		method: "GET",
		path: "/protected",
		handler_name: "headers_32_bearer_token_missing_prefix",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { pattern: "^Bearer [A-Za-z0-9-._~+/]+=*$", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_32_bearer_token_missing_prefix: headers32BearerTokenMissingPrefix,
		},
	};
}
async function headersOptionalHeaderWithNoneDefaultMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { strange_header: null };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersOptionalHeaderWithNoneDefaultMissing() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "headers_optional_header_with_none_default_missing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "strange-header": { annotation: "str", default: null, source: "header", type: "string" } },
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_optional_header_with_none_default_missing: headersOptionalHeaderWithNoneDefaultMissing,
		},
	};
}
async function headersHeaderRegexValidationFail(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const xRequestId = params["X-Request-Id"];
	if (xRequestId !== null && xRequestId !== void 0) {
		result["X-Request-Id"] = xRequestId;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHeadersHeaderRegexValidationFail() {
	const route = {
		method: "GET",
		path: "/headers/pattern",
		handler_name: "headers_header_regex_validation_fail",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "X-Request-Id": { annotation: "str", pattern: "^[0-9]{3,}$", source: "header", type: "string" } },
			required: ["X-Request-Id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_header_regex_validation_fail: headersHeaderRegexValidationFail,
		},
	};
}
async function headers31BearerTokenFormatInvalid(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const authorization = params.Authorization;
	if (authorization !== null && authorization !== void 0) {
		result.Authorization = authorization;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHeaders31BearerTokenFormatInvalid() {
	const route = {
		method: "GET",
		path: "/protected",
		handler_name: "headers_31_bearer_token_format_invalid",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { pattern: "^Bearer [A-Za-z0-9-._~+/]+=*$", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_31_bearer_token_format_invalid: headers31BearerTokenFormatInvalid,
		},
	};
}
async function headersXApiKeyOptionalHeaderSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { msg: "Hello secret" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersXApiKeyOptionalHeaderSuccess() {
	const route = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_x_api_key_optional_header_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { key: { annotation: "str", source: "header", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_x_api_key_optional_header_success: headersXApiKeyOptionalHeaderSuccess,
		},
	};
}
async function headersAuthorizationHeaderSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { credentials: "foobar", scheme: "Digest" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersAuthorizationHeaderSuccess() {
	const route = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_authorization_header_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { annotation: "str", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_authorization_header_success: headersAuthorizationHeaderSuccess,
		},
	};
}
async function headers30BearerTokenFormatValid(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 200 };
	const result = {};
	const authorization = params.Authorization;
	if (authorization !== null && authorization !== void 0) {
		result.Authorization = authorization;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHeaders30BearerTokenFormatValid() {
	const route = {
		method: "GET",
		path: "/protected",
		handler_name: "headers_30_bearer_token_format_valid",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { pattern: "^Bearer [A-Za-z0-9-._~+/]+=*$", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_30_bearer_token_format_valid: headers30BearerTokenFormatValid,
		},
	};
}
async function headersAuthorizationHeaderMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const authorization = params.Authorization;
	if (authorization !== null && authorization !== void 0) {
		result.Authorization = authorization;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHeadersAuthorizationHeaderMissing() {
	const route = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_authorization_header_missing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { annotation: "str", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_authorization_header_missing: headersAuthorizationHeaderMissing,
		},
	};
}
async function headersAcceptHeaderJson(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { accept: "application/json" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersAcceptHeaderJson() {
	const route = {
		method: "GET",
		path: "/headers/accept",
		handler_name: "headers_accept_header_json",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Accept: { annotation: "str", source: "header", type: "string" } },
			required: ["Accept"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_accept_header_json: headersAcceptHeaderJson,
		},
	};
}
async function headersAcceptEncodingHeader(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { accept_encoding: "gzip, deflate, br" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersAcceptEncodingHeader() {
	const route = {
		method: "GET",
		path: "/headers/accept-encoding",
		handler_name: "headers_accept_encoding_header",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "Accept-Encoding": { annotation: "str", source: "header", type: "string" } },
			required: ["Accept-Encoding"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_accept_encoding_header: headersAcceptEncodingHeader,
		},
	};
}
async function headersAuthorizationHeaderWrongScheme(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const authorization = params.Authorization;
	if (authorization !== null && authorization !== void 0) {
		result.Authorization = authorization;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHeadersAuthorizationHeaderWrongScheme() {
	const route = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_authorization_header_wrong_scheme",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { annotation: "str", pattern: "^Digest .+", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_authorization_header_wrong_scheme: headersAuthorizationHeaderWrongScheme,
		},
	};
}
async function headersHeaderValidationMinLengthConstraint(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const xToken = params["X-Token"];
	if (xToken !== null && xToken !== void 0) {
		result["X-Token"] = xToken;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHeadersHeaderValidationMinLengthConstraint() {
	const route = {
		method: "GET",
		path: "/headers/validated",
		handler_name: "headers_header_validation_min_length_constraint",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "X-Token": { annotation: "str", minLength: 3, source: "header", type: "string" } },
			required: ["X-Token"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_header_validation_min_length_constraint: headersHeaderValidationMinLengthConstraint,
		},
	};
}
async function headersBasicAuthenticationSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { password: "password", username: "username" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersBasicAuthenticationSuccess() {
	const route = {
		method: "GET",
		path: "/headers/basic-auth",
		handler_name: "headers_basic_authentication_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { annotation: "str", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_basic_authentication_success: headersBasicAuthenticationSuccess,
		},
	};
}
async function headersBearerTokenAuthenticationMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const authorization = params.Authorization;
	if (authorization !== null && authorization !== void 0) {
		result.Authorization = authorization;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHeadersBearerTokenAuthenticationMissing() {
	const route = {
		method: "GET",
		path: "/headers/bearer-auth",
		handler_name: "headers_bearer_token_authentication_missing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { annotation: "str", pattern: "^Bearer .+", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_bearer_token_authentication_missing: headersBearerTokenAuthenticationMissing,
		},
	};
}
async function headersXApiKeyOptionalHeaderMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { msg: "Hello World" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersXApiKeyOptionalHeaderMissing() {
	const route = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_x_api_key_optional_header_missing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { key: { annotation: "str", source: "header", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_x_api_key_optional_header_missing: headersXApiKeyOptionalHeaderMissing,
		},
	};
}
async function headersMultipleHeaderValuesXToken(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { "X-Token values": ["foo", "bar"] };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersMultipleHeaderValuesXToken() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "headers_multiple_header_values_x_token",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "x-token": { annotation: "str", source: "header", type: "string" } },
			required: ["x-token"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_multiple_header_values_x_token: headersMultipleHeaderValuesXToken,
		},
	};
}
async function headersMultipleCustomHeaders(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { x_client_version: "1.2.3", x_request_id: "req-12345", x_trace_id: "trace-abc" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersMultipleCustomHeaders() {
	const route = {
		method: "GET",
		path: "/headers/multiple",
		handler_name: "headers_multiple_custom_headers",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				"X-Client-Version": { annotation: "str", source: "header", type: "string" },
				"X-Request-Id": { annotation: "str", source: "header", type: "string" },
				"X-Trace-Id": { annotation: "str", source: "header", type: "string" },
			},
			required: ["X-Client-Version", "X-Request-Id", "X-Trace-Id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_multiple_custom_headers: headersMultipleCustomHeaders,
		},
	};
}
async function headers34ApiKeyHeaderInvalid(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const xAPIKey = params["X-API-Key"];
	if (xAPIKey !== null && xAPIKey !== void 0) {
		result["X-API-Key"] = xAPIKey;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppHeaders34ApiKeyHeaderInvalid() {
	const route = {
		method: "GET",
		path: "/api/data",
		handler_name: "headers_34_api_key_header_invalid",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "X-API-Key": { pattern: "^[a-f0-9]{32}$", source: "header", type: "string" } },
			required: ["X-API-Key"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_34_api_key_header_invalid: headers34ApiKeyHeaderInvalid,
		},
	};
}
async function headersBearerTokenAuthenticationSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { token: "valid_token_123" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersBearerTokenAuthenticationSuccess() {
	const route = {
		method: "GET",
		path: "/headers/bearer-auth",
		handler_name: "headers_bearer_token_authentication_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Authorization: { annotation: "str", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_bearer_token_authentication_success: headersBearerTokenAuthenticationSuccess,
		},
	};
}
async function headersHostHeader(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { host: "example.com:8080" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersHostHeader() {
	const route = {
		method: "GET",
		path: "/headers/host",
		handler_name: "headers_host_header",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Host: { annotation: "str", source: "header", type: "string" } },
			required: ["Host"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_host_header: headersHostHeader,
		},
	};
}
async function headersRefererHeader(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { referer: "https://example.com/page" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersRefererHeader() {
	const route = {
		method: "GET",
		path: "/headers/referer",
		handler_name: "headers_referer_header",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { Referer: { annotation: "str", source: "header", type: "string" } },
			required: ["Referer"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_referer_header: headersRefererHeader,
		},
	};
}
async function headersHeaderWithUnderscoreConversionExplicit(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { x_token: "secret123" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersHeaderWithUnderscoreConversionExplicit() {
	const route = {
		method: "GET",
		path: "/headers/underscore",
		handler_name: "headers_header_with_underscore_conversion_explicit",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "X-Token": { annotation: "str", source: "header", type: "string" } },
			required: ["X-Token"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_header_with_underscore_conversion_explicit: headersHeaderWithUnderscoreConversionExplicit,
		},
	};
}
async function headersHeaderCaseInsensitivityAccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		content_type_lower: "application/json",
		content_type_mixed: "application/json",
		content_type_upper: "application/json",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersHeaderCaseInsensitivityAccess() {
	const route = {
		method: "POST",
		path: "/echo",
		handler_name: "headers_header_case_insensitivity_access",
		request_schema: {
			additionalProperties: false,
			properties: { test: { type: "string" } },
			required: ["test"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_header_case_insensitivity_access: headersHeaderCaseInsensitivityAccess,
		},
	};
}
async function headersUserAgentHeaderCustomValue(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { "User-Agent": "Mozilla/5.0 Custom Browser" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppHeadersUserAgentHeaderCustomValue() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "headers_user_agent_header_custom_value",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "User-Agent": { annotation: "str", source: "header", type: "string" } },
			required: ["User-Agent"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			headers_user_agent_header_custom_value: headersUserAgentHeaderCustomValue,
		},
	};
}
async function compressionCompressionPayloadBelowMinSizeIsNotCompressed(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Small payload", payload: "tiny" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCompressionCompressionPayloadBelowMinSizeIsNotCompressed() {
	const config = {
		compression: {
			gzip: true,
			brotli: false,
			minSize: 4096,
			quality: 6,
		},
	};
	const route = {
		method: "GET",
		path: "/compression/skip",
		handler_name: "compression_compression_payload_below_min_size_is_not_compressed",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			compression_compression_payload_below_min_size_is_not_compressed:
				compressionCompressionPayloadBelowMinSizeIsNotCompressed,
		},
		config,
	};
}
async function compressionCompressionGzipApplied(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { vary: "Accept-Encoding" };
	const responseBody = {
		message: "Compressed payload",
		payload: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCompressionCompressionGzipApplied() {
	const config = {
		compression: {
			gzip: true,
			brotli: false,
			minSize: 0,
			quality: 4,
		},
	};
	const route = {
		method: "GET",
		path: "/compression/gzip",
		handler_name: "compression_compression_gzip_applied",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			compression_compression_gzip_applied: compressionCompressionGzipApplied,
		},
		config,
	};
}
async function validationErrorsInvalidUuidFormat(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const itemId = params.item_id;
	if (itemId !== null && itemId !== void 0) {
		result.item_id = itemId;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsInvalidUuidFormat() {
	const route = {
		method: "GET",
		path: "/items/{item_id}",
		handler_name: "validation_errors_invalid_uuid_format",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { item_id: { format: "uuid", source: "path", type: "string" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_uuid_format: validationErrorsInvalidUuidFormat,
		},
	};
}
async function validationErrorsInvalidBooleanValue(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const isActive = params.is_active;
	const q = params.q;
	if (isActive !== null && isActive !== void 0) {
		result.is_active = isActive;
	}
	if (q !== null && q !== void 0) {
		result.q = q;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsInvalidBooleanValue() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_invalid_boolean_value",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { is_active: { source: "query", type: "boolean" }, q: { source: "query", type: "string" } },
			required: ["is_active", "q"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_boolean_value: validationErrorsInvalidBooleanValue,
		},
	};
}
async function validationErrorsMissingRequiredQueryParameter(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const q = params.q;
	if (q !== null && q !== void 0) {
		result.q = q;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsMissingRequiredQueryParameter() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_missing_required_query_parameter",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { q: { source: "query", type: "string" } }, required: ["q"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_missing_required_query_parameter: validationErrorsMissingRequiredQueryParameter,
		},
	};
}
async function validationErrorsArrayMaxItemsConstraintViolation(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsArrayMaxItemsConstraintViolation() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_array_max_items_constraint_violation",
		request_schema: {
			additionalProperties: false,
			properties: {
				name: { type: "string" },
				price: { type: "number" },
				tags: { items: { type: "string" }, maxItems: 10, type: "array" },
			},
			required: ["name", "price", "tags"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_array_max_items_constraint_violation: validationErrorsArrayMaxItemsConstraintViolation,
		},
	};
}
async function validationErrorsNumericConstraintViolationGtGreaterThan(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const price = params.price;
	const q = params.q;
	if (price !== null && price !== void 0) {
		result.price = price;
	}
	if (q !== null && q !== void 0) {
		result.q = q;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsNumericConstraintViolationGtGreaterThan() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_numeric_constraint_violation_gt_greater_than",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				price: { exclusiveMinimum: 0, source: "query", type: "number" },
				q: { source: "query", type: "string" },
			},
			required: ["price", "q"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_numeric_constraint_violation_gt_greater_than:
				validationErrorsNumericConstraintViolationGtGreaterThan,
		},
	};
}
async function validationErrorsStringRegexPatternMismatch(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const q = params.q;
	if (q !== null && q !== void 0) {
		result.q = q;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsStringRegexPatternMismatch() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_string_regex_pattern_mismatch",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { q: { pattern: "^[a-zA-Z0-9_-]+$", source: "query", type: "string" } },
			required: ["q"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_string_regex_pattern_mismatch: validationErrorsStringRegexPatternMismatch,
		},
	};
}
async function validationErrorsInvalidEnumValue(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const modelName = params.model_name;
	if (modelName !== null && modelName !== void 0) {
		result.model_name = modelName;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsInvalidEnumValue() {
	const route = {
		method: "GET",
		path: "/models/{model_name}",
		handler_name: "validation_errors_invalid_enum_value",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { model_name: { enum: ["alexnet", "resnet", "lenet"], source: "path", type: "string" } },
			required: ["model_name"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_enum_value: validationErrorsInvalidEnumValue,
		},
	};
}
async function validationErrorsStringMinLengthConstraintViolation(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const q = params.q;
	if (q !== null && q !== void 0) {
		result.q = q;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsStringMinLengthConstraintViolation() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_string_min_length_constraint_violation",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { q: { minLength: 3, source: "query", type: "string" } },
			required: ["q"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_string_min_length_constraint_violation: validationErrorsStringMinLengthConstraintViolation,
		},
	};
}
async function validationErrorsMultipleValidationErrors(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsMultipleValidationErrors() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_multiple_validation_errors",
		request_schema: {
			additionalProperties: false,
			properties: {
				name: { minLength: 3, type: "string" },
				price: { exclusiveMinimum: 0, type: "integer" },
				quantity: { type: "integer" },
			},
			required: ["name", "price", "quantity"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_multiple_validation_errors: validationErrorsMultipleValidationErrors,
		},
	};
}
async function validationErrorsStringMaxLengthConstraintViolation(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const q = params.q;
	if (q !== null && q !== void 0) {
		result.q = q;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsStringMaxLengthConstraintViolation() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_string_max_length_constraint_violation",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { q: { maxLength: 50, source: "query", type: "string" } },
			required: ["q"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_string_max_length_constraint_violation: validationErrorsStringMaxLengthConstraintViolation,
		},
	};
}
async function validationErrorsNestedObjectValidationError(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsNestedObjectValidationError() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_nested_object_validation_error",
		request_schema: {
			additionalProperties: false,
			properties: {
				name: { type: "string" },
				price: { type: "number" },
				seller: {
					additionalProperties: false,
					properties: {
						address: {
							additionalProperties: false,
							properties: { city: { minLength: 3, type: "string" }, zip_code: { minLength: 5, type: "string" } },
							required: ["city", "zip_code"],
							type: "object",
						},
						name: { minLength: 3, type: "string" },
					},
					required: ["name", "address"],
					type: "object",
				},
			},
			required: ["name", "price", "seller"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_nested_object_validation_error: validationErrorsNestedObjectValidationError,
		},
	};
}
async function validationErrors10NestedErrorPath(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrors10NestedErrorPath() {
	const route = {
		method: "POST",
		path: "/profiles",
		handler_name: "validation_errors_10_nested_error_path",
		request_schema: {
			properties: {
				profile: {
					properties: {
						contact: {
							properties: { email: { format: "email", type: "string" } },
							required: ["email"],
							type: "object",
						},
					},
					required: ["contact"],
					type: "object",
				},
			},
			required: ["profile"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_10_nested_error_path: validationErrors10NestedErrorPath,
		},
	};
}
async function validationErrorsInvalidDatetimeFormat(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsInvalidDatetimeFormat() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_invalid_datetime_format",
		request_schema: {
			additionalProperties: false,
			properties: {
				created_at: { format: "date-time", type: "string" },
				name: { type: "string" },
				price: { type: "number" },
			},
			required: ["name", "price", "created_at"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_datetime_format: validationErrorsInvalidDatetimeFormat,
		},
	};
}
async function validationErrorsArrayItemValidationError(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsArrayItemValidationError() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_array_item_validation_error",
		request_schema: {
			additionalProperties: false,
			properties: {
				name: { type: "string" },
				price: { type: "number" },
				tags: { items: { type: "string" }, type: "array" },
			},
			required: ["name", "price", "tags"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_array_item_validation_error: validationErrorsArrayItemValidationError,
		},
	};
}
async function validationErrorsMissingRequiredBodyField(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsMissingRequiredBodyField() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_missing_required_body_field",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, price: { type: "string" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_missing_required_body_field: validationErrorsMissingRequiredBodyField,
		},
	};
}
async function validationErrorsBodyFieldTypeErrorStringForFloat(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsBodyFieldTypeErrorStringForFloat() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_body_field_type_error_string_for_float",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, price: { type: "number" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_body_field_type_error_string_for_float: validationErrorsBodyFieldTypeErrorStringForFloat,
		},
	};
}
async function validationErrorsMalformedJsonBody(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 400 };
	const responseBody = { detail: "Invalid request format" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppValidationErrorsMalformedJsonBody() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_malformed_json_body",
		request_schema: { type: "string" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_malformed_json_body: validationErrorsMalformedJsonBody,
		},
	};
}
async function validationErrorsQueryParamTypeErrorStringProvidedForInt(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const q = params.q;
	const skip = params.skip;
	if (q !== null && q !== void 0) {
		result.q = q;
	}
	if (skip !== null && skip !== void 0) {
		result.skip = skip;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsQueryParamTypeErrorStringProvidedForInt() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_query_param_type_error_string_provided_for_int",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { q: { source: "query", type: "string" }, skip: { source: "query", type: "integer" } },
			required: ["q", "skip"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_query_param_type_error_string_provided_for_int:
				validationErrorsQueryParamTypeErrorStringProvidedForInt,
		},
	};
}
async function validationErrorsHeaderValidationError(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const q = params.q;
	const xToken = params["x-token"];
	if (q !== null && q !== void 0) {
		result.q = q;
	}
	if (xToken !== null && xToken !== void 0) {
		result["x-token"] = xToken;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsHeaderValidationError() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_header_validation_error",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { q: { source: "query", type: "string" }, "x-token": { source: "header", type: "string" } },
			required: ["q", "x-token"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_header_validation_error: validationErrorsHeaderValidationError,
		},
	};
}
async function validationErrors09MultipleValidationErrors(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrors09MultipleValidationErrors() {
	const route = {
		method: "POST",
		path: "/users",
		handler_name: "validation_errors_09_multiple_validation_errors",
		request_schema: {
			properties: {
				age: { minimum: 18, type: "integer" },
				email: { format: "email", type: "string" },
				name: { minLength: 3, type: "string" },
			},
			required: ["name", "email", "age"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_09_multiple_validation_errors: validationErrors09MultipleValidationErrors,
		},
	};
}
async function validationErrorsNumericConstraintViolationLeLessThanOrEqual(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const limit = params.limit;
	const q = params.q;
	if (limit !== null && limit !== void 0) {
		result.limit = limit;
	}
	if (q !== null && q !== void 0) {
		result.q = q;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsNumericConstraintViolationLeLessThanOrEqual() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_numeric_constraint_violation_le_less_than_or_equal",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { limit: { maximum: 100, source: "query", type: "integer" }, q: { source: "query", type: "string" } },
			required: ["limit", "q"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_numeric_constraint_violation_le_less_than_or_equal:
				validationErrorsNumericConstraintViolationLeLessThanOrEqual,
		},
	};
}
async function validationErrorsArrayMinItemsConstraintViolation(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppValidationErrorsArrayMinItemsConstraintViolation() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_array_min_items_constraint_violation",
		request_schema: {
			additionalProperties: false,
			properties: {
				name: { type: "string" },
				price: { type: "number" },
				tags: { items: {}, minItems: 1, type: "array" },
			},
			required: ["name", "price", "tags"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			validation_errors_array_min_items_constraint_violation: validationErrorsArrayMinItemsConstraintViolation,
		},
	};
}
async function lifecycleHooksOnresponseSecurityHeadersSecurityHeadersOnResponse0(response) {
	if (!response.headers) response.headers = {};
	response.headers["X-Content-Type-Options"] = "nosniff";
	response.headers["X-Frame-Options"] = "DENY";
	response.headers["X-XSS-Protection"] = "1; mode=block";
	response.headers["Strict-Transport-Security"] = "max-age=31536000; includeSubDomains";
	return response;
}
async function lifecycleHooksOnresponseSecurityHeaders(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = {
		"strict-transport-security": "max-age=31536000; includeSubDomains",
		"x-content-type-options": "nosniff",
		"x-frame-options": "DENY",
		"x-xss-protection": "1; mode=block",
	};
	const responseBody = { message: "Response with security headers" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppLifecycleHooksOnresponseSecurityHeaders() {
	const route = {
		method: "GET",
		path: "/api/test-security-headers",
		handler_name: "lifecycle_hooks_onresponse_security_headers",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_onresponse_security_headers: lifecycleHooksOnresponseSecurityHeaders,
		},
		lifecycleHooks: {
			onResponse: [lifecycleHooksOnresponseSecurityHeadersSecurityHeadersOnResponse0],
		},
	};
}
async function lifecycleHooksPrehandlerAuthenticationFailedShortCircuitAuthenticatorPreHandler0(_request) {
	return {
		statusCode: 401,
		body: {
			error: "Unauthorized",
			message: "Invalid or expired authentication token",
		},
	};
}
async function lifecycleHooksPrehandlerAuthenticationFailedShortCircuit(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 401 };
	const responseBody = { error: "Unauthorized", message: "Invalid or expired authentication token" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppLifecycleHooksPrehandlerAuthenticationFailedShortCircuit() {
	const route = {
		method: "GET",
		path: "/api/protected-resource-fail",
		handler_name: "lifecycle_hooks_prehandler_authentication_failed_short_circuit",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_prehandler_authentication_failed_short_circuit:
				lifecycleHooksPrehandlerAuthenticationFailedShortCircuit,
		},
		lifecycleHooks: {
			preHandler: [lifecycleHooksPrehandlerAuthenticationFailedShortCircuitAuthenticatorPreHandler0],
		},
	};
}
async function lifecycleHooksPrehandlerAuthorizationCheckAuthenticatorPreHandler0(request) {
	return request;
}
async function lifecycleHooksPrehandlerAuthorizationCheckAuthorizerPreHandler1(request) {
	return request;
}
async function lifecycleHooksPrehandlerAuthorizationCheck(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Admin access granted", role: "admin", user_id: "admin-456" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppLifecycleHooksPrehandlerAuthorizationCheck() {
	const route = {
		method: "GET",
		path: "/api/admin-only",
		handler_name: "lifecycle_hooks_prehandler_authorization_check",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_prehandler_authorization_check: lifecycleHooksPrehandlerAuthorizationCheck,
		},
		lifecycleHooks: {
			preHandler: [
				lifecycleHooksPrehandlerAuthorizationCheckAuthenticatorPreHandler0,
				lifecycleHooksPrehandlerAuthorizationCheckAuthorizerPreHandler1,
			],
		},
	};
}
async function lifecycleHooksPrehandlerAuthenticationSuccessAuthenticatorPreHandler0(request) {
	return request;
}
async function lifecycleHooksPrehandlerAuthenticationSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { authenticated: true, message: "Access granted", user_id: "user-123" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppLifecycleHooksPrehandlerAuthenticationSuccess() {
	const route = {
		method: "GET",
		path: "/api/protected-resource",
		handler_name: "lifecycle_hooks_prehandler_authentication_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_prehandler_authentication_success: lifecycleHooksPrehandlerAuthenticationSuccess,
		},
		lifecycleHooks: {
			preHandler: [lifecycleHooksPrehandlerAuthenticationSuccessAuthenticatorPreHandler0],
		},
	};
}
async function lifecycleHooksPrevalidationRateLimitExceededShortCircuitRateLimiterPreValidation0(_request) {
	return {
		statusCode: 429,
		body: {
			error: "Rate limit exceeded",
			message: "Too many requests, please try again later",
		},
		headers: {
			"Retry-After": "60",
		},
	};
}
async function lifecycleHooksPrevalidationRateLimitExceededShortCircuit(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 429 };
	response.headers = { "retry-after": "60" };
	const responseBody = { error: "Rate limit exceeded", message: "Too many requests, please try again later" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppLifecycleHooksPrevalidationRateLimitExceededShortCircuit() {
	const route = {
		method: "POST",
		path: "/api/test-rate-limit-exceeded",
		handler_name: "lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit",
		request_schema: { properties: { data: { type: "string" } }, required: ["data"], type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit:
				lifecycleHooksPrevalidationRateLimitExceededShortCircuit,
		},
		lifecycleHooks: {
			preValidation: [lifecycleHooksPrevalidationRateLimitExceededShortCircuitRateLimiterPreValidation0],
		},
	};
}
async function lifecycleHooksOnerrorErrorLoggingErrorLoggerOnError0(response) {
	if (!response.headers) response.headers = {};
	response.headers["Content-Type"] = "application/json";
	return response;
}
async function lifecycleHooksOnerrorErrorLoggingErrorFormatterOnError1(response) {
	if (!response.headers) response.headers = {};
	response.headers["Content-Type"] = "application/json";
	return response;
}
async function lifecycleHooksOnerrorErrorLogging(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 500 };
	response.headers = { "content-type": "application/json" };
	const responseBody = { error: "Internal Server Error", error_id: ".*", message: "An unexpected error occurred" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppLifecycleHooksOnerrorErrorLogging() {
	const route = {
		method: "GET",
		path: "/api/test-error",
		handler_name: "lifecycle_hooks_onerror_error_logging",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_onerror_error_logging: lifecycleHooksOnerrorErrorLogging,
		},
		lifecycleHooks: {
			onError: [
				lifecycleHooksOnerrorErrorLoggingErrorLoggerOnError0,
				lifecycleHooksOnerrorErrorLoggingErrorFormatterOnError1,
			],
		},
	};
}
async function lifecycleHooksMultipleHooksAllPhasesRequestLoggerOnRequest0(request) {
	return request;
}
async function lifecycleHooksMultipleHooksAllPhasesRequestIdGeneratorOnRequest1(request) {
	return request;
}
async function lifecycleHooksMultipleHooksAllPhasesRateLimiterPreValidation0(request) {
	return request;
}
async function lifecycleHooksMultipleHooksAllPhasesAuthenticatorPreHandler0(request) {
	return request;
}
async function lifecycleHooksMultipleHooksAllPhasesAuthorizerPreHandler1(request) {
	return request;
}
async function lifecycleHooksMultipleHooksAllPhasesSecurityHeadersOnResponse0(response) {
	if (!response.headers) response.headers = {};
	response.headers["X-Content-Type-Options"] = "nosniff";
	response.headers["X-Frame-Options"] = "DENY";
	response.headers["X-XSS-Protection"] = "1; mode=block";
	response.headers["Strict-Transport-Security"] = "max-age=31536000; includeSubDomains";
	return response;
}
async function lifecycleHooksMultipleHooksAllPhasesResponseTimerOnResponse1(response) {
	if (!response.headers) response.headers = {};
	response.headers["X-Response-Time"] = "0ms";
	return response;
}
async function lifecycleHooksMultipleHooksAllPhasesAuditLoggerOnResponse2(response) {
	return response;
}
async function lifecycleHooksMultipleHooksAllPhasesErrorLoggerOnError0(response) {
	if (!response.headers) response.headers = {};
	response.headers["Content-Type"] = "application/json";
	return response;
}
async function lifecycleHooksMultipleHooksAllPhases(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = {
		"x-content-type-options": "nosniff",
		"x-frame-options": "DENY",
		"x-request-id": ".*",
		"x-response-time": ".*ms",
	};
	const responseBody = {
		action: "update_profile",
		message: "Action completed successfully",
		request_id: ".*",
		user_id: "user-123",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppLifecycleHooksMultipleHooksAllPhases() {
	const route = {
		method: "POST",
		path: "/api/full-lifecycle",
		handler_name: "lifecycle_hooks_multiple_hooks_all_phases",
		request_schema: {
			properties: { action: { type: "string" }, user_id: { type: "string" } },
			required: ["user_id", "action"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_multiple_hooks_all_phases: lifecycleHooksMultipleHooksAllPhases,
		},
		lifecycleHooks: {
			onRequest: [
				lifecycleHooksMultipleHooksAllPhasesRequestLoggerOnRequest0,
				lifecycleHooksMultipleHooksAllPhasesRequestIdGeneratorOnRequest1,
			],
			preValidation: [lifecycleHooksMultipleHooksAllPhasesRateLimiterPreValidation0],
			preHandler: [
				lifecycleHooksMultipleHooksAllPhasesAuthenticatorPreHandler0,
				lifecycleHooksMultipleHooksAllPhasesAuthorizerPreHandler1,
			],
			onResponse: [
				lifecycleHooksMultipleHooksAllPhasesSecurityHeadersOnResponse0,
				lifecycleHooksMultipleHooksAllPhasesResponseTimerOnResponse1,
				lifecycleHooksMultipleHooksAllPhasesAuditLoggerOnResponse2,
			],
			onError: [lifecycleHooksMultipleHooksAllPhasesErrorLoggerOnError0],
		},
	};
}
async function lifecycleHooksHookExecutionOrderFirstHookOnRequest0(request) {
	return request;
}
async function lifecycleHooksHookExecutionOrderSecondHookOnRequest1(request) {
	return request;
}
async function lifecycleHooksHookExecutionOrderThirdHookOnRequest2(request) {
	return request;
}
async function lifecycleHooksHookExecutionOrder(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		execution_order: ["first_hook", "second_hook", "third_hook"],
		message: "Hooks executed in order",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppLifecycleHooksHookExecutionOrder() {
	const route = {
		method: "GET",
		path: "/api/test-hook-order",
		handler_name: "lifecycle_hooks_hook_execution_order",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_hook_execution_order: lifecycleHooksHookExecutionOrder,
		},
		lifecycleHooks: {
			onRequest: [
				lifecycleHooksHookExecutionOrderFirstHookOnRequest0,
				lifecycleHooksHookExecutionOrderSecondHookOnRequest1,
				lifecycleHooksHookExecutionOrderThirdHookOnRequest2,
			],
		},
	};
}
async function lifecycleHooksOnresponseResponseTimingStartTimerOnRequest0(request) {
	return request;
}
async function lifecycleHooksOnresponseResponseTimingResponseTimerOnResponse0(response) {
	if (!response.headers) response.headers = {};
	response.headers["X-Response-Time"] = "0ms";
	return response;
}
async function lifecycleHooksOnresponseResponseTiming(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "x-response-time": ".*ms" };
	const responseBody = { message: "Response with timing info" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppLifecycleHooksOnresponseResponseTiming() {
	const route = {
		method: "GET",
		path: "/api/test-timing",
		handler_name: "lifecycle_hooks_onresponse_response_timing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_onresponse_response_timing: lifecycleHooksOnresponseResponseTiming,
		},
		lifecycleHooks: {
			onRequest: [lifecycleHooksOnresponseResponseTimingStartTimerOnRequest0],
			onResponse: [lifecycleHooksOnresponseResponseTimingResponseTimerOnResponse0],
		},
	};
}
async function lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuitAuthenticatorPreHandler0(_request) {
	return {
		statusCode: 403,
		body: {
			error: "Forbidden",
			message: "Admin role required for this endpoint",
		},
	};
}
async function lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuitAuthorizerPreHandler1(_request) {
	return {
		statusCode: 403,
		body: {
			error: "Forbidden",
			message: "Admin role required for this endpoint",
		},
	};
}
async function lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuit(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 403 };
	const responseBody = { error: "Forbidden", message: "Admin role required for this endpoint" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppLifecycleHooksPrehandlerAuthorizationForbiddenShortCircuit() {
	const route = {
		method: "GET",
		path: "/api/admin-only-forbidden",
		handler_name: "lifecycle_hooks_prehandler_authorization_forbidden_short_circuit",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_prehandler_authorization_forbidden_short_circuit:
				lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuit,
		},
		lifecycleHooks: {
			preHandler: [
				lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuitAuthenticatorPreHandler0,
				lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuitAuthorizerPreHandler1,
			],
		},
	};
}
async function lifecycleHooksOnrequestRequestLoggingRequestLoggerOnRequest0(request) {
	return request;
}
async function lifecycleHooksOnrequestRequestLoggingRequestIdGeneratorOnRequest1(request) {
	return request;
}
async function lifecycleHooksOnrequestRequestLogging(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "x-request-id": ".*" };
	const responseBody = { has_request_id: true, message: "onRequest hooks executed", request_logged: true };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppLifecycleHooksOnrequestRequestLogging() {
	const route = {
		method: "GET",
		path: "/api/test-on-request",
		handler_name: "lifecycle_hooks_onrequest_request_logging",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_onrequest_request_logging: lifecycleHooksOnrequestRequestLogging,
		},
		lifecycleHooks: {
			onRequest: [
				lifecycleHooksOnrequestRequestLoggingRequestLoggerOnRequest0,
				lifecycleHooksOnrequestRequestLoggingRequestIdGeneratorOnRequest1,
			],
		},
	};
}
async function lifecycleHooksPrevalidationRateLimitingRateLimiterPreValidation0(request) {
	return request;
}
async function lifecycleHooksPrevalidationRateLimiting(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Request accepted", rate_limit_checked: true };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppLifecycleHooksPrevalidationRateLimiting() {
	const route = {
		method: "POST",
		path: "/api/test-rate-limit",
		handler_name: "lifecycle_hooks_prevalidation_rate_limiting",
		request_schema: { properties: { data: { type: "string" } }, required: ["data"], type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_prevalidation_rate_limiting: lifecycleHooksPrevalidationRateLimiting,
		},
		lifecycleHooks: {
			preValidation: [lifecycleHooksPrevalidationRateLimitingRateLimiterPreValidation0],
		},
	};
}
async function rateLimitRateLimitBelowThresholdSucceeds(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { request: "under-limit", status: "ok" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppRateLimitRateLimitBelowThresholdSucceeds() {
	const config = {
		rateLimit: {
			perSecond: 5,
			burst: 5,
			ipBased: false,
		},
	};
	const route = {
		method: "GET",
		path: "/rate-limit/basic",
		handler_name: "rate_limit_rate_limit_below_threshold_succeeds",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			rate_limit_rate_limit_below_threshold_succeeds: rateLimitRateLimitBelowThresholdSucceeds,
		},
		config,
	};
}
async function rateLimitRateLimitExceededReturns429(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppRateLimitRateLimitExceededReturns429() {
	const config = {
		rateLimit: {
			perSecond: 1,
			burst: 1,
			ipBased: false,
		},
	};
	const route = {
		method: "GET",
		path: "/rate-limit/exceeded",
		handler_name: "rate_limit_rate_limit_exceeded_returns_429",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			rate_limit_rate_limit_exceeded_returns_429: rateLimitRateLimitExceededReturns429,
		},
		config,
	};
}
async function cookies25CookieSamesiteLax(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 200 };
	const result = {};
	const tracking = params.tracking;
	if (tracking !== null && tracking !== void 0) {
		result.tracking = tracking;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppCookies25CookieSamesiteLax() {
	const route = {
		method: "GET",
		path: "/data",
		handler_name: "cookies_25_cookie_samesite_lax",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { tracking: { samesite: "Lax", source: "cookie", type: "string" } },
			required: ["tracking"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_25_cookie_samesite_lax: cookies25CookieSamesiteLax,
		},
	};
}
async function cookiesOptionalCookieParameterSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { ads_id: "abc123" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesOptionalCookieParameterSuccess() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "cookies_optional_cookie_parameter_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { ads_id: { source: "cookie", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_optional_cookie_parameter_success: cookiesOptionalCookieParameterSuccess,
		},
	};
}
async function cookiesCookieRegexPatternValidationFail(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const trackingId = params.tracking_id;
	if (trackingId !== null && trackingId !== void 0) {
		result.tracking_id = trackingId;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppCookiesCookieRegexPatternValidationFail() {
	const route = {
		method: "GET",
		path: "/cookies/pattern",
		handler_name: "cookies_cookie_regex_pattern_validation_fail",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { tracking_id: { pattern: "^[A-Z0-9]{8}$", source: "cookie", type: "string" } },
			required: ["tracking_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_cookie_regex_pattern_validation_fail: cookiesCookieRegexPatternValidationFail,
		},
	};
}
async function cookiesResponseSessionCookieNoMaxAge(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Session cookie set" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesResponseSessionCookieNoMaxAge() {
	const route = {
		method: "POST",
		path: "/cookies/session",
		handler_name: "cookies_response_session_cookie_no_max_age",
		request_schema: {
			additionalProperties: false,
			properties: { value: { type: "string" } },
			required: ["value"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_response_session_cookie_no_max_age: cookiesResponseSessionCookieNoMaxAge,
		},
	};
}
async function cookies27CookieHttponlyFlag(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 200 };
	const result = {};
	const session = params.session;
	if (session !== null && session !== void 0) {
		result.session = session;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppCookies27CookieHttponlyFlag() {
	const route = {
		method: "GET",
		path: "/secure",
		handler_name: "cookies_27_cookie_httponly_flag",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { session: { httponly: true, source: "cookie", type: "string" } },
			required: ["session"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_27_cookie_httponly_flag: cookies27CookieHttponlyFlag,
		},
	};
}
async function cookiesResponseCookieWithAttributes(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Cookie set" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesResponseCookieWithAttributes() {
	const route = {
		method: "GET",
		path: "/cookie/set",
		handler_name: "cookies_response_cookie_with_attributes",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_attributes: cookiesResponseCookieWithAttributes,
		},
	};
}
async function cookies24CookieSamesiteStrict(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 200 };
	const result = {};
	const sessionId = params.session_id;
	if (sessionId !== null && sessionId !== void 0) {
		result.session_id = sessionId;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppCookies24CookieSamesiteStrict() {
	const route = {
		method: "GET",
		path: "/secure",
		handler_name: "cookies_24_cookie_samesite_strict",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { session_id: { samesite: "Strict", source: "cookie", type: "string" } },
			required: ["session_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_24_cookie_samesite_strict: cookies24CookieSamesiteStrict,
		},
	};
}
async function cookiesApikeyCookieAuthenticationSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { username: "secret" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesApikeyCookieAuthenticationSuccess() {
	const route = {
		method: "GET",
		path: "/users/me",
		handler_name: "cookies_apikey_cookie_authentication_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { key: { source: "cookie", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_apikey_cookie_authentication_success: cookiesApikeyCookieAuthenticationSuccess,
		},
	};
}
async function cookiesCookieValidationMinLengthConstraintSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { token: "abc" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesCookieValidationMinLengthConstraintSuccess() {
	const route = {
		method: "GET",
		path: "/cookies/min-length",
		handler_name: "cookies_cookie_validation_min_length_constraint_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { token: { source: "cookie", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_cookie_validation_min_length_constraint_success: cookiesCookieValidationMinLengthConstraintSuccess,
		},
	};
}
async function cookiesCookieValidationMinLengthFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const trackingId = params.tracking_id;
	if (trackingId !== null && trackingId !== void 0) {
		result.tracking_id = trackingId;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppCookiesCookieValidationMinLengthFailure() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "cookies_cookie_validation_min_length_failure",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { tracking_id: { minLength: 3, source: "cookie", type: "string" } },
			required: ["tracking_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_cookie_validation_min_length_failure: cookiesCookieValidationMinLengthFailure,
		},
	};
}
async function cookiesCookieValidationMaxLengthConstraintFail(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const sessionId = params.session_id;
	if (sessionId !== null && sessionId !== void 0) {
		result.session_id = sessionId;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppCookiesCookieValidationMaxLengthConstraintFail() {
	const route = {
		method: "GET",
		path: "/cookies/validated",
		handler_name: "cookies_cookie_validation_max_length_constraint_fail",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { session_id: { maxLength: 20, source: "cookie", type: "string" } },
			required: ["session_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_cookie_validation_max_length_constraint_fail: cookiesCookieValidationMaxLengthConstraintFail,
		},
	};
}
async function cookiesRequiredCookieMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const fatebookTracker = params.fatebook_tracker;
	const sessionId = params.session_id;
	if (fatebookTracker !== null && fatebookTracker !== void 0) {
		result.fatebook_tracker = fatebookTracker;
	}
	if (sessionId !== null && sessionId !== void 0) {
		result.session_id = sessionId;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppCookiesRequiredCookieMissing() {
	const route = {
		method: "GET",
		path: "/items/cookies",
		handler_name: "cookies_required_cookie_missing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				fatebook_tracker: { source: "cookie", type: "string" },
				session_id: { source: "cookie", type: "string" },
			},
			required: ["session_id"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_required_cookie_missing: cookiesRequiredCookieMissing,
		},
	};
}
async function cookiesOptionalCookieParameterMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { ads_id: null };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesOptionalCookieParameterMissing() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "cookies_optional_cookie_parameter_missing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { ads_id: { source: "cookie", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_optional_cookie_parameter_missing: cookiesOptionalCookieParameterMissing,
		},
	};
}
async function cookiesApikeyCookieAuthenticationMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	const key = params.key;
	if (key !== null && key !== void 0) {
		result.key = key;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppCookiesApikeyCookieAuthenticationMissing() {
	const route = {
		method: "GET",
		path: "/users/me/auth",
		handler_name: "cookies_apikey_cookie_authentication_missing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { key: { source: "cookie", type: "string" } }, required: ["key"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_apikey_cookie_authentication_missing: cookiesApikeyCookieAuthenticationMissing,
		},
	};
}
async function cookiesResponseMultipleCookies(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Multiple cookies set" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesResponseMultipleCookies() {
	const route = {
		method: "POST",
		path: "/cookies/multiple",
		handler_name: "cookies_response_multiple_cookies",
		request_schema: {
			additionalProperties: false,
			properties: { session: { type: "string" }, user: { type: "string" } },
			required: ["user", "session"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_response_multiple_cookies: cookiesResponseMultipleCookies,
		},
	};
}
async function cookiesResponseCookieWithSamesiteLax(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Cookie set with SameSite=Lax" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesResponseCookieWithSamesiteLax() {
	const route = {
		method: "POST",
		path: "/cookies/samesite-lax",
		handler_name: "cookies_response_cookie_with_samesite_lax",
		request_schema: {
			additionalProperties: false,
			properties: { value: { type: "string" } },
			required: ["value"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_samesite_lax: cookiesResponseCookieWithSamesiteLax,
		},
	};
}
async function cookiesResponseDeleteCookie(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Cookie deleted" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesResponseDeleteCookie() {
	const route = {
		method: "POST",
		path: "/cookies/delete",
		handler_name: "cookies_response_delete_cookie",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { session: { source: "cookie", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_response_delete_cookie: cookiesResponseDeleteCookie,
		},
	};
}
async function cookiesResponseCookieWithPathAttribute(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Cookie set with path" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesResponseCookieWithPathAttribute() {
	const route = {
		method: "POST",
		path: "/cookies/set-with-path",
		handler_name: "cookies_response_cookie_with_path_attribute",
		request_schema: {
			additionalProperties: false,
			properties: { value: { type: "string" } },
			required: ["value"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_path_attribute: cookiesResponseCookieWithPathAttribute,
		},
	};
}
async function cookiesOptionalApikeyCookieMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { msg: "Create an account first" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesOptionalApikeyCookieMissing() {
	const route = {
		method: "GET",
		path: "/users/me",
		handler_name: "cookies_optional_apikey_cookie_missing",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { key: { source: "cookie", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_optional_apikey_cookie_missing: cookiesOptionalApikeyCookieMissing,
		},
	};
}
async function cookiesResponseCookieWithSamesiteStrict(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Cookie set with SameSite=Strict" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesResponseCookieWithSamesiteStrict() {
	const route = {
		method: "POST",
		path: "/cookies/samesite-strict",
		handler_name: "cookies_response_cookie_with_samesite_strict",
		request_schema: {
			additionalProperties: false,
			properties: { value: { type: "string" } },
			required: ["value"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_samesite_strict: cookiesResponseCookieWithSamesiteStrict,
		},
	};
}
async function cookiesResponseCookieWithSamesiteNone(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Cookie set with SameSite=None" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesResponseCookieWithSamesiteNone() {
	const route = {
		method: "POST",
		path: "/cookies/samesite-none",
		handler_name: "cookies_response_cookie_with_samesite_none",
		request_schema: {
			additionalProperties: false,
			properties: { value: { type: "string" } },
			required: ["value"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_samesite_none: cookiesResponseCookieWithSamesiteNone,
		},
	};
}
async function cookiesCookieRegexPatternValidationSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { tracking_id: "ABC12345" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesCookieRegexPatternValidationSuccess() {
	const route = {
		method: "GET",
		path: "/cookies/pattern",
		handler_name: "cookies_cookie_regex_pattern_validation_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { tracking_id: { source: "cookie", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_cookie_regex_pattern_validation_success: cookiesCookieRegexPatternValidationSuccess,
		},
	};
}
async function cookiesResponseSetCookieBasic(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Come to the dark side, we have cookies" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesResponseSetCookieBasic() {
	const route = {
		method: "POST",
		path: "/cookie/",
		handler_name: "cookies_response_set_cookie_basic",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_response_set_cookie_basic: cookiesResponseSetCookieBasic,
		},
	};
}
async function cookiesMultipleCookiesSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { fatebook_tracker: "tracker456", googall_tracker: "ga789", session_id: "session123" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesMultipleCookiesSuccess() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "cookies_multiple_cookies_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				fatebook_tracker: { source: "cookie", type: "string" },
				googall_tracker: { source: "cookie", type: "string" },
				session_id: { source: "cookie", type: "string" },
			},
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_multiple_cookies_success: cookiesMultipleCookiesSuccess,
		},
	};
}
async function cookies26CookieSecureFlag(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 200 };
	const result = {};
	const authToken = params.auth_token;
	if (authToken !== null && authToken !== void 0) {
		result.auth_token = authToken;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppCookies26CookieSecureFlag() {
	const route = {
		method: "GET",
		path: "/secure",
		handler_name: "cookies_26_cookie_secure_flag",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { auth_token: { secure: true, source: "cookie", type: "string" } },
			required: ["auth_token"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_26_cookie_secure_flag: cookies26CookieSecureFlag,
		},
	};
}
async function cookiesResponseCookieWithDomainAttribute(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { message: "Cookie set with domain" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCookiesResponseCookieWithDomainAttribute() {
	const route = {
		method: "POST",
		path: "/cookies/set-with-domain",
		handler_name: "cookies_response_cookie_with_domain_attribute",
		request_schema: {
			additionalProperties: false,
			properties: { value: { type: "string" } },
			required: ["value"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_domain_attribute: cookiesResponseCookieWithDomainAttribute,
		},
	};
}
async function edgeCases19EmojiInStrings(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	const responseBody = { text: "Hello \u{1F44B} World \u{1F30D}" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCases19EmojiInStrings() {
	const route = {
		method: "POST",
		path: "/messages",
		handler_name: "edge_cases_19_emoji_in_strings",
		request_schema: {
			properties: { text: { maxLength: 100, minLength: 1, type: "string" } },
			required: ["text"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_19_emoji_in_strings: edgeCases19EmojiInStrings,
		},
	};
}
async function edgeCases12PercentEncodedSpecialChars(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { term: "hi there" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCases12PercentEncodedSpecialChars() {
	const route = {
		method: "GET",
		path: "/search",
		handler_name: "edge_cases_12_percent_encoded_special_chars",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { term: { source: "query", type: "string" } }, required: ["term"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_12_percent_encoded_special_chars: edgeCases12PercentEncodedSpecialChars,
		},
	};
}
async function edgeCasesSpecialStringValuesAndEscaping(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		backslashes: "C:\\\\Users\\\\Path",
		empty_string: "",
		quotes: `He said "hello" and 'goodbye'`,
		special_chars: `!@#$%^&*()_+-=[]{}|;':",./<>?`,
		tabs_newlines: "line1\n	line2\r\nline3",
		unicode_escapes: "Hello",
		whitespace: "   ",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCasesSpecialStringValuesAndEscaping() {
	const route = {
		method: "POST",
		path: "/strings/",
		handler_name: "edge_cases_special_string_values_and_escaping",
		request_schema: {
			additionalProperties: false,
			properties: {
				backslashes: { type: "string" },
				empty_string: { type: "string" },
				quotes: { type: "string" },
				special_chars: { type: "string" },
				tabs_newlines: { type: "string" },
				unicode_escapes: { type: "string" },
				whitespace: { type: "string" },
			},
			required: [
				"empty_string",
				"whitespace",
				"tabs_newlines",
				"quotes",
				"backslashes",
				"unicode_escapes",
				"special_chars",
			],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_special_string_values_and_escaping: edgeCasesSpecialStringValuesAndEscaping,
		},
	};
}
async function edgeCases15FloatPrecisionPreservation(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	const responseBody = { value: Math.PI };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCases15FloatPrecisionPreservation() {
	const route = {
		method: "POST",
		path: "/calculate",
		handler_name: "edge_cases_15_float_precision_preservation",
		request_schema: { properties: { value: { type: "number" } }, required: ["value"], type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_15_float_precision_preservation: edgeCases15FloatPrecisionPreservation,
		},
	};
}
async function edgeCases13EmptyStringQueryParamPreserved(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { filter: "" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCases13EmptyStringQueryParamPreserved() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "edge_cases_13_empty_string_query_param_preserved",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { filter: { source: "query", type: "string" } },
			required: ["filter"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_13_empty_string_query_param_preserved: edgeCases13EmptyStringQueryParamPreserved,
		},
	};
}
async function edgeCases24ArrayWithHoles(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { items: ["first", "third", "sixth"] };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCases24ArrayWithHoles() {
	const route = {
		method: "POST",
		path: "/items",
		handler_name: "edge_cases_24_array_with_holes",
		request_schema: {
			properties: { items: { items: { type: "string" }, type: "array" } },
			required: ["items"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_24_array_with_holes: edgeCases24ArrayWithHoles,
		},
	};
}
async function edgeCases21ScientificNotationNumber(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	const responseBody = { value: 123e3 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCases21ScientificNotationNumber() {
	const route = {
		method: "POST",
		path: "/calculate",
		handler_name: "edge_cases_21_scientific_notation_number",
		request_schema: { properties: { value: { minimum: 0, type: "number" } }, required: ["value"], type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_21_scientific_notation_number: edgeCases21ScientificNotationNumber,
		},
	};
}
async function edgeCasesFloatPrecisionAndRounding(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		precise_value: Math.PI,
		sum: 0.30000000000000004,
		very_large: 17976931348623157e292,
		very_small: 1e-10,
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCasesFloatPrecisionAndRounding() {
	const route = {
		method: "POST",
		path: "/calculations/",
		handler_name: "edge_cases_float_precision_and_rounding",
		request_schema: {
			additionalProperties: false,
			properties: {
				expected_sum: { type: "number" },
				precise_value: { type: "number" },
				value1: { type: "number" },
				value2: { type: "number" },
				very_large: { type: "number" },
				very_small: { type: "number" },
			},
			required: ["value1", "value2", "expected_sum", "precise_value", "very_small", "very_large"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_float_precision_and_rounding: edgeCasesFloatPrecisionAndRounding,
		},
	};
}
async function edgeCasesUnicodeAndEmojiHandling(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		description: "Best caf\xE9 in M\xFCnchen \u{1F1E9}\u{1F1EA}",
		emoji_reactions: "\u{1F44D}\u2764\uFE0F\u{1F602}\u{1F389}",
		id: 1,
		name: "Coffee Shop \u2615",
		tags: ["\u98DF\u3079\u7269", "\u97F3\u697D", "\u{1F4B0}"],
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCasesUnicodeAndEmojiHandling() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "edge_cases_unicode_and_emoji_handling",
		request_schema: {
			additionalProperties: false,
			properties: {
				description: { type: "string" },
				emoji_reactions: { type: "string" },
				name: { type: "string" },
				tags: { items: { type: "string" }, type: "array" },
			},
			required: ["name", "description", "tags", "emoji_reactions"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_unicode_and_emoji_handling: edgeCasesUnicodeAndEmojiHandling,
		},
	};
}
async function edgeCases17ExtremelyLongString(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppEdgeCases17ExtremelyLongString() {
	const route = {
		method: "POST",
		path: "/text",
		handler_name: "edge_cases_17_extremely_long_string",
		request_schema: {
			properties: { content: { maxLength: 1e4, type: "string" } },
			required: ["content"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_17_extremely_long_string: edgeCases17ExtremelyLongString,
		},
	};
}
async function edgeCases11Utf8QueryParameter(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { term: "caf\xE9" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCases11Utf8QueryParameter() {
	const route = {
		method: "GET",
		path: "/search",
		handler_name: "edge_cases_11_utf8_query_parameter",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { term: { source: "query", type: "string" } }, required: ["term"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_11_utf8_query_parameter: edgeCases11Utf8QueryParameter,
		},
	};
}
async function edgeCases18UnicodeNormalization(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	const responseBody = { name: "caf\xE9" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCases18UnicodeNormalization() {
	const route = {
		method: "POST",
		path: "/users",
		handler_name: "edge_cases_18_unicode_normalization",
		request_schema: { properties: { name: { minLength: 1, type: "string" } }, required: ["name"], type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_18_unicode_normalization: edgeCases18UnicodeNormalization,
		},
	};
}
async function edgeCases20NullByteInString(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppEdgeCases20NullByteInString() {
	const route = {
		method: "POST",
		path: "/files",
		handler_name: "edge_cases_20_null_byte_in_string",
		request_schema: {
			properties: { filename: { pattern: "^[^\\x00]+$", type: "string" } },
			required: ["filename"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_20_null_byte_in_string: edgeCases20NullByteInString,
		},
	};
}
async function edgeCases23DeeplyNestedJsonLimit(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 400 };
	const responseBody = { error: "Request body exceeds maximum nesting depth of 32" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCases23DeeplyNestedJsonLimit() {
	const route = {
		method: "POST",
		path: "/data",
		handler_name: "edge_cases_23_deeply_nested_json_limit",
		request_schema: { type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_23_deeply_nested_json_limit: edgeCases23DeeplyNestedJsonLimit,
		},
	};
}
async function edgeCases14LargeIntegerBoundary(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { id: 9007199254740991 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCases14LargeIntegerBoundary() {
	const route = {
		method: "GET",
		path: "/items",
		handler_name: "edge_cases_14_large_integer_boundary",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { id: { source: "query", type: "integer" } }, required: ["id"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_14_large_integer_boundary: edgeCases14LargeIntegerBoundary,
		},
	};
}
async function edgeCases22LeadingZerosInteger(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { value: 123 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCases22LeadingZerosInteger() {
	const route = {
		method: "GET",
		path: "/data",
		handler_name: "edge_cases_22_leading_zeros_integer",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { value: { annotation: "int", source: "query", type: "integer" } },
			required: ["value"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_22_leading_zeros_integer: edgeCases22LeadingZerosInteger,
		},
	};
}
async function edgeCasesLargeIntegerBoundaryValues(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		large_int: "9223372036854775807",
		max_safe_int: 9007199254740991,
		negative_large: "-9223372036854775808",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCasesLargeIntegerBoundaryValues() {
	const route = {
		method: "POST",
		path: "/numbers/",
		handler_name: "edge_cases_large_integer_boundary_values",
		request_schema: {
			additionalProperties: false,
			properties: {
				large_int: { type: "integer" },
				max_safe_int: { type: "integer" },
				negative_large: { type: "integer" },
			},
			required: ["max_safe_int", "large_int", "negative_large"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_large_integer_boundary_values: edgeCasesLargeIntegerBoundaryValues,
		},
	};
}
async function edgeCasesDeeplyNestedStructure10Levels(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { max_depth: 10, message: "Processed deeply nested structure", value_found: "deep" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCasesDeeplyNestedStructure10Levels() {
	const route = {
		method: "POST",
		path: "/nested/",
		handler_name: "edge_cases_deeply_nested_structure_10_levels",
		request_schema: {
			additionalProperties: false,
			properties: {
				level1: {
					additionalProperties: false,
					properties: {
						level2: {
							additionalProperties: false,
							properties: {
								level3: {
									additionalProperties: false,
									properties: {
										level4: {
											additionalProperties: false,
											properties: {
												level5: {
													additionalProperties: false,
													properties: {
														level6: {
															additionalProperties: false,
															properties: {
																level7: {
																	additionalProperties: false,
																	properties: {
																		level8: {
																			additionalProperties: false,
																			properties: {
																				level9: {
																					additionalProperties: false,
																					properties: {
																						level10: {
																							additionalProperties: false,
																							properties: { depth: { type: "integer" }, value: { type: "string" } },
																							required: ["value", "depth"],
																							type: "object",
																						},
																					},
																					required: ["level10"],
																					type: "object",
																				},
																			},
																			required: ["level9"],
																			type: "object",
																		},
																	},
																	required: ["level8"],
																	type: "object",
																},
															},
															required: ["level7"],
															type: "object",
														},
													},
													required: ["level6"],
													type: "object",
												},
											},
											required: ["level5"],
											type: "object",
										},
									},
									required: ["level4"],
									type: "object",
								},
							},
							required: ["level3"],
							type: "object",
						},
					},
					required: ["level2"],
					type: "object",
				},
			},
			required: ["level1"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_deeply_nested_structure_10_levels: edgeCasesDeeplyNestedStructure10Levels,
		},
	};
}
async function edgeCasesEmptyAndNullValueHandling(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		empty_array_length: 0,
		empty_object_keys: 0,
		empty_string_length: 0,
		explicit_null_is_null: true,
		false_is_false: true,
		zero_is_falsy: true,
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCasesEmptyAndNullValueHandling() {
	const route = {
		method: "POST",
		path: "/nulls/",
		handler_name: "edge_cases_empty_and_null_value_handling",
		request_schema: {
			additionalProperties: false,
			properties: {
				empty_array: { items: {}, type: "array" },
				empty_object: { additionalProperties: false, properties: {}, type: "object" },
				empty_string: { type: "string" },
				explicit_null: { type: "null" },
				false_boolean: { type: "boolean" },
				zero_number: { type: "integer" },
			},
			required: ["explicit_null", "empty_string", "empty_array", "empty_object", "zero_number", "false_boolean"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_empty_and_null_value_handling: edgeCasesEmptyAndNullValueHandling,
		},
	};
}
async function edgeCases16NegativeZeroHandling(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	const responseBody = { offset: 0 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppEdgeCases16NegativeZeroHandling() {
	const route = {
		method: "POST",
		path: "/data",
		handler_name: "edge_cases_16_negative_zero_handling",
		request_schema: { properties: { offset: { type: "number" } }, required: ["offset"], type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			edge_cases_16_negative_zero_handling: edgeCases16NegativeZeroHandling,
		},
	};
}
async function urlEncodedSimpleFormSubmissionSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { username: "johndoe" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppUrlEncodedSimpleFormSubmissionSuccess() {
	const route = {
		method: "POST",
		path: "/login/",
		handler_name: "url_encoded_simple_form_submission_success",
		request_schema: {
			properties: { password: { type: "string" }, username: { type: "string" } },
			required: ["username", "password"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_simple_form_submission_success: urlEncodedSimpleFormSubmissionSuccess,
		},
	};
}
async function urlEncoded15SpecialCharactersFieldNames(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	const responseBody = { "contact.email": "john@example.com", "user-name": "JohnDoe" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppUrlEncoded15SpecialCharactersFieldNames() {
	const route = {
		method: "POST",
		path: "/data",
		handler_name: "url_encoded_15_special_characters_field_names",
		request_schema: {
			properties: { "contact.email": { format: "email", type: "string" }, "user-name": { type: "string" } },
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_15_special_characters_field_names: urlEncoded15SpecialCharactersFieldNames,
		},
	};
}
async function urlEncodedPatternValidationFail(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppUrlEncodedPatternValidationFail() {
	const route = {
		method: "POST",
		path: "/form/validated",
		handler_name: "url_encoded_pattern_validation_fail",
		request_schema: {
			properties: { username: { pattern: "^[a-z0-9_]+$", type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_pattern_validation_fail: urlEncodedPatternValidationFail,
		},
	};
}
async function urlEncoded22AdditionalPropertiesStrictFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppUrlEncoded22AdditionalPropertiesStrictFailure() {
	const route = {
		method: "POST",
		path: "/settings",
		handler_name: "url_encoded_22_additional_properties_strict_failure",
		request_schema: {
			additionalProperties: false,
			properties: { theme: { enum: ["light", "dark"], type: "string" } },
			required: ["theme"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_22_additional_properties_strict_failure: urlEncoded22AdditionalPropertiesStrictFailure,
		},
	};
}
async function urlEncoded17PatternValidationFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppUrlEncoded17PatternValidationFailure() {
	const route = {
		method: "POST",
		path: "/accounts",
		handler_name: "url_encoded_17_pattern_validation_failure",
		request_schema: {
			properties: { account_id: { pattern: "^ACC-[0-9]{6}$", type: "string" } },
			required: ["account_id"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_17_pattern_validation_failure: urlEncoded17PatternValidationFailure,
		},
	};
}
async function urlEncoded20FormatEmailValidationFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppUrlEncoded20FormatEmailValidationFailure() {
	const route = {
		method: "POST",
		path: "/subscribe",
		handler_name: "url_encoded_20_format_email_validation_failure",
		request_schema: { properties: { email: { format: "email", type: "string" } }, required: ["email"], type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_20_format_email_validation_failure: urlEncoded20FormatEmailValidationFailure,
		},
	};
}
async function urlEncodedMultipleValuesForSameField(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { tags: ["python", "fastapi", "web"] };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppUrlEncodedMultipleValuesForSameField() {
	const route = {
		method: "POST",
		path: "/form/tags",
		handler_name: "url_encoded_multiple_values_for_same_field",
		request_schema: {
			properties: { tags: { items: { type: "string" }, type: "array" } },
			required: ["tags"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_multiple_values_for_same_field: urlEncodedMultipleValuesForSameField,
		},
	};
}
async function urlEncodedRequiredFieldMissingValidationError(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppUrlEncodedRequiredFieldMissingValidationError() {
	const route = {
		method: "POST",
		path: "/login/",
		handler_name: "url_encoded_required_field_missing_validation_error",
		request_schema: {
			properties: { password: { type: "string" }, username: { type: "string" } },
			required: ["username", "password"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_required_field_missing_validation_error: urlEncodedRequiredFieldMissingValidationError,
		},
	};
}
async function urlEncoded13ArrayFieldSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	const responseBody = { tags: ["python", "rust", "typescript"] };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppUrlEncoded13ArrayFieldSuccess() {
	const route = {
		method: "POST",
		path: "/register",
		handler_name: "url_encoded_13_array_field_success",
		request_schema: {
			properties: { tags: { items: { type: "string" }, minItems: 1, type: "array" } },
			required: ["tags"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_13_array_field_success: urlEncoded13ArrayFieldSuccess,
		},
	};
}
async function urlEncodedNumericFieldTypeConversion(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { age: 30, username: "johndoe" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppUrlEncodedNumericFieldTypeConversion() {
	const route = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_numeric_field_type_conversion",
		request_schema: {
			properties: { age: { type: "integer" }, username: { type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_numeric_field_type_conversion: urlEncodedNumericFieldTypeConversion,
		},
	};
}
async function urlEncodedSpecialCharactersEncoding(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { description: "Test & Development", name: "John Doe" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppUrlEncodedSpecialCharactersEncoding() {
	const route = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_special_characters_encoding",
		request_schema: {
			properties: { description: { type: "string" }, name: { type: "string" } },
			required: ["name"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_special_characters_encoding: urlEncodedSpecialCharactersEncoding,
		},
	};
}
async function urlEncodedBooleanFieldConversion(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { subscribe: true, username: "johndoe" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppUrlEncodedBooleanFieldConversion() {
	const route = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_boolean_field_conversion",
		request_schema: {
			properties: { subscribe: { type: "boolean" }, username: { type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_boolean_field_conversion: urlEncodedBooleanFieldConversion,
		},
	};
}
async function urlEncodedEmptyStringValue(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { description: "", username: "johndoe" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppUrlEncodedEmptyStringValue() {
	const route = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_empty_string_value",
		request_schema: {
			properties: { description: { type: "string" }, username: { type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_empty_string_value: urlEncodedEmptyStringValue,
		},
	};
}
async function urlEncodedOauth2PasswordGrantFlow(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { access_token: "johndoe", token_type: "bearer" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppUrlEncodedOauth2PasswordGrantFlow() {
	const route = {
		method: "POST",
		path: "/token",
		handler_name: "url_encoded_oauth2_password_grant_flow",
		request_schema: {
			properties: {
				grant_type: { type: "string" },
				password: { type: "string" },
				scope: { type: "string" },
				username: { type: "string" },
			},
			required: ["username", "password", "grant_type"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_oauth2_password_grant_flow: urlEncodedOauth2PasswordGrantFlow,
		},
	};
}
async function urlEncoded19ArrayMinitemsValidationFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppUrlEncoded19ArrayMinitemsValidationFailure() {
	const route = {
		method: "POST",
		path: "/tags",
		handler_name: "url_encoded_19_array_minitems_validation_failure",
		request_schema: {
			properties: { tags: { items: { type: "string" }, minItems: 2, type: "array" } },
			required: ["tags"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_19_array_minitems_validation_failure: urlEncoded19ArrayMinitemsValidationFailure,
		},
	};
}
async function urlEncodedOptionalFieldMissingSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { email: null, username: "johndoe" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppUrlEncodedOptionalFieldMissingSuccess() {
	const route = {
		method: "POST",
		path: "/register/",
		handler_name: "url_encoded_optional_field_missing_success",
		request_schema: {
			properties: {
				email: { format: "email", type: ["string", "null"] },
				password: { type: "string" },
				username: { type: "string" },
			},
			required: ["username", "password"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_optional_field_missing_success: urlEncodedOptionalFieldMissingSuccess,
		},
	};
}
async function urlEncoded14NestedObjectBracketNotation(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	const responseBody = { user: { age: 30, email: "john@example.com", name: "John Doe" } };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppUrlEncoded14NestedObjectBracketNotation() {
	const route = {
		method: "POST",
		path: "/profile",
		handler_name: "url_encoded_14_nested_object_bracket_notation",
		request_schema: {
			properties: {
				user: {
					properties: {
						age: { minimum: 0, type: "integer" },
						email: { format: "email", type: "string" },
						name: { minLength: 1, type: "string" },
					},
					required: ["name", "email"],
					type: "object",
				},
			},
			required: ["user"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_14_nested_object_bracket_notation: urlEncoded14NestedObjectBracketNotation,
		},
	};
}
async function urlEncodedStringMaxLengthValidationFail(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppUrlEncodedStringMaxLengthValidationFail() {
	const route = {
		method: "POST",
		path: "/form/validated",
		handler_name: "url_encoded_string_max_length_validation_fail",
		request_schema: {
			properties: { username: { maxLength: 20, type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_string_max_length_validation_fail: urlEncodedStringMaxLengthValidationFail,
		},
	};
}
async function urlEncoded18IntegerMinimumValidationFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppUrlEncoded18IntegerMinimumValidationFailure() {
	const route = {
		method: "POST",
		path: "/products",
		handler_name: "url_encoded_18_integer_minimum_validation_failure",
		request_schema: {
			properties: { quantity: { minimum: 1, type: "integer" } },
			required: ["quantity"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_18_integer_minimum_validation_failure: urlEncoded18IntegerMinimumValidationFailure,
		},
	};
}
async function urlEncoded21IntegerTypeCoercionFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppUrlEncoded21IntegerTypeCoercionFailure() {
	const route = {
		method: "POST",
		path: "/products",
		handler_name: "url_encoded_21_integer_type_coercion_failure",
		request_schema: { properties: { price: { type: "integer" } }, required: ["price"], type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_21_integer_type_coercion_failure: urlEncoded21IntegerTypeCoercionFailure,
		},
	};
}
async function urlEncoded16MinlengthValidationFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppUrlEncoded16MinlengthValidationFailure() {
	const route = {
		method: "POST",
		path: "/users",
		handler_name: "url_encoded_16_minlength_validation_failure",
		request_schema: {
			properties: { username: { minLength: 3, type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_16_minlength_validation_failure: urlEncoded16MinlengthValidationFailure,
		},
	};
}
async function urlEncodedStringMinLengthValidationFail(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppUrlEncodedStringMinLengthValidationFail() {
	const route = {
		method: "POST",
		path: "/form/validated",
		handler_name: "url_encoded_string_min_length_validation_fail",
		request_schema: {
			properties: { username: { minLength: 3, type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			url_encoded_string_min_length_validation_fail: urlEncodedStringMinLengthValidationFail,
		},
	};
}
async function contentTypes415UnsupportedMediaType(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 415 };
	const responseBody = { detail: "Unsupported media type" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypes415UnsupportedMediaType() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "content_types_415_unsupported_media_type",
		request_schema: { type: "string" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_415_unsupported_media_type: contentTypes415UnsupportedMediaType,
		},
	};
}
async function contentTypesXmlResponseApplicationXml(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "content-type": "application/xml" };
	const responseBody = '<?xml version="1.0"?><item><name>Item</name><price>42.0</price></item>';
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypesXmlResponseApplicationXml() {
	const route = {
		method: "GET",
		path: "/xml",
		handler_name: "content_types_xml_response_application_xml",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_xml_response_application_xml: contentTypesXmlResponseApplicationXml,
		},
	};
}
async function contentTypes14ContentTypeCaseInsensitive(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	const responseBody = { name: "test" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypes14ContentTypeCaseInsensitive() {
	const route = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_14_content_type_case_insensitive",
		request_schema: { properties: { name: { type: "string" } }, required: ["name"], type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_14_content_type_case_insensitive: contentTypes14ContentTypeCaseInsensitive,
		},
	};
}
async function contentTypesJsonWithUtf8Charset(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "content-type": "application/json; charset=utf-8" };
	const responseBody = { emoji: "\u2615", name: "Caf\xE9" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypesJsonWithUtf8Charset() {
	const route = {
		method: "GET",
		path: "/items/unicode",
		handler_name: "content_types_json_with_utf_8_charset",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_json_with_utf_8_charset: contentTypesJsonWithUtf8Charset,
		},
	};
}
async function contentTypes16TextPlainNotAccepted(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 415 };
	const responseBody = {
		detail: "Unsupported media type",
		status: 415,
		title: "Unsupported Media Type",
		type: "https://spikard.dev/errors/unsupported-media-type",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypes16TextPlainNotAccepted() {
	const route = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_16_text_plain_not_accepted",
		request_schema: { properties: { data: { type: "string" } }, required: ["data"], type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_16_text_plain_not_accepted: contentTypes16TextPlainNotAccepted,
		},
	};
}
async function contentTypesPdfResponseApplicationPdf(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "content-disposition": "attachment; filename=document.pdf", "content-type": "application/pdf" };
	const responseBody = "pdf_binary_data";
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypesPdfResponseApplicationPdf() {
	const route = {
		method: "GET",
		path: "/download/document.pdf",
		handler_name: "content_types_pdf_response_application_pdf",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_pdf_response_application_pdf: contentTypesPdfResponseApplicationPdf,
		},
	};
}
async function contentTypes20ContentLengthMismatch(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 400 };
	const responseBody = { error: "Content-Length header does not match actual body size" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypes20ContentLengthMismatch() {
	const route = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_20_content_length_mismatch",
		request_schema: { properties: { value: { type: "string" } }, type: "object" },
		response_schema: void 0,
		parameter_schema: { properties: { "Content-Length": { source: "header", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_20_content_length_mismatch: contentTypes20ContentLengthMismatch,
		},
	};
}
async function contentTypes17VendorJsonAccepted(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	const responseBody = { data: "value" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypes17VendorJsonAccepted() {
	const route = {
		method: "POST",
		path: "/api/v1/resource",
		handler_name: "content_types_17_vendor_json_accepted",
		request_schema: { properties: { data: { type: "string" } }, required: ["data"], type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_17_vendor_json_accepted: contentTypes17VendorJsonAccepted,
		},
	};
}
async function contentTypes13JsonWithCharsetUtf16(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 415 };
	const responseBody = {
		detail: "Unsupported charset 'utf-16' for JSON. Only UTF-8 is supported.",
		status: 415,
		title: "Unsupported Charset",
		type: "https://spikard.dev/errors/unsupported-charset",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypes13JsonWithCharsetUtf16() {
	const route = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_13_json_with_charset_utf16",
		request_schema: { properties: { value: { type: "string" } }, type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_13_json_with_charset_utf16: contentTypes13JsonWithCharsetUtf16,
		},
	};
}
async function contentTypesJsonResponseApplicationJson(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "content-type": "application/json" };
	const responseBody = { name: "Item", price: 42 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypesJsonResponseApplicationJson() {
	const route = {
		method: "GET",
		path: "/items/json",
		handler_name: "content_types_json_response_application_json",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_json_response_application_json: contentTypesJsonResponseApplicationJson,
		},
	};
}
async function contentTypes15MultipartBoundaryRequired(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 400 };
	const responseBody = { error: "multipart/form-data requires 'boundary' parameter" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypes15MultipartBoundaryRequired() {
	const route = {
		method: "POST",
		path: "/upload",
		handler_name: "content_types_15_multipart_boundary_required",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: {}, type: "object" },
		file_params: { document: { required: true } },
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_15_multipart_boundary_required: contentTypes15MultipartBoundaryRequired,
		},
	};
}
async function contentTypesContentNegotiationAcceptHeader(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "content-type": "application/json" };
	const responseBody = { id: 1, name: "Item" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypesContentNegotiationAcceptHeader() {
	const route = {
		method: "GET",
		path: "/accept-test/{id}",
		handler_name: "content_types_content_negotiation_accept_header",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_content_negotiation_accept_header: contentTypesContentNegotiationAcceptHeader,
		},
	};
}
async function contentTypesHtmlResponseTextHtml(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "content-type": "text/html; charset=utf-8" };
	const responseBody = "<html><body><h1>Hello</h1></body></html>";
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypesHtmlResponseTextHtml() {
	const route = {
		method: "GET",
		path: "/html",
		handler_name: "content_types_html_response_text_html",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_html_response_text_html: contentTypesHtmlResponseTextHtml,
		},
	};
}
async function contentTypesJpegImageResponseImageJpeg(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "content-type": "image/jpeg" };
	const responseBody = "jpeg_binary_data";
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypesJpegImageResponseImageJpeg() {
	const route = {
		method: "GET",
		path: "/images/photo.jpg",
		handler_name: "content_types_jpeg_image_response_image_jpeg",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_jpeg_image_response_image_jpeg: contentTypesJpegImageResponseImageJpeg,
		},
	};
}
async function contentTypes19MissingContentTypeDefaultJson(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	const responseBody = { name: "test" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypes19MissingContentTypeDefaultJson() {
	const route = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_19_missing_content_type_default_json",
		request_schema: { properties: { name: { type: "string" } }, required: ["name"], type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_19_missing_content_type_default_json: contentTypes19MissingContentTypeDefaultJson,
		},
	};
}
async function contentTypesPngImageResponseImagePng(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "content-type": "image/png" };
	const responseBody = "png_binary_data";
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypesPngImageResponseImagePng() {
	const route = {
		method: "GET",
		path: "/images/logo.png",
		handler_name: "content_types_png_image_response_image_png",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_png_image_response_image_png: contentTypesPngImageResponseImagePng,
		},
	};
}
async function contentTypesPlainTextResponseTextPlain(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "content-type": "text/plain; charset=utf-8" };
	const responseBody = "Hello, World!";
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypesPlainTextResponseTextPlain() {
	const route = {
		method: "GET",
		path: "/text",
		handler_name: "content_types_plain_text_response_text_plain",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_plain_text_response_text_plain: contentTypesPlainTextResponseTextPlain,
		},
	};
}
async function contentTypes18ContentTypeWithMultipleParams(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	const responseBody = { value: "test" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypes18ContentTypeWithMultipleParams() {
	const route = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_18_content_type_with_multiple_params",
		request_schema: { properties: { value: { type: "string" } }, type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_18_content_type_with_multiple_params: contentTypes18ContentTypeWithMultipleParams,
		},
	};
}
async function contentTypesCsvResponseTextCsv(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = {
		"content-disposition": "attachment; filename=data.csv",
		"content-type": "text/csv; charset=utf-8",
	};
	const responseBody = "id,name,price\n1,Item A,10.0\n2,Item B,20.0";
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypesCsvResponseTextCsv() {
	const route = {
		method: "GET",
		path: "/export/data.csv",
		handler_name: "content_types_csv_response_text_csv",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_csv_response_text_csv: contentTypesCsvResponseTextCsv,
		},
	};
}
async function contentTypesBinaryResponseApplicationOctetStream(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = {
		"content-disposition": "attachment; filename=file.bin",
		"content-type": "application/octet-stream",
	};
	const responseBody = "binary_data_placeholder";
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppContentTypesBinaryResponseApplicationOctetStream() {
	const route = {
		method: "GET",
		path: "/download/file.bin",
		handler_name: "content_types_binary_response_application_octet_stream",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			content_types_binary_response_application_octet_stream: contentTypesBinaryResponseApplicationOctetStream,
		},
	};
}
async function streamingStreamJsonLines(_requestJson) {
	const stream = async function* () {
		yield '{"index":0,"payload":"alpha"}\\n';
		yield '{"index":1,"payload":"beta"}\\n';
		yield '{"index":2,"payload":"gamma"}\\n';
	};
	return new StreamingResponse(stream(), {
		statusCode: 200,
		headers: { "content-type": "application/x-ndjson" },
	});
}
function createAppStreamingStreamJsonLines() {
	const route = {
		method: "GET",
		path: "/stream/json-lines",
		handler_name: "streaming_stream_json_lines",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			streaming_stream_json_lines: streamingStreamJsonLines,
		},
	};
}
async function streamingBinaryLogDownload(_requestJson) {
	const stream = async function* () {
		yield "LOG:";
		yield Buffer.from("AAECAw==", "base64");
		yield "|TAIL|";
		yield Buffer.from("Bw==", "base64");
		yield "\\n";
	};
	return new StreamingResponse(stream(), {
		statusCode: 200,
		headers: { "content-type": "application/octet-stream" },
	});
}
function createAppStreamingBinaryLogDownload() {
	const route = {
		method: "GET",
		path: "/stream/logfile",
		handler_name: "streaming_binary_log_download",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			streaming_binary_log_download: streamingBinaryLogDownload,
		},
	};
}
async function streamingChunkedCsvExport(_requestJson) {
	const stream = async function* () {
		yield "id,name,value\\n";
		yield "1,Alice,42\\n";
		yield "2,Bob,7\\n";
	};
	return new StreamingResponse(stream(), {
		statusCode: 200,
		headers: { "content-type": "text/csv" },
	});
}
function createAppStreamingChunkedCsvExport() {
	const route = {
		method: "GET",
		path: "/stream/csv-report",
		handler_name: "streaming_chunked_csv_export",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			streaming_chunked_csv_export: streamingChunkedCsvExport,
		},
	};
}
async function statusCodes408RequestTimeout(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 408 };
	response.headers = { connection: "close" };
	const responseBody = { detail: "Request timeout" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppStatusCodes408RequestTimeout() {
	const route = {
		method: "POST",
		path: "/slow-endpoint",
		handler_name: "status_codes_408_request_timeout",
		request_schema: {
			additionalProperties: false,
			properties: { data: { type: "string" } },
			required: ["data"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_408_request_timeout: statusCodes408RequestTimeout,
		},
	};
}
async function statusCodes404NotFoundResourceNotFound(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 404 };
	const responseBody = { detail: "Item not found" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppStatusCodes404NotFoundResourceNotFound() {
	const route = {
		method: "GET",
		path: "/status-test/{code}",
		handler_name: "status_codes_404_not_found_resource_not_found",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { code: { source: "path", type: "string" } }, required: ["code"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_404_not_found_resource_not_found: statusCodes404NotFoundResourceNotFound,
		},
	};
}
async function statusCodes503ServiceUnavailableServerOverload(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 503 };
	response.headers = { "retry-after": "120" };
	const responseBody = { detail: "Service temporarily unavailable" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppStatusCodes503ServiceUnavailableServerOverload() {
	const route = {
		method: "GET",
		path: "/health",
		handler_name: "status_codes_503_service_unavailable_server_overload",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_503_service_unavailable_server_overload: statusCodes503ServiceUnavailableServerOverload,
		},
	};
}
async function statusCodes422UnprocessableEntityValidationError(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppStatusCodes422UnprocessableEntityValidationError() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "status_codes_422_unprocessable_entity_validation_error",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, price: { type: "string" } },
			required: ["price", "name"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_422_unprocessable_entity_validation_error: statusCodes422UnprocessableEntityValidationError,
		},
	};
}
async function statusCodes302FoundTemporaryRedirect(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 302 };
	response.headers = { location: "/target-path" };
	response.body = null;
	return JSON.stringify(response);
}
function createAppStatusCodes302FoundTemporaryRedirect() {
	const route = {
		method: "GET",
		path: "/temp-redirect",
		handler_name: "status_codes_302_found_temporary_redirect",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_302_found_temporary_redirect: statusCodes302FoundTemporaryRedirect,
		},
	};
}
async function statusCodes304NotModifiedCachedContentValid(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 304 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppStatusCodes304NotModifiedCachedContentValid() {
	const route = {
		method: "GET",
		path: "/status-test/{code}",
		handler_name: "status_codes_304_not_modified_cached_content_valid",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: { "If-None-Match": { source: "header", type: "string" }, code: { source: "path", type: "string" } },
			required: ["code"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_304_not_modified_cached_content_valid: statusCodes304NotModifiedCachedContentValid,
		},
	};
}
async function statusCodes400BadRequestInvalidRequest(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 400 };
	const responseBody = { detail: "Invalid request format" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppStatusCodes400BadRequestInvalidRequest() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "status_codes_400_bad_request_invalid_request",
		request_schema: { type: "string" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_400_bad_request_invalid_request: statusCodes400BadRequestInvalidRequest,
		},
	};
}
async function statusCodes22501NotImplemented(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 405 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppStatusCodes22501NotImplemented() {
	const route = {
		method: "TRACE",
		path: "/data",
		handler_name: "status_codes_22_501_not_implemented",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_22_501_not_implemented: statusCodes22501NotImplemented,
		},
	};
}
async function statusCodes204NoContentSuccessWithNoBody(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 204 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppStatusCodes204NoContentSuccessWithNoBody() {
	const route = {
		method: "DELETE",
		path: "/status-test/{code}",
		handler_name: "status_codes_204_no_content_success_with_no_body",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { code: { source: "path", type: "string" } }, required: ["code"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_204_no_content_success_with_no_body: statusCodes204NoContentSuccessWithNoBody,
		},
	};
}
async function statusCodes301MovedPermanentlyPermanentRedirect(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 301 };
	response.headers = { location: "/new-path" };
	response.body = null;
	return JSON.stringify(response);
}
function createAppStatusCodes301MovedPermanentlyPermanentRedirect() {
	const route = {
		method: "GET",
		path: "/old-path",
		handler_name: "status_codes_301_moved_permanently_permanent_redirect",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_301_moved_permanently_permanent_redirect: statusCodes301MovedPermanentlyPermanentRedirect,
		},
	};
}
async function statusCodes201CreatedResourceCreated(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	const responseBody = { id: 1, name: "New Item" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppStatusCodes201CreatedResourceCreated() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "status_codes_201_created_resource_created",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" } },
			required: ["name"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_201_created_resource_created: statusCodes201CreatedResourceCreated,
		},
	};
}
async function statusCodes202AcceptedRequestAcceptedForProcessing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 202 };
	const responseBody = { message: "Task accepted for processing", task_id: "abc123" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppStatusCodes202AcceptedRequestAcceptedForProcessing() {
	const route = {
		method: "POST",
		path: "/tasks/",
		handler_name: "status_codes_202_accepted_request_accepted_for_processing",
		request_schema: {
			additionalProperties: false,
			properties: { task: { type: "string" } },
			required: ["task"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_202_accepted_request_accepted_for_processing: statusCodes202AcceptedRequestAcceptedForProcessing,
		},
	};
}
async function statusCodes307TemporaryRedirectMethodPreserved(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 307 };
	response.headers = { location: "/target-post" };
	response.body = null;
	return JSON.stringify(response);
}
function createAppStatusCodes307TemporaryRedirectMethodPreserved() {
	const route = {
		method: "POST",
		path: "/redirect-post",
		handler_name: "status_codes_307_temporary_redirect_method_preserved",
		request_schema: { additionalProperties: false, properties: {}, type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_307_temporary_redirect_method_preserved: statusCodes307TemporaryRedirectMethodPreserved,
		},
	};
}
async function statusCodes500InternalServerErrorServerError(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 500 };
	const responseBody = {
		detail: "Internal server error",
		status: 500,
		title: "Internal Server Error",
		type: "https://spikard.dev/errors/internal-server-error",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppStatusCodes500InternalServerErrorServerError() {
	const route = {
		method: "GET",
		path: "/error",
		handler_name: "status_codes_500_internal_server_error_server_error",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_500_internal_server_error_server_error: statusCodes500InternalServerErrorServerError,
		},
	};
}
async function statusCodes20414UriTooLong(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppStatusCodes20414UriTooLong() {
	const route = {
		method: "GET",
		path: "/data",
		handler_name: "status_codes_20_414_uri_too_long",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_20_414_uri_too_long: statusCodes20414UriTooLong,
		},
	};
}
async function statusCodes401UnauthorizedMissingAuthentication(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 401 };
	response.headers = { "www-authenticate": "Bearer" };
	const responseBody = { detail: "Not authenticated" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppStatusCodes401UnauthorizedMissingAuthentication() {
	const route = {
		method: "GET",
		path: "/users/me",
		handler_name: "status_codes_401_unauthorized_missing_authentication",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_401_unauthorized_missing_authentication: statusCodes401UnauthorizedMissingAuthentication,
		},
	};
}
async function statusCodes23503ServiceUnavailable(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 503 };
	response.headers = { "retry-after": "60" };
	const responseBody = {
		error: "Service Unavailable",
		message: "The service is temporarily unavailable. Please try again later.",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppStatusCodes23503ServiceUnavailable() {
	const route = {
		method: "GET",
		path: "/data",
		handler_name: "status_codes_23_503_service_unavailable",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_23_503_service_unavailable: statusCodes23503ServiceUnavailable,
		},
	};
}
async function statusCodes19413PayloadTooLarge(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 413 };
	const responseBody = {
		error: "Payload Too Large",
		message: "Request body size exceeds maximum allowed size of 1024 bytes",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppStatusCodes19413PayloadTooLarge() {
	const route = {
		method: "POST",
		path: "/upload",
		handler_name: "status_codes_19_413_payload_too_large",
		request_schema: { properties: { data: { type: "string" } }, type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_19_413_payload_too_large: statusCodes19413PayloadTooLarge,
		},
	};
}
async function statusCodes403ForbiddenInsufficientPermissions(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 403 };
	const responseBody = { detail: "Not enough permissions" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppStatusCodes403ForbiddenInsufficientPermissions() {
	const route = {
		method: "GET",
		path: "/admin/users",
		handler_name: "status_codes_403_forbidden_insufficient_permissions",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_403_forbidden_insufficient_permissions: statusCodes403ForbiddenInsufficientPermissions,
		},
	};
}
async function statusCodes21431RequestHeaderFieldsTooLarge(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 431 };
	const responseBody = {
		error: "Request Header Fields Too Large",
		message: "Request headers exceed maximum allowed size of 8192 bytes",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppStatusCodes21431RequestHeaderFieldsTooLarge() {
	const route = {
		method: "GET",
		path: "/data",
		handler_name: "status_codes_21_431_request_header_fields_too_large",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { "X-Large-Header": { source: "header", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_21_431_request_header_fields_too_large: statusCodes21431RequestHeaderFieldsTooLarge,
		},
	};
}
async function statusCodes429TooManyRequests(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 429 };
	response.headers = {
		"retry-after": "60",
		"x-ratelimit-limit": "100",
		"x-ratelimit-remaining": "0",
		"x-ratelimit-reset": "1609459200",
	};
	const responseBody = { detail: "Rate limit exceeded. Try again in 60 seconds." };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppStatusCodes429TooManyRequests() {
	const route = {
		method: "GET",
		path: "/api/resource",
		handler_name: "status_codes_429_too_many_requests",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_429_too_many_requests: statusCodes429TooManyRequests,
		},
	};
}
async function statusCodes200OkSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { id: 1, name: "Item 1" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppStatusCodes200OkSuccess() {
	const route = {
		method: "GET",
		path: "/status-test/{code}",
		handler_name: "status_codes_200_ok_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { code: { source: "path", type: "string" } }, required: ["code"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_200_ok_success: statusCodes200OkSuccess,
		},
	};
}
async function statusCodes206PartialContent(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 206 };
	response.headers = {
		"accept-ranges": "bytes",
		"content-length": "1024",
		"content-range": "bytes 0-1023/5000",
		"content-type": "application/pdf",
	};
	const contentValue = "binary_data_1024_bytes";
	let contentBytes = Buffer.from(contentValue, "utf-8");
	if (contentBytes.length < 1024) {
		const padding = Buffer.alloc(1024 - contentBytes.length, " ");
		contentBytes = Buffer.concat([contentBytes, padding]);
	}
	async function* streamContent() {
		yield contentBytes;
	}
	return new StreamingResponse(streamContent(), {
		statusCode: 206,
		headers: response.headers ?? {},
	});
}
function createAppStatusCodes206PartialContent() {
	const route = {
		method: "GET",
		path: "/files/document.pdf",
		handler_name: "status_codes_206_partial_content",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			status_codes_206_partial_content: statusCodes206PartialContent,
		},
	};
}
async function multipartMultipleValuesForSameFieldName(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		files: [
			{ content: "first file", content_type: "text/plain", filename: "file1.txt", size: 10 },
			{ content: "second file", content_type: "text/plain", filename: "file2.txt", size: 11 },
		],
		tags: ["python", "rust", "web"],
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppMultipartMultipleValuesForSameFieldName() {
	const route = {
		method: "POST",
		path: "/",
		handler_name: "multipart_multiple_values_for_same_field_name",
		request_schema: {
			additionalProperties: false,
			properties: {
				files: { items: { format: "binary", type: "string" }, type: "array" },
				tags: { items: { type: "string" }, type: "array" },
			},
			required: ["files"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_multiple_values_for_same_field_name: multipartMultipleValuesForSameFieldName,
		},
	};
}
async function multipart19FileMimeSpoofingPngAsJpeg(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	response.body = result;
	return JSON.stringify(response);
}
function createAppMultipart19FileMimeSpoofingPngAsJpeg() {
	const route = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_19_file_mime_spoofing_png_as_jpeg",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: {}, type: "object" },
		file_params: { image: { content_type: ["image/jpeg"], required: true, validate_magic_numbers: true } },
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_19_file_mime_spoofing_png_as_jpeg: multipart19FileMimeSpoofingPngAsJpeg,
		},
	};
}
async function multipart20FileMimeSpoofingJpegAsPng(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	response.body = result;
	return JSON.stringify(response);
}
function createAppMultipart20FileMimeSpoofingJpegAsPng() {
	const route = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_20_file_mime_spoofing_jpeg_as_png",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: {}, type: "object" },
		file_params: { image: { content_type: ["image/png"], required: true, validate_magic_numbers: true } },
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_20_file_mime_spoofing_jpeg_as_png: multipart20FileMimeSpoofingJpegAsPng,
		},
	};
}
async function multipart21FilePdfMagicNumberSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppMultipart21FilePdfMagicNumberSuccess() {
	const route = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_21_file_pdf_magic_number_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: {}, type: "object" },
		file_params: { document: { content_type: ["application/pdf"], required: true, validate_magic_numbers: true } },
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_21_file_pdf_magic_number_success: multipart21FilePdfMagicNumberSuccess,
		},
	};
}
async function multipartContentTypeValidationInvalidType(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppMultipartContentTypeValidationInvalidType() {
	const route = {
		method: "POST",
		path: "/files/images-only",
		handler_name: "multipart_content_type_validation_invalid_type",
		request_schema: {
			additionalProperties: false,
			properties: { file: { format: "binary", type: "string" } },
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: { properties: {}, type: "object" },
		file_params: { file: { content_type: ["image/jpeg", "image/png", "image/gif"], required: true } },
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_content_type_validation_invalid_type: multipartContentTypeValidationInvalidType,
		},
	};
}
async function multipartPdfFileUpload(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { content_type: "application/pdf", filename: "report.pdf", size: 16 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppMultipartPdfFileUpload() {
	const route = {
		method: "POST",
		path: "/files/document",
		handler_name: "multipart_pdf_file_upload",
		request_schema: {
			additionalProperties: false,
			properties: { document: { format: "binary", type: "string" } },
			required: ["document"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_pdf_file_upload: multipartPdfFileUpload,
		},
	};
}
async function multipartFileListUploadArrayOfFiles(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { filenames: ["file1.txt", "file2.txt"], total_size: 35 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppMultipartFileListUploadArrayOfFiles() {
	const route = {
		method: "POST",
		path: "/files/list",
		handler_name: "multipart_file_list_upload_array_of_files",
		request_schema: {
			additionalProperties: false,
			properties: { files: { items: { format: "binary", type: "string" }, type: "array" } },
			required: ["files"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_file_list_upload_array_of_files: multipartFileListUploadArrayOfFiles,
		},
	};
}
async function multipartOptionalFileUploadProvided(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { content_type: "text/plain", filename: "optional.txt", size: 21 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppMultipartOptionalFileUploadProvided() {
	const route = {
		method: "POST",
		path: "/files/optional",
		handler_name: "multipart_optional_file_upload_provided",
		request_schema: {
			additionalProperties: false,
			properties: { file: { format: "binary", type: "string" } },
			required: ["file"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_optional_file_upload_provided: multipartOptionalFileUploadProvided,
		},
	};
}
async function multipartFileSizeValidationTooLarge(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 413 };
	const responseBody = { detail: "File too large. Maximum size is 1MB" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppMultipartFileSizeValidationTooLarge() {
	const route = {
		method: "POST",
		path: "/files/validated",
		handler_name: "multipart_file_size_validation_too_large",
		request_schema: {
			additionalProperties: false,
			properties: { file: { format: "binary", type: "string" } },
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_file_size_validation_too_large: multipartFileSizeValidationTooLarge,
		},
	};
}
async function multipartMixedFilesAndFormData(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		active: "true",
		age: "25",
		file: { content: "file data here", content_type: "text/plain", filename: "upload.txt", size: 14 },
		username: "testuser",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppMultipartMixedFilesAndFormData() {
	const route = {
		method: "POST",
		path: "/",
		handler_name: "multipart_mixed_files_and_form_data",
		request_schema: {
			additionalProperties: false,
			properties: {
				active: { type: "string" },
				age: { type: "string" },
				file: { format: "binary", type: "string" },
				username: { type: "string" },
			},
			required: ["file"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_mixed_files_and_form_data: multipartMixedFilesAndFormData,
		},
	};
}
async function multipartSimpleFileUpload(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		test: { content: "<file content>", content_type: "text/plain", filename: "test.txt", size: 14 },
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppMultipartSimpleFileUpload() {
	const route = {
		method: "POST",
		path: "/",
		handler_name: "multipart_simple_file_upload",
		request_schema: {
			additionalProperties: false,
			properties: { test: { format: "binary", type: "string" } },
			required: ["test"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_simple_file_upload: multipartSimpleFileUpload,
		},
	};
}
async function multipartEmptyFileUpload(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { filename: "empty.txt", size: 0 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppMultipartEmptyFileUpload() {
	const route = {
		method: "POST",
		path: "/files/upload",
		handler_name: "multipart_empty_file_upload",
		request_schema: {
			additionalProperties: false,
			properties: { file: { format: "binary", type: "string" } },
			required: ["file"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_empty_file_upload: multipartEmptyFileUpload,
		},
	};
}
async function multipartOptionalFileUploadMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { file: null };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppMultipartOptionalFileUploadMissing() {
	const route = {
		method: "POST",
		path: "/files/optional",
		handler_name: "multipart_optional_file_upload_missing",
		request_schema: { additionalProperties: false, properties: {}, type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_optional_file_upload_missing: multipartOptionalFileUploadMissing,
		},
	};
}
async function multipartFileUploadWithoutFilename(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { test1: "<file1 content>" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppMultipartFileUploadWithoutFilename() {
	const route = {
		method: "POST",
		path: "/",
		handler_name: "multipart_file_upload_without_filename",
		request_schema: {
			additionalProperties: false,
			properties: { test1: { format: "binary", type: "string" } },
			required: ["test1"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_file_upload_without_filename: multipartFileUploadWithoutFilename,
		},
	};
}
async function multipart18FileMagicNumberJpegSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppMultipart18FileMagicNumberJpegSuccess() {
	const route = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_18_file_magic_number_jpeg_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: {}, type: "object" },
		file_params: { image: { content_type: ["image/jpeg"], required: true, validate_magic_numbers: true } },
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_18_file_magic_number_jpeg_success: multipart18FileMagicNumberJpegSuccess,
		},
	};
}
async function multipart22FileEmptyBuffer(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	response.body = result;
	return JSON.stringify(response);
}
function createAppMultipart22FileEmptyBuffer() {
	const route = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_22_file_empty_buffer",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: {}, type: "object" },
		file_params: { file: { required: true, validate_magic_numbers: true } },
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_22_file_empty_buffer: multipart22FileEmptyBuffer,
		},
	};
}
async function multipart17FileMagicNumberPngSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppMultipart17FileMagicNumberPngSuccess() {
	const route = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_17_file_magic_number_png_success",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: {}, type: "object" },
		file_params: { image: { content_type: ["image/png"], required: true, validate_magic_numbers: true } },
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_17_file_magic_number_png_success: multipart17FileMagicNumberPngSuccess,
		},
	};
}
async function multipartFormDataWithoutFiles(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { some: "data" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppMultipartFormDataWithoutFiles() {
	const route = {
		method: "POST",
		path: "/",
		handler_name: "multipart_form_data_without_files",
		request_schema: { additionalProperties: false, properties: { some: { type: "string" } }, type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_form_data_without_files: multipartFormDataWithoutFiles,
		},
	};
}
async function multipartMultipleFileUploads(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		test1: { content: "<file1 content>", content_type: "text/plain", filename: "test1.txt", size: 15 },
		test2: { content: "<file2 content>", content_type: "text/plain", filename: "test2.txt", size: 15 },
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppMultipartMultipleFileUploads() {
	const route = {
		method: "POST",
		path: "/",
		handler_name: "multipart_multiple_file_uploads",
		request_schema: {
			additionalProperties: false,
			properties: { test1: { format: "binary", type: "string" }, test2: { format: "binary", type: "string" } },
			required: ["test1", "test2"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_multiple_file_uploads: multipartMultipleFileUploads,
		},
	};
}
async function multipartFileUploadWithCustomHeaders(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		test2: {
			content: "<file2 content>",
			content_type: "text/plain",
			filename: "test2.txt",
			headers: [
				["content-disposition", 'form-data; name="test2"; filename="test2.txt"'],
				["content-type", "text/plain"],
				["x-custom", "f2"],
			],
			size: 15,
		},
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppMultipartFileUploadWithCustomHeaders() {
	const route = {
		method: "POST",
		path: "/",
		handler_name: "multipart_file_upload_with_custom_headers",
		request_schema: {
			additionalProperties: false,
			properties: { test2: { format: "binary", type: "string" } },
			required: ["test2"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_file_upload_with_custom_headers: multipartFileUploadWithCustomHeaders,
		},
	};
}
async function multipartRequiredFileUploadMissing(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppMultipartRequiredFileUploadMissing() {
	const route = {
		method: "POST",
		path: "/files/required",
		handler_name: "multipart_required_file_upload_missing",
		request_schema: {
			additionalProperties: false,
			properties: { file: { format: "binary", type: "string" } },
			required: ["file"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_required_file_upload_missing: multipartRequiredFileUploadMissing,
		},
	};
}
async function multipartImageFileUpload(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { content_type: "image/jpeg", filename: "photo.jpg", size: 22 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppMultipartImageFileUpload() {
	const route = {
		method: "POST",
		path: "/files/image",
		handler_name: "multipart_image_file_upload",
		request_schema: {
			additionalProperties: false,
			properties: { image: { format: "binary", type: "string" } },
			required: ["image"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			multipart_image_file_upload: multipartImageFileUpload,
		},
	};
}
async function bodyLimitsBodyUnderLimitSucceeds(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { accepted: true, note: "small" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppBodyLimitsBodyUnderLimitSucceeds() {
	const config = {
		maxBodySize: 64,
	};
	const route = {
		method: "POST",
		path: "/body-limit/under",
		handler_name: "body_limits_body_under_limit_succeeds",
		request_schema: {
			additionalProperties: false,
			properties: { note: { type: "string" } },
			required: ["note"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			body_limits_body_under_limit_succeeds: bodyLimitsBodyUnderLimitSucceeds,
		},
		config,
	};
}
async function bodyLimitsBodyOverLimitReturns413(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 413 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppBodyLimitsBodyOverLimitReturns413() {
	const config = {
		maxBodySize: 64,
	};
	const route = {
		method: "POST",
		path: "/body-limit/over",
		handler_name: "body_limits_body_over_limit_returns_413",
		request_schema: {
			additionalProperties: false,
			properties: { note: { type: "string" } },
			required: ["note"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			body_limits_body_over_limit_returns_413: bodyLimitsBodyOverLimitReturns413,
		},
		config,
	};
}
async function cors07CorsPreflightHeaderNotAllowed(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 403 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppCors07CorsPreflightHeaderNotAllowed() {
	const route = {
		method: "OPTIONS",
		path: "/api/data",
		handler_name: "cors_07_cors_preflight_header_not_allowed",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				"Access-Control-Request-Headers": { source: "header", type: "string" },
				"Access-Control-Request-Method": { source: "header", type: "string" },
				Origin: { source: "header", type: "string" },
			},
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_07_cors_preflight_header_not_allowed: cors07CorsPreflightHeaderNotAllowed,
		},
	};
}
async function corsCorsVaryHeaderForProperCaching(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = {
		"access-control-allow-origin": "https://app.example.com",
		"cache-control": "public, max-age=3600",
		vary: "Origin",
	};
	const responseBody = { data: "cacheable resource" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCorsCorsVaryHeaderForProperCaching() {
	const route = {
		method: "GET",
		path: "/api/cached-resource",
		handler_name: "cors_cors_vary_header_for_proper_caching",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_cors_vary_header_for_proper_caching: corsCorsVaryHeaderForProperCaching,
		},
	};
}
async function corsCorsPreflightForPutMethod(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 204 };
	response.headers = {
		"access-control-allow-headers": "Content-Type, X-Custom-Header",
		"access-control-allow-methods": "GET, POST, PUT, PATCH, DELETE",
		"access-control-allow-origin": "https://app.example.com",
		"access-control-max-age": "3600",
		vary: "Origin",
	};
	response.body = null;
	return JSON.stringify(response);
}
function createAppCorsCorsPreflightForPutMethod() {
	const route = {
		method: "OPTIONS",
		path: "/api/resource/123",
		handler_name: "cors_cors_preflight_for_put_method",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_cors_preflight_for_put_method: corsCorsPreflightForPutMethod,
		},
	};
}
async function corsCorsPreflightForDeleteMethod(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 204 };
	response.headers = {
		"access-control-allow-methods": "GET, POST, PUT, PATCH, DELETE",
		"access-control-allow-origin": "https://app.example.com",
		"access-control-max-age": "3600",
		vary: "Origin",
	};
	response.body = null;
	return JSON.stringify(response);
}
function createAppCorsCorsPreflightForDeleteMethod() {
	const route = {
		method: "OPTIONS",
		path: "/api/resource/456",
		handler_name: "cors_cors_preflight_for_delete_method",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_cors_preflight_for_delete_method: corsCorsPreflightForDeleteMethod,
		},
	};
}
async function corsCorsMultipleAllowedOrigins(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "access-control-allow-origin": "https://admin.example.com", vary: "Origin" };
	const responseBody = { data: "resource data" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCorsCorsMultipleAllowedOrigins() {
	const route = {
		method: "GET",
		path: "/api/data",
		handler_name: "cors_cors_multiple_allowed_origins",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_cors_multiple_allowed_origins: corsCorsMultipleAllowedOrigins,
		},
	};
}
async function corsCorsPreflightRequest(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = {
		"access-control-allow-headers": "Content-Type, X-Custom-Header",
		"access-control-allow-methods": "GET, POST, PUT, DELETE, OPTIONS",
		"access-control-allow-origin": "https://example.com",
		"access-control-max-age": "600",
	};
	const result = {};
	response.body = result;
	return JSON.stringify(response);
}
function createAppCorsCorsPreflightRequest() {
	const route = {
		method: "OPTIONS",
		path: "/items/",
		handler_name: "cors_cors_preflight_request",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_cors_preflight_request: corsCorsPreflightRequest,
		},
	};
}
async function corsCorsWithCredentials(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = {
		"access-control-allow-credentials": "true",
		"access-control-allow-origin": "https://app.example.com",
		vary: "Origin",
	};
	const responseBody = { username: "john" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCorsCorsWithCredentials() {
	const route = {
		method: "GET",
		path: "/api/user/profile",
		handler_name: "cors_cors_with_credentials",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_cors_with_credentials: corsCorsWithCredentials,
		},
	};
}
async function corsCorsRegexPatternMatchingForOrigins(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "access-control-allow-origin": "https://subdomain.example.com", vary: "Origin" };
	const responseBody = { data: "resource data" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCorsCorsRegexPatternMatchingForOrigins() {
	const route = {
		method: "GET",
		path: "/api/data",
		handler_name: "cors_cors_regex_pattern_matching_for_origins",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_cors_regex_pattern_matching_for_origins: corsCorsRegexPatternMatchingForOrigins,
		},
	};
}
async function cors08CorsMaxAge(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 204 };
	response.headers = {
		"access-control-allow-headers": "Content-Type",
		"access-control-allow-methods": "POST",
		"access-control-allow-origin": "https://example.com",
		"access-control-max-age": "3600",
	};
	response.body = null;
	return JSON.stringify(response);
}
function createAppCors08CorsMaxAge() {
	const route = {
		method: "OPTIONS",
		path: "/api/data",
		handler_name: "cors_08_cors_max_age",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				"Access-Control-Request-Headers": { source: "header", type: "string" },
				"Access-Control-Request-Method": { source: "header", type: "string" },
				Origin: { source: "header", type: "string" },
			},
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_08_cors_max_age: cors08CorsMaxAge,
		},
	};
}
async function cors10CorsOriginNull(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 403 };
	const responseBody = { error: "Origin 'null' is not allowed" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCors10CorsOriginNull() {
	const route = {
		method: "GET",
		path: "/api/data",
		handler_name: "cors_10_cors_origin_null",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { Origin: { source: "header", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_10_cors_origin_null: cors10CorsOriginNull,
		},
	};
}
async function corsCorsWildcardOrigin(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "access-control-allow-origin": "*" };
	const responseBody = { data: "public" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCorsCorsWildcardOrigin() {
	const route = {
		method: "GET",
		path: "/public/data",
		handler_name: "cors_cors_wildcard_origin",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_cors_wildcard_origin: corsCorsWildcardOrigin,
		},
	};
}
async function corsCorsSafelistedHeadersWithoutPreflight(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "access-control-allow-origin": "https://app.example.com", vary: "Origin" };
	const responseBody = { message: "Success" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCorsCorsSafelistedHeadersWithoutPreflight() {
	const route = {
		method: "POST",
		path: "/api/form",
		handler_name: "cors_cors_safelisted_headers_without_preflight",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_cors_safelisted_headers_without_preflight: corsCorsSafelistedHeadersWithoutPreflight,
		},
	};
}
async function corsCorsPrivateNetworkAccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 204 };
	response.headers = {
		"access-control-allow-methods": "GET, POST",
		"access-control-allow-origin": "https://public.example.com",
		"access-control-allow-private-network": "true",
		vary: "Origin",
	};
	response.body = null;
	return JSON.stringify(response);
}
function createAppCorsCorsPrivateNetworkAccess() {
	const route = {
		method: "OPTIONS",
		path: "/api/local-resource",
		handler_name: "cors_cors_private_network_access",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_cors_private_network_access: corsCorsPrivateNetworkAccess,
		},
	};
}
async function corsCorsOriginCaseSensitivity(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { vary: "Origin" };
	const result = {};
	response.body = result;
	return JSON.stringify(response);
}
function createAppCorsCorsOriginCaseSensitivity() {
	const route = {
		method: "GET",
		path: "/api/data",
		handler_name: "cors_cors_origin_case_sensitivity",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_cors_origin_case_sensitivity: corsCorsOriginCaseSensitivity,
		},
	};
}
async function corsCorsRequestBlocked(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 403 };
	const responseBody = { detail: "CORS request from origin 'https://malicious-site.com' not allowed" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCorsCorsRequestBlocked() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "cors_cors_request_blocked",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { Origin: { source: "header", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_cors_request_blocked: corsCorsRequestBlocked,
		},
	};
}
async function corsSimpleCorsRequest(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	response.headers = { "access-control-allow-origin": "https://example.com", vary: "Origin" };
	const responseBody = { items: [] };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppCorsSimpleCorsRequest() {
	const route = {
		method: "GET",
		path: "/items/",
		handler_name: "cors_simple_cors_request",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_simple_cors_request: corsSimpleCorsRequest,
		},
	};
}
async function cors09CorsExposeHeaders(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response = { status: 200 };
	response.headers = {
		"access-control-allow-origin": "https://example.com",
		"access-control-expose-headers": "X-Total-Count, X-Request-Id",
		"x-request-id": "abc123",
		"x-total-count": "42",
	};
	const result = {};
	const origin = params.Origin;
	if (origin !== null && origin !== void 0) {
		result.Origin = origin;
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppCors09CorsExposeHeaders() {
	const route = {
		method: "GET",
		path: "/api/data",
		handler_name: "cors_09_cors_expose_headers",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: { properties: { Origin: { source: "header", type: "string" } }, type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_09_cors_expose_headers: cors09CorsExposeHeaders,
		},
	};
}
async function cors06CorsPreflightMethodNotAllowed(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 403 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppCors06CorsPreflightMethodNotAllowed() {
	const route = {
		method: "OPTIONS",
		path: "/api/data",
		handler_name: "cors_06_cors_preflight_method_not_allowed",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: {
			properties: {
				"Access-Control-Request-Headers": { source: "header", type: "string" },
				"Access-Control-Request-Method": { source: "header", type: "string" },
				Origin: { source: "header", type: "string" },
			},
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			cors_06_cors_preflight_method_not_allowed: cors06CorsPreflightMethodNotAllowed,
		},
	};
}
async function jsonBodiesUuidFieldInvalidFormat(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodiesUuidFieldInvalidFormat() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_uuid_field_invalid_format",
		request_schema: {
			additionalProperties: false,
			properties: { item_id: { format: "uuid", type: "string" }, name: { type: "string" } },
			required: ["name", "item_id"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_uuid_field_invalid_format: jsonBodiesUuidFieldInvalidFormat,
		},
	};
}
async function jsonBodies44ConstValidationFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodies44ConstValidationFailure() {
	const route = {
		method: "POST",
		path: "/api/v1/data",
		handler_name: "json_bodies_44_const_validation_failure",
		request_schema: {
			properties: { data: { type: "string" }, version: { const: "1.0", type: "string" } },
			required: ["version", "data"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_44_const_validation_failure: jsonBodies44ConstValidationFailure,
		},
	};
}
async function jsonBodiesBooleanFieldSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { in_stock: true, name: "Item", price: 42 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesBooleanFieldSuccess() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_boolean_field_success",
		request_schema: {
			additionalProperties: false,
			properties: { in_stock: { type: "boolean" }, name: { type: "string" }, price: { type: "number" } },
			required: ["name", "price", "in_stock"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_boolean_field_success: jsonBodiesBooleanFieldSuccess,
		},
	};
}
async function jsonBodiesNumericLeValidationSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { name: "Item", price: 100 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesNumericLeValidationSuccess() {
	const route = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_numeric_le_validation_success",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, price: { type: "number" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_numeric_le_validation_success: jsonBodiesNumericLeValidationSuccess,
		},
	};
}
async function jsonBodiesDeeplyNestedObjects(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		name: "Product",
		price: 100,
		seller: {
			address: { city: "Springfield", country: { code: "US", name: "USA" }, street: "123 Main St" },
			name: "John Doe",
		},
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesDeeplyNestedObjects() {
	const route = {
		method: "POST",
		path: "/items/nested",
		handler_name: "json_bodies_deeply_nested_objects",
		request_schema: {
			additionalProperties: false,
			properties: {
				name: { type: "string" },
				price: { type: "number" },
				seller: {
					additionalProperties: false,
					properties: {
						address: {
							additionalProperties: false,
							properties: {
								city: { type: "string" },
								country: {
									additionalProperties: false,
									properties: { code: { type: "string" }, name: { type: "string" } },
									required: ["name", "code"],
									type: "object",
								},
								street: { type: "string" },
							},
							required: ["street", "city", "country"],
							type: "object",
						},
						name: { type: "string" },
					},
					required: ["name", "address"],
					type: "object",
				},
			},
			required: ["name", "price", "seller"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_deeply_nested_objects: jsonBodiesDeeplyNestedObjects,
		},
	};
}
async function jsonBodiesOptionalFieldsOmitted(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { description: null, name: "Foo", price: 35.4, tax: null };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesOptionalFieldsOmitted() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_optional_fields_omitted",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, price: { type: "number" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_optional_fields_omitted: jsonBodiesOptionalFieldsOmitted,
		},
	};
}
async function jsonBodiesUuidFieldSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item_id: "c892496f-b1fd-4b91-bdb8-b46f92df1716", name: "Item" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesUuidFieldSuccess() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_uuid_field_success",
		request_schema: {
			additionalProperties: false,
			properties: { item_id: { format: "uuid", type: "string" }, name: { type: "string" } },
			required: ["name", "item_id"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_uuid_field_success: jsonBodiesUuidFieldSuccess,
		},
	};
}
async function jsonBodiesDateFieldSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { event_date: "2024-03-15", name: "Conference" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesDateFieldSuccess() {
	const route = {
		method: "POST",
		path: "/events/",
		handler_name: "json_bodies_date_field_success",
		request_schema: {
			additionalProperties: false,
			properties: { event_date: { type: "string" }, name: { type: "string" } },
			required: ["name", "event_date"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_date_field_success: jsonBodiesDateFieldSuccess,
		},
	};
}
async function jsonBodies47MaxpropertiesValidationFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodies47MaxpropertiesValidationFailure() {
	const route = {
		method: "POST",
		path: "/config",
		handler_name: "json_bodies_47_maxproperties_validation_failure",
		request_schema: { maxProperties: 3, type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_47_maxproperties_validation_failure: jsonBodies47MaxpropertiesValidationFailure,
		},
	};
}
async function jsonBodies46MinpropertiesValidationFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodies46MinpropertiesValidationFailure() {
	const route = {
		method: "POST",
		path: "/config",
		handler_name: "json_bodies_46_minproperties_validation_failure",
		request_schema: { minProperties: 2, type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_46_minproperties_validation_failure: jsonBodies46MinpropertiesValidationFailure,
		},
	};
}
async function jsonBodiesStringMinLengthValidationFail(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodiesStringMinLengthValidationFail() {
	const route = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_string_min_length_validation_fail",
		request_schema: {
			additionalProperties: false,
			properties: { name: { minLength: 3, type: "string" }, price: { type: "number" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_string_min_length_validation_fail: jsonBodiesStringMinLengthValidationFail,
		},
	};
}
async function jsonBodiesFieldTypeValidationInvalidType(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodiesFieldTypeValidationInvalidType() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_field_type_validation_invalid_type",
		request_schema: {
			additionalProperties: false,
			properties: {
				description: { type: "string" },
				name: { type: "string" },
				price: { type: "number" },
				tax: { type: "number" },
			},
			required: ["name", "description", "price", "tax"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_field_type_validation_invalid_type: jsonBodiesFieldTypeValidationInvalidType,
		},
	};
}
async function jsonBodies36OneofSchemaMultipleMatchFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodies36OneofSchemaMultipleMatchFailure() {
	const route = {
		method: "POST",
		path: "/payment",
		handler_name: "json_bodies_36_oneof_schema_multiple_match_failure",
		request_schema: {
			oneOf: [
				{
					properties: { credit_card: { pattern: "^[0-9]{16}$", type: "string" } },
					required: ["credit_card"],
					type: "object",
				},
				{
					properties: { paypal_email: { format: "email", type: "string" } },
					required: ["paypal_email"],
					type: "object",
				},
			],
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_36_oneof_schema_multiple_match_failure: jsonBodies36OneofSchemaMultipleMatchFailure,
		},
	};
}
async function jsonBodiesNestedObjectSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		image: { name: "Product Image", url: "https://example.com/image.jpg" },
		name: "Foo",
		price: 42,
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesNestedObjectSuccess() {
	const route = {
		method: "POST",
		path: "/items/nested",
		handler_name: "json_bodies_nested_object_success",
		request_schema: {
			additionalProperties: false,
			properties: {
				image: {
					additionalProperties: false,
					properties: { name: { type: "string" }, url: { type: "string" } },
					required: ["url", "name"],
					type: "object",
				},
				name: { type: "string" },
				price: { type: "number" },
			},
			required: ["name", "price", "image"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_nested_object_success: jsonBodiesNestedObjectSuccess,
		},
	};
}
async function jsonBodies41NotSchemaSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppJsonBodies41NotSchemaSuccess() {
	const route = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_41_not_schema_success",
		request_schema: {
			properties: { username: { not: { enum: ["admin", "root", "system"] }, type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_41_not_schema_success: jsonBodies41NotSchemaSuccess,
		},
	};
}
async function jsonBodiesStringMaxLengthValidationFail(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodiesStringMaxLengthValidationFail() {
	const route = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_string_max_length_validation_fail",
		request_schema: {
			additionalProperties: false,
			properties: { name: { maxLength: 50, type: "string" }, price: { type: "number" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_string_max_length_validation_fail: jsonBodiesStringMaxLengthValidationFail,
		},
	};
}
async function jsonBodies50DeepNesting4Levels(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppJsonBodies50DeepNesting4Levels() {
	const route = {
		method: "POST",
		path: "/data",
		handler_name: "json_bodies_50_deep_nesting_4_levels",
		request_schema: {
			properties: {
				user: {
					properties: {
						profile: {
							properties: {
								contact: {
									properties: {
										address: { properties: { street: { type: "string" } }, required: ["street"], type: "object" },
									},
									required: ["address"],
									type: "object",
								},
							},
							required: ["contact"],
							type: "object",
						},
					},
					required: ["profile"],
					type: "object",
				},
			},
			required: ["user"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_50_deep_nesting_4_levels: jsonBodies50DeepNesting4Levels,
		},
	};
}
async function jsonBodies48DependenciesValidationSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppJsonBodies48DependenciesValidationSuccess() {
	const route = {
		method: "POST",
		path: "/billing",
		handler_name: "json_bodies_48_dependencies_validation_success",
		request_schema: {
			dependencies: { credit_card: ["billing_address"] },
			properties: { billing_address: { type: "string" }, credit_card: { type: "string" }, name: { type: "string" } },
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_48_dependencies_validation_success: jsonBodies48DependenciesValidationSuccess,
		},
	};
}
async function jsonBodiesPatchPartialUpdate(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { description: "Original description", name: "Original Item", price: 45 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesPatchPartialUpdate() {
	const route = {
		method: "PATCH",
		path: "/items/{id}",
		handler_name: "json_bodies_patch_partial_update",
		request_schema: { properties: { price: { type: "number" } }, required: ["price"], type: "object" },
		response_schema: void 0,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_patch_partial_update: jsonBodiesPatchPartialUpdate,
		},
	};
}
async function jsonBodies30NestedObjectMissingField(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodies30NestedObjectMissingField() {
	const route = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_30_nested_object_missing_field",
		request_schema: {
			properties: {
				profile: {
					properties: { email: { format: "email", type: "string" }, name: { minLength: 1, type: "string" } },
					required: ["name", "email"],
					type: "object",
				},
			},
			required: ["profile"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_30_nested_object_missing_field: jsonBodies30NestedObjectMissingField,
		},
	};
}
async function jsonBodiesDatetimeFieldSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { created_at: "2024-03-15T10:30:00Z", name: "Meeting" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesDatetimeFieldSuccess() {
	const route = {
		method: "POST",
		path: "/events/",
		handler_name: "json_bodies_datetime_field_success",
		request_schema: {
			additionalProperties: false,
			properties: { created_at: { format: "date-time", type: "string" }, name: { type: "string" } },
			required: ["name", "created_at"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_datetime_field_success: jsonBodiesDatetimeFieldSuccess,
		},
	};
}
async function jsonBodiesStringPatternValidationSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { name: "Item", sku: "ABC1234" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesStringPatternValidationSuccess() {
	const route = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_string_pattern_validation_success",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, sku: { type: "string" } },
			required: ["name", "sku"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_string_pattern_validation_success: jsonBodiesStringPatternValidationSuccess,
		},
	};
}
async function jsonBodiesExtraFieldsIgnoredNoAdditionalproperties(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { name: "Item", price: 42 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesExtraFieldsIgnoredNoAdditionalproperties() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_extra_fields_ignored_no_additionalproperties",
		request_schema: {
			additionalProperties: false,
			properties: {
				another_extra: { type: "integer" },
				extra_field: { type: "string" },
				name: { type: "string" },
				price: { type: "number" },
			},
			required: ["name", "price", "extra_field", "another_extra"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_extra_fields_ignored_no_additionalproperties: jsonBodiesExtraFieldsIgnoredNoAdditionalproperties,
		},
	};
}
async function jsonBodies40AnyofSchemaFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodies40AnyofSchemaFailure() {
	const route = {
		method: "POST",
		path: "/contact",
		handler_name: "json_bodies_40_anyof_schema_failure",
		request_schema: {
			anyOf: [{ required: ["email"] }, { required: ["phone"] }],
			properties: { email: { format: "email", type: "string" }, name: { type: "string" }, phone: { type: "string" } },
			required: ["name"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_40_anyof_schema_failure: jsonBodies40AnyofSchemaFailure,
		},
	};
}
async function jsonBodies39AnyofSchemaMultipleMatchSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppJsonBodies39AnyofSchemaMultipleMatchSuccess() {
	const route = {
		method: "POST",
		path: "/contact",
		handler_name: "json_bodies_39_anyof_schema_multiple_match_success",
		request_schema: {
			anyOf: [{ required: ["email"] }, { required: ["phone"] }],
			properties: { email: { format: "email", type: "string" }, name: { type: "string" }, phone: { type: "string" } },
			required: ["name"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_39_anyof_schema_multiple_match_success: jsonBodies39AnyofSchemaMultipleMatchSuccess,
		},
	};
}
async function jsonBodiesArrayOfPrimitiveValues(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { name: "Product", ratings: [4.5, 4.8, 5, 4.2], tags: ["electronics", "gadget", "new"] };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesArrayOfPrimitiveValues() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_array_of_primitive_values",
		request_schema: {
			additionalProperties: false,
			properties: {
				name: { type: "string" },
				ratings: { items: { type: "number" }, type: "array" },
				tags: { items: { type: "string" }, type: "array" },
			},
			required: ["name", "tags", "ratings"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_array_of_primitive_values: jsonBodiesArrayOfPrimitiveValues,
		},
	};
}
async function jsonBodiesNumericGeValidationFail(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodiesNumericGeValidationFail() {
	const route = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_numeric_ge_validation_fail",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, price: { minimum: 1, type: "number" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_numeric_ge_validation_fail: jsonBodiesNumericGeValidationFail,
		},
	};
}
async function jsonBodies37OneofSchemaNoMatchFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodies37OneofSchemaNoMatchFailure() {
	const route = {
		method: "POST",
		path: "/payment",
		handler_name: "json_bodies_37_oneof_schema_no_match_failure",
		request_schema: {
			oneOf: [
				{
					properties: { credit_card: { pattern: "^[0-9]{16}$", type: "string" } },
					required: ["credit_card"],
					type: "object",
				},
				{
					properties: { paypal_email: { format: "email", type: "string" } },
					required: ["paypal_email"],
					type: "object",
				},
			],
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_37_oneof_schema_no_match_failure: jsonBodies37OneofSchemaNoMatchFailure,
		},
	};
}
async function jsonBodiesEmptyArrayValidationFail(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodiesEmptyArrayValidationFail() {
	const route = {
		method: "POST",
		path: "/items/list-validated",
		handler_name: "json_bodies_empty_array_validation_fail",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, tags: { items: {}, minItems: 1, type: "array" } },
			required: ["name", "tags"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_empty_array_validation_fail: jsonBodiesEmptyArrayValidationFail,
		},
	};
}
async function jsonBodies38AnyofSchemaSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppJsonBodies38AnyofSchemaSuccess() {
	const route = {
		method: "POST",
		path: "/contact",
		handler_name: "json_bodies_38_anyof_schema_success",
		request_schema: {
			anyOf: [{ required: ["email"] }, { required: ["phone"] }],
			properties: { name: { type: "string" } },
			required: ["name"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_38_anyof_schema_success: jsonBodies38AnyofSchemaSuccess,
		},
	};
}
async function jsonBodiesEmptyJsonObject(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { description: null, name: null, price: null, tax: null };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesEmptyJsonObject() {
	const route = {
		method: "POST",
		path: "/items/optional-all",
		handler_name: "json_bodies_empty_json_object",
		request_schema: { additionalProperties: false, properties: {}, type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_empty_json_object: jsonBodiesEmptyJsonObject,
		},
	};
}
async function jsonBodiesStringPatternValidationFail(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodiesStringPatternValidationFail() {
	const route = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_string_pattern_validation_fail",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, sku: { pattern: "^[A-Z]{3}[0-9]{4}$", type: "string" } },
			required: ["name", "sku"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_string_pattern_validation_fail: jsonBodiesStringPatternValidationFail,
		},
	};
}
async function jsonBodies49DependenciesValidationFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodies49DependenciesValidationFailure() {
	const route = {
		method: "POST",
		path: "/billing",
		handler_name: "json_bodies_49_dependencies_validation_failure",
		request_schema: {
			dependencies: { credit_card: ["billing_address"] },
			properties: { billing_address: { type: "string" }, credit_card: { type: "string" }, name: { type: "string" } },
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_49_dependencies_validation_failure: jsonBodies49DependenciesValidationFailure,
		},
	};
}
async function jsonBodiesSimpleJsonObjectSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { description: "A very nice Item", name: "Foo", price: 35.4, tax: 3.2 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesSimpleJsonObjectSuccess() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_simple_json_object_success",
		request_schema: {
			additionalProperties: false,
			properties: {
				description: { type: "string" },
				name: { type: "string" },
				price: { type: "number" },
				tax: { type: "number" },
			},
			required: ["name", "description", "price", "tax"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_simple_json_object_success: jsonBodiesSimpleJsonObjectSuccess,
		},
	};
}
async function jsonBodiesRequiredFieldMissingValidationError(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodiesRequiredFieldMissingValidationError() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_required_field_missing_validation_error",
		request_schema: {
			additionalProperties: false,
			properties: { description: { type: "string" }, name: { type: "string" }, price: { type: "number" } },
			required: ["description", "price", "name"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_required_field_missing_validation_error: jsonBodiesRequiredFieldMissingValidationError,
		},
	};
}
async function jsonBodies35OneofSchemaSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppJsonBodies35OneofSchemaSuccess() {
	const route = {
		method: "POST",
		path: "/payment",
		handler_name: "json_bodies_35_oneof_schema_success",
		request_schema: {
			oneOf: [
				{
					properties: { credit_card: { pattern: "^[0-9]{16}$", type: "string" } },
					required: ["credit_card"],
					type: "object",
				},
				{
					properties: { paypal_email: { format: "email", type: "string" } },
					required: ["paypal_email"],
					type: "object",
				},
			],
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_35_oneof_schema_success: jsonBodies35OneofSchemaSuccess,
		},
	};
}
async function jsonBodiesEnumFieldInvalidValue(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodiesEnumFieldInvalidValue() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_enum_field_invalid_value",
		request_schema: {
			additionalProperties: false,
			properties: {
				category: { enum: ["electronics", "clothing", "books"], type: "string" },
				name: { type: "string" },
			},
			required: ["name", "category"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_enum_field_invalid_value: jsonBodiesEnumFieldInvalidValue,
		},
	};
}
async function jsonBodiesEnumFieldSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { category: "electronics", name: "Item" };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesEnumFieldSuccess() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_enum_field_success",
		request_schema: {
			additionalProperties: false,
			properties: { category: { type: "string" }, name: { type: "string" } },
			required: ["name", "category"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_enum_field_success: jsonBodiesEnumFieldSuccess,
		},
	};
}
async function jsonBodies33AllofSchemaComposition(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppJsonBodies33AllofSchemaComposition() {
	const route = {
		method: "POST",
		path: "/items",
		handler_name: "json_bodies_33_allof_schema_composition",
		request_schema: {
			allOf: [
				{ properties: { name: { type: "string" } }, required: ["name"], type: "object" },
				{ properties: { price: { minimum: 0, type: "number" } }, required: ["price"], type: "object" },
			],
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_33_allof_schema_composition: jsonBodies33AllofSchemaComposition,
		},
	};
}
async function jsonBodies45MinpropertiesValidationSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppJsonBodies45MinpropertiesValidationSuccess() {
	const route = {
		method: "POST",
		path: "/config",
		handler_name: "json_bodies_45_minproperties_validation_success",
		request_schema: { minProperties: 2, type: "object" },
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_45_minproperties_validation_success: jsonBodies45MinpropertiesValidationSuccess,
		},
	};
}
async function jsonBodiesBodyWithQueryParameters(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { item: { name: "Item", price: 42 }, limit: 10 };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesBodyWithQueryParameters() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_body_with_query_parameters",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, price: { type: "number" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: {
			properties: { limit: { source: "query", type: "integer" } },
			required: ["limit"],
			type: "object",
		},
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_body_with_query_parameters: jsonBodiesBodyWithQueryParameters,
		},
	};
}
async function jsonBodies42NotSchemaFailure(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodies42NotSchemaFailure() {
	const route = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_42_not_schema_failure",
		request_schema: {
			properties: { username: { not: { enum: ["admin", "root", "system"] }, type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_42_not_schema_failure: jsonBodies42NotSchemaFailure,
		},
	};
}
async function jsonBodies43ConstValidationSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppJsonBodies43ConstValidationSuccess() {
	const route = {
		method: "POST",
		path: "/api/v1/data",
		handler_name: "json_bodies_43_const_validation_success",
		request_schema: {
			properties: { data: { type: "string" }, version: { const: "1.0", type: "string" } },
			required: ["version", "data"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_43_const_validation_success: jsonBodies43ConstValidationSuccess,
		},
	};
}
async function jsonBodies32SchemaRefDefinitions(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppJsonBodies32SchemaRefDefinitions() {
	const route = {
		method: "POST",
		path: "/products",
		handler_name: "json_bodies_32_schema_ref_definitions",
		request_schema: {
			definitions: {
				Product: {
					properties: { name: { type: "string" }, price: { minimum: 0, type: "number" } },
					required: ["name", "price"],
					type: "object",
				},
			},
			properties: { product: { $ref: "#/definitions/Product" } },
			required: ["product"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_32_schema_ref_definitions: jsonBodies32SchemaRefDefinitions,
		},
	};
}
async function jsonBodies29NestedObjectValidationSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppJsonBodies29NestedObjectValidationSuccess() {
	const route = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_29_nested_object_validation_success",
		request_schema: {
			properties: {
				profile: {
					properties: { email: { format: "email", type: "string" }, name: { minLength: 1, type: "string" } },
					required: ["name", "email"],
					type: "object",
				},
			},
			required: ["profile"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_29_nested_object_validation_success: jsonBodies29NestedObjectValidationSuccess,
		},
	};
}
async function jsonBodies34AdditionalPropertiesFalse(requestJson) {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 422 };
	const result = {};
	if (body !== null && body !== void 0) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}
function createAppJsonBodies34AdditionalPropertiesFalse() {
	const route = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_34_additional_properties_false",
		request_schema: {
			additionalProperties: false,
			properties: { email: { type: "string" }, name: { type: "string" } },
			required: ["name"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_34_additional_properties_false: jsonBodies34AdditionalPropertiesFalse,
		},
	};
}
async function jsonBodiesNullValueForOptionalField(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = { description: null, name: "Item", price: 42, tax: null };
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesNullValueForOptionalField() {
	const route = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_null_value_for_optional_field",
		request_schema: {
			additionalProperties: false,
			properties: {
				description: { type: "null" },
				name: { type: "string" },
				price: { type: "number" },
				tax: { type: "null" },
			},
			required: ["name", "price", "description", "tax"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_null_value_for_optional_field: jsonBodiesNullValueForOptionalField,
		},
	};
}
async function jsonBodies31NullablePropertyNullValue(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}
function createAppJsonBodies31NullablePropertyNullValue() {
	const route = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_31_nullable_property_null_value",
		request_schema: {
			properties: { description: { type: ["string", "null"] }, name: { type: "string" } },
			required: ["name"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_31_nullable_property_null_value: jsonBodies31NullablePropertyNullValue,
		},
	};
}
async function jsonBodiesArrayOfObjectsSuccess(requestJson) {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response = { status: 200 };
	const responseBody = {
		images: [
			{ name: "Front", url: "https://example.com/img1.jpg" },
			{ name: "Back", url: "https://example.com/img2.jpg" },
		],
		name: "Product Bundle",
		tags: ["electronics", "gadget"],
	};
	response.body = responseBody;
	return JSON.stringify(response);
}
function createAppJsonBodiesArrayOfObjectsSuccess() {
	const route = {
		method: "POST",
		path: "/items/list",
		handler_name: "json_bodies_array_of_objects_success",
		request_schema: {
			additionalProperties: false,
			properties: {
				images: {
					items: {
						additionalProperties: false,
						properties: { name: { type: "string" }, url: { type: "string" } },
						required: ["url", "name"],
						type: "object",
					},
					type: "array",
				},
				name: { type: "string" },
				tags: { items: { type: "string" }, type: "array" },
			},
			required: ["name", "tags", "images"],
			type: "object",
		},
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			json_bodies_array_of_objects_success: jsonBodiesArrayOfObjectsSuccess,
		},
	};
}
async function sseHandlerNotifications(_requestJson) {
	const events = [
		SystemAlertMessageSchema.parse({
			level: "critical",
			message: "Database connection pool exhausted",
			source: "database-service",
			timestamp: "2024-01-15T10:30:00Z",
			type: "system_alert",
		}),
		NotificationBatchMessageSchema.parse([
			{ message: "example_message", timestamp: "2024-01-15T10:30:00Z", type: "example_type" },
			{ message: "example_message", timestamp: "2024-01-15T10:30:00Z", type: "example_type" },
		]),
		UserNotificationMessageSchema.parse({
			body: "You have received a new direct message",
			priority: "high",
			timestamp: "2024-01-15T10:30:00Z",
			title: "New message from John",
			type: "user_notification",
			userId: "user_12345",
		}),
		StatusUpdateMessageSchema.parse({
			message: "All systems operational",
			metadata: { region: "us-east-1", uptime: 99.99 },
			service: "payment-gateway",
			status: "operational",
			timestamp: "2024-01-15T10:30:00Z",
			type: "status_update",
		}),
	];
	async function* eventStream() {
		for (const payload of events) {
			yield `data: ${JSON.stringify(payload)}

`;
		}
	}
	return new StreamingResponse(eventStream(), {
		statusCode: 200,
		headers: {
			"content-type": "text/event-stream",
			"cache-control": "no-cache",
		},
	});
}
function createAppSseNotifications() {
	const route = {
		method: "GET",
		path: "/notifications",
		handler_name: "sseHandlerNotifications",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			sseHandlerNotifications,
		},
	};
}
const ChatChannelSchema = z.union([ChatMessageMessageSchema, UserJoinedMessageSchema, UserLeftMessageSchema]);
async function websocketHandlerChat(message) {
	const payload = ChatChannelSchema.parse(normalizeWebsocketPayload(message));
	const response = { ...payload, validated: true };
	return JSON.stringify(response);
}
function createAppWebsocketChat() {
	const route = {
		method: "GET",
		path: "/chat",
		handler_name: "websocketHandlerChat",
		request_schema: void 0,
		response_schema: void 0,
		parameter_schema: void 0,
		file_params: void 0,
		is_async: true,
	};
	return {
		routes: [route],
		handlers: {
			websocketHandlerChat,
		},
	};
}
export {
	ChatAckMessageSchema,
	ChatMessageMessageSchema,
	NotificationBatchMessageSchema,
	StatusUpdateMessageSchema,
	SystemAlertMessageSchema,
	UserJoinedMessageSchema,
	UserLeftMessageSchema,
	UserNotificationMessageSchema,
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
	createAppBackgroundBackgroundEventLogging,
	createAppBackgroundBackgroundEventLoggingSecondPayload,
	createAppBodyLimitsBodyOverLimitReturns413,
	createAppBodyLimitsBodyUnderLimitSucceeds,
	createAppCompressionCompressionGzipApplied,
	createAppCompressionCompressionPayloadBelowMinSizeIsNotCompressed,
	createAppContentTypes13JsonWithCharsetUtf16,
	createAppContentTypes14ContentTypeCaseInsensitive,
	createAppContentTypes15MultipartBoundaryRequired,
	createAppContentTypes16TextPlainNotAccepted,
	createAppContentTypes17VendorJsonAccepted,
	createAppContentTypes18ContentTypeWithMultipleParams,
	createAppContentTypes19MissingContentTypeDefaultJson,
	createAppContentTypes20ContentLengthMismatch,
	createAppContentTypes415UnsupportedMediaType,
	createAppContentTypesBinaryResponseApplicationOctetStream,
	createAppContentTypesContentNegotiationAcceptHeader,
	createAppContentTypesCsvResponseTextCsv,
	createAppContentTypesHtmlResponseTextHtml,
	createAppContentTypesJpegImageResponseImageJpeg,
	createAppContentTypesJsonResponseApplicationJson,
	createAppContentTypesJsonWithUtf8Charset,
	createAppContentTypesPdfResponseApplicationPdf,
	createAppContentTypesPlainTextResponseTextPlain,
	createAppContentTypesPngImageResponseImagePng,
	createAppContentTypesXmlResponseApplicationXml,
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
	createAppCors06CorsPreflightMethodNotAllowed,
	createAppCors07CorsPreflightHeaderNotAllowed,
	createAppCors08CorsMaxAge,
	createAppCors09CorsExposeHeaders,
	createAppCors10CorsOriginNull,
	createAppCorsCorsMultipleAllowedOrigins,
	createAppCorsCorsOriginCaseSensitivity,
	createAppCorsCorsPreflightForDeleteMethod,
	createAppCorsCorsPreflightForPutMethod,
	createAppCorsCorsPreflightRequest,
	createAppCorsCorsPrivateNetworkAccess,
	createAppCorsCorsRegexPatternMatchingForOrigins,
	createAppCorsCorsRequestBlocked,
	createAppCorsCorsSafelistedHeadersWithoutPreflight,
	createAppCorsCorsVaryHeaderForProperCaching,
	createAppCorsCorsWildcardOrigin,
	createAppCorsCorsWithCredentials,
	createAppCorsSimpleCorsRequest,
	createAppEdgeCases11Utf8QueryParameter,
	createAppEdgeCases12PercentEncodedSpecialChars,
	createAppEdgeCases13EmptyStringQueryParamPreserved,
	createAppEdgeCases14LargeIntegerBoundary,
	createAppEdgeCases15FloatPrecisionPreservation,
	createAppEdgeCases16NegativeZeroHandling,
	createAppEdgeCases17ExtremelyLongString,
	createAppEdgeCases18UnicodeNormalization,
	createAppEdgeCases19EmojiInStrings,
	createAppEdgeCases20NullByteInString,
	createAppEdgeCases21ScientificNotationNumber,
	createAppEdgeCases22LeadingZerosInteger,
	createAppEdgeCases23DeeplyNestedJsonLimit,
	createAppEdgeCases24ArrayWithHoles,
	createAppEdgeCasesDeeplyNestedStructure10Levels,
	createAppEdgeCasesEmptyAndNullValueHandling,
	createAppEdgeCasesFloatPrecisionAndRounding,
	createAppEdgeCasesLargeIntegerBoundaryValues,
	createAppEdgeCasesSpecialStringValuesAndEscaping,
	createAppEdgeCasesUnicodeAndEmojiHandling,
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
	createAppHttpMethodsDeleteRemoveResource,
	createAppHttpMethodsDeleteResourceNotFound,
	createAppHttpMethodsDeleteWithResponseBody,
	createAppHttpMethodsHeadGetMetadataWithoutBody,
	createAppHttpMethodsOptionsCorsPreflightRequest,
	createAppHttpMethodsPatchPartialUpdate,
	createAppHttpMethodsPatchUpdateMultipleFields,
	createAppHttpMethodsPutCompleteResourceReplacement,
	createAppHttpMethodsPutCreateResourceIfDoesnTExist,
	createAppHttpMethodsPutIdempotentOperation,
	createAppHttpMethodsPutMissingRequiredField,
	createAppHttpMethodsPutValidationError,
	createAppJsonBodies29NestedObjectValidationSuccess,
	createAppJsonBodies30NestedObjectMissingField,
	createAppJsonBodies31NullablePropertyNullValue,
	createAppJsonBodies32SchemaRefDefinitions,
	createAppJsonBodies33AllofSchemaComposition,
	createAppJsonBodies34AdditionalPropertiesFalse,
	createAppJsonBodies35OneofSchemaSuccess,
	createAppJsonBodies36OneofSchemaMultipleMatchFailure,
	createAppJsonBodies37OneofSchemaNoMatchFailure,
	createAppJsonBodies38AnyofSchemaSuccess,
	createAppJsonBodies39AnyofSchemaMultipleMatchSuccess,
	createAppJsonBodies40AnyofSchemaFailure,
	createAppJsonBodies41NotSchemaSuccess,
	createAppJsonBodies42NotSchemaFailure,
	createAppJsonBodies43ConstValidationSuccess,
	createAppJsonBodies44ConstValidationFailure,
	createAppJsonBodies45MinpropertiesValidationSuccess,
	createAppJsonBodies46MinpropertiesValidationFailure,
	createAppJsonBodies47MaxpropertiesValidationFailure,
	createAppJsonBodies48DependenciesValidationSuccess,
	createAppJsonBodies49DependenciesValidationFailure,
	createAppJsonBodies50DeepNesting4Levels,
	createAppJsonBodiesArrayOfObjectsSuccess,
	createAppJsonBodiesArrayOfPrimitiveValues,
	createAppJsonBodiesBodyWithQueryParameters,
	createAppJsonBodiesBooleanFieldSuccess,
	createAppJsonBodiesDateFieldSuccess,
	createAppJsonBodiesDatetimeFieldSuccess,
	createAppJsonBodiesDeeplyNestedObjects,
	createAppJsonBodiesEmptyArrayValidationFail,
	createAppJsonBodiesEmptyJsonObject,
	createAppJsonBodiesEnumFieldInvalidValue,
	createAppJsonBodiesEnumFieldSuccess,
	createAppJsonBodiesExtraFieldsIgnoredNoAdditionalproperties,
	createAppJsonBodiesFieldTypeValidationInvalidType,
	createAppJsonBodiesNestedObjectSuccess,
	createAppJsonBodiesNullValueForOptionalField,
	createAppJsonBodiesNumericGeValidationFail,
	createAppJsonBodiesNumericLeValidationSuccess,
	createAppJsonBodiesOptionalFieldsOmitted,
	createAppJsonBodiesPatchPartialUpdate,
	createAppJsonBodiesRequiredFieldMissingValidationError,
	createAppJsonBodiesSimpleJsonObjectSuccess,
	createAppJsonBodiesStringMaxLengthValidationFail,
	createAppJsonBodiesStringMinLengthValidationFail,
	createAppJsonBodiesStringPatternValidationFail,
	createAppJsonBodiesStringPatternValidationSuccess,
	createAppJsonBodiesUuidFieldInvalidFormat,
	createAppJsonBodiesUuidFieldSuccess,
	createAppLifecycleHooksHookExecutionOrder,
	createAppLifecycleHooksMultipleHooksAllPhases,
	createAppLifecycleHooksOnerrorErrorLogging,
	createAppLifecycleHooksOnrequestRequestLogging,
	createAppLifecycleHooksOnresponseResponseTiming,
	createAppLifecycleHooksOnresponseSecurityHeaders,
	createAppLifecycleHooksPrehandlerAuthenticationFailedShortCircuit,
	createAppLifecycleHooksPrehandlerAuthenticationSuccess,
	createAppLifecycleHooksPrehandlerAuthorizationCheck,
	createAppLifecycleHooksPrehandlerAuthorizationForbiddenShortCircuit,
	createAppLifecycleHooksPrevalidationRateLimitExceededShortCircuit,
	createAppLifecycleHooksPrevalidationRateLimiting,
	createAppMultipart17FileMagicNumberPngSuccess,
	createAppMultipart18FileMagicNumberJpegSuccess,
	createAppMultipart19FileMimeSpoofingPngAsJpeg,
	createAppMultipart20FileMimeSpoofingJpegAsPng,
	createAppMultipart21FilePdfMagicNumberSuccess,
	createAppMultipart22FileEmptyBuffer,
	createAppMultipartContentTypeValidationInvalidType,
	createAppMultipartEmptyFileUpload,
	createAppMultipartFileListUploadArrayOfFiles,
	createAppMultipartFileSizeValidationTooLarge,
	createAppMultipartFileUploadWithCustomHeaders,
	createAppMultipartFileUploadWithoutFilename,
	createAppMultipartFormDataWithoutFiles,
	createAppMultipartImageFileUpload,
	createAppMultipartMixedFilesAndFormData,
	createAppMultipartMultipleFileUploads,
	createAppMultipartMultipleValuesForSameFieldName,
	createAppMultipartOptionalFileUploadMissing,
	createAppMultipartOptionalFileUploadProvided,
	createAppMultipartPdfFileUpload,
	createAppMultipartRequiredFileUploadMissing,
	createAppMultipartSimpleFileUpload,
	createAppPathParams20UuidV3PathParamSuccess,
	createAppPathParams21UuidV5PathParamSuccess,
	createAppPathParams24DateFormatPathParamSuccess,
	createAppPathParams25DateFormatInvalidFailure,
	createAppPathParams27DatetimeFormatPathParamSuccess,
	createAppPathParams28DurationFormatPathParamSuccess,
	createAppPathParams29DecimalPathParamSuccess,
	createAppPathParams30StringMinlengthPathSuccess,
	createAppPathParams31StringMinlengthPathFailure,
	createAppPathParams32StringMaxlengthPathFailure,
	createAppPathParams33StringPatternPathSuccess,
	createAppPathParams34StringPatternPathFailure,
	createAppPathParams35NegativeIntegerPathParam,
	createAppPathParamsBooleanPathParameterNumeric1,
	createAppPathParamsBooleanPathParameterTrue,
	createAppPathParamsDatePathParameterSuccess,
	createAppPathParamsEnumPathParameterInvalidValue,
	createAppPathParamsEnumPathParameterSuccess,
	createAppPathParamsFloatPathParameterSuccess,
	createAppPathParamsIntegerPathParameterInvalidString,
	createAppPathParamsIntegerPathParameterSuccess,
	createAppPathParamsIntegerPathParameterWithCombinedLtAndGtConstraintsSuccess,
	createAppPathParamsIntegerPathParameterWithGeConstraintSuccess,
	createAppPathParamsIntegerPathParameterWithGtConstraintFailure,
	createAppPathParamsIntegerPathParameterWithGtConstraintSuccess,
	createAppPathParamsIntegerPathParameterWithLeConstraintSuccess,
	createAppPathParamsIntegerPathParameterWithLtConstraintSuccess,
	createAppPathParamsMultiplePathParametersSuccess,
	createAppPathParamsPathParameterTypeSyntaxInvalidUuid,
	createAppPathParamsPathParameterTypeSyntaxWithOverride,
	createAppPathParamsPathParameterWithTypeSyntaxInteger,
	createAppPathParamsPathParameterWithTypeSyntaxUuid,
	createAppPathParamsPathTypeParameterFilePath,
	createAppPathParamsStringPathParameterSuccess,
	createAppPathParamsStringPathParameterWithMaxLengthFailure,
	createAppPathParamsStringPathParameterWithMinLengthFailure,
	createAppPathParamsUuidPathParameterSuccess,
	createAppQueryParams42NegativeIntegerQueryParam,
	createAppQueryParams43ScientificNotationFloat,
	createAppQueryParams44StringMinlengthValidationSuccess,
	createAppQueryParams45StringMinlengthValidationFailure,
	createAppQueryParams46StringMaxlengthValidationFailure,
	createAppQueryParams47PatternValidationEmailSuccess,
	createAppQueryParams48PatternValidationEmailFailure,
	createAppQueryParams49IntegerGtConstraintSuccess,
	createAppQueryParams50IntegerGtConstraintFailure,
	createAppQueryParams51IntegerGeConstraintBoundary,
	createAppQueryParams52IntegerLeConstraintBoundary,
	createAppQueryParams53IntegerLeConstraintFailure,
	createAppQueryParams54ArrayMinitemsConstraintSuccess,
	createAppQueryParams55ArrayMinitemsConstraintFailure,
	createAppQueryParams56ArrayMaxitemsConstraintFailure,
	createAppQueryParams57BooleanEmptyStringCoercion,
	createAppQueryParams58FormatEmailSuccess,
	createAppQueryParams59FormatEmailFailure,
	createAppQueryParams60FormatIpv4Success,
	createAppQueryParams61FormatIpv4Failure,
	createAppQueryParams62FormatIpv6Success,
	createAppQueryParams63FormatUriSuccess,
	createAppQueryParams64FormatUriFailure,
	createAppQueryParams65FormatHostnameSuccess,
	createAppQueryParams66MultipleofConstraintSuccess,
	createAppQueryParams67MultipleofConstraintFailure,
	createAppQueryParams68ArrayUniqueitemsSuccess,
	createAppQueryParams69ArrayUniqueitemsFailure,
	createAppQueryParams70ArraySeparatorPipe,
	createAppQueryParams71ArraySeparatorSemicolon,
	createAppQueryParams72ArraySeparatorSpace,
	createAppQueryParamsArrayQueryParameterEmptyArray,
	createAppQueryParamsArrayQueryParameterSingleValue,
	createAppQueryParamsBooleanQueryParameterNumeric1,
	createAppQueryParamsBooleanQueryParameterTrue,
	createAppQueryParamsDateQueryParameterSuccess,
	createAppQueryParamsDatetimeQueryParameterSuccess,
	createAppQueryParamsEnumQueryParameterInvalidValue,
	createAppQueryParamsEnumQueryParameterSuccess,
	createAppQueryParamsFloatQueryParamWithGeConstraintSuccess,
	createAppQueryParamsIntegerQueryParamWithGeConstraintBoundary,
	createAppQueryParamsIntegerQueryParamWithGtConstraintValid,
	createAppQueryParamsIntegerQueryParamWithLeConstraintBoundary,
	createAppQueryParamsIntegerQueryParamWithLtConstraintValid,
	createAppQueryParamsIntegerWithDefaultValueNotProvided,
	createAppQueryParamsIntegerWithDefaultValueOverride,
	createAppQueryParamsListOfIntegersMultipleValues,
	createAppQueryParamsListOfStringsMultipleValues,
	createAppQueryParamsListQueryParameterRequiredButMissing,
	createAppQueryParamsListWithDefaultEmptyArrayNoValuesProvided,
	createAppQueryParamsMultipleQueryParametersWithDifferentTypes,
	createAppQueryParamsOptionalIntegerQueryParameterMissing,
	createAppQueryParamsOptionalQueryParameterWithDefaultValue,
	createAppQueryParamsOptionalStringQueryParameterMissing,
	createAppQueryParamsOptionalStringQueryParameterProvided,
	createAppQueryParamsQueryParameterWithSpecialCharactersUrlEncoding,
	createAppQueryParamsQueryParameterWithUrlEncodedSpace,
	createAppQueryParamsQueryParameterWithUrlEncodedSpecialCharacters,
	createAppQueryParamsRequiredIntegerQueryParameterFloatValue,
	createAppQueryParamsRequiredIntegerQueryParameterInvalidType,
	createAppQueryParamsRequiredIntegerQueryParameterMissing,
	createAppQueryParamsRequiredIntegerQueryParameterSuccess,
	createAppQueryParamsRequiredStringQueryParameterMissing,
	createAppQueryParamsRequiredStringQueryParameterSuccess,
	createAppQueryParamsStringQueryParamWithMaxLengthConstraintFail,
	createAppQueryParamsStringQueryParamWithMinLengthConstraintFail,
	createAppQueryParamsStringQueryParamWithRegexPatternFail,
	createAppQueryParamsStringValidationWithRegexFailure,
	createAppQueryParamsStringValidationWithRegexSuccess,
	createAppQueryParamsUuidQueryParameterInvalidFormat,
	createAppQueryParamsUuidQueryParameterSuccess,
	createAppRateLimitRateLimitBelowThresholdSucceeds,
	createAppRateLimitRateLimitExceededReturns429,
	createAppRequestIdRequestIdHeaderIsPreserved,
	createAppRequestIdRequestIdIsGeneratedWhenNotProvided,
	createAppRequestIdRequestIdMiddlewareCanBeDisabled,
	createAppRequestTimeoutRequestCompletesBeforeTimeout,
	createAppRequestTimeoutRequestExceedsTimeout,
	createAppSseNotifications,
	createAppStaticFilesStaticFileServerReturnsTextFile,
	createAppStaticFilesStaticServerReturnsIndexHtmlForDirectory,
	createAppStatusCodes19413PayloadTooLarge,
	createAppStatusCodes200OkSuccess,
	createAppStatusCodes201CreatedResourceCreated,
	createAppStatusCodes202AcceptedRequestAcceptedForProcessing,
	createAppStatusCodes20414UriTooLong,
	createAppStatusCodes204NoContentSuccessWithNoBody,
	createAppStatusCodes206PartialContent,
	createAppStatusCodes21431RequestHeaderFieldsTooLarge,
	createAppStatusCodes22501NotImplemented,
	createAppStatusCodes23503ServiceUnavailable,
	createAppStatusCodes301MovedPermanentlyPermanentRedirect,
	createAppStatusCodes302FoundTemporaryRedirect,
	createAppStatusCodes304NotModifiedCachedContentValid,
	createAppStatusCodes307TemporaryRedirectMethodPreserved,
	createAppStatusCodes400BadRequestInvalidRequest,
	createAppStatusCodes401UnauthorizedMissingAuthentication,
	createAppStatusCodes403ForbiddenInsufficientPermissions,
	createAppStatusCodes404NotFoundResourceNotFound,
	createAppStatusCodes408RequestTimeout,
	createAppStatusCodes422UnprocessableEntityValidationError,
	createAppStatusCodes429TooManyRequests,
	createAppStatusCodes500InternalServerErrorServerError,
	createAppStatusCodes503ServiceUnavailableServerOverload,
	createAppStreamingBinaryLogDownload,
	createAppStreamingChunkedCsvExport,
	createAppStreamingStreamJsonLines,
	createAppUrlEncoded13ArrayFieldSuccess,
	createAppUrlEncoded14NestedObjectBracketNotation,
	createAppUrlEncoded15SpecialCharactersFieldNames,
	createAppUrlEncoded16MinlengthValidationFailure,
	createAppUrlEncoded17PatternValidationFailure,
	createAppUrlEncoded18IntegerMinimumValidationFailure,
	createAppUrlEncoded19ArrayMinitemsValidationFailure,
	createAppUrlEncoded20FormatEmailValidationFailure,
	createAppUrlEncoded21IntegerTypeCoercionFailure,
	createAppUrlEncoded22AdditionalPropertiesStrictFailure,
	createAppUrlEncodedBooleanFieldConversion,
	createAppUrlEncodedEmptyStringValue,
	createAppUrlEncodedMultipleValuesForSameField,
	createAppUrlEncodedNumericFieldTypeConversion,
	createAppUrlEncodedOauth2PasswordGrantFlow,
	createAppUrlEncodedOptionalFieldMissingSuccess,
	createAppUrlEncodedPatternValidationFail,
	createAppUrlEncodedRequiredFieldMissingValidationError,
	createAppUrlEncodedSimpleFormSubmissionSuccess,
	createAppUrlEncodedSpecialCharactersEncoding,
	createAppUrlEncodedStringMaxLengthValidationFail,
	createAppUrlEncodedStringMinLengthValidationFail,
	createAppValidationErrors09MultipleValidationErrors,
	createAppValidationErrors10NestedErrorPath,
	createAppValidationErrorsArrayItemValidationError,
	createAppValidationErrorsArrayMaxItemsConstraintViolation,
	createAppValidationErrorsArrayMinItemsConstraintViolation,
	createAppValidationErrorsBodyFieldTypeErrorStringForFloat,
	createAppValidationErrorsHeaderValidationError,
	createAppValidationErrorsInvalidBooleanValue,
	createAppValidationErrorsInvalidDatetimeFormat,
	createAppValidationErrorsInvalidEnumValue,
	createAppValidationErrorsInvalidUuidFormat,
	createAppValidationErrorsMalformedJsonBody,
	createAppValidationErrorsMissingRequiredBodyField,
	createAppValidationErrorsMissingRequiredQueryParameter,
	createAppValidationErrorsMultipleValidationErrors,
	createAppValidationErrorsNestedObjectValidationError,
	createAppValidationErrorsNumericConstraintViolationGtGreaterThan,
	createAppValidationErrorsNumericConstraintViolationLeLessThanOrEqual,
	createAppValidationErrorsQueryParamTypeErrorStringProvidedForInt,
	createAppValidationErrorsStringMaxLengthConstraintViolation,
	createAppValidationErrorsStringMinLengthConstraintViolation,
	createAppValidationErrorsStringRegexPatternMismatch,
	createAppWebsocketChat,
};
