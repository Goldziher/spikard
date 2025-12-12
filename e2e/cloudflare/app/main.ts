/**
 * Generated E2E test application with per-fixture app factories.
 * @generated
 */

import { Buffer } from "node:buffer";
import { z } from "zod";
import type { RouteMetadata, ServerConfig, SpikardApp } from "../../packages/wasm/src/index.ts";
import { Spikard, StreamingResponse } from "../../packages/wasm/src/index.ts";

type HandlerResponse = {
	status: number;
	headers?: Record<string, string>;
	body?: unknown;
};
type HookRequest = {
	body?: unknown;
	headers?: Record<string, string>;
	params?: Record<string, unknown>;
	[key: string]: unknown;
};
type HookResponse = {
	statusCode?: number;
	body?: unknown;
	headers?: Record<string, string>;
	[key: string]: unknown;
};
type HookResult = HookRequest | HookResponse;

function normalizeWebsocketPayload(message: unknown): unknown {
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
		const view = message as ArrayBufferView;
		const buffer = Buffer.from(view.buffer, view.byteOffset, view.byteLength);
		return JSON.parse(buffer.toString("utf-8"));
	}
	return message;
}

const UserJoinedMessageSchema = z.object({
	timestamp: z.string(),
	type: z.literal("userJoined"),
	user: z.string(),
});

type UserJoinedMessage = {
	timestamp: string;
	type: string;
	user: string;
};

const SystemAlertMessageSchema = z.object({
	level: z.union([z.literal("info"), z.literal("warning"), z.literal("error"), z.literal("critical")]),
	message: z.string(),
	source: z.string().optional(),
	timestamp: z.string(),
	type: z.literal("system_alert"),
});

type SystemAlertMessage = {
	level: string;
	message: string;
	source: string | undefined;
	timestamp: string;
	type: string;
};

const ChatAckMessageSchema = z.object({
	messageId: z.string(),
	status: z.union([z.literal("queued"), z.literal("delivered"), z.literal("rejected")]),
	timestamp: z.string(),
	type: z.literal("chatAck"),
});

type ChatAckMessage = {
	messageId: string;
	status: string;
	timestamp: string;
	type: string;
};

const StatusUpdateMessageSchema = z.object({
	message: z.string().optional(),
	metadata: z.record(z.string(), z.unknown()).optional(),
	service: z.string(),
	status: z.union([z.literal("operational"), z.literal("degraded"), z.literal("outage"), z.literal("maintenance")]),
	timestamp: z.string(),
	type: z.literal("status_update"),
});

type StatusUpdateMessage = {
	message: string | undefined;
	metadata: Record<string, unknown> | undefined;
	service: string;
	status: string;
	timestamp: string;
	type: string;
};

const NotificationBatchMessageSchema = z.array(
	z.object({
		message: z.string(),
		timestamp: z.string(),
		type: z.string(),
	}),
);

type NotificationBatchMessage = {
	message: string;
	timestamp: string;
	type: string;
}[];

const ChatMessageMessageSchema = z.object({
	text: z.string(),
	timestamp: z.string(),
	type: z.literal("message"),
	user: z.string(),
});

type ChatMessageMessage = {
	text: string;
	timestamp: string;
	type: string;
	user: string;
};

const UserLeftMessageSchema = z.object({
	timestamp: z.string(),
	type: z.literal("userLeft"),
	user: z.string(),
});

type UserLeftMessage = {
	timestamp: string;
	type: string;
	user: string;
};

const UserNotificationMessageSchema = z.object({
	body: z.string(),
	priority: z.union([z.literal("low"), z.literal("normal"), z.literal("high"), z.literal("urgent")]).optional(),
	timestamp: z.string(),
	title: z.string(),
	type: z.literal("user_notification"),
	userId: z.string(),
});

type UserNotificationMessage = {
	body: string;
	priority: string | undefined;
	timestamp: string;
	title: string;
	type: string;
	userId: string;
};

const BACKGROUND_STATE: Record<string, unknown[]> = {};

// Cleanup state tracking for DI fixtures
const CLEANUP_STATE: Record<string, string[]> = {};

/**
 * Handler for OPTIONS /api/data
 */
async function cors07CorsPreflightHeaderNotAllowed(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 403 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppCors07CorsPreflightHeaderNotAllowed(): SpikardApp {
	const route: RouteMetadata = {
		method: "OPTIONS",
		path: "/api/data",
		handler_name: "cors_07_cors_preflight_header_not_allowed",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				"Access-Control-Request-Headers": { source: "header", type: "string" },
				"Access-Control-Request-Method": { source: "header", type: "string" },
				Origin: { source: "header", type: "string" },
			},
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_07_cors_preflight_header_not_allowed: cors07CorsPreflightHeaderNotAllowed,
		},
	};
}

/**
 * Handler for GET /api/cached-resource
 */
async function corsCorsVaryHeaderForProperCaching(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {
		"access-control-allow-origin": "https://app.example.com",
		"cache-control": "public, max-age=3600",
		vary: "Origin",
	};
	const responseBody = { data: "cacheable resource" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCorsCorsVaryHeaderForProperCaching(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/cached-resource",
		handler_name: "cors_cors_vary_header_for_proper_caching",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_cors_vary_header_for_proper_caching: corsCorsVaryHeaderForProperCaching,
		},
	};
}

/**
 * Handler for OPTIONS /api/resource/123
 */
async function corsCorsPreflightForPutMethod(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 204 };
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

export function createAppCorsCorsPreflightForPutMethod(): SpikardApp {
	const route: RouteMetadata = {
		method: "OPTIONS",
		path: "/api/resource/123",
		handler_name: "cors_cors_preflight_for_put_method",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_cors_preflight_for_put_method: corsCorsPreflightForPutMethod,
		},
	};
}

/**
 * Handler for OPTIONS /api/resource/456
 */
async function corsCorsPreflightForDeleteMethod(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 204 };
	response.headers = {
		"access-control-allow-methods": "GET, POST, PUT, PATCH, DELETE",
		"access-control-allow-origin": "https://app.example.com",
		"access-control-max-age": "3600",
		vary: "Origin",
	};
	response.body = null;
	return JSON.stringify(response);
}

export function createAppCorsCorsPreflightForDeleteMethod(): SpikardApp {
	const route: RouteMetadata = {
		method: "OPTIONS",
		path: "/api/resource/456",
		handler_name: "cors_cors_preflight_for_delete_method",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_cors_preflight_for_delete_method: corsCorsPreflightForDeleteMethod,
		},
	};
}

/**
 * Handler for GET /api/data
 */
async function corsCorsMultipleAllowedOrigins(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "access-control-allow-origin": "https://admin.example.com", vary: "Origin" };
	const responseBody = { data: "resource data" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCorsCorsMultipleAllowedOrigins(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "cors_cors_multiple_allowed_origins",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_cors_multiple_allowed_origins: corsCorsMultipleAllowedOrigins,
		},
	};
}

/**
 * Handler for OPTIONS /items/
 */
async function corsCorsPreflightRequest(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {
		"access-control-allow-headers": "Content-Type, X-Custom-Header",
		"access-control-allow-methods": "GET, POST, PUT, DELETE, OPTIONS",
		"access-control-allow-origin": "https://example.com",
		"access-control-max-age": "600",
	};
	const result: Record<string, unknown> = {};
	response.body = result;
	return JSON.stringify(response);
}

export function createAppCorsCorsPreflightRequest(): SpikardApp {
	const route: RouteMetadata = {
		method: "OPTIONS",
		path: "/items/",
		handler_name: "cors_cors_preflight_request",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_cors_preflight_request: corsCorsPreflightRequest,
		},
	};
}

/**
 * Handler for GET /api/user/profile
 */
async function corsCorsWithCredentials(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {
		"access-control-allow-credentials": "true",
		"access-control-allow-origin": "https://app.example.com",
		vary: "Origin",
	};
	const responseBody = { username: "john" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCorsCorsWithCredentials(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/user/profile",
		handler_name: "cors_cors_with_credentials",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_cors_with_credentials: corsCorsWithCredentials,
		},
	};
}

/**
 * Handler for GET /api/data
 */
async function corsCorsRegexPatternMatchingForOrigins(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "access-control-allow-origin": "https://subdomain.example.com", vary: "Origin" };
	const responseBody = { data: "resource data" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCorsCorsRegexPatternMatchingForOrigins(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "cors_cors_regex_pattern_matching_for_origins",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_cors_regex_pattern_matching_for_origins: corsCorsRegexPatternMatchingForOrigins,
		},
	};
}

/**
 * Handler for OPTIONS /api/data
 */
async function cors08CorsMaxAge(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 204 };
	response.headers = {
		"access-control-allow-headers": "Content-Type",
		"access-control-allow-methods": "POST",
		"access-control-allow-origin": "https://example.com",
		"access-control-max-age": "3600",
	};
	response.body = null;
	return JSON.stringify(response);
}

export function createAppCors08CorsMaxAge(): SpikardApp {
	const route: RouteMetadata = {
		method: "OPTIONS",
		path: "/api/data",
		handler_name: "cors_08_cors_max_age",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				"Access-Control-Request-Headers": { source: "header", type: "string" },
				"Access-Control-Request-Method": { source: "header", type: "string" },
				Origin: { source: "header", type: "string" },
			},
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_08_cors_max_age: cors08CorsMaxAge,
		},
	};
}

/**
 * Handler for GET /api/data
 */
async function cors10CorsOriginNull(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 403 };
	const responseBody = { error: "Origin 'null' is not allowed" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCors10CorsOriginNull(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "cors_10_cors_origin_null",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { Origin: { source: "header", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_10_cors_origin_null: cors10CorsOriginNull,
		},
	};
}

/**
 * Handler for GET /public/data
 */
async function corsCorsWildcardOrigin(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "access-control-allow-origin": "*" };
	const responseBody = { data: "public" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCorsCorsWildcardOrigin(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/public/data",
		handler_name: "cors_cors_wildcard_origin",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_cors_wildcard_origin: corsCorsWildcardOrigin,
		},
	};
}

/**
 * Handler for POST /api/form
 */
async function corsCorsSafelistedHeadersWithoutPreflight(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "access-control-allow-origin": "https://app.example.com", vary: "Origin" };
	const responseBody = { message: "Success" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCorsCorsSafelistedHeadersWithoutPreflight(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/api/form",
		handler_name: "cors_cors_safelisted_headers_without_preflight",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_cors_safelisted_headers_without_preflight: corsCorsSafelistedHeadersWithoutPreflight,
		},
	};
}

/**
 * Handler for OPTIONS /api/local-resource
 */
async function corsCorsPrivateNetworkAccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 204 };
	response.headers = {
		"access-control-allow-methods": "GET, POST",
		"access-control-allow-origin": "https://public.example.com",
		"access-control-allow-private-network": "true",
		vary: "Origin",
	};
	response.body = null;
	return JSON.stringify(response);
}

export function createAppCorsCorsPrivateNetworkAccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "OPTIONS",
		path: "/api/local-resource",
		handler_name: "cors_cors_private_network_access",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_cors_private_network_access: corsCorsPrivateNetworkAccess,
		},
	};
}

/**
 * Handler for GET /api/data
 */
async function corsCorsOriginCaseSensitivity(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { vary: "Origin" };
	const result: Record<string, unknown> = {};
	response.body = result;
	return JSON.stringify(response);
}

export function createAppCorsCorsOriginCaseSensitivity(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "cors_cors_origin_case_sensitivity",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_cors_origin_case_sensitivity: corsCorsOriginCaseSensitivity,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function corsCorsRequestBlocked(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 403 };
	const responseBody = { detail: "CORS request from origin 'https://malicious-site.com' not allowed" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCorsCorsRequestBlocked(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "cors_cors_request_blocked",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { Origin: { source: "header", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_cors_request_blocked: corsCorsRequestBlocked,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function corsSimpleCorsRequest(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "access-control-allow-origin": "https://example.com", vary: "Origin" };
	const responseBody = { items: [] };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCorsSimpleCorsRequest(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "cors_simple_cors_request",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_simple_cors_request: corsSimpleCorsRequest,
		},
	};
}

/**
 * Handler for GET /api/data
 */
async function cors09CorsExposeHeaders(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {
		"access-control-allow-origin": "https://example.com",
		"access-control-expose-headers": "X-Total-Count, X-Request-Id",
		"x-request-id": "abc123",
		"x-total-count": "42",
	};
	const result: Record<string, unknown> = {};
	const origin = params.Origin;
	if (origin !== null && origin !== undefined) {
		result.Origin = origin;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppCors09CorsExposeHeaders(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "cors_09_cors_expose_headers",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { Origin: { source: "header", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_09_cors_expose_headers: cors09CorsExposeHeaders,
		},
	};
}

/**
 * Handler for OPTIONS /api/data
 */
async function cors06CorsPreflightMethodNotAllowed(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 403 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppCors06CorsPreflightMethodNotAllowed(): SpikardApp {
	const route: RouteMetadata = {
		method: "OPTIONS",
		path: "/api/data",
		handler_name: "cors_06_cors_preflight_method_not_allowed",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				"Access-Control-Request-Headers": { source: "header", type: "string" },
				"Access-Control-Request-Method": { source: "header", type: "string" },
				Origin: { source: "header", type: "string" },
			},
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_06_cors_preflight_method_not_allowed: cors06CorsPreflightMethodNotAllowed,
		},
	};
}

/**
 * Handler for GET /data
 */
async function cookies25CookieSamesiteLax(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const result: Record<string, unknown> = {};
	const tracking = params.tracking;
	if (tracking !== null && tracking !== undefined) {
		result.tracking = tracking;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppCookies25CookieSamesiteLax(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/data",
		handler_name: "cookies_25_cookie_samesite_lax",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { tracking: { samesite: "Lax", source: "cookie", type: "string" } },
			required: ["tracking"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_25_cookie_samesite_lax: cookies25CookieSamesiteLax,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function cookiesOptionalCookieParameterSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { ads_id: "abc123" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesOptionalCookieParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "cookies_optional_cookie_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { ads_id: { source: "cookie", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_optional_cookie_parameter_success: cookiesOptionalCookieParameterSuccess,
		},
	};
}

/**
 * Handler for GET /cookies/pattern
 */
async function cookiesCookieRegexPatternValidationFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const trackingId = params.tracking_id;
	if (trackingId !== null && trackingId !== undefined) {
		result.tracking_id = trackingId;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppCookiesCookieRegexPatternValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/cookies/pattern",
		handler_name: "cookies_cookie_regex_pattern_validation_fail",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { tracking_id: { pattern: "^[A-Z0-9]{8}$", source: "cookie", type: "string" } },
			required: ["tracking_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_regex_pattern_validation_fail: cookiesCookieRegexPatternValidationFail,
		},
	};
}

/**
 * Handler for POST /cookies/session
 */
async function cookiesResponseSessionCookieNoMaxAge(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Session cookie set" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseSessionCookieNoMaxAge(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/session",
		handler_name: "cookies_response_session_cookie_no_max_age",
		request_schema: {
			additionalProperties: false,
			properties: { value: { type: "string" } },
			required: ["value"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_session_cookie_no_max_age: cookiesResponseSessionCookieNoMaxAge,
		},
	};
}

/**
 * Handler for GET /secure
 */
async function cookies27CookieHttponlyFlag(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const result: Record<string, unknown> = {};
	const session = params.session;
	if (session !== null && session !== undefined) {
		result.session = session;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppCookies27CookieHttponlyFlag(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/secure",
		handler_name: "cookies_27_cookie_httponly_flag",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { session: { httponly: true, source: "cookie", type: "string" } },
			required: ["session"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_27_cookie_httponly_flag: cookies27CookieHttponlyFlag,
		},
	};
}

/**
 * Handler for GET /cookie/set
 */
async function cookiesResponseCookieWithAttributes(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Cookie set" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseCookieWithAttributes(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/cookie/set",
		handler_name: "cookies_response_cookie_with_attributes",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_attributes: cookiesResponseCookieWithAttributes,
		},
	};
}

/**
 * Handler for GET /secure
 */
async function cookies24CookieSamesiteStrict(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const result: Record<string, unknown> = {};
	const sessionId = params.session_id;
	if (sessionId !== null && sessionId !== undefined) {
		result.session_id = sessionId;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppCookies24CookieSamesiteStrict(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/secure",
		handler_name: "cookies_24_cookie_samesite_strict",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { session_id: { samesite: "Strict", source: "cookie", type: "string" } },
			required: ["session_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_24_cookie_samesite_strict: cookies24CookieSamesiteStrict,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function cookiesApikeyCookieAuthenticationSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { username: "secret" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesApikeyCookieAuthenticationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "cookies_apikey_cookie_authentication_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { key: { source: "cookie", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_apikey_cookie_authentication_success: cookiesApikeyCookieAuthenticationSuccess,
		},
	};
}

/**
 * Handler for GET /cookies/min-length
 */
async function cookiesCookieValidationMinLengthConstraintSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { token: "abc" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesCookieValidationMinLengthConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/cookies/min-length",
		handler_name: "cookies_cookie_validation_min_length_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { token: { source: "cookie", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_validation_min_length_constraint_success: cookiesCookieValidationMinLengthConstraintSuccess,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function cookiesCookieValidationMinLengthFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const trackingId = params.tracking_id;
	if (trackingId !== null && trackingId !== undefined) {
		result.tracking_id = trackingId;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppCookiesCookieValidationMinLengthFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "cookies_cookie_validation_min_length_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { tracking_id: { minLength: 3, source: "cookie", type: "string" } },
			required: ["tracking_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_validation_min_length_failure: cookiesCookieValidationMinLengthFailure,
		},
	};
}

/**
 * Handler for GET /cookies/validated
 */
async function cookiesCookieValidationMaxLengthConstraintFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const sessionId = params.session_id;
	if (sessionId !== null && sessionId !== undefined) {
		result.session_id = sessionId;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppCookiesCookieValidationMaxLengthConstraintFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/cookies/validated",
		handler_name: "cookies_cookie_validation_max_length_constraint_fail",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { session_id: { maxLength: 20, source: "cookie", type: "string" } },
			required: ["session_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_validation_max_length_constraint_fail: cookiesCookieValidationMaxLengthConstraintFail,
		},
	};
}

/**
 * Handler for GET /items/cookies
 */
async function cookiesRequiredCookieMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const fatebookTracker = params.fatebook_tracker;
	const sessionId = params.session_id;
	if (fatebookTracker !== null && fatebookTracker !== undefined) {
		result.fatebook_tracker = fatebookTracker;
	}
	if (sessionId !== null && sessionId !== undefined) {
		result.session_id = sessionId;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppCookiesRequiredCookieMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/cookies",
		handler_name: "cookies_required_cookie_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				fatebook_tracker: { source: "cookie", type: "string" },
				session_id: { source: "cookie", type: "string" },
			},
			required: ["session_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_required_cookie_missing: cookiesRequiredCookieMissing,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function cookiesOptionalCookieParameterMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { ads_id: null };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesOptionalCookieParameterMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "cookies_optional_cookie_parameter_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { ads_id: { source: "cookie", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_optional_cookie_parameter_missing: cookiesOptionalCookieParameterMissing,
		},
	};
}

/**
 * Handler for GET /users/me/auth
 */
async function cookiesApikeyCookieAuthenticationMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const key = params.key;
	if (key !== null && key !== undefined) {
		result.key = key;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppCookiesApikeyCookieAuthenticationMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me/auth",
		handler_name: "cookies_apikey_cookie_authentication_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { key: { source: "cookie", type: "string" } }, required: ["key"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_apikey_cookie_authentication_missing: cookiesApikeyCookieAuthenticationMissing,
		},
	};
}

/**
 * Handler for POST /cookies/multiple
 */
async function cookiesResponseMultipleCookies(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Multiple cookies set" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseMultipleCookies(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/multiple",
		handler_name: "cookies_response_multiple_cookies",
		request_schema: {
			additionalProperties: false,
			properties: { session: { type: "string" }, user: { type: "string" } },
			required: ["user", "session"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_multiple_cookies: cookiesResponseMultipleCookies,
		},
	};
}

/**
 * Handler for POST /cookies/samesite-lax
 */
async function cookiesResponseCookieWithSamesiteLax(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Cookie set with SameSite=Lax" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseCookieWithSamesiteLax(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/samesite-lax",
		handler_name: "cookies_response_cookie_with_samesite_lax",
		request_schema: {
			additionalProperties: false,
			properties: { value: { type: "string" } },
			required: ["value"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_samesite_lax: cookiesResponseCookieWithSamesiteLax,
		},
	};
}

/**
 * Handler for POST /cookies/delete
 */
async function cookiesResponseDeleteCookie(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Cookie deleted" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseDeleteCookie(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/delete",
		handler_name: "cookies_response_delete_cookie",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { session: { source: "cookie", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_delete_cookie: cookiesResponseDeleteCookie,
		},
	};
}

/**
 * Handler for POST /cookies/set-with-path
 */
async function cookiesResponseCookieWithPathAttribute(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Cookie set with path" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseCookieWithPathAttribute(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/set-with-path",
		handler_name: "cookies_response_cookie_with_path_attribute",
		request_schema: {
			additionalProperties: false,
			properties: { value: { type: "string" } },
			required: ["value"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_path_attribute: cookiesResponseCookieWithPathAttribute,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function cookiesOptionalApikeyCookieMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { msg: "Create an account first" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesOptionalApikeyCookieMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "cookies_optional_apikey_cookie_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { key: { source: "cookie", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_optional_apikey_cookie_missing: cookiesOptionalApikeyCookieMissing,
		},
	};
}

/**
 * Handler for POST /cookies/samesite-strict
 */
async function cookiesResponseCookieWithSamesiteStrict(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Cookie set with SameSite=Strict" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseCookieWithSamesiteStrict(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/samesite-strict",
		handler_name: "cookies_response_cookie_with_samesite_strict",
		request_schema: {
			additionalProperties: false,
			properties: { value: { type: "string" } },
			required: ["value"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_samesite_strict: cookiesResponseCookieWithSamesiteStrict,
		},
	};
}

/**
 * Handler for POST /cookies/samesite-none
 */
async function cookiesResponseCookieWithSamesiteNone(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Cookie set with SameSite=None" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseCookieWithSamesiteNone(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/samesite-none",
		handler_name: "cookies_response_cookie_with_samesite_none",
		request_schema: {
			additionalProperties: false,
			properties: { value: { type: "string" } },
			required: ["value"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_samesite_none: cookiesResponseCookieWithSamesiteNone,
		},
	};
}

/**
 * Handler for GET /cookies/pattern
 */
async function cookiesCookieRegexPatternValidationSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { tracking_id: "ABC12345" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesCookieRegexPatternValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/cookies/pattern",
		handler_name: "cookies_cookie_regex_pattern_validation_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { tracking_id: { source: "cookie", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_regex_pattern_validation_success: cookiesCookieRegexPatternValidationSuccess,
		},
	};
}

/**
 * Handler for POST /cookie/
 */
async function cookiesResponseSetCookieBasic(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Come to the dark side, we have cookies" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseSetCookieBasic(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookie/",
		handler_name: "cookies_response_set_cookie_basic",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_set_cookie_basic: cookiesResponseSetCookieBasic,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function cookiesMultipleCookiesSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { fatebook_tracker: "tracker456", googall_tracker: "ga789", session_id: "session123" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesMultipleCookiesSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "cookies_multiple_cookies_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				fatebook_tracker: { source: "cookie", type: "string" },
				googall_tracker: { source: "cookie", type: "string" },
				session_id: { source: "cookie", type: "string" },
			},
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_multiple_cookies_success: cookiesMultipleCookiesSuccess,
		},
	};
}

/**
 * Handler for GET /secure
 */
async function cookies26CookieSecureFlag(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const result: Record<string, unknown> = {};
	const authToken = params.auth_token;
	if (authToken !== null && authToken !== undefined) {
		result.auth_token = authToken;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppCookies26CookieSecureFlag(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/secure",
		handler_name: "cookies_26_cookie_secure_flag",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { auth_token: { secure: true, source: "cookie", type: "string" } },
			required: ["auth_token"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_26_cookie_secure_flag: cookies26CookieSecureFlag,
		},
	};
}

/**
 * Handler for POST /cookies/set-with-domain
 */
async function cookiesResponseCookieWithDomainAttribute(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Cookie set with domain" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseCookieWithDomainAttribute(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/set-with-domain",
		handler_name: "cookies_response_cookie_with_domain_attribute",
		request_schema: {
			additionalProperties: false,
			properties: { value: { type: "string" } },
			required: ["value"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_domain_attribute: cookiesResponseCookieWithDomainAttribute,
		},
	};
}

/**
 * Handler for GET /timeouts/slow
 */
async function requestTimeoutRequestExceedsTimeout(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 408 };
	await new Promise((resolve) => setTimeout(resolve, 1500));
	response.body = null;
	return JSON.stringify(response);
}

export function createAppRequestTimeoutRequestExceedsTimeout(): SpikardApp {
	const config: ServerConfig = {
		requestTimeout: 1,
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/timeouts/slow",
		handler_name: "request_timeout_request_exceeds_timeout",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

/**
 * Handler for GET /timeouts/fast
 */
async function requestTimeoutRequestCompletesBeforeTimeout(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	await new Promise((resolve) => setTimeout(resolve, 100));
	const responseBody = { duration: "fast", status: "ok" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppRequestTimeoutRequestCompletesBeforeTimeout(): SpikardApp {
	const config: ServerConfig = {
		requestTimeout: 2,
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/timeouts/fast",
		handler_name: "request_timeout_request_completes_before_timeout",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

/**
 * Handler for GET /request-id/preserved
 */
async function requestIdRequestIdHeaderIsPreserved(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "x-request-id": "trace-123" };
	const responseBody = { echo: "trace-123", status: "preserved" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppRequestIdRequestIdHeaderIsPreserved(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/request-id/preserved",
		handler_name: "request_id_request_id_header_is_preserved",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			request_id_request_id_header_is_preserved: requestIdRequestIdHeaderIsPreserved,
		},
	};
}

/**
 * Handler for GET /request-id/disabled
 */
async function requestIdRequestIdMiddlewareCanBeDisabled(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { status: "no-request-id" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppRequestIdRequestIdMiddlewareCanBeDisabled(): SpikardApp {
	const config: ServerConfig = {
		enableRequestId: false,
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/request-id/disabled",
		handler_name: "request_id_request_id_middleware_can_be_disabled",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

/**
 * Handler for GET /request-id/generated
 */
async function requestIdRequestIdIsGeneratedWhenNotProvided(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "x-request-id": "00000000-0000-4000-8000-000000000000" };
	const responseBody = { status: "generated" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppRequestIdRequestIdIsGeneratedWhenNotProvided(): SpikardApp {
	const config: ServerConfig = {
		enableRequestId: true,
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/request-id/generated",
		handler_name: "request_id_request_id_is_generated_when_not_provided",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

/**
 * Handler for POST /slow-endpoint
 */
async function statusCodes408RequestTimeout(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 408 };
	response.headers = { connection: "close" };
	const responseBody = { detail: "Request timeout" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes408RequestTimeout(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/slow-endpoint",
		handler_name: "status_codes_408_request_timeout",
		request_schema: {
			additionalProperties: false,
			properties: { data: { type: "string" } },
			required: ["data"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_408_request_timeout: statusCodes408RequestTimeout,
		},
	};
}

/**
 * Handler for GET /status-test/{code}
 */
async function statusCodes404NotFoundResourceNotFound(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 404 };
	const responseBody = { detail: "Item not found" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes404NotFoundResourceNotFound(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/status-test/{code}",
		handler_name: "status_codes_404_not_found_resource_not_found",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { code: { source: "path", type: "string" } }, required: ["code"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_404_not_found_resource_not_found: statusCodes404NotFoundResourceNotFound,
		},
	};
}

/**
 * Handler for GET /health
 */
async function statusCodes503ServiceUnavailableServerOverload(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 503 };
	response.headers = { "retry-after": "120" };
	const responseBody = { detail: "Service temporarily unavailable" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes503ServiceUnavailableServerOverload(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/health",
		handler_name: "status_codes_503_service_unavailable_server_overload",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_503_service_unavailable_server_overload: statusCodes503ServiceUnavailableServerOverload,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function statusCodes422UnprocessableEntityValidationError(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppStatusCodes422UnprocessableEntityValidationError(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "status_codes_422_unprocessable_entity_validation_error",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, price: { type: "string" } },
			required: ["price", "name"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_422_unprocessable_entity_validation_error: statusCodes422UnprocessableEntityValidationError,
		},
	};
}

/**
 * Handler for GET /temp-redirect
 */
async function statusCodes302FoundTemporaryRedirect(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 302 };
	response.headers = { location: "/target-path" };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppStatusCodes302FoundTemporaryRedirect(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/temp-redirect",
		handler_name: "status_codes_302_found_temporary_redirect",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_302_found_temporary_redirect: statusCodes302FoundTemporaryRedirect,
		},
	};
}

/**
 * Handler for GET /status-test/{code}
 */
async function statusCodes304NotModifiedCachedContentValid(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 304 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppStatusCodes304NotModifiedCachedContentValid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/status-test/{code}",
		handler_name: "status_codes_304_not_modified_cached_content_valid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "If-None-Match": { source: "header", type: "string" }, code: { source: "path", type: "string" } },
			required: ["code"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_304_not_modified_cached_content_valid: statusCodes304NotModifiedCachedContentValid,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function statusCodes400BadRequestInvalidRequest(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 400 };
	const responseBody = { detail: "Invalid request format" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes400BadRequestInvalidRequest(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "status_codes_400_bad_request_invalid_request",
		request_schema: { type: "string" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_400_bad_request_invalid_request: statusCodes400BadRequestInvalidRequest,
		},
	};
}

/**
 * Handler for TRACE /data
 */
async function statusCodes22501NotImplemented(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 405 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppStatusCodes22501NotImplemented(): SpikardApp {
	const route: RouteMetadata = {
		method: "TRACE",
		path: "/data",
		handler_name: "status_codes_22_501_not_implemented",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_22_501_not_implemented: statusCodes22501NotImplemented,
		},
	};
}

/**
 * Handler for DELETE /status-test/{code}
 */
async function statusCodes204NoContentSuccessWithNoBody(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 204 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppStatusCodes204NoContentSuccessWithNoBody(): SpikardApp {
	const route: RouteMetadata = {
		method: "DELETE",
		path: "/status-test/{code}",
		handler_name: "status_codes_204_no_content_success_with_no_body",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { code: { source: "path", type: "string" } }, required: ["code"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_204_no_content_success_with_no_body: statusCodes204NoContentSuccessWithNoBody,
		},
	};
}

/**
 * Handler for GET /old-path
 */
async function statusCodes301MovedPermanentlyPermanentRedirect(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 301 };
	response.headers = { location: "/new-path" };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppStatusCodes301MovedPermanentlyPermanentRedirect(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/old-path",
		handler_name: "status_codes_301_moved_permanently_permanent_redirect",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_301_moved_permanently_permanent_redirect: statusCodes301MovedPermanentlyPermanentRedirect,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function statusCodes201CreatedResourceCreated(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = { id: 1, name: "New Item" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes201CreatedResourceCreated(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "status_codes_201_created_resource_created",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" } },
			required: ["name"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_201_created_resource_created: statusCodes201CreatedResourceCreated,
		},
	};
}

/**
 * Handler for POST /tasks/
 */
async function statusCodes202AcceptedRequestAcceptedForProcessing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 202 };
	const responseBody = { message: "Task accepted for processing", task_id: "abc123" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes202AcceptedRequestAcceptedForProcessing(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/tasks/",
		handler_name: "status_codes_202_accepted_request_accepted_for_processing",
		request_schema: {
			additionalProperties: false,
			properties: { task: { type: "string" } },
			required: ["task"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_202_accepted_request_accepted_for_processing: statusCodes202AcceptedRequestAcceptedForProcessing,
		},
	};
}

/**
 * Handler for POST /redirect-post
 */
async function statusCodes307TemporaryRedirectMethodPreserved(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 307 };
	response.headers = { location: "/target-post" };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppStatusCodes307TemporaryRedirectMethodPreserved(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/redirect-post",
		handler_name: "status_codes_307_temporary_redirect_method_preserved",
		request_schema: { additionalProperties: false, properties: {}, type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_307_temporary_redirect_method_preserved: statusCodes307TemporaryRedirectMethodPreserved,
		},
	};
}

/**
 * Handler for GET /error
 */
async function statusCodes500InternalServerErrorServerError(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 500 };
	const responseBody = {
		detail: "Internal server error",
		status: 500,
		title: "Internal Server Error",
		type: "https://spikard.dev/errors/internal-server-error",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes500InternalServerErrorServerError(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/error",
		handler_name: "status_codes_500_internal_server_error_server_error",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_500_internal_server_error_server_error: statusCodes500InternalServerErrorServerError,
		},
	};
}

/**
 * Handler for GET /data
 */
async function statusCodes20414UriTooLong(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppStatusCodes20414UriTooLong(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/data",
		handler_name: "status_codes_20_414_uri_too_long",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_20_414_uri_too_long: statusCodes20414UriTooLong,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function statusCodes401UnauthorizedMissingAuthentication(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	response.headers = { "www-authenticate": "Bearer" };
	const responseBody = { detail: "Not authenticated" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes401UnauthorizedMissingAuthentication(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "status_codes_401_unauthorized_missing_authentication",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_401_unauthorized_missing_authentication: statusCodes401UnauthorizedMissingAuthentication,
		},
	};
}

/**
 * Handler for GET /data
 */
async function statusCodes23503ServiceUnavailable(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 503 };
	response.headers = { "retry-after": "60" };
	const responseBody = {
		error: "Service Unavailable",
		message: "The service is temporarily unavailable. Please try again later.",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes23503ServiceUnavailable(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/data",
		handler_name: "status_codes_23_503_service_unavailable",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_23_503_service_unavailable: statusCodes23503ServiceUnavailable,
		},
	};
}

/**
 * Handler for POST /upload
 */
async function statusCodes19413PayloadTooLarge(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 413 };
	const responseBody = {
		error: "Payload Too Large",
		message: "Request body size exceeds maximum allowed size of 1024 bytes",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes19413PayloadTooLarge(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "status_codes_19_413_payload_too_large",
		request_schema: { properties: { data: { type: "string" } }, type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_19_413_payload_too_large: statusCodes19413PayloadTooLarge,
		},
	};
}

/**
 * Handler for GET /admin/users
 */
async function statusCodes403ForbiddenInsufficientPermissions(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 403 };
	const responseBody = { detail: "Not enough permissions" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes403ForbiddenInsufficientPermissions(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/admin/users",
		handler_name: "status_codes_403_forbidden_insufficient_permissions",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_403_forbidden_insufficient_permissions: statusCodes403ForbiddenInsufficientPermissions,
		},
	};
}

/**
 * Handler for GET /data
 */
async function statusCodes21431RequestHeaderFieldsTooLarge(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 431 };
	const responseBody = {
		error: "Request Header Fields Too Large",
		message: "Request headers exceed maximum allowed size of 8192 bytes",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes21431RequestHeaderFieldsTooLarge(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/data",
		handler_name: "status_codes_21_431_request_header_fields_too_large",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { "X-Large-Header": { source: "header", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_21_431_request_header_fields_too_large: statusCodes21431RequestHeaderFieldsTooLarge,
		},
	};
}

/**
 * Handler for GET /api/resource
 */
async function statusCodes429TooManyRequests(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 429 };
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

export function createAppStatusCodes429TooManyRequests(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/resource",
		handler_name: "status_codes_429_too_many_requests",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_429_too_many_requests: statusCodes429TooManyRequests,
		},
	};
}

/**
 * Handler for GET /status-test/{code}
 */
async function statusCodes200OkSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { id: 1, name: "Item 1" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes200OkSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/status-test/{code}",
		handler_name: "status_codes_200_ok_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { code: { source: "path", type: "string" } }, required: ["code"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_200_ok_success: statusCodes200OkSuccess,
		},
	};
}

/**
 * Handler for GET /files/document.pdf
 */
async function statusCodes206PartialContent(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 206 };
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

export function createAppStatusCodes206PartialContent(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/files/document.pdf",
		handler_name: "status_codes_206_partial_content",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_206_partial_content: statusCodes206PartialContent,
		},
	};
}

/**
 * Handler for OPTIONS /items/
 */
async function httpMethodsOptionsCorsPreflightRequest(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {
		"access-control-allow-headers": "Content-Type",
		"access-control-allow-methods": "GET, POST, PUT, DELETE, OPTIONS",
		"access-control-allow-origin": "https://example.com",
		"access-control-max-age": "86400",
	};
	const result: Record<string, unknown> = {};
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHttpMethodsOptionsCorsPreflightRequest(): SpikardApp {
	const route: RouteMetadata = {
		method: "OPTIONS",
		path: "/items/",
		handler_name: "http_methods_options_cors_preflight_request",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_options_cors_preflight_request: httpMethodsOptionsCorsPreflightRequest,
		},
	};
}

/**
 * Handler for DELETE /items/{id}
 */
async function httpMethodsDeleteRemoveResource(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppHttpMethodsDeleteRemoveResource(): SpikardApp {
	const route: RouteMetadata = {
		method: "DELETE",
		path: "/items/{id}",
		handler_name: "http_methods_delete_remove_resource",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_delete_remove_resource: httpMethodsDeleteRemoveResource,
		},
	};
}

/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutCreateResourceIfDoesnTExist(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { id: 999, name: "New Item", price: 49.99 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHttpMethodsPutCreateResourceIfDoesnTExist(): SpikardApp {
	const route: RouteMetadata = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_create_resource_if_doesn_t_exist",
		request_schema: {
			properties: { id: { type: "integer" }, name: { type: "string" }, price: { type: "number" } },
			required: ["id", "name", "price"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_create_resource_if_doesn_t_exist: httpMethodsPutCreateResourceIfDoesnTExist,
		},
	};
}

/**
 * Handler for PATCH /items/{id}
 */
async function httpMethodsPatchUpdateMultipleFields(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { id: 1, in_stock: false, name: "Updated Name", price: 89.99 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHttpMethodsPatchUpdateMultipleFields(): SpikardApp {
	const route: RouteMetadata = {
		method: "PATCH",
		path: "/items/{id}",
		handler_name: "http_methods_patch_update_multiple_fields",
		request_schema: {
			properties: { in_stock: { type: "boolean" }, name: { type: "string" }, price: { type: "number" } },
			required: ["in_stock", "name", "price"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_patch_update_multiple_fields: httpMethodsPatchUpdateMultipleFields,
		},
	};
}

/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutValidationError(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const id = params.id;
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	if (id !== null && id !== undefined) {
		result.id = id;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHttpMethodsPutValidationError(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_validation_error: httpMethodsPutValidationError,
		},
	};
}

/**
 * Handler for HEAD /items/{id}
 */
async function httpMethodsHeadGetMetadataWithoutBody(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-length": "85", "content-type": "application/json" };
	const result: Record<string, unknown> = {};
	const id = params.id;
	if (id !== null && id !== undefined) {
		result.id = id;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHttpMethodsHeadGetMetadataWithoutBody(): SpikardApp {
	const route: RouteMetadata = {
		method: "HEAD",
		path: "/items/{id}",
		handler_name: "http_methods_head_get_metadata_without_body",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_head_get_metadata_without_body: httpMethodsHeadGetMetadataWithoutBody,
		},
	};
}

/**
 * Handler for DELETE /items/{id}
 */
async function httpMethodsDeleteWithResponseBody(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { id: 1, message: "Item deleted successfully", name: "Deleted Item" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHttpMethodsDeleteWithResponseBody(): SpikardApp {
	const route: RouteMetadata = {
		method: "DELETE",
		path: "/items/{id}",
		handler_name: "http_methods_delete_with_response_body",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_delete_with_response_body: httpMethodsDeleteWithResponseBody,
		},
	};
}

/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutMissingRequiredField(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const id = params.id;
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	if (id !== null && id !== undefined) {
		result.id = id;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHttpMethodsPutMissingRequiredField(): SpikardApp {
	const route: RouteMetadata = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_missing_required_field",
		request_schema: {
			properties: { id: { type: "integer" }, name: { type: "string" }, price: { type: "string" } },
			required: ["price"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_missing_required_field: httpMethodsPutMissingRequiredField,
		},
	};
}

/**
 * Handler for PATCH /items/{id}
 */
async function httpMethodsPatchPartialUpdate(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { id: 1, in_stock: true, name: "Existing Item", price: 79.99 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHttpMethodsPatchPartialUpdate(): SpikardApp {
	const route: RouteMetadata = {
		method: "PATCH",
		path: "/items/{id}",
		handler_name: "http_methods_patch_partial_update",
		request_schema: { properties: { price: { type: "number" } }, required: ["price"], type: "object" },
		response_schema: undefined,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_patch_partial_update: httpMethodsPatchPartialUpdate,
		},
	};
}

/**
 * Handler for DELETE /items/{id}
 */
async function httpMethodsDeleteResourceNotFound(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppHttpMethodsDeleteResourceNotFound(): SpikardApp {
	const route: RouteMetadata = {
		method: "DELETE",
		path: "/items/{id}",
		handler_name: "http_methods_delete_resource_not_found",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_delete_resource_not_found: httpMethodsDeleteResourceNotFound,
		},
	};
}

/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutIdempotentOperation(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { id: 1, name: "Fixed Name", price: 50.0 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHttpMethodsPutIdempotentOperation(): SpikardApp {
	const route: RouteMetadata = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_idempotent_operation",
		request_schema: {
			properties: { id: { type: "integer" }, name: { type: "string" }, price: { type: "number" } },
			required: ["id", "name", "price"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_idempotent_operation: httpMethodsPutIdempotentOperation,
		},
	};
}

/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutCompleteResourceReplacement(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
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

export function createAppHttpMethodsPutCompleteResourceReplacement(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_complete_resource_replacement: httpMethodsPutCompleteResourceReplacement,
		},
	};
}

/**
 * Handler for POST /login/
 */
async function urlEncodedSimpleFormSubmissionSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { username: "johndoe" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedSimpleFormSubmissionSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/login/",
		handler_name: "url_encoded_simple_form_submission_success",
		request_schema: {
			properties: { password: { type: "string" }, username: { type: "string" } },
			required: ["username", "password"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_simple_form_submission_success: urlEncodedSimpleFormSubmissionSuccess,
		},
	};
}

/**
 * Handler for POST /data
 */
async function urlEncoded15SpecialCharactersFieldNames(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = { "contact.email": "john@example.com", "user-name": "JohnDoe" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncoded15SpecialCharactersFieldNames(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "url_encoded_15_special_characters_field_names",
		request_schema: {
			properties: { "contact.email": { format: "email", type: "string" }, "user-name": { type: "string" } },
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_15_special_characters_field_names: urlEncoded15SpecialCharactersFieldNames,
		},
	};
}

/**
 * Handler for POST /form/validated
 */
async function urlEncodedPatternValidationFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppUrlEncodedPatternValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/validated",
		handler_name: "url_encoded_pattern_validation_fail",
		request_schema: {
			properties: { username: { pattern: "^[a-z0-9_]+$", type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_pattern_validation_fail: urlEncodedPatternValidationFail,
		},
	};
}

/**
 * Handler for POST /settings
 */
async function urlEncoded22AdditionalPropertiesStrictFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppUrlEncoded22AdditionalPropertiesStrictFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/settings",
		handler_name: "url_encoded_22_additional_properties_strict_failure",
		request_schema: {
			additionalProperties: false,
			properties: { theme: { enum: ["light", "dark"], type: "string" } },
			required: ["theme"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_22_additional_properties_strict_failure: urlEncoded22AdditionalPropertiesStrictFailure,
		},
	};
}

/**
 * Handler for POST /accounts
 */
async function urlEncoded17PatternValidationFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppUrlEncoded17PatternValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/accounts",
		handler_name: "url_encoded_17_pattern_validation_failure",
		request_schema: {
			properties: { account_id: { pattern: "^ACC-[0-9]{6}$", type: "string" } },
			required: ["account_id"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_17_pattern_validation_failure: urlEncoded17PatternValidationFailure,
		},
	};
}

/**
 * Handler for POST /subscribe
 */
async function urlEncoded20FormatEmailValidationFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppUrlEncoded20FormatEmailValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/subscribe",
		handler_name: "url_encoded_20_format_email_validation_failure",
		request_schema: { properties: { email: { format: "email", type: "string" } }, required: ["email"], type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_20_format_email_validation_failure: urlEncoded20FormatEmailValidationFailure,
		},
	};
}

/**
 * Handler for POST /form/tags
 */
async function urlEncodedMultipleValuesForSameField(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { tags: ["python", "fastapi", "web"] };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedMultipleValuesForSameField(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/tags",
		handler_name: "url_encoded_multiple_values_for_same_field",
		request_schema: {
			properties: { tags: { items: { type: "string" }, type: "array" } },
			required: ["tags"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_multiple_values_for_same_field: urlEncodedMultipleValuesForSameField,
		},
	};
}

/**
 * Handler for POST /login/
 */
async function urlEncodedRequiredFieldMissingValidationError(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppUrlEncodedRequiredFieldMissingValidationError(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/login/",
		handler_name: "url_encoded_required_field_missing_validation_error",
		request_schema: {
			properties: { password: { type: "string" }, username: { type: "string" } },
			required: ["username", "password"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_required_field_missing_validation_error: urlEncodedRequiredFieldMissingValidationError,
		},
	};
}

/**
 * Handler for POST /register
 */
async function urlEncoded13ArrayFieldSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = { tags: ["python", "rust", "typescript"] };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncoded13ArrayFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/register",
		handler_name: "url_encoded_13_array_field_success",
		request_schema: {
			properties: { tags: { items: { type: "string" }, minItems: 1, type: "array" } },
			required: ["tags"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_13_array_field_success: urlEncoded13ArrayFieldSuccess,
		},
	};
}

/**
 * Handler for POST /form/
 */
async function urlEncodedNumericFieldTypeConversion(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { age: 30, username: "johndoe" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedNumericFieldTypeConversion(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_numeric_field_type_conversion",
		request_schema: {
			properties: { age: { type: "integer" }, username: { type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_numeric_field_type_conversion: urlEncodedNumericFieldTypeConversion,
		},
	};
}

/**
 * Handler for POST /form/
 */
async function urlEncodedSpecialCharactersEncoding(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { description: "Test & Development", name: "John Doe" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedSpecialCharactersEncoding(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_special_characters_encoding",
		request_schema: {
			properties: { description: { type: "string" }, name: { type: "string" } },
			required: ["name"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_special_characters_encoding: urlEncodedSpecialCharactersEncoding,
		},
	};
}

/**
 * Handler for POST /form/
 */
async function urlEncodedBooleanFieldConversion(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { subscribe: true, username: "johndoe" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedBooleanFieldConversion(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_boolean_field_conversion",
		request_schema: {
			properties: { subscribe: { type: "boolean" }, username: { type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_boolean_field_conversion: urlEncodedBooleanFieldConversion,
		},
	};
}

/**
 * Handler for POST /form/
 */
async function urlEncodedEmptyStringValue(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { description: "", username: "johndoe" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedEmptyStringValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_empty_string_value",
		request_schema: {
			properties: { description: { type: "string" }, username: { type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_empty_string_value: urlEncodedEmptyStringValue,
		},
	};
}

/**
 * Handler for POST /token
 */
async function urlEncodedOauth2PasswordGrantFlow(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { access_token: "johndoe", token_type: "bearer" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedOauth2PasswordGrantFlow(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_oauth2_password_grant_flow: urlEncodedOauth2PasswordGrantFlow,
		},
	};
}

/**
 * Handler for POST /tags
 */
async function urlEncoded19ArrayMinitemsValidationFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppUrlEncoded19ArrayMinitemsValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/tags",
		handler_name: "url_encoded_19_array_minitems_validation_failure",
		request_schema: {
			properties: { tags: { items: { type: "string" }, minItems: 2, type: "array" } },
			required: ["tags"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_19_array_minitems_validation_failure: urlEncoded19ArrayMinitemsValidationFailure,
		},
	};
}

/**
 * Handler for POST /register/
 */
async function urlEncodedOptionalFieldMissingSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { email: null, username: "johndoe" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedOptionalFieldMissingSuccess(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_optional_field_missing_success: urlEncodedOptionalFieldMissingSuccess,
		},
	};
}

/**
 * Handler for POST /profile
 */
async function urlEncoded14NestedObjectBracketNotation(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = { user: { age: 30, email: "john@example.com", name: "John Doe" } };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncoded14NestedObjectBracketNotation(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_14_nested_object_bracket_notation: urlEncoded14NestedObjectBracketNotation,
		},
	};
}

/**
 * Handler for POST /form/validated
 */
async function urlEncodedStringMaxLengthValidationFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppUrlEncodedStringMaxLengthValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/validated",
		handler_name: "url_encoded_string_max_length_validation_fail",
		request_schema: {
			properties: { username: { maxLength: 20, type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_string_max_length_validation_fail: urlEncodedStringMaxLengthValidationFail,
		},
	};
}

/**
 * Handler for POST /products
 */
async function urlEncoded18IntegerMinimumValidationFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppUrlEncoded18IntegerMinimumValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/products",
		handler_name: "url_encoded_18_integer_minimum_validation_failure",
		request_schema: {
			properties: { quantity: { minimum: 1, type: "integer" } },
			required: ["quantity"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_18_integer_minimum_validation_failure: urlEncoded18IntegerMinimumValidationFailure,
		},
	};
}

/**
 * Handler for POST /products
 */
async function urlEncoded21IntegerTypeCoercionFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppUrlEncoded21IntegerTypeCoercionFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/products",
		handler_name: "url_encoded_21_integer_type_coercion_failure",
		request_schema: { properties: { price: { type: "integer" } }, required: ["price"], type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_21_integer_type_coercion_failure: urlEncoded21IntegerTypeCoercionFailure,
		},
	};
}

/**
 * Handler for POST /users
 */
async function urlEncoded16MinlengthValidationFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppUrlEncoded16MinlengthValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "url_encoded_16_minlength_validation_failure",
		request_schema: {
			properties: { username: { minLength: 3, type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_16_minlength_validation_failure: urlEncoded16MinlengthValidationFailure,
		},
	};
}

/**
 * Handler for POST /form/validated
 */
async function urlEncodedStringMinLengthValidationFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppUrlEncodedStringMinLengthValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/validated",
		handler_name: "url_encoded_string_min_length_validation_fail",
		request_schema: {
			properties: { username: { minLength: 3, type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_string_min_length_validation_fail: urlEncodedStringMinLengthValidationFail,
		},
	};
}

/**
 * Handler for GET /headers/pattern
 */
async function headersHeaderRegexValidationSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { x_request_id: "12345" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersHeaderRegexValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/pattern",
		handler_name: "headers_header_regex_validation_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "X-Request-Id": { annotation: "str", pattern: "^[0-9]{3,}$", source: "header", type: "string" } },
			required: ["X-Request-Id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_regex_validation_success: headersHeaderRegexValidationSuccess,
		},
	};
}

/**
 * Handler for GET /api/data
 */
async function headers33ApiKeyHeaderValid(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const result: Record<string, unknown> = {};
	const xAPIKey = params["X-API-Key"];
	if (xAPIKey !== null && xAPIKey !== undefined) {
		result["X-API-Key"] = xAPIKey;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHeaders33ApiKeyHeaderValid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "headers_33_api_key_header_valid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "X-API-Key": { pattern: "^[a-f0-9]{32}$", source: "header", type: "string" } },
			required: ["X-API-Key"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_33_api_key_header_valid: headers33ApiKeyHeaderValid,
		},
	};
}

/**
 * Handler for GET /headers/content-type
 */
async function headersContentTypeHeaderApplicationJson(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { content_type: "application/json" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersContentTypeHeaderApplicationJson(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/content-type",
		handler_name: "headers_content_type_header_application_json",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "Content-Type": { annotation: "str", source: "header", type: "string" } },
			required: ["Content-Type"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_content_type_header_application_json: headersContentTypeHeaderApplicationJson,
		},
	};
}

/**
 * Handler for GET /headers/accept-language
 */
async function headersAcceptLanguageHeader(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { accept_language: "en-US,en;q=0.9" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersAcceptLanguageHeader(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/accept-language",
		handler_name: "headers_accept_language_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "Accept-Language": { annotation: "str", source: "header", type: "string" } },
			required: ["Accept-Language"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_accept_language_header: headersAcceptLanguageHeader,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function headersXApiKeyRequiredHeaderSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { username: "secret" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersXApiKeyRequiredHeaderSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_x_api_key_required_header_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { key: { annotation: "str", source: "header", type: "string" } },
			required: ["key"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_x_api_key_required_header_success: headersXApiKeyRequiredHeaderSuccess,
		},
	};
}

/**
 * Handler for GET /headers/max-length
 */
async function headersHeaderValidationMaxLengthConstraintFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const xSessionId = params["X-Session-Id"];
	if (xSessionId !== null && xSessionId !== undefined) {
		result["X-Session-Id"] = xSessionId;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHeadersHeaderValidationMaxLengthConstraintFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/max-length",
		handler_name: "headers_header_validation_max_length_constraint_fail",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "X-Session-Id": { annotation: "str", maxLength: 20, source: "header", type: "string" } },
			required: ["X-Session-Id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_validation_max_length_constraint_fail: headersHeaderValidationMaxLengthConstraintFail,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function headersXApiKeyRequiredHeaderMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const xAPIKey = params["X-API-Key"];
	if (xAPIKey !== null && xAPIKey !== undefined) {
		result["X-API-Key"] = xAPIKey;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHeadersXApiKeyRequiredHeaderMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_x_api_key_required_header_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "X-API-Key": { annotation: "str", source: "header", type: "string" } },
			required: ["X-API-Key"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_x_api_key_required_header_missing: headersXApiKeyRequiredHeaderMissing,
		},
	};
}

/**
 * Handler for GET /headers/origin
 */
async function headersOriginHeader(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { origin: "https://example.com" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersOriginHeader(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/origin",
		handler_name: "headers_origin_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Origin: { annotation: "str", source: "header", type: "string" } },
			required: ["Origin"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_origin_header: headersOriginHeader,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function headersUserAgentHeaderDefaultValue(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { "User-Agent": "testclient" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersUserAgentHeaderDefaultValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "headers_user_agent_header_default_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "User-Agent": { annotation: "str", default: "testclient", source: "header", type: "string" } },
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_user_agent_header_default_value: headersUserAgentHeaderDefaultValue,
		},
	};
}

/**
 * Handler for GET /protected
 */
async function headers32BearerTokenMissingPrefix(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const authorization = params.Authorization;
	if (authorization !== null && authorization !== undefined) {
		result.Authorization = authorization;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHeaders32BearerTokenMissingPrefix(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/protected",
		handler_name: "headers_32_bearer_token_missing_prefix",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { pattern: "^Bearer [A-Za-z0-9-._~+/]+=*$", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_32_bearer_token_missing_prefix: headers32BearerTokenMissingPrefix,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function headersOptionalHeaderWithNoneDefaultMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { strange_header: null };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersOptionalHeaderWithNoneDefaultMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "headers_optional_header_with_none_default_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "strange-header": { annotation: "str", default: null, source: "header", type: "string" } },
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_optional_header_with_none_default_missing: headersOptionalHeaderWithNoneDefaultMissing,
		},
	};
}

/**
 * Handler for GET /headers/pattern
 */
async function headersHeaderRegexValidationFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const xRequestId = params["X-Request-Id"];
	if (xRequestId !== null && xRequestId !== undefined) {
		result["X-Request-Id"] = xRequestId;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHeadersHeaderRegexValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/pattern",
		handler_name: "headers_header_regex_validation_fail",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "X-Request-Id": { annotation: "str", pattern: "^[0-9]{3,}$", source: "header", type: "string" } },
			required: ["X-Request-Id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_regex_validation_fail: headersHeaderRegexValidationFail,
		},
	};
}

/**
 * Handler for GET /protected
 */
async function headers31BearerTokenFormatInvalid(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const authorization = params.Authorization;
	if (authorization !== null && authorization !== undefined) {
		result.Authorization = authorization;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHeaders31BearerTokenFormatInvalid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/protected",
		handler_name: "headers_31_bearer_token_format_invalid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { pattern: "^Bearer [A-Za-z0-9-._~+/]+=*$", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_31_bearer_token_format_invalid: headers31BearerTokenFormatInvalid,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function headersXApiKeyOptionalHeaderSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { msg: "Hello secret" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersXApiKeyOptionalHeaderSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_x_api_key_optional_header_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { key: { annotation: "str", source: "header", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_x_api_key_optional_header_success: headersXApiKeyOptionalHeaderSuccess,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function headersAuthorizationHeaderSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { credentials: "foobar", scheme: "Digest" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersAuthorizationHeaderSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_authorization_header_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { annotation: "str", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_authorization_header_success: headersAuthorizationHeaderSuccess,
		},
	};
}

/**
 * Handler for GET /protected
 */
async function headers30BearerTokenFormatValid(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const result: Record<string, unknown> = {};
	const authorization = params.Authorization;
	if (authorization !== null && authorization !== undefined) {
		result.Authorization = authorization;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHeaders30BearerTokenFormatValid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/protected",
		handler_name: "headers_30_bearer_token_format_valid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { pattern: "^Bearer [A-Za-z0-9-._~+/]+=*$", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_30_bearer_token_format_valid: headers30BearerTokenFormatValid,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function headersAuthorizationHeaderMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const authorization = params.Authorization;
	if (authorization !== null && authorization !== undefined) {
		result.Authorization = authorization;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHeadersAuthorizationHeaderMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_authorization_header_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { annotation: "str", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_authorization_header_missing: headersAuthorizationHeaderMissing,
		},
	};
}

/**
 * Handler for GET /headers/accept
 */
async function headersAcceptHeaderJson(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { accept: "application/json" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersAcceptHeaderJson(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/accept",
		handler_name: "headers_accept_header_json",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Accept: { annotation: "str", source: "header", type: "string" } },
			required: ["Accept"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_accept_header_json: headersAcceptHeaderJson,
		},
	};
}

/**
 * Handler for GET /headers/accept-encoding
 */
async function headersAcceptEncodingHeader(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { accept_encoding: "gzip, deflate, br" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersAcceptEncodingHeader(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/accept-encoding",
		handler_name: "headers_accept_encoding_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "Accept-Encoding": { annotation: "str", source: "header", type: "string" } },
			required: ["Accept-Encoding"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_accept_encoding_header: headersAcceptEncodingHeader,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function headersAuthorizationHeaderWrongScheme(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const authorization = params.Authorization;
	if (authorization !== null && authorization !== undefined) {
		result.Authorization = authorization;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHeadersAuthorizationHeaderWrongScheme(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_authorization_header_wrong_scheme",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { annotation: "str", pattern: "^Digest .+", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_authorization_header_wrong_scheme: headersAuthorizationHeaderWrongScheme,
		},
	};
}

/**
 * Handler for GET /headers/validated
 */
async function headersHeaderValidationMinLengthConstraint(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const xToken = params["X-Token"];
	if (xToken !== null && xToken !== undefined) {
		result["X-Token"] = xToken;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHeadersHeaderValidationMinLengthConstraint(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/validated",
		handler_name: "headers_header_validation_min_length_constraint",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "X-Token": { annotation: "str", minLength: 3, source: "header", type: "string" } },
			required: ["X-Token"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_validation_min_length_constraint: headersHeaderValidationMinLengthConstraint,
		},
	};
}

/**
 * Handler for GET /headers/basic-auth
 */
async function headersBasicAuthenticationSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { password: "password", username: "username" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersBasicAuthenticationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/basic-auth",
		handler_name: "headers_basic_authentication_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { annotation: "str", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_basic_authentication_success: headersBasicAuthenticationSuccess,
		},
	};
}

/**
 * Handler for GET /headers/bearer-auth
 */
async function headersBearerTokenAuthenticationMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const authorization = params.Authorization;
	if (authorization !== null && authorization !== undefined) {
		result.Authorization = authorization;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHeadersBearerTokenAuthenticationMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/bearer-auth",
		handler_name: "headers_bearer_token_authentication_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { annotation: "str", pattern: "^Bearer .+", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_bearer_token_authentication_missing: headersBearerTokenAuthenticationMissing,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function headersXApiKeyOptionalHeaderMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { msg: "Hello World" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersXApiKeyOptionalHeaderMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_x_api_key_optional_header_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { key: { annotation: "str", source: "header", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_x_api_key_optional_header_missing: headersXApiKeyOptionalHeaderMissing,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function headersMultipleHeaderValuesXToken(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { "X-Token values": ["foo", "bar"] };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersMultipleHeaderValuesXToken(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "headers_multiple_header_values_x_token",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "x-token": { annotation: "str", source: "header", type: "string" } },
			required: ["x-token"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_multiple_header_values_x_token: headersMultipleHeaderValuesXToken,
		},
	};
}

/**
 * Handler for GET /headers/multiple
 */
async function headersMultipleCustomHeaders(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { x_client_version: "1.2.3", x_request_id: "req-12345", x_trace_id: "trace-abc" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersMultipleCustomHeaders(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/multiple",
		handler_name: "headers_multiple_custom_headers",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				"X-Client-Version": { annotation: "str", source: "header", type: "string" },
				"X-Request-Id": { annotation: "str", source: "header", type: "string" },
				"X-Trace-Id": { annotation: "str", source: "header", type: "string" },
			},
			required: ["X-Client-Version", "X-Request-Id", "X-Trace-Id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_multiple_custom_headers: headersMultipleCustomHeaders,
		},
	};
}

/**
 * Handler for GET /api/data
 */
async function headers34ApiKeyHeaderInvalid(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const xAPIKey = params["X-API-Key"];
	if (xAPIKey !== null && xAPIKey !== undefined) {
		result["X-API-Key"] = xAPIKey;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppHeaders34ApiKeyHeaderInvalid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "headers_34_api_key_header_invalid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "X-API-Key": { pattern: "^[a-f0-9]{32}$", source: "header", type: "string" } },
			required: ["X-API-Key"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_34_api_key_header_invalid: headers34ApiKeyHeaderInvalid,
		},
	};
}

/**
 * Handler for GET /headers/bearer-auth
 */
async function headersBearerTokenAuthenticationSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { token: "valid_token_123" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersBearerTokenAuthenticationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/bearer-auth",
		handler_name: "headers_bearer_token_authentication_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { annotation: "str", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_bearer_token_authentication_success: headersBearerTokenAuthenticationSuccess,
		},
	};
}

/**
 * Handler for GET /headers/host
 */
async function headersHostHeader(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { host: "example.com:8080" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersHostHeader(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/host",
		handler_name: "headers_host_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Host: { annotation: "str", source: "header", type: "string" } },
			required: ["Host"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_host_header: headersHostHeader,
		},
	};
}

/**
 * Handler for GET /headers/referer
 */
async function headersRefererHeader(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { referer: "https://example.com/page" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersRefererHeader(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/referer",
		handler_name: "headers_referer_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Referer: { annotation: "str", source: "header", type: "string" } },
			required: ["Referer"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_referer_header: headersRefererHeader,
		},
	};
}

/**
 * Handler for GET /headers/underscore
 */
async function headersHeaderWithUnderscoreConversionExplicit(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { x_token: "secret123" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersHeaderWithUnderscoreConversionExplicit(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/underscore",
		handler_name: "headers_header_with_underscore_conversion_explicit",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "X-Token": { annotation: "str", source: "header", type: "string" } },
			required: ["X-Token"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_with_underscore_conversion_explicit: headersHeaderWithUnderscoreConversionExplicit,
		},
	};
}

/**
 * Handler for POST /echo
 */
async function headersHeaderCaseInsensitivityAccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {
		content_type_lower: "application/json",
		content_type_mixed: "application/json",
		content_type_upper: "application/json",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersHeaderCaseInsensitivityAccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/echo",
		handler_name: "headers_header_case_insensitivity_access",
		request_schema: {
			additionalProperties: false,
			properties: { test: { type: "string" } },
			required: ["test"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_case_insensitivity_access: headersHeaderCaseInsensitivityAccess,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function headersUserAgentHeaderCustomValue(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { "User-Agent": "Mozilla/5.0 Custom Browser" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersUserAgentHeaderCustomValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "headers_user_agent_header_custom_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "User-Agent": { annotation: "str", source: "header", type: "string" } },
			required: ["User-Agent"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_user_agent_header_custom_value: headersUserAgentHeaderCustomValue,
		},
	};
}

/**
 * Handler for POST /
 */
async function multipartMultipleValuesForSameFieldName(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
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

export function createAppMultipartMultipleValuesForSameFieldName(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_multiple_values_for_same_field_name: multipartMultipleValuesForSameFieldName,
		},
	};
}

/**
 * Handler for POST /upload
 */
async function multipart19FileMimeSpoofingPngAsJpeg(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	response.body = result;
	return JSON.stringify(response);
}

export function createAppMultipart19FileMimeSpoofingPngAsJpeg(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_19_file_mime_spoofing_png_as_jpeg",
		request_schema: undefined,
		response_schema: undefined,
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

/**
 * Handler for POST /upload
 */
async function multipart20FileMimeSpoofingJpegAsPng(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	response.body = result;
	return JSON.stringify(response);
}

export function createAppMultipart20FileMimeSpoofingJpegAsPng(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_20_file_mime_spoofing_jpeg_as_png",
		request_schema: undefined,
		response_schema: undefined,
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

/**
 * Handler for POST /upload
 */
async function multipart21FilePdfMagicNumberSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppMultipart21FilePdfMagicNumberSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_21_file_pdf_magic_number_success",
		request_schema: undefined,
		response_schema: undefined,
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

/**
 * Handler for POST /files/images-only
 */
async function multipartContentTypeValidationInvalidType(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppMultipartContentTypeValidationInvalidType(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/images-only",
		handler_name: "multipart_content_type_validation_invalid_type",
		request_schema: {
			additionalProperties: false,
			properties: { file: { format: "binary", type: "string" } },
			type: "object",
		},
		response_schema: undefined,
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

/**
 * Handler for POST /files/document
 */
async function multipartPdfFileUpload(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { content_type: "application/pdf", filename: "report.pdf", size: 16 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartPdfFileUpload(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/document",
		handler_name: "multipart_pdf_file_upload",
		request_schema: {
			additionalProperties: false,
			properties: { document: { format: "binary", type: "string" } },
			required: ["document"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_pdf_file_upload: multipartPdfFileUpload,
		},
	};
}

/**
 * Handler for POST /files/list
 */
async function multipartFileListUploadArrayOfFiles(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { filenames: ["file1.txt", "file2.txt"], total_size: 35 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartFileListUploadArrayOfFiles(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/list",
		handler_name: "multipart_file_list_upload_array_of_files",
		request_schema: {
			additionalProperties: false,
			properties: { files: { items: { format: "binary", type: "string" }, type: "array" } },
			required: ["files"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_file_list_upload_array_of_files: multipartFileListUploadArrayOfFiles,
		},
	};
}

/**
 * Handler for POST /files/optional
 */
async function multipartOptionalFileUploadProvided(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { content_type: "text/plain", filename: "optional.txt", size: 21 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartOptionalFileUploadProvided(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/optional",
		handler_name: "multipart_optional_file_upload_provided",
		request_schema: {
			additionalProperties: false,
			properties: { file: { format: "binary", type: "string" } },
			required: ["file"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_optional_file_upload_provided: multipartOptionalFileUploadProvided,
		},
	};
}

/**
 * Handler for POST /files/validated
 */
async function multipartFileSizeValidationTooLarge(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 413 };
	const responseBody = { detail: "File too large. Maximum size is 1MB" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartFileSizeValidationTooLarge(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/validated",
		handler_name: "multipart_file_size_validation_too_large",
		request_schema: {
			additionalProperties: false,
			properties: { file: { format: "binary", type: "string" } },
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_file_size_validation_too_large: multipartFileSizeValidationTooLarge,
		},
	};
}

/**
 * Handler for POST /
 */
async function multipartMixedFilesAndFormData(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {
		active: "true",
		age: "25",
		file: { content: "file data here", content_type: "text/plain", filename: "upload.txt", size: 14 },
		username: "testuser",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartMixedFilesAndFormData(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_mixed_files_and_form_data: multipartMixedFilesAndFormData,
		},
	};
}

/**
 * Handler for POST /
 */
async function multipartSimpleFileUpload(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {
		test: { content: "<file content>", content_type: "text/plain", filename: "test.txt", size: 14 },
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartSimpleFileUpload(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_simple_file_upload",
		request_schema: {
			additionalProperties: false,
			properties: { test: { format: "binary", type: "string" } },
			required: ["test"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_simple_file_upload: multipartSimpleFileUpload,
		},
	};
}

/**
 * Handler for POST /files/upload
 */
async function multipartEmptyFileUpload(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { filename: "empty.txt", size: 0 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartEmptyFileUpload(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/upload",
		handler_name: "multipart_empty_file_upload",
		request_schema: {
			additionalProperties: false,
			properties: { file: { format: "binary", type: "string" } },
			required: ["file"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_empty_file_upload: multipartEmptyFileUpload,
		},
	};
}

/**
 * Handler for POST /files/optional
 */
async function multipartOptionalFileUploadMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { file: null };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartOptionalFileUploadMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/optional",
		handler_name: "multipart_optional_file_upload_missing",
		request_schema: { additionalProperties: false, properties: {}, type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_optional_file_upload_missing: multipartOptionalFileUploadMissing,
		},
	};
}

/**
 * Handler for POST /
 */
async function multipartFileUploadWithoutFilename(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { test1: "<file1 content>" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartFileUploadWithoutFilename(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_file_upload_without_filename",
		request_schema: {
			additionalProperties: false,
			properties: { test1: { format: "binary", type: "string" } },
			required: ["test1"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_file_upload_without_filename: multipartFileUploadWithoutFilename,
		},
	};
}

/**
 * Handler for POST /upload
 */
async function multipart18FileMagicNumberJpegSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppMultipart18FileMagicNumberJpegSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_18_file_magic_number_jpeg_success",
		request_schema: undefined,
		response_schema: undefined,
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

/**
 * Handler for POST /upload
 */
async function multipart22FileEmptyBuffer(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	response.body = result;
	return JSON.stringify(response);
}

export function createAppMultipart22FileEmptyBuffer(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_22_file_empty_buffer",
		request_schema: undefined,
		response_schema: undefined,
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

/**
 * Handler for POST /upload
 */
async function multipart17FileMagicNumberPngSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppMultipart17FileMagicNumberPngSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_17_file_magic_number_png_success",
		request_schema: undefined,
		response_schema: undefined,
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

/**
 * Handler for POST /
 */
async function multipartFormDataWithoutFiles(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { some: "data" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartFormDataWithoutFiles(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_form_data_without_files",
		request_schema: { additionalProperties: false, properties: { some: { type: "string" } }, type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_form_data_without_files: multipartFormDataWithoutFiles,
		},
	};
}

/**
 * Handler for POST /
 */
async function multipartMultipleFileUploads(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {
		test1: { content: "<file1 content>", content_type: "text/plain", filename: "test1.txt", size: 15 },
		test2: { content: "<file2 content>", content_type: "text/plain", filename: "test2.txt", size: 15 },
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartMultipleFileUploads(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_multiple_file_uploads",
		request_schema: {
			additionalProperties: false,
			properties: { test1: { format: "binary", type: "string" }, test2: { format: "binary", type: "string" } },
			required: ["test1", "test2"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_multiple_file_uploads: multipartMultipleFileUploads,
		},
	};
}

/**
 * Handler for POST /
 */
async function multipartFileUploadWithCustomHeaders(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
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

export function createAppMultipartFileUploadWithCustomHeaders(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_file_upload_with_custom_headers",
		request_schema: {
			additionalProperties: false,
			properties: { test2: { format: "binary", type: "string" } },
			required: ["test2"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_file_upload_with_custom_headers: multipartFileUploadWithCustomHeaders,
		},
	};
}

/**
 * Handler for POST /files/required
 */
async function multipartRequiredFileUploadMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppMultipartRequiredFileUploadMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/required",
		handler_name: "multipart_required_file_upload_missing",
		request_schema: {
			additionalProperties: false,
			properties: { file: { format: "binary", type: "string" } },
			required: ["file"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_required_file_upload_missing: multipartRequiredFileUploadMissing,
		},
	};
}

/**
 * Handler for POST /files/image
 */
async function multipartImageFileUpload(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { content_type: "image/jpeg", filename: "photo.jpg", size: 22 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartImageFileUpload(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/image",
		handler_name: "multipart_image_file_upload",
		request_schema: {
			additionalProperties: false,
			properties: { image: { format: "binary", type: "string" } },
			required: ["image"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_image_file_upload: multipartImageFileUpload,
		},
	};
}

/**
 * Handler for GET /rate-limit/basic
 */
async function rateLimitRateLimitBelowThresholdSucceeds(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { request: "under-limit", status: "ok" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppRateLimitRateLimitBelowThresholdSucceeds(): SpikardApp {
	const config: ServerConfig = {
		rateLimit: {
			perSecond: 5,
			burst: 5,
			ipBased: false,
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/rate-limit/basic",
		handler_name: "rate_limit_rate_limit_below_threshold_succeeds",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

/**
 * Handler for GET /rate-limit/exceeded
 */
async function rateLimitRateLimitExceededReturns429(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppRateLimitRateLimitExceededReturns429(): SpikardApp {
	const config: ServerConfig = {
		rateLimit: {
			perSecond: 1,
			burst: 1,
			ipBased: false,
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/rate-limit/exceeded",
		handler_name: "rate_limit_rate_limit_exceeded_returns_429",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

/**
 * Handler for GET /compression/skip
 */
async function compressionCompressionPayloadBelowMinSizeIsNotCompressed(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Small payload", payload: "tiny" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCompressionCompressionPayloadBelowMinSizeIsNotCompressed(): SpikardApp {
	const config: ServerConfig = {
		compression: {
			gzip: true,
			brotli: false,
			minSize: 4096,
			quality: 6,
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/compression/skip",
		handler_name: "compression_compression_payload_below_min_size_is_not_compressed",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

/**
 * Handler for GET /compression/gzip
 */
async function compressionCompressionGzipApplied(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { vary: "Accept-Encoding" };
	const responseBody = {
		message: "Compressed payload",
		payload: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCompressionCompressionGzipApplied(): SpikardApp {
	const config: ServerConfig = {
		compression: {
			gzip: true,
			brotli: false,
			minSize: 0,
			quality: 4,
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/compression/gzip",
		handler_name: "compression_compression_gzip_applied",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

/**
 * Handler for GET /api/protected
 */
async function authJwtMalformedTokenFormat(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {
		detail: "Malformed JWT token: expected 3 parts separated by dots, found 2",
		status: 401,
		title: "Malformed JWT token",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtMalformedTokenFormat(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_jwt_malformed_token_format",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /api/protected
 */
async function authBearerTokenWithoutPrefix(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {
		detail: "Authorization header must use Bearer scheme: 'Bearer <token>'",
		status: 401,
		title: "Invalid Authorization header format",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthBearerTokenWithoutPrefix(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_bearer_token_without_prefix",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /protected/user
 */
async function authJwtAuthenticationValidToken(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Access granted", user_id: "user123" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtAuthenticationValidToken(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
			audience: ["https://api.example.com"],
			issuer: "https://auth.example.com",
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_valid_token",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /api/data
 */
async function authApiKeyRotationOldKeyStillValid(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "x-api-key-deprecated": "true" };
	const responseBody = { data: "sensitive information", message: "Access granted" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthApiKeyRotationOldKeyStillValid(): SpikardApp {
	const config: ServerConfig = {
		apiKeyAuth: {
			keys: ["sk_test_old_123456", "sk_test_new_789012"],
			headerName: "X-API-Key",
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_rotation_old_key_still_valid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "X-API-Key": { description: "API key for authentication", source: "header", type: "string" } },
			required: ["X-API-Key"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /api/protected
 */
async function authJwtInvalidIssuer(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {
		detail: "Token issuer is invalid, expected 'https://auth.example.com'",
		status: 401,
		title: "JWT validation failed",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtInvalidIssuer(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
			issuer: "https://auth.example.com",
			leeway: 0,
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_jwt_invalid_issuer",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /api/protected
 */
async function authJwtWithMultipleAudiences(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Access granted", user_id: "user123" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtWithMultipleAudiences(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
			audience: ["https://api.example.com"],
			issuer: "https://auth.example.com",
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_jwt_with_multiple_audiences",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /api/data
 */
async function authApiKeyInQueryParameter(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { data: "sensitive information", message: "Access granted" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthApiKeyInQueryParameter(): SpikardApp {
	const config: ServerConfig = {
		apiKeyAuth: {
			keys: ["sk_test_123456", "sk_test_789012"],
			headerName: "X-API-Key",
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_in_query_parameter",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

/**
 * Handler for GET /protected/user
 */
async function authJwtAuthenticationExpiredToken(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {
		detail: "Token has expired",
		status: 401,
		title: "JWT validation failed",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtAuthenticationExpiredToken(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_expired_token",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /api/data
 */
async function authApiKeyAuthenticationInvalidKey(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {
		detail: "The provided API key is not valid",
		status: 401,
		title: "Invalid API key",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthApiKeyAuthenticationInvalidKey(): SpikardApp {
	const config: ServerConfig = {
		apiKeyAuth: {
			keys: ["sk_test_123456", "sk_test_789012"],
			headerName: "X-API-Key",
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_authentication_invalid_key",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "X-API-Key": { source: "header", type: "string" } },
			required: ["X-API-Key"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /api/protected
 */
async function authJwtNotBeforeClaimInFuture(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {
		detail: "JWT not valid yet, not before claim is in the future",
		status: 401,
		title: "JWT validation failed",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtNotBeforeClaimInFuture(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
			leeway: 0,
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_jwt_not_before_claim_in_future",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /api/data
 */
async function authMultipleAuthenticationSchemesJwtPrecedence(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { auth_method: "jwt", message: "Access granted", user_id: "user123" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthMultipleAuthenticationSchemesJwtPrecedence(): SpikardApp {
	const config: ServerConfig = {
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

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_multiple_authentication_schemes_jwt_precedence",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" },
				"X-API-Key": { description: "API key for authentication", source: "header", type: "string" },
			},
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /api/admin
 */
async function authJwtMissingRequiredCustomClaims(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 403 };
	const responseBody = {
		detail: "Required claims 'role' and 'permissions' missing from JWT",
		status: 403,
		title: "Forbidden",
		type: "https://spikard.dev/errors/forbidden",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtMissingRequiredCustomClaims(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
			audience: ["https://api.example.com"],
			issuer: "https://auth.example.com",
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/admin",
		handler_name: "auth_jwt_missing_required_custom_claims",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { description: "JWT token in Bearer format", source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /api/data
 */
async function authApiKeyAuthenticationValidKey(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { data: "sensitive information", message: "Access granted" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthApiKeyAuthenticationValidKey(): SpikardApp {
	const config: ServerConfig = {
		apiKeyAuth: {
			keys: ["sk_test_123456", "sk_test_789012"],
			headerName: "X-API-Key",
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_authentication_valid_key",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "X-API-Key": { description: "API key for authentication", source: "header", type: "string" } },
			required: ["X-API-Key"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /api/data
 */
async function authApiKeyWithCustomHeaderName(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { data: "sensitive information", message: "Access granted" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthApiKeyWithCustomHeaderName(): SpikardApp {
	const config: ServerConfig = {
		apiKeyAuth: {
			keys: ["sk_test_123456", "sk_test_789012"],
			headerName: "X-API-Token",
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_with_custom_header_name",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { "X-API-Token": { description: "API token for authentication", source: "header", type: "string" } },
			required: ["X-API-Token"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /api/data
 */
async function authApiKeyAuthenticationMissingHeader(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {
		detail: "Expected 'X-API-Key' header or 'api_key' query parameter with valid API key",
		status: 401,
		title: "Missing API key",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthApiKeyAuthenticationMissingHeader(): SpikardApp {
	const config: ServerConfig = {
		apiKeyAuth: {
			keys: ["sk_test_123456", "sk_test_789012"],
			headerName: "X-API-Key",
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_authentication_missing_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: {}, type: "object" },
		file_params: undefined,
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

/**
 * Handler for GET /protected/user
 */
async function authJwtAuthenticationInvalidSignature(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {
		detail: "Token signature is invalid",
		status: 401,
		title: "JWT validation failed",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtAuthenticationInvalidSignature(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_invalid_signature",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /protected/user
 */
async function authJwtAuthenticationMissingAuthorizationHeader(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {
		detail: "Expected 'Authorization: Bearer <token>'",
		status: 401,
		title: "Missing or invalid Authorization header",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtAuthenticationMissingAuthorizationHeader(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_missing_authorization_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: {}, type: "object" },
		file_params: undefined,
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

/**
 * Handler for GET /protected/user
 */
async function authJwtAuthenticationInvalidAudience(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {
		detail: "Token audience is invalid",
		status: 401,
		title: "JWT validation failed",
		type: "https://spikard.dev/errors/unauthorized",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtAuthenticationInvalidAudience(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
			secret: "test-secret-key-do-not-use-in-production",
			algorithm: "HS256",
			audience: ["https://api.example.com"],
		},
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_invalid_audience",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { Authorization: { source: "header", type: "string" } },
			required: ["Authorization"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /path/bool/{item_id}
 */
async function pathParamsBooleanPathParameterTrue(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item_id: true };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsBooleanPathParameterTrue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/bool/{item_id}",
		handler_name: "path_params_boolean_path_parameter_true",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { source: "path", type: "boolean" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_boolean_path_parameter_true: pathParamsBooleanPathParameterTrue,
		},
	};
}

/**
 * Handler for GET /prices/{amount}
 */
async function pathParams29DecimalPathParamSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { amount: "19.99" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParams29DecimalPathParamSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/prices/{amount}",
		handler_name: "path_params_29_decimal_path_param_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { amount: { format: "decimal", source: "path", type: "string" } },
			required: ["amount"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_29_decimal_path_param_success: pathParams29DecimalPathParamSuccess,
		},
	};
}

/**
 * Handler for GET /path/param-lt-gt/{item_id}
 */
async function pathParamsIntegerPathParameterWithCombinedLtAndGtConstraintsSuccess(
	requestJson: string,
): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item_id: 2 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsIntegerPathParameterWithCombinedLtAndGtConstraintsSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-lt-gt/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { exclusiveMaximum: 3, exclusiveMinimum: 1, source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /repos/{owner}/{repo}
 */
async function pathParams33StringPatternPathSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { owner: "spikard-labs", repo: "spikard-http" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParams33StringPatternPathSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/repos/{owner}/{repo}",
		handler_name: "path_params_33_string_pattern_path_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				owner: { pattern: "^[a-zA-Z0-9-]+$", source: "path", type: "string" },
				repo: { pattern: "^[a-zA-Z0-9-_]+$", source: "path", type: "string" },
			},
			required: ["owner", "repo"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_33_string_pattern_path_success: pathParams33StringPatternPathSuccess,
		},
	};
}

/**
 * Handler for GET /users/{username}
 */
async function pathParams31StringMinlengthPathFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const username = params.username;
	if (username !== null && username !== undefined) {
		result.username = username;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppPathParams31StringMinlengthPathFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/{username}",
		handler_name: "path_params_31_string_minlength_path_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { username: { minLength: 3, source: "path", type: "string" } },
			required: ["username"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_31_string_minlength_path_failure: pathParams31StringMinlengthPathFailure,
		},
	};
}

/**
 * Handler for GET /offset/{value}
 */
async function pathParams35NegativeIntegerPathParam(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { value: -100 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParams35NegativeIntegerPathParam(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/offset/{value}",
		handler_name: "path_params_35_negative_integer_path_param",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { value: { source: "path", type: "integer" } },
			required: ["value"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_35_negative_integer_path_param: pathParams35NegativeIntegerPathParam,
		},
	};
}

/**
 * Handler for GET /models/{model_name}
 */
async function pathParamsEnumPathParameterInvalidValue(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const modelName = params.model_name;
	if (modelName !== null && modelName !== undefined) {
		result.model_name = modelName;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppPathParamsEnumPathParameterInvalidValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/models/{model_name}",
		handler_name: "path_params_enum_path_parameter_invalid_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { model_name: { enum: ["alexnet", "resnet", "lenet"], source: "path", type: "string" } },
			required: ["model_name"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_enum_path_parameter_invalid_value: pathParamsEnumPathParameterInvalidValue,
		},
	};
}

/**
 * Handler for GET /bookings/{timestamp}
 */
async function pathParams27DatetimeFormatPathParamSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { timestamp: "2025-10-30T14:30:00Z" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParams27DatetimeFormatPathParamSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/bookings/{timestamp}",
		handler_name: "path_params_27_datetime_format_path_param_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { timestamp: { format: "date-time", source: "path", type: "string" } },
			required: ["timestamp"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_27_datetime_format_path_param_success: pathParams27DatetimeFormatPathParamSuccess,
		},
	};
}

/**
 * Handler for GET /events/{date}
 */
async function pathParams25DateFormatInvalidFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const date = params.date;
	if (date !== null && date !== undefined) {
		result.date = date.toISOString();
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppPathParams25DateFormatInvalidFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/events/{date}",
		handler_name: "path_params_25_date_format_invalid_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { date: { format: "date", source: "path", type: "string" } },
			required: ["date"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_25_date_format_invalid_failure: pathParams25DateFormatInvalidFailure,
		},
	};
}

/**
 * Handler for GET /path/param-lt/{item_id}
 */
async function pathParamsIntegerPathParameterWithLtConstraintSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item_id: 2 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsIntegerPathParameterWithLtConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-lt/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_lt_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { exclusiveMaximum: 3, source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /path/param-gt/{item_id}
 */
async function pathParamsIntegerPathParameterWithGtConstraintSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item_id: 42 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsIntegerPathParameterWithGtConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-gt/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_gt_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { exclusiveMinimum: 3, source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /delays/{duration}
 */
async function pathParams28DurationFormatPathParamSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { duration: "P1DT2H30M" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParams28DurationFormatPathParamSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/delays/{duration}",
		handler_name: "path_params_28_duration_format_path_param_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { duration: { format: "duration", source: "path", type: "string" } },
			required: ["duration"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_28_duration_format_path_param_success: pathParams28DurationFormatPathParamSuccess,
		},
	};
}

/**
 * Handler for GET /type-syntax/items-count/{count:int}
 */
async function pathParamsPathParameterTypeSyntaxWithOverride(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { count: "50" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsPathParameterTypeSyntaxWithOverride(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/type-syntax/items-count/{count:int}",
		handler_name: "path_params_path_parameter_type_syntax_with_override",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { count: { maximum: 100, minimum: 1, source: "path", type: "integer" } },
			required: ["count"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_path_parameter_type_syntax_with_override: pathParamsPathParameterTypeSyntaxWithOverride,
		},
	};
}

/**
 * Handler for GET /items/{id}
 */
async function pathParams20UuidV3PathParamSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { id: "e8b5a51d-11c8-3310-a6ab-367563f20686" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParams20UuidV3PathParamSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/{id}",
		handler_name: "path_params_20_uuid_v3_path_param_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { id: { format: "uuid", source: "path", type: "string", uuidVersion: "3" } },
			required: ["id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_20_uuid_v3_path_param_success: pathParams20UuidV3PathParamSuccess,
		},
	};
}

/**
 * Handler for GET /path/int/{item_id}
 */
async function pathParamsIntegerPathParameterInvalidString(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const itemId = params.item_id;
	if (itemId !== null && itemId !== undefined) {
		result.item_id = itemId;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppPathParamsIntegerPathParameterInvalidString(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/int/{item_id}",
		handler_name: "path_params_integer_path_parameter_invalid_string",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_invalid_string: pathParamsIntegerPathParameterInvalidString,
		},
	};
}

/**
 * Handler for GET /users/{username}
 */
async function pathParams30StringMinlengthPathSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { username: "alice" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParams30StringMinlengthPathSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/{username}",
		handler_name: "path_params_30_string_minlength_path_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { username: { minLength: 3, source: "path", type: "string" } },
			required: ["username"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_30_string_minlength_path_success: pathParams30StringMinlengthPathSuccess,
		},
	};
}

/**
 * Handler for GET /path/param-le/{item_id}
 */
async function pathParamsIntegerPathParameterWithLeConstraintSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item_id: 3 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsIntegerPathParameterWithLeConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-le/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_le_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { maximum: 3, source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /type-syntax/items/{id:uuid}
 */
async function pathParamsPathParameterTypeSyntaxInvalidUuid(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	response.body = result;
	return JSON.stringify(response);
}

export function createAppPathParamsPathParameterTypeSyntaxInvalidUuid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/type-syntax/items/{id:uuid}",
		handler_name: "path_params_path_parameter_type_syntax_invalid_uuid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_path_parameter_type_syntax_invalid_uuid: pathParamsPathParameterTypeSyntaxInvalidUuid,
		},
	};
}

/**
 * Handler for GET /files/{file_path:path}
 */
async function pathParamsPathTypeParameterFilePath(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { file_path: "home/johndoe/myfile.txt" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsPathTypeParameterFilePath(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/files/{file_path:path}",
		handler_name: "path_params_path_type_parameter_file_path",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { file_path: { source: "path", type: "string" } },
			required: ["file_path"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_path_type_parameter_file_path: pathParamsPathTypeParameterFilePath,
		},
	};
}

/**
 * Handler for GET /type-syntax/items/{id:uuid}
 */
async function pathParamsPathParameterWithTypeSyntaxUuid(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { id: "550e8400-e29b-41d4-a716-446655440000" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsPathParameterWithTypeSyntaxUuid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/type-syntax/items/{id:uuid}",
		handler_name: "path_params_path_parameter_with_type_syntax_uuid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_path_parameter_with_type_syntax_uuid: pathParamsPathParameterWithTypeSyntaxUuid,
		},
	};
}

/**
 * Handler for GET /users/{username}
 */
async function pathParams32StringMaxlengthPathFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const username = params.username;
	if (username !== null && username !== undefined) {
		result.username = username;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppPathParams32StringMaxlengthPathFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/{username}",
		handler_name: "path_params_32_string_maxlength_path_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { username: { maxLength: 20, source: "path", type: "string" } },
			required: ["username"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_32_string_maxlength_path_failure: pathParams32StringMaxlengthPathFailure,
		},
	};
}

/**
 * Handler for GET /path/int/{item_id}
 */
async function pathParamsIntegerPathParameterSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item_id: 42 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsIntegerPathParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/int/{item_id}",
		handler_name: "path_params_integer_path_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_success: pathParamsIntegerPathParameterSuccess,
		},
	};
}

/**
 * Handler for GET /repos/{owner}
 */
async function pathParams34StringPatternPathFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const owner = params.owner;
	if (owner !== null && owner !== undefined) {
		result.owner = owner;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppPathParams34StringPatternPathFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/repos/{owner}",
		handler_name: "path_params_34_string_pattern_path_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { owner: { pattern: "^[a-zA-Z0-9-]+$", source: "path", type: "string" } },
			required: ["owner"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_34_string_pattern_path_failure: pathParams34StringPatternPathFailure,
		},
	};
}

/**
 * Handler for GET /items/{id}
 */
async function pathParams21UuidV5PathParamSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { id: "630eb68f-e0fa-5ecc-887a-7c7a62614681" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParams21UuidV5PathParamSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/{id}",
		handler_name: "path_params_21_uuid_v5_path_param_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { id: { format: "uuid", source: "path", type: "string", uuidVersion: "5" } },
			required: ["id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_21_uuid_v5_path_param_success: pathParams21UuidV5PathParamSuccess,
		},
	};
}

/**
 * Handler for GET /path/param-maxlength/{item_id}
 */
async function pathParamsStringPathParameterWithMaxLengthFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const itemId = params.item_id;
	if (itemId !== null && itemId !== undefined) {
		result.item_id = itemId;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppPathParamsStringPathParameterWithMaxLengthFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-maxlength/{item_id}",
		handler_name: "path_params_string_path_parameter_with_max_length_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { maxLength: 3, source: "path", type: "string" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_string_path_parameter_with_max_length_failure: pathParamsStringPathParameterWithMaxLengthFailure,
		},
	};
}

/**
 * Handler for GET /path/param-minlength/{item_id}
 */
async function pathParamsStringPathParameterWithMinLengthFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const itemId = params.item_id;
	if (itemId !== null && itemId !== undefined) {
		result.item_id = itemId;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppPathParamsStringPathParameterWithMinLengthFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-minlength/{item_id}",
		handler_name: "path_params_string_path_parameter_with_min_length_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { minLength: 3, source: "path", type: "string" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_string_path_parameter_with_min_length_failure: pathParamsStringPathParameterWithMinLengthFailure,
		},
	};
}

/**
 * Handler for GET /{version}/{service_id}/{user_id}/{order_id}
 */
async function pathParamsMultiplePathParametersSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {
		order_id: "c892496f-b1fd-4b91-bdb8-b46f92df1716",
		service_id: 1,
		user_id: "abc",
		version: 1.0,
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsMultiplePathParametersSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/{version}/{service_id}/{user_id}/{order_id}",
		handler_name: "path_params_multiple_path_parameters_success",
		request_schema: undefined,
		response_schema: undefined,
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
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_multiple_path_parameters_success: pathParamsMultiplePathParametersSuccess,
		},
	};
}

/**
 * Handler for GET /date/{date_param}
 */
async function pathParamsDatePathParameterSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { date_param: "2023-07-15" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsDatePathParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/date/{date_param}",
		handler_name: "path_params_date_path_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { date_param: { format: "date", source: "path", type: "string" } },
			required: ["date_param"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_date_path_parameter_success: pathParamsDatePathParameterSuccess,
		},
	};
}

/**
 * Handler for GET /path/param-gt/{item_id}
 */
async function pathParamsIntegerPathParameterWithGtConstraintFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const itemId = params.item_id;
	if (itemId !== null && itemId !== undefined) {
		result.item_id = itemId;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppPathParamsIntegerPathParameterWithGtConstraintFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-gt/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_gt_constraint_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { exclusiveMinimum: 3, source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /events/{date}
 */
async function pathParams24DateFormatPathParamSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { date: "2025-10-30" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParams24DateFormatPathParamSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/events/{date}",
		handler_name: "path_params_24_date_format_path_param_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { date: { format: "date", source: "path", type: "string" } },
			required: ["date"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_24_date_format_path_param_success: pathParams24DateFormatPathParamSuccess,
		},
	};
}

/**
 * Handler for GET /path/float/{item_id}
 */
async function pathParamsFloatPathParameterSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item_id: 42.5 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsFloatPathParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/float/{item_id}",
		handler_name: "path_params_float_path_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { source: "path", type: "number" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_float_path_parameter_success: pathParamsFloatPathParameterSuccess,
		},
	};
}

/**
 * Handler for GET /type-syntax/users/{user_id:int}
 */
async function pathParamsPathParameterWithTypeSyntaxInteger(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { user_id: "42" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsPathParameterWithTypeSyntaxInteger(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/type-syntax/users/{user_id:int}",
		handler_name: "path_params_path_parameter_with_type_syntax_integer",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_path_parameter_with_type_syntax_integer: pathParamsPathParameterWithTypeSyntaxInteger,
		},
	};
}

/**
 * Handler for GET /path/str/{item_id}
 */
async function pathParamsStringPathParameterSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item_id: "foobar" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsStringPathParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/str/{item_id}",
		handler_name: "path_params_string_path_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { source: "path", type: "string" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_string_path_parameter_success: pathParamsStringPathParameterSuccess,
		},
	};
}

/**
 * Handler for GET /items/{item_id}
 */
async function pathParamsUuidPathParameterSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item_id: "ec38df32-ceda-4cfa-9b4a-1aeb94ad551a" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsUuidPathParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/{item_id}",
		handler_name: "path_params_uuid_path_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { format: "uuid", source: "path", type: "string" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_uuid_path_parameter_success: pathParamsUuidPathParameterSuccess,
		},
	};
}

/**
 * Handler for GET /path/param-ge/{item_id}
 */
async function pathParamsIntegerPathParameterWithGeConstraintSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item_id: 3 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsIntegerPathParameterWithGeConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-ge/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_ge_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { minimum: 3, source: "path", type: "integer" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /models/{model_name}
 */
async function pathParamsEnumPathParameterSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { model_name: "alexnet" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsEnumPathParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/models/{model_name}",
		handler_name: "path_params_enum_path_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { model_name: { enum: ["alexnet", "lenet", "resnet"], source: "path", type: "string" } },
			required: ["model_name"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_enum_path_parameter_success: pathParamsEnumPathParameterSuccess,
		},
	};
}

/**
 * Handler for GET /path/bool/{item_id}
 */
async function pathParamsBooleanPathParameterNumeric1(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item_id: true };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppPathParamsBooleanPathParameterNumeric1(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/bool/{item_id}",
		handler_name: "path_params_boolean_path_parameter_numeric_1",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { source: "path", type: "boolean" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_boolean_path_parameter_numeric_1: pathParamsBooleanPathParameterNumeric1,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function contentTypes415UnsupportedMediaType(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 415 };
	const responseBody = { detail: "Unsupported media type" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes415UnsupportedMediaType(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "content_types_415_unsupported_media_type",
		request_schema: { type: "string" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_415_unsupported_media_type: contentTypes415UnsupportedMediaType,
		},
	};
}

/**
 * Handler for GET /xml
 */
async function contentTypesXmlResponseApplicationXml(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/xml" };
	const responseBody = '<?xml version="1.0"?><item><name>Item</name><price>42.0</price></item>';
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypesXmlResponseApplicationXml(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/xml",
		handler_name: "content_types_xml_response_application_xml",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_xml_response_application_xml: contentTypesXmlResponseApplicationXml,
		},
	};
}

/**
 * Handler for POST /data
 */
async function contentTypes14ContentTypeCaseInsensitive(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = { name: "test" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes14ContentTypeCaseInsensitive(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_14_content_type_case_insensitive",
		request_schema: { properties: { name: { type: "string" } }, required: ["name"], type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_14_content_type_case_insensitive: contentTypes14ContentTypeCaseInsensitive,
		},
	};
}

/**
 * Handler for GET /items/unicode
 */
async function contentTypesJsonWithUtf8Charset(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/json; charset=utf-8" };
	const responseBody = { emoji: "☕", name: "Café" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypesJsonWithUtf8Charset(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/unicode",
		handler_name: "content_types_json_with_utf_8_charset",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_json_with_utf_8_charset: contentTypesJsonWithUtf8Charset,
		},
	};
}

/**
 * Handler for POST /data
 */
async function contentTypes16TextPlainNotAccepted(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 415 };
	const responseBody = {
		detail: "Unsupported media type",
		status: 415,
		title: "Unsupported Media Type",
		type: "https://spikard.dev/errors/unsupported-media-type",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes16TextPlainNotAccepted(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_16_text_plain_not_accepted",
		request_schema: { properties: { data: { type: "string" } }, required: ["data"], type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_16_text_plain_not_accepted: contentTypes16TextPlainNotAccepted,
		},
	};
}

/**
 * Handler for GET /download/document.pdf
 */
async function contentTypesPdfResponseApplicationPdf(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-disposition": "attachment; filename=document.pdf", "content-type": "application/pdf" };
	const responseBody = "pdf_binary_data";
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypesPdfResponseApplicationPdf(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/download/document.pdf",
		handler_name: "content_types_pdf_response_application_pdf",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_pdf_response_application_pdf: contentTypesPdfResponseApplicationPdf,
		},
	};
}

/**
 * Handler for POST /data
 */
async function contentTypes20ContentLengthMismatch(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 400 };
	const responseBody = { error: "Content-Length header does not match actual body size" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes20ContentLengthMismatch(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_20_content_length_mismatch",
		request_schema: { properties: { value: { type: "string" } }, type: "object" },
		response_schema: undefined,
		parameter_schema: { properties: { "Content-Length": { source: "header", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_20_content_length_mismatch: contentTypes20ContentLengthMismatch,
		},
	};
}

/**
 * Handler for POST /api/v1/resource
 */
async function contentTypes17VendorJsonAccepted(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = { data: "value" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes17VendorJsonAccepted(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/api/v1/resource",
		handler_name: "content_types_17_vendor_json_accepted",
		request_schema: { properties: { data: { type: "string" } }, required: ["data"], type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_17_vendor_json_accepted: contentTypes17VendorJsonAccepted,
		},
	};
}

/**
 * Handler for POST /data
 */
async function contentTypes13JsonWithCharsetUtf16(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 415 };
	const responseBody = {
		detail: "Unsupported charset 'utf-16' for JSON. Only UTF-8 is supported.",
		status: 415,
		title: "Unsupported Charset",
		type: "https://spikard.dev/errors/unsupported-charset",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes13JsonWithCharsetUtf16(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_13_json_with_charset_utf16",
		request_schema: { properties: { value: { type: "string" } }, type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_13_json_with_charset_utf16: contentTypes13JsonWithCharsetUtf16,
		},
	};
}

/**
 * Handler for GET /items/json
 */
async function contentTypesJsonResponseApplicationJson(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/json" };
	const responseBody = { name: "Item", price: 42.0 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypesJsonResponseApplicationJson(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/json",
		handler_name: "content_types_json_response_application_json",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_json_response_application_json: contentTypesJsonResponseApplicationJson,
		},
	};
}

/**
 * Handler for POST /upload
 */
async function contentTypes15MultipartBoundaryRequired(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 400 };
	const responseBody = { error: "multipart/form-data requires 'boundary' parameter" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes15MultipartBoundaryRequired(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "content_types_15_multipart_boundary_required",
		request_schema: undefined,
		response_schema: undefined,
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

/**
 * Handler for GET /accept-test/{id}
 */
async function contentTypesContentNegotiationAcceptHeader(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/json" };
	const responseBody = { id: 1, name: "Item" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypesContentNegotiationAcceptHeader(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/accept-test/{id}",
		handler_name: "content_types_content_negotiation_accept_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_content_negotiation_accept_header: contentTypesContentNegotiationAcceptHeader,
		},
	};
}

/**
 * Handler for GET /html
 */
async function contentTypesHtmlResponseTextHtml(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "text/html; charset=utf-8" };
	const responseBody = "<html><body><h1>Hello</h1></body></html>";
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypesHtmlResponseTextHtml(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/html",
		handler_name: "content_types_html_response_text_html",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_html_response_text_html: contentTypesHtmlResponseTextHtml,
		},
	};
}

/**
 * Handler for GET /images/photo.jpg
 */
async function contentTypesJpegImageResponseImageJpeg(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "image/jpeg" };
	const responseBody = "jpeg_binary_data";
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypesJpegImageResponseImageJpeg(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/images/photo.jpg",
		handler_name: "content_types_jpeg_image_response_image_jpeg",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_jpeg_image_response_image_jpeg: contentTypesJpegImageResponseImageJpeg,
		},
	};
}

/**
 * Handler for POST /data
 */
async function contentTypes19MissingContentTypeDefaultJson(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = { name: "test" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes19MissingContentTypeDefaultJson(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_19_missing_content_type_default_json",
		request_schema: { properties: { name: { type: "string" } }, required: ["name"], type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_19_missing_content_type_default_json: contentTypes19MissingContentTypeDefaultJson,
		},
	};
}

/**
 * Handler for GET /images/logo.png
 */
async function contentTypesPngImageResponseImagePng(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "image/png" };
	const responseBody = "png_binary_data";
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypesPngImageResponseImagePng(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/images/logo.png",
		handler_name: "content_types_png_image_response_image_png",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_png_image_response_image_png: contentTypesPngImageResponseImagePng,
		},
	};
}

/**
 * Handler for GET /text
 */
async function contentTypesPlainTextResponseTextPlain(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "text/plain; charset=utf-8" };
	const responseBody = "Hello, World!";
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypesPlainTextResponseTextPlain(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/text",
		handler_name: "content_types_plain_text_response_text_plain",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_plain_text_response_text_plain: contentTypesPlainTextResponseTextPlain,
		},
	};
}

/**
 * Handler for POST /data
 */
async function contentTypes18ContentTypeWithMultipleParams(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = { value: "test" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes18ContentTypeWithMultipleParams(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_18_content_type_with_multiple_params",
		request_schema: { properties: { value: { type: "string" } }, type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_18_content_type_with_multiple_params: contentTypes18ContentTypeWithMultipleParams,
		},
	};
}

/**
 * Handler for GET /export/data.csv
 */
async function contentTypesCsvResponseTextCsv(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {
		"content-disposition": "attachment; filename=data.csv",
		"content-type": "text/csv; charset=utf-8",
	};
	const responseBody = "id,name,price\n1,Item A,10.0\n2,Item B,20.0";
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypesCsvResponseTextCsv(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/export/data.csv",
		handler_name: "content_types_csv_response_text_csv",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_csv_response_text_csv: contentTypesCsvResponseTextCsv,
		},
	};
}

/**
 * Handler for GET /download/file.bin
 */
async function contentTypesBinaryResponseApplicationOctetStream(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {
		"content-disposition": "attachment; filename=file.bin",
		"content-type": "application/octet-stream",
	};
	const responseBody = "binary_data_placeholder";
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypesBinaryResponseApplicationOctetStream(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/download/file.bin",
		handler_name: "content_types_binary_response_application_octet_stream",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_binary_response_application_octet_stream: contentTypesBinaryResponseApplicationOctetStream,
		},
	};
}

/**
 * Handler for POST /background/events
 */
async function backgroundBackgroundEventLoggingSecondPayload(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 202 };
	response.headers = { "content-type": "application/json" };
	BACKGROUND_STATE.background_background_event_logging_second_payload =
		BACKGROUND_STATE.background_background_event_logging_second_payload ?? [];
	const state = BACKGROUND_STATE.background_background_event_logging_second_payload as unknown[];
	const value = body && typeof body === "object" ? body.event : undefined;
	if (value === undefined || value === null) {
		throw new Error("background task requires request body value");
	}
	void Promise.resolve().then(() => void state.push(value));
	response.body = null;
	return JSON.stringify(response);
}

async function backgroundBackgroundEventLoggingSecondPayloadBackgroundState(): Promise<string> {
	const state = BACKGROUND_STATE.background_background_event_logging_second_payload ?? [];
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/json" };
	response.body = { events: state };
	return JSON.stringify(response);
}

export function createAppBackgroundBackgroundEventLoggingSecondPayload(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/background/events",
		handler_name: "background_background_event_logging_second_payload",
		request_schema: {
			additionalProperties: false,
			properties: { event: { type: "string" } },
			required: ["event"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	const backgroundRoute: RouteMetadata = {
		method: "GET",
		path: "/background/events",
		handler_name: "background_background_event_logging_second_payload_background_state",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

/**
 * Handler for POST /background/events
 */
async function backgroundBackgroundEventLogging(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 202 };
	response.headers = { "content-type": "application/json" };
	BACKGROUND_STATE.background_background_event_logging = BACKGROUND_STATE.background_background_event_logging ?? [];
	const state = BACKGROUND_STATE.background_background_event_logging as unknown[];
	const value = body && typeof body === "object" ? body.event : undefined;
	if (value === undefined || value === null) {
		throw new Error("background task requires request body value");
	}
	void Promise.resolve().then(() => void state.push(value));
	response.body = null;
	return JSON.stringify(response);
}

async function backgroundBackgroundEventLoggingBackgroundState(): Promise<string> {
	const state = BACKGROUND_STATE.background_background_event_logging ?? [];
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/json" };
	response.body = { events: state };
	return JSON.stringify(response);
}

export function createAppBackgroundBackgroundEventLogging(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/background/events",
		handler_name: "background_background_event_logging",
		request_schema: {
			additionalProperties: false,
			properties: { event: { type: "string" } },
			required: ["event"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	const backgroundRoute: RouteMetadata = {
		method: "GET",
		path: "/background/events",
		handler_name: "background_background_event_logging_background_state",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

/**
 * Handler for GET /stream/json-lines
 */
async function streamingStreamJsonLines(_requestJson: string): Promise<StreamingResponse> {
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

export function createAppStreamingStreamJsonLines(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/stream/json-lines",
		handler_name: "streaming_stream_json_lines",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			streaming_stream_json_lines: streamingStreamJsonLines,
		},
	};
}

/**
 * Handler for GET /stream/logfile
 */
async function streamingBinaryLogDownload(_requestJson: string): Promise<StreamingResponse> {
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

export function createAppStreamingBinaryLogDownload(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/stream/logfile",
		handler_name: "streaming_binary_log_download",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			streaming_binary_log_download: streamingBinaryLogDownload,
		},
	};
}

/**
 * Handler for GET /stream/csv-report
 */
async function streamingChunkedCsvExport(_requestJson: string): Promise<StreamingResponse> {
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

export function createAppStreamingChunkedCsvExport(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/stream/csv-report",
		handler_name: "streaming_chunked_csv_export",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			streaming_chunked_csv_export: streamingChunkedCsvExport,
		},
	};
}

/**
 * Handler for GET /api/override-test
 */
async function diRouteLevelDependencyOverrideSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const api_key_validator = request.dependencies?.api_key_validator ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = { mode: "test", strict: false };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiRouteLevelDependencyOverrideSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("apiKeyValidator", { mode: "test", strict: false });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/override-test",
		handler_name: "di_route_level_dependency_override_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
		di_route_level_dependency_override_success: diRouteLevelDependencyOverrideSuccess,
	};
	return app;
}

function diCircularDependencyDetectionErrorServiceA(serviceB): unknown {
	// Factory for service_a
	return { _factory: "service_a", _random: Math.random() };
}

function diCircularDependencyDetectionErrorServiceB(serviceA): unknown {
	// Factory for service_b
	return { _factory: "service_b", _random: Math.random() };
}

/**
 * Handler for GET /api/circular
 */
async function diCircularDependencyDetectionError(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const service_a = request.dependencies?.service_a ?? null;
	const response: HandlerResponse = { status: 500 };
	const responseBody = {
		detail: "Circular dependency detected",
		errors: [
			{
				cycle: ["service_a", "service_b", "service_a"],
				msg: "Circular dependency detected in dependency graph",
				type: "circular_dependency",
			},
		],
		status: 500,
		title: "Dependency Resolution Failed",
		type: "https://spikard.dev/errors/dependency-error",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiCircularDependencyDetectionError(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/circular",
		handler_name: "di_circular_dependency_detection_error",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			di_circular_dependency_detection_error: diCircularDependencyDetectionError,
		},
	};
}

function diFactoryDependencySuccessTimestampGenerator(): unknown {
	// Factory for timestamp_generator
	return { _factory: "timestamp_generator", _random: Math.random() };
}

/**
 * Handler for GET /api/timestamp
 */
async function diFactoryDependencySuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const timestamp_generator = request.dependencies?.timestamp_generator ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = { timestamp: "<<present>>" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiFactoryDependencySuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("timestampGenerator", diFactoryDependencySuccessTimestampGenerator);

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/timestamp",
		handler_name: "di_factory_dependency_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
		di_factory_dependency_success: diFactoryDependencySuccess,
	};
	return app;
}

/**
 * Handler for GET /api/config
 */
async function diValueDependencyInjectionSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const app_name = request.dependencies?.app_name ?? null;
	const version = request.dependencies?.version ?? null;
	const max_connections = request.dependencies?.max_connections ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = { app_name: "SpikardApp", max_connections: 100, version: "1.0.0" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiValueDependencyInjectionSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("appName", "SpikardApp");
	app.provide("maxConnections", 100);
	app.provide("version", "1.0.0");

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/config",
		handler_name: "di_value_dependency_injection_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
		di_value_dependency_injection_success: diValueDependencyInjectionSuccess,
	};
	return app;
}

/**
 * Handler for GET /api/node-destructure
 */
async function diNodeJsObjectDestructuringInjectionSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const db = request.dependencies?.db ?? null;
	const logger = request.dependencies?.logger ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = { db_name: "PostgreSQL", log_level: "info" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiNodeJsObjectDestructuringInjectionSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("db", { connected: true, name: "PostgreSQL" });
	app.provide("logger", { enabled: true, level: "info" });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/node-destructure",
		handler_name: "di_node_js_object_destructuring_injection_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
		di_node_js_object_destructuring_injection_success: diNodeJsObjectDestructuringInjectionSuccess,
	};
	return app;
}

async function diNestedDependencies3LevelsSuccessDbPool(config): Promise<unknown> {
	// Async factory for db_pool
	// Simulate async DB connection
	return { connected: true, poolId: Math.random().toString() };
}

function diNestedDependencies3LevelsSuccessAuthService(dbPool, cache): unknown {
	// Factory for auth_service
	return { _factory: "auth_service", _random: Math.random() };
}

async function diNestedDependencies3LevelsSuccessCache(config): Promise<unknown> {
	// Async factory for cache
	// Simulate async cache connection
	return { ready: true, cacheId: Math.random().toString() };
}

/**
 * Handler for GET /api/auth-status
 */
async function diNestedDependencies3LevelsSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const auth_service = request.dependencies?.auth_service ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = { auth_enabled: true, has_cache: true, has_db: true };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiNestedDependencies3LevelsSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("config", { cache_ttl: 300, db_url: "postgresql://localhost/mydb" });
	app.provide("cache", diNestedDependencies3LevelsSuccessCache, { cacheable: true });
	app.provide("dbPool", diNestedDependencies3LevelsSuccessDbPool, { cacheable: true });
	app.provide("authService", diNestedDependencies3LevelsSuccessAuthService, { cacheable: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/auth-status",
		handler_name: "di_nested_dependencies_3_levels_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
		di_nested_dependencies_3_levels_success: diNestedDependencies3LevelsSuccess,
	};
	return app;
}

/**
 * Handler for GET /api/type-mismatch
 */
async function diTypeMismatchInDependencyResolutionError(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const config = request.dependencies?.config ?? null;
	const response: HandlerResponse = { status: 500 };
	const responseBody = {
		detail: "Dependency type mismatch",
		errors: [
			{
				actual_type: "string",
				dependency_key: "config",
				expected_type: "object",
				msg: "Dependency 'config' type mismatch: expected object, got string",
				type: "type_mismatch",
			},
		],
		status: 500,
		title: "Dependency Resolution Failed",
		type: "https://spikard.dev/errors/dependency-error",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiTypeMismatchInDependencyResolutionError(): SpikardApp {
	const app = new Spikard();

	app.provide("config", "string_config");

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/type-mismatch",
		handler_name: "di_type_mismatch_in_dependency_resolution_error",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
		di_type_mismatch_in_dependency_resolution_error: diTypeMismatchInDependencyResolutionError,
	};
	return app;
}

/**
 * Handler for GET /api/missing-dep
 */
async function diMissingDependencyError(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const non_existent_service = request.dependencies?.non_existent_service ?? null;
	const response: HandlerResponse = { status: 500 };
	const responseBody = {
		detail: "Required dependency not found",
		errors: [
			{
				dependency_key: "non_existent_service",
				msg: "Dependency 'non_existent_service' is not registered",
				type: "missing_dependency",
			},
		],
		status: 500,
		title: "Dependency Resolution Failed",
		type: "https://spikard.dev/errors/dependency-error",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiMissingDependencyError(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/missing-dep",
		handler_name: "di_missing_dependency_error",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			di_missing_dependency_error: diMissingDependencyError,
		},
	};
}

/**
 * Handler for GET /api/python-name-inject
 */
async function diPythonParameterNameBasedInjectionSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const db_pool = request.dependencies?.db_pool ?? null;
	const cache = request.dependencies?.cache ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = { cache_status: "ready", db_status: "connected" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiPythonParameterNameBasedInjectionSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("cache", { status: "ready" });
	app.provide("dbPool", { status: "connected" });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/python-name-inject",
		handler_name: "di_python_parameter_name_based_injection_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
		di_python_parameter_name_based_injection_success: diPythonParameterNameBasedInjectionSuccess,
	};
	return app;
}

async function diDependencyInjectionInLifecycleHooksSuccessLogRequestOnRequest0(
	request: HookRequest,
): Promise<HookResult> {
	// Mock onRequest hook: log_request
	return request;
}

async function diDependencyInjectionInLifecycleHooksSuccessAuthCheckPreHandler0(
	request: HookRequest,
): Promise<HookResult> {
	// Mock preHandler hook: auth_check
	return request;
}

/**
 * Handler for GET /api/hook-di-test
 */
async function diDependencyInjectionInLifecycleHooksSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "x-auth-mode": "strict", "x-log-level": "debug" };
	const responseBody = { authenticated: true, logged: true };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiDependencyInjectionInLifecycleHooksSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("authService", { enabled: true, strict_mode: true });
	app.provide("logger", { level: "debug" });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/hook-di-test",
		handler_name: "di_dependency_injection_in_lifecycle_hooks_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
		di_dependency_injection_in_lifecycle_hooks_success: diDependencyInjectionInLifecycleHooksSuccess,
	};
	app.lifecycleHooks = {
		onRequest: [diDependencyInjectionInLifecycleHooksSuccessLogRequestOnRequest0],
		preHandler: [diDependencyInjectionInLifecycleHooksSuccessAuthCheckPreHandler0],
	};
	return app;
}

/**
 * Handler for GET /api/ruby-kwargs
 */
async function diRubyKeywordArgumentInjectionSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const db_pool = request.dependencies?.db_pool ?? null;
	const session = request.dependencies?.session ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = { adapter: "postgresql", user_id: 42 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiRubyKeywordArgumentInjectionSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("dbPool", { adapter: "postgresql", pool_size: 5 });
	app.provide("session", { session_id: "abc123", user_id: 42 });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/ruby-kwargs",
		handler_name: "di_ruby_keyword_argument_injection_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
		di_ruby_keyword_argument_injection_success: diRubyKeywordArgumentInjectionSuccess,
	};
	return app;
}

async function* diMultipleDependenciesWithCleanupSuccessCacheConnection(): AsyncGenerator<unknown, void, unknown> {
	// Factory for cache_connection with cleanup
	// Initialize cleanup state
	CLEANUP_STATE.di_multiple_dependencies_with_cleanup_success =
		CLEANUP_STATE.di_multiple_dependencies_with_cleanup_success || [];
	CLEANUP_STATE.di_multiple_dependencies_with_cleanup_success.push("session_opened");
	// Create resource
	const resource = { id: "00000000-0000-0000-0000-00000000002d", opened: true };
	try {
		yield resource;
	} finally {
		// Cleanup resource
		CLEANUP_STATE.di_multiple_dependencies_with_cleanup_success.push("session_closed");
	}
}

async function* diMultipleDependenciesWithCleanupSuccessSession(
	dbConnection,
	cacheConnection,
): AsyncGenerator<unknown, void, unknown> {
	// Factory for session with cleanup
	// Initialize cleanup state
	CLEANUP_STATE.di_multiple_dependencies_with_cleanup_success =
		CLEANUP_STATE.di_multiple_dependencies_with_cleanup_success || [];
	CLEANUP_STATE.di_multiple_dependencies_with_cleanup_success.push("session_opened");
	// Create resource
	const resource = { id: "00000000-0000-0000-0000-00000000002d", opened: true };
	try {
		yield resource;
	} finally {
		// Cleanup resource
		CLEANUP_STATE.di_multiple_dependencies_with_cleanup_success.push("session_closed");
	}
}

async function* diMultipleDependenciesWithCleanupSuccessDbConnection(): AsyncGenerator<unknown, void, unknown> {
	// Factory for db_connection with cleanup
	// Initialize cleanup state
	CLEANUP_STATE.di_multiple_dependencies_with_cleanup_success =
		CLEANUP_STATE.di_multiple_dependencies_with_cleanup_success || [];
	CLEANUP_STATE.di_multiple_dependencies_with_cleanup_success.push("session_opened");
	// Create resource
	const resource = { id: "00000000-0000-0000-0000-00000000002d", opened: true };
	try {
		yield resource;
	} finally {
		// Cleanup resource
		CLEANUP_STATE.di_multiple_dependencies_with_cleanup_success.push("session_closed");
	}
}

/**
 * Handler for GET /api/multi-cleanup-test
 */
async function diMultipleDependenciesWithCleanupSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const session = request.dependencies?.session ?? null;
	const response: HandlerResponse = { status: 200 };
	BACKGROUND_STATE.di_multiple_dependencies_with_cleanup_success =
		BACKGROUND_STATE.di_multiple_dependencies_with_cleanup_success ?? [];
	const state = BACKGROUND_STATE.di_multiple_dependencies_with_cleanup_success as unknown[];
	const value = body && typeof body === "object" ? body.event : undefined;
	if (value === undefined || value === null) {
		throw new Error("background task requires request body value");
	}
	void Promise.resolve().then(() => void state.push(value));
	response.body = null;
	return JSON.stringify(response);
}

async function diMultipleDependenciesWithCleanupSuccessBackgroundState(): Promise<string> {
	const state = BACKGROUND_STATE.di_multiple_dependencies_with_cleanup_success ?? [];
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/json" };
	response.body = { cleanup_order: state };
	return JSON.stringify(response);
}

async function diMultipleDependenciesWithCleanupSuccessCleanupState(): Promise<string> {
	// Return cleanup events
	const cleanupEvents = CLEANUP_STATE.di_multiple_dependencies_with_cleanup_success || [];
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/json" };
	response.body = { cleanup_events: cleanupEvents };
	return JSON.stringify(response);
}

export function createAppDiMultipleDependenciesWithCleanupSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("cacheConnection", diMultipleDependenciesWithCleanupSuccessCacheConnection, { cacheable: true });
	app.provide("dbConnection", diMultipleDependenciesWithCleanupSuccessDbConnection, { cacheable: true });
	app.provide("session", diMultipleDependenciesWithCleanupSuccessSession, { cacheable: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/multi-cleanup-test",
		handler_name: "di_multiple_dependencies_with_cleanup_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	const backgroundRoute: RouteMetadata = {
		method: "GET",
		path: "/api/multi-cleanup-state",
		handler_name: "di_multiple_dependencies_with_cleanup_success_background_state",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	const cleanupRoute: RouteMetadata = {
		method: "GET",
		path: "/api/cleanup-state",
		handler_name: "di_multiple_dependencies_with_cleanup_success_cleanup_state",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route, backgroundRoute, cleanupRoute];
	app.handlers = {
		di_multiple_dependencies_with_cleanup_success: diMultipleDependenciesWithCleanupSuccess,
		di_multiple_dependencies_with_cleanup_success_background_state:
			diMultipleDependenciesWithCleanupSuccessBackgroundState,
		di_multiple_dependencies_with_cleanup_success_cleanup_state: diMultipleDependenciesWithCleanupSuccessCleanupState,
	};
	return app;
}

function diMixedSingletonAndPerRequestCachingSuccessRequestContext(dbPool): unknown {
	// Factory for request_context
	return { _factory: "request_context", _random: Math.random() };
}

function diMixedSingletonAndPerRequestCachingSuccessDbPool(appConfig): unknown {
	// Factory for db_pool
	return { _factory: "db_pool", _random: Math.random() };
}

/**
 * Handler for GET /api/mixed-caching
 */
async function diMixedSingletonAndPerRequestCachingSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const app_config = request.dependencies?.app_config ?? null;
	const db_pool = request.dependencies?.db_pool ?? null;
	const request_context = request.dependencies?.request_context ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = { app_name: "MyApp", context_id: "<<uuid>>", pool_id: "<<uuid>>" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiMixedSingletonAndPerRequestCachingSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("appConfig", { app_name: "MyApp", version: "2.0" });
	app.provide("dbPool", diMixedSingletonAndPerRequestCachingSuccessDbPool, { singleton: true });
	app.provide("requestContext", diMixedSingletonAndPerRequestCachingSuccessRequestContext, { cacheable: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/mixed-caching",
		handler_name: "di_mixed_singleton_and_per_request_caching_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
		di_mixed_singleton_and_per_request_caching_success: diMixedSingletonAndPerRequestCachingSuccess,
	};
	return app;
}

async function* diResourceCleanupAfterRequestSuccessDbSession(): AsyncGenerator<unknown, void, unknown> {
	// Factory for db_session with cleanup
	// Initialize cleanup state
	CLEANUP_STATE.di_resource_cleanup_after_request_success =
		CLEANUP_STATE.di_resource_cleanup_after_request_success || [];
	CLEANUP_STATE.di_resource_cleanup_after_request_success.push("session_opened");
	// Create resource
	const resource = { id: "00000000-0000-0000-0000-000000000029", opened: true };
	try {
		yield resource;
	} finally {
		// Cleanup resource
		CLEANUP_STATE.di_resource_cleanup_after_request_success.push("session_closed");
	}
}

/**
 * Handler for GET /api/cleanup-test
 */
async function diResourceCleanupAfterRequestSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const db_session = request.dependencies?.db_session ?? null;
	const response: HandlerResponse = { status: 200 };
	BACKGROUND_STATE.di_resource_cleanup_after_request_success =
		BACKGROUND_STATE.di_resource_cleanup_after_request_success ?? [];
	const state = BACKGROUND_STATE.di_resource_cleanup_after_request_success as unknown[];
	const value = body && typeof body === "object" ? body.session_id : undefined;
	if (value === undefined || value === null) {
		throw new Error("background task requires request body value");
	}
	void Promise.resolve().then(() => void state.push(value));
	response.body = null;
	return JSON.stringify(response);
}

async function diResourceCleanupAfterRequestSuccessBackgroundState(): Promise<string> {
	const state = BACKGROUND_STATE.di_resource_cleanup_after_request_success ?? [];
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/json" };
	response.body = { cleanup_events: state };
	return JSON.stringify(response);
}

async function diResourceCleanupAfterRequestSuccessCleanupState(): Promise<string> {
	// Return cleanup events
	const cleanupEvents = CLEANUP_STATE.di_resource_cleanup_after_request_success || [];
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/json" };
	response.body = { cleanup_events: cleanupEvents };
	return JSON.stringify(response);
}

export function createAppDiResourceCleanupAfterRequestSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("dbSession", diResourceCleanupAfterRequestSuccessDbSession, { cacheable: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/cleanup-test",
		handler_name: "di_resource_cleanup_after_request_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	const backgroundRoute: RouteMetadata = {
		method: "GET",
		path: "/api/cleanup-state",
		handler_name: "di_resource_cleanup_after_request_success_background_state",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	const cleanupRoute: RouteMetadata = {
		method: "GET",
		path: "/api/cleanup-state",
		handler_name: "di_resource_cleanup_after_request_success_cleanup_state",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route, backgroundRoute, cleanupRoute];
	app.handlers = {
		di_resource_cleanup_after_request_success: diResourceCleanupAfterRequestSuccess,
		di_resource_cleanup_after_request_success_background_state: diResourceCleanupAfterRequestSuccessBackgroundState,
		di_resource_cleanup_after_request_success_cleanup_state: diResourceCleanupAfterRequestSuccessCleanupState,
	};
	return app;
}

/**
 * Handler for GET /api/python-type-inject
 */
async function diPythonTypeAnnotationBasedInjectionSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const database_pool = request.dependencies?.database_pool ?? null;
	const cache_client = request.dependencies?.cache_client ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = { cache_type: "Redis", pool_type: "PostgreSQL" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiPythonTypeAnnotationBasedInjectionSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("cacheClient", { cache_type: "Redis", ttl: 300 });
	app.provide("databasePool", { max_connections: 20, pool_type: "PostgreSQL" });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/python-type-inject",
		handler_name: "di_python_type_annotation_based_injection_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
		di_python_type_annotation_based_injection_success: diPythonTypeAnnotationBasedInjectionSuccess,
	};
	return app;
}

function diPerRequestDependencyCachingSuccessRequestIdGenerator(): unknown {
	// Factory for request_id_generator
	return { _factory: "request_id_generator", _random: Math.random() };
}

/**
 * Handler for GET /api/request-id
 */
async function diPerRequestDependencyCachingSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const request_id_generator = request.dependencies?.request_id_generator ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = { first_id: "<<uuid>>", second_id: "<<same_as:first_id>>" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiPerRequestDependencyCachingSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("requestIdGenerator", diPerRequestDependencyCachingSuccessRequestIdGenerator, { cacheable: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/request-id",
		handler_name: "di_per_request_dependency_caching_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
		di_per_request_dependency_caching_success: diPerRequestDependencyCachingSuccess,
	};
	return app;
}

function diSingletonDependencyCachingSuccessAppCounter(): unknown {
	// Factory for app_counter
	return { _factory: "app_counter", _random: Math.random() };
}

/**
 * Handler for GET /api/app-counter
 */
async function diSingletonDependencyCachingSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const app_counter = request.dependencies?.app_counter ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = { count: 1, counter_id: "<<uuid>>" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiSingletonDependencyCachingSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("appCounter", diSingletonDependencyCachingSuccessAppCounter, { singleton: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/app-counter",
		handler_name: "di_singleton_dependency_caching_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
		di_singleton_dependency_caching_success: diSingletonDependencyCachingSuccess,
	};
	return app;
}

async function diAsyncFactoryDependencySuccessDbPool(): Promise<unknown> {
	// Async factory for db_pool
	// Simulate async DB connection
	return { connected: true, poolId: Math.random().toString() };
}

/**
 * Handler for GET /api/db-status
 */
async function diAsyncFactoryDependencySuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const db_pool = request.dependencies?.db_pool ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = { max_size: 10, pool_status: "connected" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiAsyncFactoryDependencySuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("dbPool", diAsyncFactoryDependencySuccessDbPool, { cacheable: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/db-status",
		handler_name: "di_async_factory_dependency_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
		di_async_factory_dependency_success: diAsyncFactoryDependencySuccess,
	};
	return app;
}

/**
 * Handler for POST /body-limit/under
 */
async function bodyLimitsBodyUnderLimitSucceeds(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { accepted: true, note: "small" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppBodyLimitsBodyUnderLimitSucceeds(): SpikardApp {
	const config: ServerConfig = {
		maxBodySize: 64,
	};

	const route: RouteMetadata = {
		method: "POST",
		path: "/body-limit/under",
		handler_name: "body_limits_body_under_limit_succeeds",
		request_schema: {
			additionalProperties: false,
			properties: { note: { type: "string" } },
			required: ["note"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

/**
 * Handler for POST /body-limit/over
 */
async function bodyLimitsBodyOverLimitReturns413(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 413 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppBodyLimitsBodyOverLimitReturns413(): SpikardApp {
	const config: ServerConfig = {
		maxBodySize: 64,
	};

	const route: RouteMetadata = {
		method: "POST",
		path: "/body-limit/over",
		handler_name: "body_limits_body_over_limit_returns_413",
		request_schema: {
			additionalProperties: false,
			properties: { note: { type: "string" } },
			required: ["note"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

/**
 * Handler for GET /items/{item_id}
 */
async function validationErrorsInvalidUuidFormat(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const itemId = params.item_id;
	if (itemId !== null && itemId !== undefined) {
		result.item_id = itemId;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsInvalidUuidFormat(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/{item_id}",
		handler_name: "validation_errors_invalid_uuid_format",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { format: "uuid", source: "path", type: "string" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_uuid_format: validationErrorsInvalidUuidFormat,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsInvalidBooleanValue(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const isActive = params.is_active;
	const q = params.q;
	if (isActive !== null && isActive !== undefined) {
		result.is_active = isActive;
	}
	if (q !== null && q !== undefined) {
		result.q = q;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsInvalidBooleanValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_invalid_boolean_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { is_active: { source: "query", type: "boolean" }, q: { source: "query", type: "string" } },
			required: ["is_active", "q"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_boolean_value: validationErrorsInvalidBooleanValue,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsMissingRequiredQueryParameter(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = params.q;
	if (q !== null && q !== undefined) {
		result.q = q;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsMissingRequiredQueryParameter(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_missing_required_query_parameter",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { q: { source: "query", type: "string" } }, required: ["q"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_missing_required_query_parameter: validationErrorsMissingRequiredQueryParameter,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsArrayMaxItemsConstraintViolation(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsArrayMaxItemsConstraintViolation(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_array_max_items_constraint_violation: validationErrorsArrayMaxItemsConstraintViolation,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsNumericConstraintViolationGtGreaterThan(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const price = params.price;
	const q = params.q;
	if (price !== null && price !== undefined) {
		result.price = price;
	}
	if (q !== null && q !== undefined) {
		result.q = q;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsNumericConstraintViolationGtGreaterThan(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_numeric_constraint_violation_gt_greater_than",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				price: { exclusiveMinimum: 0, source: "query", type: "number" },
				q: { source: "query", type: "string" },
			},
			required: ["price", "q"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /items/
 */
async function validationErrorsStringRegexPatternMismatch(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = params.q;
	if (q !== null && q !== undefined) {
		result.q = q;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsStringRegexPatternMismatch(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_string_regex_pattern_mismatch",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { q: { pattern: "^[a-zA-Z0-9_-]+$", source: "query", type: "string" } },
			required: ["q"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_string_regex_pattern_mismatch: validationErrorsStringRegexPatternMismatch,
		},
	};
}

/**
 * Handler for GET /models/{model_name}
 */
async function validationErrorsInvalidEnumValue(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const modelName = params.model_name;
	if (modelName !== null && modelName !== undefined) {
		result.model_name = modelName;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsInvalidEnumValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/models/{model_name}",
		handler_name: "validation_errors_invalid_enum_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { model_name: { enum: ["alexnet", "resnet", "lenet"], source: "path", type: "string" } },
			required: ["model_name"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_enum_value: validationErrorsInvalidEnumValue,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsStringMinLengthConstraintViolation(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = params.q;
	if (q !== null && q !== undefined) {
		result.q = q;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsStringMinLengthConstraintViolation(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_string_min_length_constraint_violation",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { q: { minLength: 3, source: "query", type: "string" } },
			required: ["q"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_string_min_length_constraint_violation: validationErrorsStringMinLengthConstraintViolation,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsMultipleValidationErrors(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsMultipleValidationErrors(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_multiple_validation_errors: validationErrorsMultipleValidationErrors,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsStringMaxLengthConstraintViolation(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = params.q;
	if (q !== null && q !== undefined) {
		result.q = q;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsStringMaxLengthConstraintViolation(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_string_max_length_constraint_violation",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { q: { maxLength: 50, source: "query", type: "string" } },
			required: ["q"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_string_max_length_constraint_violation: validationErrorsStringMaxLengthConstraintViolation,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsNestedObjectValidationError(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsNestedObjectValidationError(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_nested_object_validation_error: validationErrorsNestedObjectValidationError,
		},
	};
}

/**
 * Handler for POST /profiles
 */
async function validationErrors10NestedErrorPath(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrors10NestedErrorPath(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_10_nested_error_path: validationErrors10NestedErrorPath,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsInvalidDatetimeFormat(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsInvalidDatetimeFormat(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_datetime_format: validationErrorsInvalidDatetimeFormat,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsArrayItemValidationError(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsArrayItemValidationError(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_array_item_validation_error: validationErrorsArrayItemValidationError,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsMissingRequiredBodyField(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsMissingRequiredBodyField(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_missing_required_body_field",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, price: { type: "string" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_missing_required_body_field: validationErrorsMissingRequiredBodyField,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsBodyFieldTypeErrorStringForFloat(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsBodyFieldTypeErrorStringForFloat(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_body_field_type_error_string_for_float",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, price: { type: "number" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_body_field_type_error_string_for_float: validationErrorsBodyFieldTypeErrorStringForFloat,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsMalformedJsonBody(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 400 };
	const responseBody = { detail: "Invalid request format" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppValidationErrorsMalformedJsonBody(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_malformed_json_body",
		request_schema: { type: "string" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_malformed_json_body: validationErrorsMalformedJsonBody,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsQueryParamTypeErrorStringProvidedForInt(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = params.q;
	const skip = params.skip;
	if (q !== null && q !== undefined) {
		result.q = q;
	}
	if (skip !== null && skip !== undefined) {
		result.skip = skip;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsQueryParamTypeErrorStringProvidedForInt(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_query_param_type_error_string_provided_for_int",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { q: { source: "query", type: "string" }, skip: { source: "query", type: "integer" } },
			required: ["q", "skip"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /items/
 */
async function validationErrorsHeaderValidationError(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = params.q;
	const xToken = params["x-token"];
	if (q !== null && q !== undefined) {
		result.q = q;
	}
	if (xToken !== null && xToken !== undefined) {
		result["x-token"] = xToken;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsHeaderValidationError(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_header_validation_error",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { q: { source: "query", type: "string" }, "x-token": { source: "header", type: "string" } },
			required: ["q", "x-token"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_header_validation_error: validationErrorsHeaderValidationError,
		},
	};
}

/**
 * Handler for POST /users
 */
async function validationErrors09MultipleValidationErrors(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrors09MultipleValidationErrors(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_09_multiple_validation_errors: validationErrors09MultipleValidationErrors,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsNumericConstraintViolationLeLessThanOrEqual(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const limit = params.limit;
	const q = params.q;
	if (limit !== null && limit !== undefined) {
		result.limit = limit;
	}
	if (q !== null && q !== undefined) {
		result.q = q;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsNumericConstraintViolationLeLessThanOrEqual(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_numeric_constraint_violation_le_less_than_or_equal",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { limit: { maximum: 100, source: "query", type: "integer" }, q: { source: "query", type: "string" } },
			required: ["limit", "q"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for POST /items/
 */
async function validationErrorsArrayMinItemsConstraintViolation(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppValidationErrorsArrayMinItemsConstraintViolation(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_array_min_items_constraint_violation: validationErrorsArrayMinItemsConstraintViolation,
		},
	};
}

async function lifecycleHooksOnresponseSecurityHeadersSecurityHeadersOnResponse0(
	response: HookResponse,
): Promise<HookResponse> {
	// onResponse hook: security_headers - Adds security headers
	if (!response.headers) response.headers = {};
	response.headers["X-Content-Type-Options"] = "nosniff";
	response.headers["X-Frame-Options"] = "DENY";
	response.headers["X-XSS-Protection"] = "1; mode=block";
	response.headers["Strict-Transport-Security"] = "max-age=31536000; includeSubDomains";
	return response;
}

/**
 * Handler for GET /api/test-security-headers
 */
async function lifecycleHooksOnresponseSecurityHeaders(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
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

export function createAppLifecycleHooksOnresponseSecurityHeaders(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/test-security-headers",
		handler_name: "lifecycle_hooks_onresponse_security_headers",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

async function lifecycleHooksPrehandlerAuthenticationFailedShortCircuitAuthenticatorPreHandler0(
	_request: HookRequest,
): Promise<HookResult> {
	// preHandler hook: authenticator - Short circuits with 401
	return {
		statusCode: 401,
		body: {
			error: "Unauthorized",
			message: "Invalid or expired authentication token",
		},
	};
}

/**
 * Handler for GET /api/protected-resource-fail
 */
async function lifecycleHooksPrehandlerAuthenticationFailedShortCircuit(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = { error: "Unauthorized", message: "Invalid or expired authentication token" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppLifecycleHooksPrehandlerAuthenticationFailedShortCircuit(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/protected-resource-fail",
		handler_name: "lifecycle_hooks_prehandler_authentication_failed_short_circuit",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

async function lifecycleHooksPrehandlerAuthorizationCheckAuthenticatorPreHandler0(
	request: HookRequest,
): Promise<HookResult> {
	// Mock preHandler hook: authenticator
	return request;
}

async function lifecycleHooksPrehandlerAuthorizationCheckAuthorizerPreHandler1(
	request: HookRequest,
): Promise<HookResult> {
	// Mock preHandler hook: authorizer
	return request;
}

/**
 * Handler for GET /api/admin-only
 */
async function lifecycleHooksPrehandlerAuthorizationCheck(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Admin access granted", role: "admin", user_id: "admin-456" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppLifecycleHooksPrehandlerAuthorizationCheck(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/admin-only",
		handler_name: "lifecycle_hooks_prehandler_authorization_check",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

async function lifecycleHooksPrehandlerAuthenticationSuccessAuthenticatorPreHandler0(
	request: HookRequest,
): Promise<HookResult> {
	// Mock preHandler hook: authenticator
	return request;
}

/**
 * Handler for GET /api/protected-resource
 */
async function lifecycleHooksPrehandlerAuthenticationSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { authenticated: true, message: "Access granted", user_id: "user-123" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppLifecycleHooksPrehandlerAuthenticationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/protected-resource",
		handler_name: "lifecycle_hooks_prehandler_authentication_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

async function lifecycleHooksPrevalidationRateLimitExceededShortCircuitRateLimiterPreValidation0(
	_request: HookRequest,
): Promise<HookResult> {
	// preValidation hook: rate_limiter - Short circuits with 429
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

/**
 * Handler for POST /api/test-rate-limit-exceeded
 */
async function lifecycleHooksPrevalidationRateLimitExceededShortCircuit(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 429 };
	response.headers = { "retry-after": "60" };
	const responseBody = { error: "Rate limit exceeded", message: "Too many requests, please try again later" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppLifecycleHooksPrevalidationRateLimitExceededShortCircuit(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/api/test-rate-limit-exceeded",
		handler_name: "lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit",
		request_schema: { properties: { data: { type: "string" } }, required: ["data"], type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

async function lifecycleHooksOnerrorErrorLoggingErrorLoggerOnError0(response: HookResponse): Promise<HookResponse> {
	// onError hook: error_logger - Format error response
	if (!response.headers) response.headers = {};
	response.headers["Content-Type"] = "application/json";
	return response;
}

async function lifecycleHooksOnerrorErrorLoggingErrorFormatterOnError1(response: HookResponse): Promise<HookResponse> {
	// onError hook: error_formatter - Format error response
	if (!response.headers) response.headers = {};
	response.headers["Content-Type"] = "application/json";
	return response;
}

/**
 * Handler for GET /api/test-error
 */
async function lifecycleHooksOnerrorErrorLogging(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 500 };
	response.headers = { "content-type": "application/json" };
	const responseBody = { error: "Internal Server Error", error_id: ".*", message: "An unexpected error occurred" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppLifecycleHooksOnerrorErrorLogging(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/test-error",
		handler_name: "lifecycle_hooks_onerror_error_logging",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

async function lifecycleHooksMultipleHooksAllPhasesRequestLoggerOnRequest0(request: HookRequest): Promise<HookResult> {
	// Mock onRequest hook: request_logger
	return request;
}

async function lifecycleHooksMultipleHooksAllPhasesRequestIdGeneratorOnRequest1(
	request: HookRequest,
): Promise<HookResult> {
	// Mock onRequest hook: request_id_generator
	return request;
}

async function lifecycleHooksMultipleHooksAllPhasesRateLimiterPreValidation0(
	request: HookRequest,
): Promise<HookResult> {
	// Mock preValidation hook: rate_limiter
	return request;
}

async function lifecycleHooksMultipleHooksAllPhasesAuthenticatorPreHandler0(request: HookRequest): Promise<HookResult> {
	// Mock preHandler hook: authenticator
	return request;
}

async function lifecycleHooksMultipleHooksAllPhasesAuthorizerPreHandler1(request: HookRequest): Promise<HookResult> {
	// Mock preHandler hook: authorizer
	return request;
}

async function lifecycleHooksMultipleHooksAllPhasesSecurityHeadersOnResponse0(
	response: HookResponse,
): Promise<HookResponse> {
	// onResponse hook: security_headers - Adds security headers
	if (!response.headers) response.headers = {};
	response.headers["X-Content-Type-Options"] = "nosniff";
	response.headers["X-Frame-Options"] = "DENY";
	response.headers["X-XSS-Protection"] = "1; mode=block";
	response.headers["Strict-Transport-Security"] = "max-age=31536000; includeSubDomains";
	return response;
}

async function lifecycleHooksMultipleHooksAllPhasesResponseTimerOnResponse1(
	response: HookResponse,
): Promise<HookResponse> {
	// onResponse hook: response_timer - Adds timing header
	if (!response.headers) response.headers = {};
	response.headers["X-Response-Time"] = "0ms";
	return response;
}

async function lifecycleHooksMultipleHooksAllPhasesAuditLoggerOnResponse2(
	response: HookResponse,
): Promise<HookResponse> {
	// Mock onResponse hook: audit_logger
	return response;
}

async function lifecycleHooksMultipleHooksAllPhasesErrorLoggerOnError0(response: HookResponse): Promise<HookResponse> {
	// onError hook: error_logger - Format error response
	if (!response.headers) response.headers = {};
	response.headers["Content-Type"] = "application/json";
	return response;
}

/**
 * Handler for POST /api/full-lifecycle
 */
async function lifecycleHooksMultipleHooksAllPhases(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
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

export function createAppLifecycleHooksMultipleHooksAllPhases(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/api/full-lifecycle",
		handler_name: "lifecycle_hooks_multiple_hooks_all_phases",
		request_schema: {
			properties: { action: { type: "string" }, user_id: { type: "string" } },
			required: ["user_id", "action"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

async function lifecycleHooksHookExecutionOrderFirstHookOnRequest0(request: HookRequest): Promise<HookResult> {
	// Mock onRequest hook: first_hook
	return request;
}

async function lifecycleHooksHookExecutionOrderSecondHookOnRequest1(request: HookRequest): Promise<HookResult> {
	// Mock onRequest hook: second_hook
	return request;
}

async function lifecycleHooksHookExecutionOrderThirdHookOnRequest2(request: HookRequest): Promise<HookResult> {
	// Mock onRequest hook: third_hook
	return request;
}

/**
 * Handler for GET /api/test-hook-order
 */
async function lifecycleHooksHookExecutionOrder(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {
		execution_order: ["first_hook", "second_hook", "third_hook"],
		message: "Hooks executed in order",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppLifecycleHooksHookExecutionOrder(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/test-hook-order",
		handler_name: "lifecycle_hooks_hook_execution_order",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

async function lifecycleHooksOnresponseResponseTimingStartTimerOnRequest0(request: HookRequest): Promise<HookResult> {
	// Mock onRequest hook: start_timer
	return request;
}

async function lifecycleHooksOnresponseResponseTimingResponseTimerOnResponse0(
	response: HookResponse,
): Promise<HookResponse> {
	// onResponse hook: response_timer - Adds timing header
	if (!response.headers) response.headers = {};
	response.headers["X-Response-Time"] = "0ms";
	return response;
}

/**
 * Handler for GET /api/test-timing
 */
async function lifecycleHooksOnresponseResponseTiming(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "x-response-time": ".*ms" };
	const responseBody = { message: "Response with timing info" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppLifecycleHooksOnresponseResponseTiming(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/test-timing",
		handler_name: "lifecycle_hooks_onresponse_response_timing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

async function lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuitAuthenticatorPreHandler0(
	_request: HookRequest,
): Promise<HookResult> {
	// preHandler hook: authenticator - Short circuits with 403
	return {
		statusCode: 403,
		body: {
			error: "Forbidden",
			message: "Admin role required for this endpoint",
		},
	};
}

async function lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuitAuthorizerPreHandler1(
	_request: HookRequest,
): Promise<HookResult> {
	// preHandler hook: authorizer - Short circuits with 403
	return {
		statusCode: 403,
		body: {
			error: "Forbidden",
			message: "Admin role required for this endpoint",
		},
	};
}

/**
 * Handler for GET /api/admin-only-forbidden
 */
async function lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuit(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 403 };
	const responseBody = { error: "Forbidden", message: "Admin role required for this endpoint" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppLifecycleHooksPrehandlerAuthorizationForbiddenShortCircuit(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/admin-only-forbidden",
		handler_name: "lifecycle_hooks_prehandler_authorization_forbidden_short_circuit",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

async function lifecycleHooksOnrequestRequestLoggingRequestLoggerOnRequest0(request: HookRequest): Promise<HookResult> {
	// Mock onRequest hook: request_logger
	return request;
}

async function lifecycleHooksOnrequestRequestLoggingRequestIdGeneratorOnRequest1(
	request: HookRequest,
): Promise<HookResult> {
	// Mock onRequest hook: request_id_generator
	return request;
}

/**
 * Handler for GET /api/test-on-request
 */
async function lifecycleHooksOnrequestRequestLogging(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = { "x-request-id": ".*" };
	const responseBody = { has_request_id: true, message: "onRequest hooks executed", request_logged: true };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppLifecycleHooksOnrequestRequestLogging(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/test-on-request",
		handler_name: "lifecycle_hooks_onrequest_request_logging",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

async function lifecycleHooksPrevalidationRateLimitingRateLimiterPreValidation0(
	request: HookRequest,
): Promise<HookResult> {
	// Mock preValidation hook: rate_limiter
	return request;
}

/**
 * Handler for POST /api/test-rate-limit
 */
async function lifecycleHooksPrevalidationRateLimiting(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { message: "Request accepted", rate_limit_checked: true };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppLifecycleHooksPrevalidationRateLimiting(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/api/test-rate-limit",
		handler_name: "lifecycle_hooks_prevalidation_rate_limiting",
		request_schema: { properties: { data: { type: "string" } }, required: ["data"], type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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

/**
 * Handler for POST /messages
 */
async function edgeCases19EmojiInStrings(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = { text: "Hello 👋 World 🌍" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases19EmojiInStrings(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/messages",
		handler_name: "edge_cases_19_emoji_in_strings",
		request_schema: {
			properties: { text: { maxLength: 100, minLength: 1, type: "string" } },
			required: ["text"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_19_emoji_in_strings: edgeCases19EmojiInStrings,
		},
	};
}

/**
 * Handler for GET /search
 */
async function edgeCases12PercentEncodedSpecialChars(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { term: "hi there" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases12PercentEncodedSpecialChars(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/search",
		handler_name: "edge_cases_12_percent_encoded_special_chars",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { term: { source: "query", type: "string" } }, required: ["term"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_12_percent_encoded_special_chars: edgeCases12PercentEncodedSpecialChars,
		},
	};
}

/**
 * Handler for POST /strings/
 */
async function edgeCasesSpecialStringValuesAndEscaping(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {
		backslashes: "C:\\\\Users\\\\Path",
		empty_string: "",
		quotes: "He said \"hello\" and 'goodbye'",
		special_chars: "!@#$%^&*()_+-=[]{}|;':\",./<>?",
		tabs_newlines: "line1\n\tline2\r\nline3",
		unicode_escapes: "Hello",
		whitespace: "   ",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCasesSpecialStringValuesAndEscaping(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_special_string_values_and_escaping: edgeCasesSpecialStringValuesAndEscaping,
		},
	};
}

/**
 * Handler for POST /calculate
 */
async function edgeCases15FloatPrecisionPreservation(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = { value: 3.141592653589793 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases15FloatPrecisionPreservation(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/calculate",
		handler_name: "edge_cases_15_float_precision_preservation",
		request_schema: { properties: { value: { type: "number" } }, required: ["value"], type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_15_float_precision_preservation: edgeCases15FloatPrecisionPreservation,
		},
	};
}

/**
 * Handler for GET /items
 */
async function edgeCases13EmptyStringQueryParamPreserved(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { filter: "" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases13EmptyStringQueryParamPreserved(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "edge_cases_13_empty_string_query_param_preserved",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { filter: { source: "query", type: "string" } },
			required: ["filter"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_13_empty_string_query_param_preserved: edgeCases13EmptyStringQueryParamPreserved,
		},
	};
}

/**
 * Handler for POST /items
 */
async function edgeCases24ArrayWithHoles(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { items: ["first", "third", "sixth"] };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases24ArrayWithHoles(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items",
		handler_name: "edge_cases_24_array_with_holes",
		request_schema: {
			properties: { items: { items: { type: "string" }, type: "array" } },
			required: ["items"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_24_array_with_holes: edgeCases24ArrayWithHoles,
		},
	};
}

/**
 * Handler for POST /calculate
 */
async function edgeCases21ScientificNotationNumber(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = { value: 123000 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases21ScientificNotationNumber(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/calculate",
		handler_name: "edge_cases_21_scientific_notation_number",
		request_schema: { properties: { value: { minimum: 0, type: "number" } }, required: ["value"], type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_21_scientific_notation_number: edgeCases21ScientificNotationNumber,
		},
	};
}

/**
 * Handler for POST /calculations/
 */
async function edgeCasesFloatPrecisionAndRounding(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {
		precise_value: 3.141592653589793,
		sum: 0.30000000000000004,
		very_large: 1.7976931348623157e308,
		very_small: 1e-10,
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCasesFloatPrecisionAndRounding(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_float_precision_and_rounding: edgeCasesFloatPrecisionAndRounding,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function edgeCasesUnicodeAndEmojiHandling(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {
		description: "Best café in München 🇩🇪",
		emoji_reactions: "👍❤️😂🎉",
		id: 1,
		name: "Coffee Shop ☕",
		tags: ["食べ物", "音楽", "💰"],
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCasesUnicodeAndEmojiHandling(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_unicode_and_emoji_handling: edgeCasesUnicodeAndEmojiHandling,
		},
	};
}

/**
 * Handler for POST /text
 */
async function edgeCases17ExtremelyLongString(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppEdgeCases17ExtremelyLongString(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/text",
		handler_name: "edge_cases_17_extremely_long_string",
		request_schema: {
			properties: { content: { maxLength: 10000, type: "string" } },
			required: ["content"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_17_extremely_long_string: edgeCases17ExtremelyLongString,
		},
	};
}

/**
 * Handler for GET /search
 */
async function edgeCases11Utf8QueryParameter(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { term: "café" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases11Utf8QueryParameter(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/search",
		handler_name: "edge_cases_11_utf8_query_parameter",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { term: { source: "query", type: "string" } }, required: ["term"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_11_utf8_query_parameter: edgeCases11Utf8QueryParameter,
		},
	};
}

/**
 * Handler for POST /users
 */
async function edgeCases18UnicodeNormalization(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = { name: "café" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases18UnicodeNormalization(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "edge_cases_18_unicode_normalization",
		request_schema: { properties: { name: { minLength: 1, type: "string" } }, required: ["name"], type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_18_unicode_normalization: edgeCases18UnicodeNormalization,
		},
	};
}

/**
 * Handler for POST /files
 */
async function edgeCases20NullByteInString(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppEdgeCases20NullByteInString(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files",
		handler_name: "edge_cases_20_null_byte_in_string",
		request_schema: {
			properties: { filename: { pattern: "^[^\\x00]+$", type: "string" } },
			required: ["filename"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_20_null_byte_in_string: edgeCases20NullByteInString,
		},
	};
}

/**
 * Handler for POST /data
 */
async function edgeCases23DeeplyNestedJsonLimit(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 400 };
	const responseBody = { error: "Request body exceeds maximum nesting depth of 32" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases23DeeplyNestedJsonLimit(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "edge_cases_23_deeply_nested_json_limit",
		request_schema: { type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_23_deeply_nested_json_limit: edgeCases23DeeplyNestedJsonLimit,
		},
	};
}

/**
 * Handler for GET /items
 */
async function edgeCases14LargeIntegerBoundary(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { id: 9007199254740991 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases14LargeIntegerBoundary(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "edge_cases_14_large_integer_boundary",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { id: { source: "query", type: "integer" } }, required: ["id"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_14_large_integer_boundary: edgeCases14LargeIntegerBoundary,
		},
	};
}

/**
 * Handler for GET /data
 */
async function edgeCases22LeadingZerosInteger(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { value: 123 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases22LeadingZerosInteger(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/data",
		handler_name: "edge_cases_22_leading_zeros_integer",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { value: { annotation: "int", source: "query", type: "integer" } },
			required: ["value"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_22_leading_zeros_integer: edgeCases22LeadingZerosInteger,
		},
	};
}

/**
 * Handler for POST /numbers/
 */
async function edgeCasesLargeIntegerBoundaryValues(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {
		large_int: "9223372036854775807",
		max_safe_int: 9007199254740991,
		negative_large: "-9223372036854775808",
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCasesLargeIntegerBoundaryValues(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_large_integer_boundary_values: edgeCasesLargeIntegerBoundaryValues,
		},
	};
}

/**
 * Handler for POST /nested/
 */
async function edgeCasesDeeplyNestedStructure10Levels(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { max_depth: 10, message: "Processed deeply nested structure", value_found: "deep" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCasesDeeplyNestedStructure10Levels(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_deeply_nested_structure_10_levels: edgeCasesDeeplyNestedStructure10Levels,
		},
	};
}

/**
 * Handler for POST /nulls/
 */
async function edgeCasesEmptyAndNullValueHandling(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
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

export function createAppEdgeCasesEmptyAndNullValueHandling(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_empty_and_null_value_handling: edgeCasesEmptyAndNullValueHandling,
		},
	};
}

/**
 * Handler for POST /data
 */
async function edgeCases16NegativeZeroHandling(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = { offset: 0 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases16NegativeZeroHandling(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "edge_cases_16_negative_zero_handling",
		request_schema: { properties: { offset: { type: "number" } }, required: ["offset"], type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_16_negative_zero_handling: edgeCases16NegativeZeroHandling,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function queryParamsStringValidationWithRegexSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item_query: "fixedquery" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsStringValidationWithRegexSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "query_params_string_validation_with_regex_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_query: { annotation: "str", pattern: "^fixedquery$", source: "query", type: "string" } },
			required: ["item_query"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_string_validation_with_regex_success: queryParamsStringValidationWithRegexSuccess,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams49IntegerGtConstraintSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { limit: 5 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams49IntegerGtConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_49_integer_gt_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { limit: { exclusiveMinimum: 0, source: "query", type: "integer" } },
			required: ["limit"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_49_integer_gt_constraint_success: queryParams49IntegerGtConstraintSuccess,
		},
	};
}

/**
 * Handler for GET /query/enum
 */
async function queryParamsEnumQueryParameterInvalidValue(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const model = params.model;
	if (model !== null && model !== undefined) {
		result.model = model;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParamsEnumQueryParameterInvalidValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/enum",
		handler_name: "query_params_enum_query_parameter_invalid_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				model: { annotation: "str", enum: ["alexnet", "resnet", "lenet"], source: "query", type: "string" },
			},
			required: ["model"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_enum_query_parameter_invalid_value: queryParamsEnumQueryParameterInvalidValue,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams68ArrayUniqueitemsSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { ids: [1, 2, 3, 4] };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams68ArrayUniqueitemsSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_68_array_uniqueitems_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { ids: { items: { type: "integer" }, source: "query", type: "array", uniqueItems: true } },
			required: ["ids"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_68_array_uniqueitems_success: queryParams68ArrayUniqueitemsSuccess,
		},
	};
}

/**
 * Handler for GET /subscribe
 */
async function queryParams47PatternValidationEmailSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { email: "user@example.com" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams47PatternValidationEmailSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/subscribe",
		handler_name: "query_params_47_pattern_validation_email_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				email: { pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$", source: "query", type: "string" },
			},
			required: ["email"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_47_pattern_validation_email_success: queryParams47PatternValidationEmailSuccess,
		},
	};
}

/**
 * Handler for GET /query/int
 */
async function queryParamsRequiredIntegerQueryParameterSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = "foo bar 42";
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsRequiredIntegerQueryParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int",
		handler_name: "query_params_required_integer_query_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { query: { annotation: "int", source: "query", type: "integer" } },
			required: ["query"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_success: queryParamsRequiredIntegerQueryParameterSuccess,
		},
	};
}

/**
 * Handler for GET /query
 */
async function queryParamsRequiredStringQueryParameterMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const query = params.query;
	if (query !== null && query !== undefined) {
		result.query = query;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParamsRequiredStringQueryParameterMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query",
		handler_name: "query_params_required_string_query_parameter_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { query: { annotation: "str", source: "query", type: "string" } },
			required: ["query"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_string_query_parameter_missing: queryParamsRequiredStringQueryParameterMissing,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams57BooleanEmptyStringCoercion(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { active: false };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams57BooleanEmptyStringCoercion(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_57_boolean_empty_string_coercion",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { active: { source: "query", type: "boolean" } },
			required: ["active"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_57_boolean_empty_string_coercion: queryParams57BooleanEmptyStringCoercion,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams52IntegerLeConstraintBoundary(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { limit: 100 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams52IntegerLeConstraintBoundary(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_52_integer_le_constraint_boundary",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { limit: { maximum: 100, source: "query", type: "integer" } },
			required: ["limit"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_52_integer_le_constraint_boundary: queryParams52IntegerLeConstraintBoundary,
		},
	};
}

/**
 * Handler for GET /query/list-default
 */
async function queryParamsListWithDefaultEmptyArrayNoValuesProvided(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppQueryParamsListWithDefaultEmptyArrayNoValuesProvided(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/list-default",
		handler_name: "query_params_list_with_default_empty_array_no_values_provided",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				tags: { annotation: "list[str]", default: [], items: { type: "string" }, source: "query", type: "array" },
			},
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /query/date
 */
async function queryParamsDateQueryParameterSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { event_date: "2024-01-15" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsDateQueryParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/date",
		handler_name: "query_params_date_query_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { event_date: { annotation: "str", format: "date", source: "query", type: "string" } },
			required: ["event_date"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_date_query_parameter_success: queryParamsDateQueryParameterSuccess,
		},
	};
}

/**
 * Handler for GET /query/str-max-length
 */
async function queryParamsStringQueryParamWithMaxLengthConstraintFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const name = params.name;
	if (name !== null && name !== undefined) {
		result.name = name;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParamsStringQueryParamWithMaxLengthConstraintFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/str-max-length",
		handler_name: "query_params_string_query_param_with_max_length_constraint_fail",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { name: { annotation: "str", maxLength: 10, source: "query", type: "string" } },
			required: ["name"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /search
 */
async function queryParams45StringMinlengthValidationFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const term = params.term;
	if (term !== null && term !== undefined) {
		result.term = term;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParams45StringMinlengthValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/search",
		handler_name: "query_params_45_string_minlength_validation_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { term: { minLength: 3, source: "query", type: "string" } },
			required: ["term"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_45_string_minlength_validation_failure: queryParams45StringMinlengthValidationFailure,
		},
	};
}

/**
 * Handler for GET /query/int/default
 */
async function queryParamsIntegerWithDefaultValueOverride(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = "foo bar 50";
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsIntegerWithDefaultValueOverride(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int/default",
		handler_name: "query_params_integer_with_default_value_override",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { query: { annotation: "int", default: 10, source: "query", type: "integer" } },
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_with_default_value_override: queryParamsIntegerWithDefaultValueOverride,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams67MultipleofConstraintFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const quantity = params.quantity;
	if (quantity !== null && quantity !== undefined) {
		result.quantity = quantity;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParams67MultipleofConstraintFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_67_multipleof_constraint_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { quantity: { multipleOf: 5, source: "query", type: "integer" } },
			required: ["quantity"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_67_multipleof_constraint_failure: queryParams67MultipleofConstraintFailure,
		},
	};
}

/**
 * Handler for GET /subscribe
 */
async function queryParams58FormatEmailSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { email: "user@example.com" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams58FormatEmailSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/subscribe",
		handler_name: "query_params_58_format_email_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { email: { format: "email", source: "query", type: "string" } },
			required: ["email"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_58_format_email_success: queryParams58FormatEmailSuccess,
		},
	};
}

/**
 * Handler for GET /query/int-ge
 */
async function queryParamsIntegerQueryParamWithGeConstraintBoundary(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { value: 10 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsIntegerQueryParamWithGeConstraintBoundary(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int-ge",
		handler_name: "query_params_integer_query_param_with_ge_constraint_boundary",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { value: { annotation: "int", minimum: 10, source: "query", type: "integer" } },
			required: ["value"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /query/int-gt
 */
async function queryParamsIntegerQueryParamWithGtConstraintValid(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { value: 1 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsIntegerQueryParamWithGtConstraintValid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int-gt",
		handler_name: "query_params_integer_query_param_with_gt_constraint_valid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { value: { annotation: "int", exclusiveMinimum: 0, source: "query", type: "integer" } },
			required: ["value"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_query_param_with_gt_constraint_valid: queryParamsIntegerQueryParamWithGtConstraintValid,
		},
	};
}

/**
 * Handler for GET /query/int
 */
async function queryParamsRequiredIntegerQueryParameterInvalidType(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const query = params.query;
	if (query !== null && query !== undefined) {
		result.query = query;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParamsRequiredIntegerQueryParameterInvalidType(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int",
		handler_name: "query_params_required_integer_query_parameter_invalid_type",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { query: { annotation: "int", source: "query", type: "integer" } },
			required: ["query"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_invalid_type: queryParamsRequiredIntegerQueryParameterInvalidType,
		},
	};
}

/**
 * Handler for GET /query/int
 */
async function queryParamsRequiredIntegerQueryParameterFloatValue(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const query = params.query;
	if (query !== null && query !== undefined) {
		result.query = query;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParamsRequiredIntegerQueryParameterFloatValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int",
		handler_name: "query_params_required_integer_query_parameter_float_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { query: { annotation: "int", source: "query", type: "integer" } },
			required: ["query"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_float_value: queryParamsRequiredIntegerQueryParameterFloatValue,
		},
	};
}

/**
 * Handler for GET /query/basic
 */
async function queryParamsQueryParameterWithUrlEncodedSpecialCharacters(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { name: "test&value=123" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsQueryParameterWithUrlEncodedSpecialCharacters(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/basic",
		handler_name: "query_params_query_parameter_with_url_encoded_special_characters",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { name: { annotation: "str", source: "query", type: "string" } },
			required: ["name"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /subscribe
 */
async function queryParams59FormatEmailFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const email = params.email;
	if (email !== null && email !== undefined) {
		result.email = email;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParams59FormatEmailFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/subscribe",
		handler_name: "query_params_59_format_email_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { email: { format: "email", source: "query", type: "string" } },
			required: ["email"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_59_format_email_failure: queryParams59FormatEmailFailure,
		},
	};
}

/**
 * Handler for GET /stats
 */
async function queryParams43ScientificNotationFloat(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { threshold: 0.0015 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams43ScientificNotationFloat(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/stats",
		handler_name: "query_params_43_scientific_notation_float",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { threshold: { annotation: "float", source: "query", type: "number" } },
			required: ["threshold"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_43_scientific_notation_float: queryParams43ScientificNotationFloat,
		},
	};
}

/**
 * Handler for GET /redirect
 */
async function queryParams63FormatUriSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { url: "https://example.com/path?query=value" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams63FormatUriSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/redirect",
		handler_name: "query_params_63_format_uri_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { url: { format: "uri", source: "query", type: "string" } },
			required: ["url"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_63_format_uri_success: queryParams63FormatUriSuccess,
		},
	};
}

/**
 * Handler for GET /query/bool
 */
async function queryParamsBooleanQueryParameterNumeric1(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { flag: true };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsBooleanQueryParameterNumeric1(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/bool",
		handler_name: "query_params_boolean_query_parameter_numeric_1",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { flag: { annotation: "bool", source: "query", type: "boolean" } },
			required: ["flag"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_boolean_query_parameter_numeric_1: queryParamsBooleanQueryParameterNumeric1,
		},
	};
}

/**
 * Handler for GET /query/str-min-length
 */
async function queryParamsStringQueryParamWithMinLengthConstraintFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const name = params.name;
	if (name !== null && name !== undefined) {
		result.name = name;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParamsStringQueryParamWithMinLengthConstraintFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/str-min-length",
		handler_name: "query_params_string_query_param_with_min_length_constraint_fail",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { name: { annotation: "str", minLength: 3, source: "query", type: "string" } },
			required: ["name"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /query/optional
 */
async function queryParamsOptionalStringQueryParameterProvided(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = "foo bar baz";
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsOptionalStringQueryParameterProvided(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/optional",
		handler_name: "query_params_optional_string_query_parameter_provided",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { query: { annotation: "str", source: "query", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_optional_string_query_parameter_provided: queryParamsOptionalStringQueryParameterProvided,
		},
	};
}

/**
 * Handler for GET /query/list
 */
async function queryParamsListOfIntegersMultipleValues(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = [1, 2];
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsListOfIntegersMultipleValues(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/list",
		handler_name: "query_params_list_of_integers_multiple_values",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				device_ids: { annotation: "list[int]", items: { type: "integer" }, source: "query", type: "array" },
			},
			required: ["device_ids"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_list_of_integers_multiple_values: queryParamsListOfIntegersMultipleValues,
		},
	};
}

/**
 * Handler for GET /query/int-lt
 */
async function queryParamsIntegerQueryParamWithLtConstraintValid(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { value: 49 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsIntegerQueryParamWithLtConstraintValid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int-lt",
		handler_name: "query_params_integer_query_param_with_lt_constraint_valid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { value: { annotation: "int", exclusiveMaximum: 50, source: "query", type: "integer" } },
			required: ["value"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_query_param_with_lt_constraint_valid: queryParamsIntegerQueryParamWithLtConstraintValid,
		},
	};
}

/**
 * Handler for GET /items/negative
 */
async function queryParams42NegativeIntegerQueryParam(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { offset: -10 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams42NegativeIntegerQueryParam(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/negative",
		handler_name: "query_params_42_negative_integer_query_param",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { offset: { annotation: "int", source: "query", type: "integer" } },
			required: ["offset"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_42_negative_integer_query_param: queryParams42NegativeIntegerQueryParam,
		},
	};
}

/**
 * Handler for GET /search
 */
async function queryParams46StringMaxlengthValidationFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const term = params.term;
	if (term !== null && term !== undefined) {
		result.term = term;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParams46StringMaxlengthValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/search",
		handler_name: "query_params_46_string_maxlength_validation_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { term: { maxLength: 10, source: "query", type: "string" } },
			required: ["term"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_46_string_maxlength_validation_failure: queryParams46StringMaxlengthValidationFailure,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams56ArrayMaxitemsConstraintFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const tags = params.tags;
	if (tags !== null && tags !== undefined) {
		result.tags = tags;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParams56ArrayMaxitemsConstraintFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_56_array_maxitems_constraint_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { tags: { items: { type: "string" }, maxItems: 5, source: "query", type: "array" } },
			required: ["tags"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_56_array_maxitems_constraint_failure: queryParams56ArrayMaxitemsConstraintFailure,
		},
	};
}

/**
 * Handler for GET /query/pattern
 */
async function queryParamsStringQueryParamWithRegexPatternFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const code = params.code;
	if (code !== null && code !== undefined) {
		result.code = code;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParamsStringQueryParamWithRegexPatternFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/pattern",
		handler_name: "query_params_string_query_param_with_regex_pattern_fail",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { code: { annotation: "str", pattern: "^[0-9]{3,}$", source: "query", type: "string" } },
			required: ["code"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_string_query_param_with_regex_pattern_fail: queryParamsStringQueryParamWithRegexPatternFail,
		},
	};
}

/**
 * Handler for GET /search
 */
async function queryParams44StringMinlengthValidationSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { term: "foo" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams44StringMinlengthValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/search",
		handler_name: "query_params_44_string_minlength_validation_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { term: { minLength: 3, source: "query", type: "string" } },
			required: ["term"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_44_string_minlength_validation_success: queryParams44StringMinlengthValidationSuccess,
		},
	};
}

/**
 * Handler for GET /network
 */
async function queryParams61FormatIpv4Failure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const ip = params.ip;
	if (ip !== null && ip !== undefined) {
		result.ip = ip;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParams61FormatIpv4Failure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/network",
		handler_name: "query_params_61_format_ipv4_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { ip: { format: "ipv4", source: "query", type: "string" } },
			required: ["ip"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_61_format_ipv4_failure: queryParams61FormatIpv4Failure,
		},
	};
}

/**
 * Handler for GET /subscribe
 */
async function queryParams48PatternValidationEmailFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const email = params.email;
	if (email !== null && email !== undefined) {
		result.email = email;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParams48PatternValidationEmailFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/subscribe",
		handler_name: "query_params_48_pattern_validation_email_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				email: { pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$", source: "query", type: "string" },
			},
			required: ["email"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_48_pattern_validation_email_failure: queryParams48PatternValidationEmailFailure,
		},
	};
}

/**
 * Handler for GET /query/int
 */
async function queryParamsRequiredIntegerQueryParameterMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const query = params.query;
	if (query !== null && query !== undefined) {
		result.query = query;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParamsRequiredIntegerQueryParameterMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int",
		handler_name: "query_params_required_integer_query_parameter_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { query: { annotation: "int", source: "query", type: "integer" } },
			required: ["query"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_missing: queryParamsRequiredIntegerQueryParameterMissing,
		},
	};
}

/**
 * Handler for GET /test
 */
async function queryParamsQueryParameterWithSpecialCharactersUrlEncoding(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { email: "x@test.com", special: "&@A.ac" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsQueryParameterWithSpecialCharactersUrlEncoding(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/test",
		handler_name: "query_params_query_parameter_with_special_characters_url_encoding",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				email: { annotation: "str", source: "query", type: "string" },
				special: { annotation: "str", source: "query", type: "string" },
			},
			required: ["email", "special"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /query/list
 */
async function queryParamsListQueryParameterRequiredButMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const deviceIds = params.device_ids;
	if (deviceIds !== null && deviceIds !== undefined) {
		result.device_ids = deviceIds;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParamsListQueryParameterRequiredButMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/list",
		handler_name: "query_params_list_query_parameter_required_but_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				device_ids: { annotation: "list[int]", items: { type: "integer" }, source: "query", type: "array" },
			},
			required: ["device_ids"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_list_query_parameter_required_but_missing: queryParamsListQueryParameterRequiredButMissing,
		},
	};
}

/**
 * Handler for GET /query
 */
async function queryParamsRequiredStringQueryParameterSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = "foo bar baz";
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsRequiredStringQueryParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query",
		handler_name: "query_params_required_string_query_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { query: { annotation: "str", source: "query", type: "string" } },
			required: ["query"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_string_query_parameter_success: queryParamsRequiredStringQueryParameterSuccess,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams66MultipleofConstraintSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { quantity: 15 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams66MultipleofConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_66_multipleof_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { quantity: { multipleOf: 5, source: "query", type: "integer" } },
			required: ["quantity"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_66_multipleof_constraint_success: queryParams66MultipleofConstraintSuccess,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams53IntegerLeConstraintFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const limit = params.limit;
	if (limit !== null && limit !== undefined) {
		result.limit = limit;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParams53IntegerLeConstraintFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_53_integer_le_constraint_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { limit: { maximum: 100, source: "query", type: "integer" } },
			required: ["limit"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_53_integer_le_constraint_failure: queryParams53IntegerLeConstraintFailure,
		},
	};
}

/**
 * Handler for GET /query/multi-type
 */
async function queryParamsMultipleQueryParametersWithDifferentTypes(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { active: true, age: 30, name: "john", score: 95.5 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsMultipleQueryParametersWithDifferentTypes(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/multi-type",
		handler_name: "query_params_multiple_query_parameters_with_different_types",
		request_schema: undefined,
		response_schema: undefined,
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
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_multiple_query_parameters_with_different_types: queryParamsMultipleQueryParametersWithDifferentTypes,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams71ArraySeparatorSemicolon(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { colors: ["red", "green", "blue"] };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams71ArraySeparatorSemicolon(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_71_array_separator_semicolon",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { colors: { items: { type: "string" }, separator: ";", source: "query", type: "array" } },
			required: ["colors"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_71_array_separator_semicolon: queryParams71ArraySeparatorSemicolon,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams70ArraySeparatorPipe(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { tags: ["python", "rust", "typescript"] };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams70ArraySeparatorPipe(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_70_array_separator_pipe",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { tags: { items: { type: "string" }, separator: "|", source: "query", type: "array" } },
			required: ["tags"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_70_array_separator_pipe: queryParams70ArraySeparatorPipe,
		},
	};
}

/**
 * Handler for GET /query/int/default
 */
async function queryParamsIntegerWithDefaultValueNotProvided(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = "foo bar 10";
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsIntegerWithDefaultValueNotProvided(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int/default",
		handler_name: "query_params_integer_with_default_value_not_provided",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { query: { annotation: "int", default: 10, source: "query", type: "integer" } },
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_with_default_value_not_provided: queryParamsIntegerWithDefaultValueNotProvided,
		},
	};
}

/**
 * Handler for GET /query/bool
 */
async function queryParamsBooleanQueryParameterTrue(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { flag: true };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsBooleanQueryParameterTrue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/bool",
		handler_name: "query_params_boolean_query_parameter_true",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { flag: { annotation: "bool", source: "query", type: "boolean" } },
			required: ["flag"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_boolean_query_parameter_true: queryParamsBooleanQueryParameterTrue,
		},
	};
}

/**
 * Handler for GET /query/int-le
 */
async function queryParamsIntegerQueryParamWithLeConstraintBoundary(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { value: 100 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsIntegerQueryParamWithLeConstraintBoundary(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int-le",
		handler_name: "query_params_integer_query_param_with_le_constraint_boundary",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { value: { annotation: "int", maximum: 100, source: "query", type: "integer" } },
			required: ["value"],
			type: "object",
		},
		file_params: undefined,
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

/**
 * Handler for GET /query/float-ge
 */
async function queryParamsFloatQueryParamWithGeConstraintSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { price: 0.01 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsFloatQueryParamWithGeConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/float-ge",
		handler_name: "query_params_float_query_param_with_ge_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { price: { annotation: "float", minimum: 0.01, source: "query", type: "number" } },
			required: ["price"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_float_query_param_with_ge_constraint_success: queryParamsFloatQueryParamWithGeConstraintSuccess,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams51IntegerGeConstraintBoundary(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { offset: 0 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams51IntegerGeConstraintBoundary(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_51_integer_ge_constraint_boundary",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { offset: { minimum: 0, source: "query", type: "integer" } },
			required: ["offset"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_51_integer_ge_constraint_boundary: queryParams51IntegerGeConstraintBoundary,
		},
	};
}

/**
 * Handler for GET /query/int/optional
 */
async function queryParamsOptionalIntegerQueryParameterMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = "foo bar None";
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsOptionalIntegerQueryParameterMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int/optional",
		handler_name: "query_params_optional_integer_query_parameter_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { query: { annotation: "int", source: "query", type: "integer" } },
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_optional_integer_query_parameter_missing: queryParamsOptionalIntegerQueryParameterMissing,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams69ArrayUniqueitemsFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const ids = params.ids;
	if (ids !== null && ids !== undefined) {
		result.ids = ids;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParams69ArrayUniqueitemsFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_69_array_uniqueitems_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { ids: { items: { type: "integer" }, source: "query", type: "array", uniqueItems: true } },
			required: ["ids"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_69_array_uniqueitems_failure: queryParams69ArrayUniqueitemsFailure,
		},
	};
}

/**
 * Handler for GET /search
 */
async function queryParams72ArraySeparatorSpace(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { keywords: ["rust", "web", "framework"] };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams72ArraySeparatorSpace(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/search",
		handler_name: "query_params_72_array_separator_space",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { keywords: { items: { type: "string" }, separator: " ", source: "query", type: "array" } },
			required: ["keywords"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_72_array_separator_space: queryParams72ArraySeparatorSpace,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function queryParamsStringValidationWithRegexFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const itemQuery = params.item_query;
	if (itemQuery !== null && itemQuery !== undefined) {
		result.item_query = itemQuery;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParamsStringValidationWithRegexFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "query_params_string_validation_with_regex_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_query: { annotation: "str", pattern: "^fixedquery$", source: "query", type: "string" } },
			required: ["item_query"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_string_validation_with_regex_failure: queryParamsStringValidationWithRegexFailure,
		},
	};
}

/**
 * Handler for GET /dns
 */
async function queryParams65FormatHostnameSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { host: "api.example.com" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams65FormatHostnameSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/dns",
		handler_name: "query_params_65_format_hostname_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { host: { format: "hostname", source: "query", type: "string" } },
			required: ["host"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_65_format_hostname_success: queryParams65FormatHostnameSuccess,
		},
	};
}

/**
 * Handler for GET /query/basic
 */
async function queryParamsQueryParameterWithUrlEncodedSpace(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { name: "hello world" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsQueryParameterWithUrlEncodedSpace(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/basic",
		handler_name: "query_params_query_parameter_with_url_encoded_space",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { name: { annotation: "str", source: "query", type: "string" } },
			required: ["name"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_query_parameter_with_url_encoded_space: queryParamsQueryParameterWithUrlEncodedSpace,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function queryParamsListOfStringsMultipleValues(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { q: ["foo", "bar"] };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsListOfStringsMultipleValues(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "query_params_list_of_strings_multiple_values",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { q: { annotation: "list[str]", items: { type: "string" }, source: "query", type: "array" } },
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_list_of_strings_multiple_values: queryParamsListOfStringsMultipleValues,
		},
	};
}

/**
 * Handler for GET /query/optional-default
 */
async function queryParamsOptionalQueryParameterWithDefaultValue(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { limit: 10 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsOptionalQueryParameterWithDefaultValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/optional-default",
		handler_name: "query_params_optional_query_parameter_with_default_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { limit: { annotation: "int", default: 10, source: "query", type: "integer" } },
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_optional_query_parameter_with_default_value: queryParamsOptionalQueryParameterWithDefaultValue,
		},
	};
}

/**
 * Handler for GET /network/ipv6
 */
async function queryParams62FormatIpv6Success(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { ip: "2001:0db8:85a3:0000:0000:8a2e:0370:7334" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams62FormatIpv6Success(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/network/ipv6",
		handler_name: "query_params_62_format_ipv6_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { ip: { format: "ipv6", source: "query", type: "string" } },
			required: ["ip"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_62_format_ipv6_success: queryParams62FormatIpv6Success,
		},
	};
}

/**
 * Handler for GET /query/list-default
 */
async function queryParamsArrayQueryParameterSingleValue(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = ["apple"];
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsArrayQueryParameterSingleValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/list-default",
		handler_name: "query_params_array_query_parameter_single_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				tags: { annotation: "list[str]", default: [], items: { type: "string" }, source: "query", type: "array" },
			},
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_array_query_parameter_single_value: queryParamsArrayQueryParameterSingleValue,
		},
	};
}

/**
 * Handler for GET /query/optional
 */
async function queryParamsOptionalStringQueryParameterMissing(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = "foo bar None";
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsOptionalStringQueryParameterMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/optional",
		handler_name: "query_params_optional_string_query_parameter_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: { properties: { query: { annotation: "str", source: "query", type: "string" } }, type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_optional_string_query_parameter_missing: queryParamsOptionalStringQueryParameterMissing,
		},
	};
}

/**
 * Handler for GET /query/datetime
 */
async function queryParamsDatetimeQueryParameterSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { timestamp: "2024-01-15T10:30:00Z" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsDatetimeQueryParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/datetime",
		handler_name: "query_params_datetime_query_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { timestamp: { annotation: "str", format: "date-time", source: "query", type: "string" } },
			required: ["timestamp"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_datetime_query_parameter_success: queryParamsDatetimeQueryParameterSuccess,
		},
	};
}

/**
 * Handler for GET /query/uuid
 */
async function queryParamsUuidQueryParameterInvalidFormat(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const itemId = params.item_id;
	if (itemId !== null && itemId !== undefined) {
		result.item_id = itemId;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParamsUuidQueryParameterInvalidFormat(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/uuid",
		handler_name: "query_params_uuid_query_parameter_invalid_format",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { annotation: "str", format: "uuid", source: "query", type: "string" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_uuid_query_parameter_invalid_format: queryParamsUuidQueryParameterInvalidFormat,
		},
	};
}

/**
 * Handler for GET /query/list-default
 */
async function queryParamsArrayQueryParameterEmptyArray(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppQueryParamsArrayQueryParameterEmptyArray(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/list-default",
		handler_name: "query_params_array_query_parameter_empty_array",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				tags: { annotation: "list[str]", default: [], items: { type: "string" }, source: "query", type: "array" },
			},
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_array_query_parameter_empty_array: queryParamsArrayQueryParameterEmptyArray,
		},
	};
}

/**
 * Handler for GET /query/enum
 */
async function queryParamsEnumQueryParameterSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { model: "alexnet" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsEnumQueryParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/enum",
		handler_name: "query_params_enum_query_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: {
				model: { annotation: "str", enum: ["alexnet", "resnet", "lenet"], source: "query", type: "string" },
			},
			required: ["model"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_enum_query_parameter_success: queryParamsEnumQueryParameterSuccess,
		},
	};
}

/**
 * Handler for GET /query/uuid
 */
async function queryParamsUuidQueryParameterSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item_id: "c892496f-b1fd-4b91-bdb8-b46f92df1716" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParamsUuidQueryParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/uuid",
		handler_name: "query_params_uuid_query_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { item_id: { annotation: "str", format: "uuid", source: "query", type: "string" } },
			required: ["item_id"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_uuid_query_parameter_success: queryParamsUuidQueryParameterSuccess,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams50IntegerGtConstraintFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const limit = params.limit;
	if (limit !== null && limit !== undefined) {
		result.limit = limit;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParams50IntegerGtConstraintFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_50_integer_gt_constraint_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { limit: { exclusiveMinimum: 0, source: "query", type: "integer" } },
			required: ["limit"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_50_integer_gt_constraint_failure: queryParams50IntegerGtConstraintFailure,
		},
	};
}

/**
 * Handler for GET /redirect
 */
async function queryParams64FormatUriFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const url = params.url;
	if (url !== null && url !== undefined) {
		result.url = url;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParams64FormatUriFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/redirect",
		handler_name: "query_params_64_format_uri_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { url: { format: "uri", source: "query", type: "string" } },
			required: ["url"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_64_format_uri_failure: queryParams64FormatUriFailure,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams54ArrayMinitemsConstraintSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { ids: [1, 2, 3] };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams54ArrayMinitemsConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_54_array_minitems_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { ids: { items: { type: "integer" }, minItems: 2, source: "query", type: "array" } },
			required: ["ids"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_54_array_minitems_constraint_success: queryParams54ArrayMinitemsConstraintSuccess,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams55ArrayMinitemsConstraintFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const ids = params.ids;
	if (ids !== null && ids !== undefined) {
		result.ids = ids;
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppQueryParams55ArrayMinitemsConstraintFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_55_array_minitems_constraint_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { ids: { items: { type: "integer" }, minItems: 2, source: "query", type: "array" } },
			required: ["ids"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_55_array_minitems_constraint_failure: queryParams55ArrayMinitemsConstraintFailure,
		},
	};
}

/**
 * Handler for GET /network
 */
async function queryParams60FormatIpv4Success(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { ip: "192.168.1.1" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppQueryParams60FormatIpv4Success(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/network",
		handler_name: "query_params_60_format_ipv4_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {
			properties: { ip: { format: "ipv4", source: "query", type: "string" } },
			required: ["ip"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_60_format_ipv4_success: queryParams60FormatIpv4Success,
		},
	};
}

export function createAppStaticFilesStaticFileServerReturnsTextFile(): SpikardApp {
	const config: ServerConfig = {
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

export function createAppStaticFilesStaticServerReturnsIndexHtmlForDirectory(): SpikardApp {
	const config: ServerConfig = {
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

/**
 * Handler for POST /items/
 */
async function jsonBodiesUuidFieldInvalidFormat(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodiesUuidFieldInvalidFormat(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_uuid_field_invalid_format",
		request_schema: {
			additionalProperties: false,
			properties: { item_id: { format: "uuid", type: "string" }, name: { type: "string" } },
			required: ["name", "item_id"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_uuid_field_invalid_format: jsonBodiesUuidFieldInvalidFormat,
		},
	};
}

/**
 * Handler for POST /api/v1/data
 */
async function jsonBodies44ConstValidationFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodies44ConstValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/api/v1/data",
		handler_name: "json_bodies_44_const_validation_failure",
		request_schema: {
			properties: { data: { type: "string" }, version: { const: "1.0", type: "string" } },
			required: ["version", "data"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_44_const_validation_failure: jsonBodies44ConstValidationFailure,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesBooleanFieldSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { in_stock: true, name: "Item", price: 42.0 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesBooleanFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_boolean_field_success",
		request_schema: {
			additionalProperties: false,
			properties: { in_stock: { type: "boolean" }, name: { type: "string" }, price: { type: "number" } },
			required: ["name", "price", "in_stock"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_boolean_field_success: jsonBodiesBooleanFieldSuccess,
		},
	};
}

/**
 * Handler for POST /items/validated
 */
async function jsonBodiesNumericLeValidationSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { name: "Item", price: 100.0 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesNumericLeValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_numeric_le_validation_success",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, price: { type: "number" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_numeric_le_validation_success: jsonBodiesNumericLeValidationSuccess,
		},
	};
}

/**
 * Handler for POST /items/nested
 */
async function jsonBodiesDeeplyNestedObjects(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {
		name: "Product",
		price: 100.0,
		seller: {
			address: { city: "Springfield", country: { code: "US", name: "USA" }, street: "123 Main St" },
			name: "John Doe",
		},
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesDeeplyNestedObjects(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_deeply_nested_objects: jsonBodiesDeeplyNestedObjects,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesOptionalFieldsOmitted(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { description: null, name: "Foo", price: 35.4, tax: null };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesOptionalFieldsOmitted(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_optional_fields_omitted",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, price: { type: "number" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_optional_fields_omitted: jsonBodiesOptionalFieldsOmitted,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesUuidFieldSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item_id: "c892496f-b1fd-4b91-bdb8-b46f92df1716", name: "Item" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesUuidFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_uuid_field_success",
		request_schema: {
			additionalProperties: false,
			properties: { item_id: { format: "uuid", type: "string" }, name: { type: "string" } },
			required: ["name", "item_id"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_uuid_field_success: jsonBodiesUuidFieldSuccess,
		},
	};
}

/**
 * Handler for POST /events/
 */
async function jsonBodiesDateFieldSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { event_date: "2024-03-15", name: "Conference" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesDateFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/events/",
		handler_name: "json_bodies_date_field_success",
		request_schema: {
			additionalProperties: false,
			properties: { event_date: { type: "string" }, name: { type: "string" } },
			required: ["name", "event_date"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_date_field_success: jsonBodiesDateFieldSuccess,
		},
	};
}

/**
 * Handler for POST /config
 */
async function jsonBodies47MaxpropertiesValidationFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodies47MaxpropertiesValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/config",
		handler_name: "json_bodies_47_maxproperties_validation_failure",
		request_schema: { maxProperties: 3, type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_47_maxproperties_validation_failure: jsonBodies47MaxpropertiesValidationFailure,
		},
	};
}

/**
 * Handler for POST /config
 */
async function jsonBodies46MinpropertiesValidationFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodies46MinpropertiesValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/config",
		handler_name: "json_bodies_46_minproperties_validation_failure",
		request_schema: { minProperties: 2, type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_46_minproperties_validation_failure: jsonBodies46MinpropertiesValidationFailure,
		},
	};
}

/**
 * Handler for POST /items/validated
 */
async function jsonBodiesStringMinLengthValidationFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodiesStringMinLengthValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_string_min_length_validation_fail",
		request_schema: {
			additionalProperties: false,
			properties: { name: { minLength: 3, type: "string" }, price: { type: "number" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_string_min_length_validation_fail: jsonBodiesStringMinLengthValidationFail,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesFieldTypeValidationInvalidType(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodiesFieldTypeValidationInvalidType(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_field_type_validation_invalid_type: jsonBodiesFieldTypeValidationInvalidType,
		},
	};
}

/**
 * Handler for POST /payment
 */
async function jsonBodies36OneofSchemaMultipleMatchFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodies36OneofSchemaMultipleMatchFailure(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_36_oneof_schema_multiple_match_failure: jsonBodies36OneofSchemaMultipleMatchFailure,
		},
	};
}

/**
 * Handler for POST /items/nested
 */
async function jsonBodiesNestedObjectSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {
		image: { name: "Product Image", url: "https://example.com/image.jpg" },
		name: "Foo",
		price: 42.0,
	};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesNestedObjectSuccess(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_nested_object_success: jsonBodiesNestedObjectSuccess,
		},
	};
}

/**
 * Handler for POST /users
 */
async function jsonBodies41NotSchemaSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppJsonBodies41NotSchemaSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_41_not_schema_success",
		request_schema: {
			properties: { username: { not: { enum: ["admin", "root", "system"] }, type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_41_not_schema_success: jsonBodies41NotSchemaSuccess,
		},
	};
}

/**
 * Handler for POST /items/validated
 */
async function jsonBodiesStringMaxLengthValidationFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodiesStringMaxLengthValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_string_max_length_validation_fail",
		request_schema: {
			additionalProperties: false,
			properties: { name: { maxLength: 50, type: "string" }, price: { type: "number" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_string_max_length_validation_fail: jsonBodiesStringMaxLengthValidationFail,
		},
	};
}

/**
 * Handler for POST /data
 */
async function jsonBodies50DeepNesting4Levels(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppJsonBodies50DeepNesting4Levels(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_50_deep_nesting_4_levels: jsonBodies50DeepNesting4Levels,
		},
	};
}

/**
 * Handler for POST /billing
 */
async function jsonBodies48DependenciesValidationSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppJsonBodies48DependenciesValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/billing",
		handler_name: "json_bodies_48_dependencies_validation_success",
		request_schema: {
			dependencies: { credit_card: ["billing_address"] },
			properties: { billing_address: { type: "string" }, credit_card: { type: "string" }, name: { type: "string" } },
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_48_dependencies_validation_success: jsonBodies48DependenciesValidationSuccess,
		},
	};
}

/**
 * Handler for PATCH /items/{id}
 */
async function jsonBodiesPatchPartialUpdate(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { description: "Original description", name: "Original Item", price: 45.0 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesPatchPartialUpdate(): SpikardApp {
	const route: RouteMetadata = {
		method: "PATCH",
		path: "/items/{id}",
		handler_name: "json_bodies_patch_partial_update",
		request_schema: { properties: { price: { type: "number" } }, required: ["price"], type: "object" },
		response_schema: undefined,
		parameter_schema: { properties: { id: { source: "path", type: "string" } }, required: ["id"], type: "object" },
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_patch_partial_update: jsonBodiesPatchPartialUpdate,
		},
	};
}

/**
 * Handler for POST /users
 */
async function jsonBodies30NestedObjectMissingField(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodies30NestedObjectMissingField(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_30_nested_object_missing_field: jsonBodies30NestedObjectMissingField,
		},
	};
}

/**
 * Handler for POST /events/
 */
async function jsonBodiesDatetimeFieldSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { created_at: "2024-03-15T10:30:00Z", name: "Meeting" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesDatetimeFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/events/",
		handler_name: "json_bodies_datetime_field_success",
		request_schema: {
			additionalProperties: false,
			properties: { created_at: { format: "date-time", type: "string" }, name: { type: "string" } },
			required: ["name", "created_at"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_datetime_field_success: jsonBodiesDatetimeFieldSuccess,
		},
	};
}

/**
 * Handler for POST /items/validated
 */
async function jsonBodiesStringPatternValidationSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { name: "Item", sku: "ABC1234" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesStringPatternValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_string_pattern_validation_success",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, sku: { type: "string" } },
			required: ["name", "sku"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_string_pattern_validation_success: jsonBodiesStringPatternValidationSuccess,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesExtraFieldsIgnoredNoAdditionalproperties(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { name: "Item", price: 42.0 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesExtraFieldsIgnoredNoAdditionalproperties(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_extra_fields_ignored_no_additionalproperties: jsonBodiesExtraFieldsIgnoredNoAdditionalproperties,
		},
	};
}

/**
 * Handler for POST /contact
 */
async function jsonBodies40AnyofSchemaFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodies40AnyofSchemaFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/contact",
		handler_name: "json_bodies_40_anyof_schema_failure",
		request_schema: {
			anyOf: [{ required: ["email"] }, { required: ["phone"] }],
			properties: { email: { format: "email", type: "string" }, name: { type: "string" }, phone: { type: "string" } },
			required: ["name"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_40_anyof_schema_failure: jsonBodies40AnyofSchemaFailure,
		},
	};
}

/**
 * Handler for POST /contact
 */
async function jsonBodies39AnyofSchemaMultipleMatchSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppJsonBodies39AnyofSchemaMultipleMatchSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/contact",
		handler_name: "json_bodies_39_anyof_schema_multiple_match_success",
		request_schema: {
			anyOf: [{ required: ["email"] }, { required: ["phone"] }],
			properties: { email: { format: "email", type: "string" }, name: { type: "string" }, phone: { type: "string" } },
			required: ["name"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_39_anyof_schema_multiple_match_success: jsonBodies39AnyofSchemaMultipleMatchSuccess,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesArrayOfPrimitiveValues(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { name: "Product", ratings: [4.5, 4.8, 5.0, 4.2], tags: ["electronics", "gadget", "new"] };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesArrayOfPrimitiveValues(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_array_of_primitive_values: jsonBodiesArrayOfPrimitiveValues,
		},
	};
}

/**
 * Handler for POST /items/validated
 */
async function jsonBodiesNumericGeValidationFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodiesNumericGeValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_numeric_ge_validation_fail",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, price: { minimum: 1, type: "number" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_numeric_ge_validation_fail: jsonBodiesNumericGeValidationFail,
		},
	};
}

/**
 * Handler for POST /payment
 */
async function jsonBodies37OneofSchemaNoMatchFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodies37OneofSchemaNoMatchFailure(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_37_oneof_schema_no_match_failure: jsonBodies37OneofSchemaNoMatchFailure,
		},
	};
}

/**
 * Handler for POST /items/list-validated
 */
async function jsonBodiesEmptyArrayValidationFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodiesEmptyArrayValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/list-validated",
		handler_name: "json_bodies_empty_array_validation_fail",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, tags: { items: {}, minItems: 1, type: "array" } },
			required: ["name", "tags"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_empty_array_validation_fail: jsonBodiesEmptyArrayValidationFail,
		},
	};
}

/**
 * Handler for POST /contact
 */
async function jsonBodies38AnyofSchemaSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppJsonBodies38AnyofSchemaSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/contact",
		handler_name: "json_bodies_38_anyof_schema_success",
		request_schema: {
			anyOf: [{ required: ["email"] }, { required: ["phone"] }],
			properties: { name: { type: "string" } },
			required: ["name"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_38_anyof_schema_success: jsonBodies38AnyofSchemaSuccess,
		},
	};
}

/**
 * Handler for POST /items/optional-all
 */
async function jsonBodiesEmptyJsonObject(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { description: null, name: null, price: null, tax: null };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesEmptyJsonObject(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/optional-all",
		handler_name: "json_bodies_empty_json_object",
		request_schema: { additionalProperties: false, properties: {}, type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_empty_json_object: jsonBodiesEmptyJsonObject,
		},
	};
}

/**
 * Handler for POST /items/validated
 */
async function jsonBodiesStringPatternValidationFail(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodiesStringPatternValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_string_pattern_validation_fail",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, sku: { pattern: "^[A-Z]{3}[0-9]{4}$", type: "string" } },
			required: ["name", "sku"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_string_pattern_validation_fail: jsonBodiesStringPatternValidationFail,
		},
	};
}

/**
 * Handler for POST /billing
 */
async function jsonBodies49DependenciesValidationFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodies49DependenciesValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/billing",
		handler_name: "json_bodies_49_dependencies_validation_failure",
		request_schema: {
			dependencies: { credit_card: ["billing_address"] },
			properties: { billing_address: { type: "string" }, credit_card: { type: "string" }, name: { type: "string" } },
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_49_dependencies_validation_failure: jsonBodies49DependenciesValidationFailure,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesSimpleJsonObjectSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { description: "A very nice Item", name: "Foo", price: 35.4, tax: 3.2 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesSimpleJsonObjectSuccess(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_simple_json_object_success: jsonBodiesSimpleJsonObjectSuccess,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesRequiredFieldMissingValidationError(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodiesRequiredFieldMissingValidationError(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_required_field_missing_validation_error",
		request_schema: {
			additionalProperties: false,
			properties: { description: { type: "string" }, name: { type: "string" }, price: { type: "number" } },
			required: ["description", "price", "name"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_required_field_missing_validation_error: jsonBodiesRequiredFieldMissingValidationError,
		},
	};
}

/**
 * Handler for POST /payment
 */
async function jsonBodies35OneofSchemaSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppJsonBodies35OneofSchemaSuccess(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_35_oneof_schema_success: jsonBodies35OneofSchemaSuccess,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesEnumFieldInvalidValue(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodiesEnumFieldInvalidValue(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_enum_field_invalid_value: jsonBodiesEnumFieldInvalidValue,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesEnumFieldSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { category: "electronics", name: "Item" };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesEnumFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_enum_field_success",
		request_schema: {
			additionalProperties: false,
			properties: { category: { type: "string" }, name: { type: "string" } },
			required: ["name", "category"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_enum_field_success: jsonBodiesEnumFieldSuccess,
		},
	};
}

/**
 * Handler for POST /items
 */
async function jsonBodies33AllofSchemaComposition(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppJsonBodies33AllofSchemaComposition(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items",
		handler_name: "json_bodies_33_allof_schema_composition",
		request_schema: {
			allOf: [
				{ properties: { name: { type: "string" } }, required: ["name"], type: "object" },
				{ properties: { price: { minimum: 0, type: "number" } }, required: ["price"], type: "object" },
			],
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_33_allof_schema_composition: jsonBodies33AllofSchemaComposition,
		},
	};
}

/**
 * Handler for POST /config
 */
async function jsonBodies45MinpropertiesValidationSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppJsonBodies45MinpropertiesValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/config",
		handler_name: "json_bodies_45_minproperties_validation_success",
		request_schema: { minProperties: 2, type: "object" },
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_45_minproperties_validation_success: jsonBodies45MinpropertiesValidationSuccess,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesBodyWithQueryParameters(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { item: { name: "Item", price: 42.0 }, limit: 10 };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesBodyWithQueryParameters(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_body_with_query_parameters",
		request_schema: {
			additionalProperties: false,
			properties: { name: { type: "string" }, price: { type: "number" } },
			required: ["name", "price"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: {
			properties: { limit: { source: "query", type: "integer" } },
			required: ["limit"],
			type: "object",
		},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_body_with_query_parameters: jsonBodiesBodyWithQueryParameters,
		},
	};
}

/**
 * Handler for POST /users
 */
async function jsonBodies42NotSchemaFailure(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodies42NotSchemaFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_42_not_schema_failure",
		request_schema: {
			properties: { username: { not: { enum: ["admin", "root", "system"] }, type: "string" } },
			required: ["username"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_42_not_schema_failure: jsonBodies42NotSchemaFailure,
		},
	};
}

/**
 * Handler for POST /api/v1/data
 */
async function jsonBodies43ConstValidationSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppJsonBodies43ConstValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/api/v1/data",
		handler_name: "json_bodies_43_const_validation_success",
		request_schema: {
			properties: { data: { type: "string" }, version: { const: "1.0", type: "string" } },
			required: ["version", "data"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_43_const_validation_success: jsonBodies43ConstValidationSuccess,
		},
	};
}

/**
 * Handler for POST /products
 */
async function jsonBodies32SchemaRefDefinitions(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppJsonBodies32SchemaRefDefinitions(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_32_schema_ref_definitions: jsonBodies32SchemaRefDefinitions,
		},
	};
}

/**
 * Handler for POST /users
 */
async function jsonBodies29NestedObjectValidationSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppJsonBodies29NestedObjectValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_29_nested_object_validation_success: jsonBodies29NestedObjectValidationSuccess,
		},
	};
}

/**
 * Handler for POST /users
 */
async function jsonBodies34AdditionalPropertiesFalse(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (body !== null && body !== undefined) {
		if (typeof body === "object") {
			Object.assign(result, body);
		} else {
			result.body = body;
		}
	}
	response.body = result;
	return JSON.stringify(response);
}

export function createAppJsonBodies34AdditionalPropertiesFalse(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_34_additional_properties_false",
		request_schema: {
			additionalProperties: false,
			properties: { email: { type: "string" }, name: { type: "string" } },
			required: ["name"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_34_additional_properties_false: jsonBodies34AdditionalPropertiesFalse,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesNullValueForOptionalField(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = { description: null, name: "Item", price: 42.0, tax: null };
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesNullValueForOptionalField(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_null_value_for_optional_field: jsonBodiesNullValueForOptionalField,
		},
	};
}

/**
 * Handler for POST /users
 */
async function jsonBodies31NullablePropertyNullValue(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppJsonBodies31NullablePropertyNullValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_31_nullable_property_null_value",
		request_schema: {
			properties: { description: { type: ["string", "null"] }, name: { type: "string" } },
			required: ["name"],
			type: "object",
		},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_31_nullable_property_null_value: jsonBodies31NullablePropertyNullValue,
		},
	};
}

/**
 * Handler for POST /items/list
 */
async function jsonBodiesArrayOfObjectsSuccess(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
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

export function createAppJsonBodiesArrayOfObjectsSuccess(): SpikardApp {
	const route: RouteMetadata = {
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
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_array_of_objects_success: jsonBodiesArrayOfObjectsSuccess,
		},
	};
}

async function sseHandlerNotifications(_requestJson: string): Promise<StreamingResponse> {
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
			yield `data: ${JSON.stringify(payload)}\n\n`;
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

export function createAppSseNotifications(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/notifications",
		handler_name: "sseHandlerNotifications",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
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
type ChatChannelMessage = z.infer<typeof ChatChannelSchema>;
type ChatChannelMessageResponse = ChatChannelMessage & { validated: true };

async function websocketHandlerChat(message: unknown): Promise<string> {
	const payload: ChatChannelMessage = ChatChannelSchema.parse(normalizeWebsocketPayload(message));
	const response: ChatChannelMessageResponse = { ...payload, validated: true };
	return JSON.stringify(response);
}

export function createAppWebsocketChat(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/chat",
		handler_name: "websocketHandlerChat",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			websocketHandlerChat,
		},
	};
}
// App factory functions:
// - createAppCors07CorsPreflightHeaderNotAllowed() for cors / 07_cors_preflight_header_not_allowed
// - createAppCorsCorsVaryHeaderForProperCaching() for cors / CORS Vary header for proper caching
// - createAppCorsCorsPreflightForPutMethod() for cors / CORS preflight for PUT method
// - createAppCorsCorsPreflightForDeleteMethod() for cors / CORS preflight for DELETE method
// - createAppCorsCorsMultipleAllowedOrigins() for cors / CORS multiple allowed origins
// - createAppCorsCorsPreflightRequest() for cors / CORS preflight request
// - createAppCorsCorsWithCredentials() for cors / CORS with credentials
// - createAppCorsCorsRegexPatternMatchingForOrigins() for cors / CORS regex pattern matching for origins
// - createAppCors08CorsMaxAge() for cors / 08_cors_max_age
// - createAppCors10CorsOriginNull() for cors / 10_cors_origin_null
// - createAppCorsCorsWildcardOrigin() for cors / CORS wildcard origin
// - createAppCorsCorsSafelistedHeadersWithoutPreflight() for cors / CORS safelisted headers without preflight
// - createAppCorsCorsPrivateNetworkAccess() for cors / CORS Private Network Access
// - createAppCorsCorsOriginCaseSensitivity() for cors / CORS origin case sensitivity
// - createAppCorsCorsRequestBlocked() for cors / CORS request blocked
// - createAppCorsSimpleCorsRequest() for cors / Simple CORS request
// - createAppCors09CorsExposeHeaders() for cors / 09_cors_expose_headers
// - createAppCors06CorsPreflightMethodNotAllowed() for cors / 06_cors_preflight_method_not_allowed
// - createAppCookies25CookieSamesiteLax() for cookies / 25_cookie_samesite_lax
// - createAppCookiesOptionalCookieParameterSuccess() for cookies / Optional cookie parameter - success
// - createAppCookiesCookieRegexPatternValidationFail() for cookies / Cookie regex pattern validation - fail
// - createAppCookiesResponseSessionCookieNoMaxAge() for cookies / Response - session cookie (no max_age)
// - createAppCookies27CookieHttponlyFlag() for cookies / 27_cookie_httponly_flag
// - createAppCookiesResponseCookieWithAttributes() for cookies / Response cookie with attributes
// - createAppCookies24CookieSamesiteStrict() for cookies / 24_cookie_samesite_strict
// - createAppCookiesApikeyCookieAuthenticationSuccess() for cookies / APIKey cookie authentication - success
// - createAppCookiesCookieValidationMinLengthConstraintSuccess() for cookies / Cookie validation - min_length constraint success
// - createAppCookiesCookieValidationMinLengthFailure() for cookies / Cookie validation - min_length failure
// - createAppCookiesCookieValidationMaxLengthConstraintFail() for cookies / Cookie validation - max_length constraint fail
// - createAppCookiesRequiredCookieMissing() for cookies / Required cookie - missing
// - createAppCookiesOptionalCookieParameterMissing() for cookies / Optional cookie parameter - missing
// - createAppCookiesApikeyCookieAuthenticationMissing() for cookies / APIKey cookie authentication - missing
// - createAppCookiesResponseMultipleCookies() for cookies / Response - multiple cookies
// - createAppCookiesResponseCookieWithSamesiteLax() for cookies / Response cookie with SameSite=Lax
// - createAppCookiesResponseDeleteCookie() for cookies / Response - delete cookie
// - createAppCookiesResponseCookieWithPathAttribute() for cookies / Response cookie with path attribute
// - createAppCookiesOptionalApikeyCookieMissing() for cookies / Optional APIKey cookie - missing
// - createAppCookiesResponseCookieWithSamesiteStrict() for cookies / Response cookie with SameSite=Strict
// - createAppCookiesResponseCookieWithSamesiteNone() for cookies / Response cookie with SameSite=None
// - createAppCookiesCookieRegexPatternValidationSuccess() for cookies / Cookie regex pattern validation - success
// - createAppCookiesResponseSetCookieBasic() for cookies / Response set cookie - basic
// - createAppCookiesMultipleCookiesSuccess() for cookies / Multiple cookies - success
// - createAppCookies26CookieSecureFlag() for cookies / 26_cookie_secure_flag
// - createAppCookiesResponseCookieWithDomainAttribute() for cookies / Response cookie with domain attribute
// - createAppRequestTimeoutRequestExceedsTimeout() for request_timeout / Request exceeds timeout
// - createAppRequestTimeoutRequestCompletesBeforeTimeout() for request_timeout / Request completes before timeout
// - createAppRequestIdRequestIdHeaderIsPreserved() for request_id / Request ID header is preserved
// - createAppRequestIdRequestIdMiddlewareCanBeDisabled() for request_id / Request ID middleware can be disabled
// - createAppRequestIdRequestIdIsGeneratedWhenNotProvided() for request_id / Request ID is generated when not provided
// - createAppStatusCodes408RequestTimeout() for status_codes / 408 Request Timeout
// - createAppStatusCodes404NotFoundResourceNotFound() for status_codes / 404 Not Found - Resource not found
// - createAppStatusCodes503ServiceUnavailableServerOverload() for status_codes / 503 Service Unavailable - Server overload
// - createAppStatusCodes422UnprocessableEntityValidationError() for status_codes / 422 Unprocessable Entity - Validation error
// - createAppStatusCodes302FoundTemporaryRedirect() for status_codes / 302 Found - Temporary redirect
// - createAppStatusCodes304NotModifiedCachedContentValid() for status_codes / 304 Not Modified - Cached content valid
// - createAppStatusCodes400BadRequestInvalidRequest() for status_codes / 400 Bad Request - Invalid request
// - createAppStatusCodes22501NotImplemented() for status_codes / 22_501_not_implemented
// - createAppStatusCodes204NoContentSuccessWithNoBody() for status_codes / 204 No Content - Success with no body
// - createAppStatusCodes301MovedPermanentlyPermanentRedirect() for status_codes / 301 Moved Permanently - Permanent redirect
// - createAppStatusCodes201CreatedResourceCreated() for status_codes / 201 Created - Resource created
// - createAppStatusCodes202AcceptedRequestAcceptedForProcessing() for status_codes / 202 Accepted - Request accepted for processing
// - createAppStatusCodes307TemporaryRedirectMethodPreserved() for status_codes / 307 Temporary Redirect - Method preserved
// - createAppStatusCodes500InternalServerErrorServerError() for status_codes / 500 Internal Server Error - Server error
// - createAppStatusCodes20414UriTooLong() for status_codes / 20_414_uri_too_long
// - createAppStatusCodes401UnauthorizedMissingAuthentication() for status_codes / 401 Unauthorized - Missing authentication
// - createAppStatusCodes23503ServiceUnavailable() for status_codes / 23_503_service_unavailable
// - createAppStatusCodes19413PayloadTooLarge() for status_codes / 19_413_payload_too_large
// - createAppStatusCodes403ForbiddenInsufficientPermissions() for status_codes / 403 Forbidden - Insufficient permissions
// - createAppStatusCodes21431RequestHeaderFieldsTooLarge() for status_codes / 21_431_request_header_fields_too_large
// - createAppStatusCodes429TooManyRequests() for status_codes / 429 Too Many Requests
// - createAppStatusCodes200OkSuccess() for status_codes / 200 OK - Success
// - createAppStatusCodes206PartialContent() for status_codes / 206 Partial Content
// - createAppHttpMethodsOptionsCorsPreflightRequest() for http_methods / OPTIONS - CORS preflight request
// - createAppHttpMethodsDeleteRemoveResource() for http_methods / DELETE - Remove resource
// - createAppHttpMethodsPutCreateResourceIfDoesnTExist() for http_methods / PUT - Create resource if doesn't exist
// - createAppHttpMethodsPatchUpdateMultipleFields() for http_methods / PATCH - Update multiple fields
// - createAppHttpMethodsPutValidationError() for http_methods / PUT - Validation error
// - createAppHttpMethodsHeadGetMetadataWithoutBody() for http_methods / HEAD - Get metadata without body
// - createAppHttpMethodsDeleteWithResponseBody() for http_methods / DELETE - With response body
// - createAppHttpMethodsPutMissingRequiredField() for http_methods / PUT - Missing required field
// - createAppHttpMethodsPatchPartialUpdate() for http_methods / PATCH - Partial update
// - createAppHttpMethodsDeleteResourceNotFound() for http_methods / DELETE - Resource not found
// - createAppHttpMethodsPutIdempotentOperation() for http_methods / PUT - Idempotent operation
// - createAppHttpMethodsPutCompleteResourceReplacement() for http_methods / PUT - Complete resource replacement
// - createAppUrlEncodedSimpleFormSubmissionSuccess() for url_encoded / Simple form submission - success
// - createAppUrlEncoded15SpecialCharactersFieldNames() for url_encoded / 15_special_characters_field_names
// - createAppUrlEncodedPatternValidationFail() for url_encoded / Pattern validation - fail
// - createAppUrlEncoded22AdditionalPropertiesStrictFailure() for url_encoded / 22_additional_properties_strict_failure
// - createAppUrlEncoded17PatternValidationFailure() for url_encoded / 17_pattern_validation_failure
// - createAppUrlEncoded20FormatEmailValidationFailure() for url_encoded / 20_format_email_validation_failure
// - createAppUrlEncodedMultipleValuesForSameField() for url_encoded / Multiple values for same field
// - createAppUrlEncodedRequiredFieldMissingValidationError() for url_encoded / Required field missing - validation error
// - createAppUrlEncoded13ArrayFieldSuccess() for url_encoded / 13_array_field_success
// - createAppUrlEncodedNumericFieldTypeConversion() for url_encoded / Numeric field type conversion
// - createAppUrlEncodedSpecialCharactersEncoding() for url_encoded / Special characters encoding
// - createAppUrlEncodedBooleanFieldConversion() for url_encoded / Boolean field conversion
// - createAppUrlEncodedEmptyStringValue() for url_encoded / Empty string value
// - createAppUrlEncodedOauth2PasswordGrantFlow() for url_encoded / OAuth2 password grant flow
// - createAppUrlEncoded19ArrayMinitemsValidationFailure() for url_encoded / 19_array_minitems_validation_failure
// - createAppUrlEncodedOptionalFieldMissingSuccess() for url_encoded / Optional field missing - success
// - createAppUrlEncoded14NestedObjectBracketNotation() for url_encoded / 14_nested_object_bracket_notation
// - createAppUrlEncodedStringMaxLengthValidationFail() for url_encoded / String max_length validation - fail
// - createAppUrlEncoded18IntegerMinimumValidationFailure() for url_encoded / 18_integer_minimum_validation_failure
// - createAppUrlEncoded21IntegerTypeCoercionFailure() for url_encoded / 21_integer_type_coercion_failure
// - createAppUrlEncoded16MinlengthValidationFailure() for url_encoded / 16_minlength_validation_failure
// - createAppUrlEncodedStringMinLengthValidationFail() for url_encoded / String min_length validation - fail
// - createAppHeadersHeaderRegexValidationSuccess() for headers / Header regex validation - success
// - createAppHeaders33ApiKeyHeaderValid() for headers / 33_api_key_header_valid
// - createAppHeadersContentTypeHeaderApplicationJson() for headers / Content-Type header - application/json
// - createAppHeadersAcceptLanguageHeader() for headers / Accept-Language header
// - createAppHeadersXApiKeyRequiredHeaderSuccess() for headers / X-API-Key required header - success
// - createAppHeadersHeaderValidationMaxLengthConstraintFail() for headers / Header validation - max_length constraint fail
// - createAppHeadersXApiKeyRequiredHeaderMissing() for headers / X-API-Key required header - missing
// - createAppHeadersOriginHeader() for headers / Origin header
// - createAppHeadersUserAgentHeaderDefaultValue() for headers / User-Agent header - default value
// - createAppHeaders32BearerTokenMissingPrefix() for headers / 32_bearer_token_missing_prefix
// - createAppHeadersOptionalHeaderWithNoneDefaultMissing() for headers / Optional header with None default - missing
// - createAppHeadersHeaderRegexValidationFail() for headers / Header regex validation - fail
// - createAppHeaders31BearerTokenFormatInvalid() for headers / 31_bearer_token_format_invalid
// - createAppHeadersXApiKeyOptionalHeaderSuccess() for headers / X-API-Key optional header - success
// - createAppHeadersAuthorizationHeaderSuccess() for headers / Authorization header - success
// - createAppHeaders30BearerTokenFormatValid() for headers / 30_bearer_token_format_valid
// - createAppHeadersAuthorizationHeaderMissing() for headers / Authorization header - missing
// - createAppHeadersAcceptHeaderJson() for headers / Accept header - JSON
// - createAppHeadersAcceptEncodingHeader() for headers / Accept-Encoding header
// - createAppHeadersAuthorizationHeaderWrongScheme() for headers / Authorization header - wrong scheme
// - createAppHeadersHeaderValidationMinLengthConstraint() for headers / Header validation - min_length constraint
// - createAppHeadersBasicAuthenticationSuccess() for headers / Basic authentication - success
// - createAppHeadersBearerTokenAuthenticationMissing() for headers / Bearer token authentication - missing
// - createAppHeadersXApiKeyOptionalHeaderMissing() for headers / X-API-Key optional header - missing
// - createAppHeadersMultipleHeaderValuesXToken() for headers / Multiple header values - X-Token
// - createAppHeadersMultipleCustomHeaders() for headers / Multiple custom headers
// - createAppHeaders34ApiKeyHeaderInvalid() for headers / 34_api_key_header_invalid
// - createAppHeadersBearerTokenAuthenticationSuccess() for headers / Bearer token authentication - success
// - createAppHeadersHostHeader() for headers / Host header
// - createAppHeadersRefererHeader() for headers / Referer header
// - createAppHeadersHeaderWithUnderscoreConversionExplicit() for headers / Header with underscore conversion - explicit
// - createAppHeadersHeaderCaseInsensitivityAccess() for headers / Header case insensitivity - access
// - createAppHeadersUserAgentHeaderCustomValue() for headers / User-Agent header - custom value
// - createAppMultipartMultipleValuesForSameFieldName() for multipart / Multiple values for same field name
// - createAppMultipart19FileMimeSpoofingPngAsJpeg() for multipart / 19_file_mime_spoofing_png_as_jpeg
// - createAppMultipart20FileMimeSpoofingJpegAsPng() for multipart / 20_file_mime_spoofing_jpeg_as_png
// - createAppMultipart21FilePdfMagicNumberSuccess() for multipart / 21_file_pdf_magic_number_success
// - createAppMultipartContentTypeValidationInvalidType() for multipart / Content-Type validation - invalid type
// - createAppMultipartPdfFileUpload() for multipart / PDF file upload
// - createAppMultipartFileListUploadArrayOfFiles() for multipart / File list upload (array of files)
// - createAppMultipartOptionalFileUploadProvided() for multipart / Optional file upload - provided
// - createAppMultipartFileSizeValidationTooLarge() for multipart / File size validation - too large
// - createAppMultipartMixedFilesAndFormData() for multipart / Mixed files and form data
// - createAppMultipartSimpleFileUpload() for multipart / Simple file upload
// - createAppMultipartEmptyFileUpload() for multipart / Empty file upload
// - createAppMultipartOptionalFileUploadMissing() for multipart / Optional file upload - missing
// - createAppMultipartFileUploadWithoutFilename() for multipart / File upload without filename
// - createAppMultipart18FileMagicNumberJpegSuccess() for multipart / 18_file_magic_number_jpeg_success
// - createAppMultipart22FileEmptyBuffer() for multipart / 22_file_empty_buffer
// - createAppMultipart17FileMagicNumberPngSuccess() for multipart / 17_file_magic_number_png_success
// - createAppMultipartFormDataWithoutFiles() for multipart / Form data without files
// - createAppMultipartMultipleFileUploads() for multipart / Multiple file uploads
// - createAppMultipartFileUploadWithCustomHeaders() for multipart / File upload with custom headers
// - createAppMultipartRequiredFileUploadMissing() for multipart / Required file upload - missing
// - createAppMultipartImageFileUpload() for multipart / Image file upload
// - createAppRateLimitRateLimitBelowThresholdSucceeds() for rate_limit / Rate limit below threshold succeeds
// - createAppRateLimitRateLimitExceededReturns429() for rate_limit / Rate limit exceeded returns 429
// - createAppCompressionCompressionPayloadBelowMinSizeIsNotCompressed() for compression / Compression - payload below min_size is not compressed
// - createAppCompressionCompressionGzipApplied() for compression / Compression - gzip applied
// - createAppAuthJwtMalformedTokenFormat() for auth / JWT malformed token format
// - createAppAuthBearerTokenWithoutPrefix() for auth / Bearer token without prefix
// - createAppAuthJwtAuthenticationValidToken() for auth / JWT authentication - valid token
// - createAppAuthApiKeyRotationOldKeyStillValid() for auth / API key rotation - old key still valid
// - createAppAuthJwtInvalidIssuer() for auth / JWT invalid issuer
// - createAppAuthJwtWithMultipleAudiences() for auth / JWT with multiple audiences
// - createAppAuthApiKeyInQueryParameter() for auth / API key in query parameter
// - createAppAuthJwtAuthenticationExpiredToken() for auth / JWT authentication - expired token
// - createAppAuthApiKeyAuthenticationInvalidKey() for auth / API key authentication - invalid key
// - createAppAuthJwtNotBeforeClaimInFuture() for auth / JWT not before claim in future
// - createAppAuthMultipleAuthenticationSchemesJwtPrecedence() for auth / Multiple authentication schemes - JWT precedence
// - createAppAuthJwtMissingRequiredCustomClaims() for auth / JWT missing required custom claims
// - createAppAuthApiKeyAuthenticationValidKey() for auth / API key authentication - valid key
// - createAppAuthApiKeyWithCustomHeaderName() for auth / API key with custom header name
// - createAppAuthApiKeyAuthenticationMissingHeader() for auth / API key authentication - missing header
// - createAppAuthJwtAuthenticationInvalidSignature() for auth / JWT authentication - invalid signature
// - createAppAuthJwtAuthenticationMissingAuthorizationHeader() for auth / JWT authentication - missing Authorization header
// - createAppAuthJwtAuthenticationInvalidAudience() for auth / JWT authentication - invalid audience
// - createAppPathParamsBooleanPathParameterTrue() for path_params / Boolean path parameter - True
// - createAppPathParams29DecimalPathParamSuccess() for path_params / 29_decimal_path_param_success
// - createAppPathParamsIntegerPathParameterWithCombinedLtAndGtConstraintsSuccess() for path_params / Integer path parameter with combined lt and gt constraints - success
// - createAppPathParams33StringPatternPathSuccess() for path_params / 33_string_pattern_path_success
// - createAppPathParams31StringMinlengthPathFailure() for path_params / 31_string_minlength_path_failure
// - createAppPathParams35NegativeIntegerPathParam() for path_params / 35_negative_integer_path_param
// - createAppPathParamsEnumPathParameterInvalidValue() for path_params / Enum path parameter - invalid value
// - createAppPathParams27DatetimeFormatPathParamSuccess() for path_params / 27_datetime_format_path_param_success
// - createAppPathParams25DateFormatInvalidFailure() for path_params / 25_date_format_invalid_failure
// - createAppPathParamsIntegerPathParameterWithLtConstraintSuccess() for path_params / Integer path parameter with lt constraint - success
// - createAppPathParamsIntegerPathParameterWithGtConstraintSuccess() for path_params / Integer path parameter with gt constraint - success
// - createAppPathParams28DurationFormatPathParamSuccess() for path_params / 28_duration_format_path_param_success
// - createAppPathParamsPathParameterTypeSyntaxWithOverride() for path_params / Path parameter type syntax with override
// - createAppPathParams20UuidV3PathParamSuccess() for path_params / 20_uuid_v3_path_param_success
// - createAppPathParamsIntegerPathParameterInvalidString() for path_params / Integer path parameter - invalid string
// - createAppPathParams30StringMinlengthPathSuccess() for path_params / 30_string_minlength_path_success
// - createAppPathParamsIntegerPathParameterWithLeConstraintSuccess() for path_params / Integer path parameter with le constraint - success
// - createAppPathParamsPathParameterTypeSyntaxInvalidUuid() for path_params / Path parameter type syntax - invalid UUID
// - createAppPathParamsPathTypeParameterFilePath() for path_params / Path type parameter - file path
// - createAppPathParamsPathParameterWithTypeSyntaxUuid() for path_params / Path parameter with type syntax - UUID
// - createAppPathParams32StringMaxlengthPathFailure() for path_params / 32_string_maxlength_path_failure
// - createAppPathParamsIntegerPathParameterSuccess() for path_params / Integer path parameter - success
// - createAppPathParams34StringPatternPathFailure() for path_params / 34_string_pattern_path_failure
// - createAppPathParams21UuidV5PathParamSuccess() for path_params / 21_uuid_v5_path_param_success
// - createAppPathParamsStringPathParameterWithMaxLengthFailure() for path_params / String path parameter with max_length - failure
// - createAppPathParamsStringPathParameterWithMinLengthFailure() for path_params / String path parameter with min_length - failure
// - createAppPathParamsMultiplePathParametersSuccess() for path_params / Multiple path parameters - success
// - createAppPathParamsDatePathParameterSuccess() for path_params / Date path parameter - success
// - createAppPathParamsIntegerPathParameterWithGtConstraintFailure() for path_params / Integer path parameter with gt constraint - failure
// - createAppPathParams24DateFormatPathParamSuccess() for path_params / 24_date_format_path_param_success
// - createAppPathParamsFloatPathParameterSuccess() for path_params / Float path parameter - success
// - createAppPathParamsPathParameterWithTypeSyntaxInteger() for path_params / Path parameter with type syntax - integer
// - createAppPathParamsStringPathParameterSuccess() for path_params / String path parameter - success
// - createAppPathParamsUuidPathParameterSuccess() for path_params / UUID path parameter - success
// - createAppPathParamsIntegerPathParameterWithGeConstraintSuccess() for path_params / Integer path parameter with ge constraint - success
// - createAppPathParamsEnumPathParameterSuccess() for path_params / Enum path parameter - success
// - createAppPathParamsBooleanPathParameterNumeric1() for path_params / Boolean path parameter - numeric 1
// - createAppContentTypes415UnsupportedMediaType() for content_types / 415 Unsupported Media Type
// - createAppContentTypesXmlResponseApplicationXml() for content_types / XML response - application/xml
// - createAppContentTypes14ContentTypeCaseInsensitive() for content_types / 14_content_type_case_insensitive
// - createAppContentTypesJsonWithUtf8Charset() for content_types / JSON with UTF-8 charset
// - createAppContentTypes16TextPlainNotAccepted() for content_types / 16_text_plain_not_accepted
// - createAppContentTypesPdfResponseApplicationPdf() for content_types / PDF response - application/pdf
// - createAppContentTypes20ContentLengthMismatch() for content_types / 20_content_length_mismatch
// - createAppContentTypes17VendorJsonAccepted() for content_types / 17_vendor_json_accepted
// - createAppContentTypes13JsonWithCharsetUtf16() for content_types / 13_json_with_charset_utf16
// - createAppContentTypesJsonResponseApplicationJson() for content_types / JSON response - application/json
// - createAppContentTypes15MultipartBoundaryRequired() for content_types / 15_multipart_boundary_required
// - createAppContentTypesContentNegotiationAcceptHeader() for content_types / Content negotiation - Accept header
// - createAppContentTypesHtmlResponseTextHtml() for content_types / HTML response - text/html
// - createAppContentTypesJpegImageResponseImageJpeg() for content_types / JPEG image response - image/jpeg
// - createAppContentTypes19MissingContentTypeDefaultJson() for content_types / 19_missing_content_type_default_json
// - createAppContentTypesPngImageResponseImagePng() for content_types / PNG image response - image/png
// - createAppContentTypesPlainTextResponseTextPlain() for content_types / Plain text response - text/plain
// - createAppContentTypes18ContentTypeWithMultipleParams() for content_types / 18_content_type_with_multiple_params
// - createAppContentTypesCsvResponseTextCsv() for content_types / CSV response - text/csv
// - createAppContentTypesBinaryResponseApplicationOctetStream() for content_types / Binary response - application/octet-stream
// - createAppBackgroundBackgroundEventLoggingSecondPayload() for background / Background event logging - second payload
// - createAppBackgroundBackgroundEventLogging() for background / Background event logging
// - createAppStreamingStreamJsonLines() for streaming / Stream JSON lines
// - createAppStreamingBinaryLogDownload() for streaming / Binary log download
// - createAppStreamingChunkedCsvExport() for streaming / Chunked CSV export
// - createAppDiRouteLevelDependencyOverrideSuccess() for di / Route-level dependency override - success
// - createAppDiCircularDependencyDetectionError() for di / Circular dependency detection - error
// - createAppDiFactoryDependencySuccess() for di / Factory dependency - success
// - createAppDiValueDependencyInjectionSuccess() for di / Value dependency injection - success
// - createAppDiNodeJsObjectDestructuringInjectionSuccess() for di / Node.js object destructuring injection - success
// - createAppDiNestedDependencies3LevelsSuccess() for di / Nested dependencies (3 levels) - success
// - createAppDiTypeMismatchInDependencyResolutionError() for di / Type mismatch in dependency resolution - error
// - createAppDiMissingDependencyError() for di / Missing dependency - error
// - createAppDiPythonParameterNameBasedInjectionSuccess() for di / Python parameter name-based injection - success
// - createAppDiDependencyInjectionInLifecycleHooksSuccess() for di / Dependency injection in lifecycle hooks - success
// - createAppDiRubyKeywordArgumentInjectionSuccess() for di / Ruby keyword argument injection - success
// - createAppDiMultipleDependenciesWithCleanupSuccess() for di / Multiple dependencies with cleanup - success
// - createAppDiMixedSingletonAndPerRequestCachingSuccess() for di / Mixed singleton and per-request caching - success
// - createAppDiResourceCleanupAfterRequestSuccess() for di / Resource cleanup after request - success
// - createAppDiPythonTypeAnnotationBasedInjectionSuccess() for di / Python type annotation-based injection - success
// - createAppDiPerRequestDependencyCachingSuccess() for di / Per-request dependency caching - success
// - createAppDiSingletonDependencyCachingSuccess() for di / Singleton dependency caching - success
// - createAppDiAsyncFactoryDependencySuccess() for di / Async factory dependency - success
// - createAppBodyLimitsBodyUnderLimitSucceeds() for body_limits / Body under limit succeeds
// - createAppBodyLimitsBodyOverLimitReturns413() for body_limits / Body over limit returns 413
// - createAppValidationErrorsInvalidUuidFormat() for validation_errors / Invalid UUID format
// - createAppValidationErrorsInvalidBooleanValue() for validation_errors / Invalid boolean value
// - createAppValidationErrorsMissingRequiredQueryParameter() for validation_errors / Missing required query parameter
// - createAppValidationErrorsArrayMaxItemsConstraintViolation() for validation_errors / Array max_items constraint violation
// - createAppValidationErrorsNumericConstraintViolationGtGreaterThan() for validation_errors / Numeric constraint violation - gt (greater than)
// - createAppValidationErrorsStringRegexPatternMismatch() for validation_errors / String regex pattern mismatch
// - createAppValidationErrorsInvalidEnumValue() for validation_errors / Invalid enum value
// - createAppValidationErrorsStringMinLengthConstraintViolation() for validation_errors / String min_length constraint violation
// - createAppValidationErrorsMultipleValidationErrors() for validation_errors / Multiple validation errors
// - createAppValidationErrorsStringMaxLengthConstraintViolation() for validation_errors / String max_length constraint violation
// - createAppValidationErrorsNestedObjectValidationError() for validation_errors / Nested object validation error
// - createAppValidationErrors10NestedErrorPath() for validation_errors / 10_nested_error_path
// - createAppValidationErrorsInvalidDatetimeFormat() for validation_errors / Invalid datetime format
// - createAppValidationErrorsArrayItemValidationError() for validation_errors / Array item validation error
// - createAppValidationErrorsMissingRequiredBodyField() for validation_errors / Missing required body field
// - createAppValidationErrorsBodyFieldTypeErrorStringForFloat() for validation_errors / Body field type error - string for float
// - createAppValidationErrorsMalformedJsonBody() for validation_errors / Malformed JSON body
// - createAppValidationErrorsQueryParamTypeErrorStringProvidedForInt() for validation_errors / Query param type error - string provided for int
// - createAppValidationErrorsHeaderValidationError() for validation_errors / Header validation error
// - createAppValidationErrors09MultipleValidationErrors() for validation_errors / 09_multiple_validation_errors
// - createAppValidationErrorsNumericConstraintViolationLeLessThanOrEqual() for validation_errors / Numeric constraint violation - le (less than or equal)
// - createAppValidationErrorsArrayMinItemsConstraintViolation() for validation_errors / Array min_items constraint violation
// - createAppLifecycleHooksOnresponseSecurityHeaders() for lifecycle_hooks / onResponse - Security Headers
// - createAppLifecycleHooksPrehandlerAuthenticationFailedShortCircuit() for lifecycle_hooks / preHandler - Authentication Failed (Short Circuit)
// - createAppLifecycleHooksPrehandlerAuthorizationCheck() for lifecycle_hooks / preHandler - Authorization Check
// - createAppLifecycleHooksPrehandlerAuthenticationSuccess() for lifecycle_hooks / preHandler - Authentication Success
// - createAppLifecycleHooksPrevalidationRateLimitExceededShortCircuit() for lifecycle_hooks / preValidation - Rate Limit Exceeded (Short Circuit)
// - createAppLifecycleHooksOnerrorErrorLogging() for lifecycle_hooks / onError - Error Logging
// - createAppLifecycleHooksMultipleHooksAllPhases() for lifecycle_hooks / Multiple Hooks - All Phases
// - createAppLifecycleHooksHookExecutionOrder() for lifecycle_hooks / Hook Execution Order
// - createAppLifecycleHooksOnresponseResponseTiming() for lifecycle_hooks / onResponse - Response Timing
// - createAppLifecycleHooksPrehandlerAuthorizationForbiddenShortCircuit() for lifecycle_hooks / preHandler - Authorization Forbidden (Short Circuit)
// - createAppLifecycleHooksOnrequestRequestLogging() for lifecycle_hooks / onRequest - Request Logging
// - createAppLifecycleHooksPrevalidationRateLimiting() for lifecycle_hooks / preValidation - Rate Limiting
// - createAppEdgeCases19EmojiInStrings() for edge_cases / 19_emoji_in_strings
// - createAppEdgeCases12PercentEncodedSpecialChars() for edge_cases / 12_percent_encoded_special_chars
// - createAppEdgeCasesSpecialStringValuesAndEscaping() for edge_cases / Special string values and escaping
// - createAppEdgeCases15FloatPrecisionPreservation() for edge_cases / 15_float_precision_preservation
// - createAppEdgeCases13EmptyStringQueryParamPreserved() for edge_cases / 13_empty_string_query_param_preserved
// - createAppEdgeCases24ArrayWithHoles() for edge_cases / 24_array_with_holes
// - createAppEdgeCases21ScientificNotationNumber() for edge_cases / 21_scientific_notation_number
// - createAppEdgeCasesFloatPrecisionAndRounding() for edge_cases / Float precision and rounding
// - createAppEdgeCasesUnicodeAndEmojiHandling() for edge_cases / Unicode and emoji handling
// - createAppEdgeCases17ExtremelyLongString() for edge_cases / 17_extremely_long_string
// - createAppEdgeCases11Utf8QueryParameter() for edge_cases / 11_utf8_query_parameter
// - createAppEdgeCases18UnicodeNormalization() for edge_cases / 18_unicode_normalization
// - createAppEdgeCases20NullByteInString() for edge_cases / 20_null_byte_in_string
// - createAppEdgeCases23DeeplyNestedJsonLimit() for edge_cases / 23_deeply_nested_json_limit
// - createAppEdgeCases14LargeIntegerBoundary() for edge_cases / 14_large_integer_boundary
// - createAppEdgeCases22LeadingZerosInteger() for edge_cases / 22_leading_zeros_integer
// - createAppEdgeCasesLargeIntegerBoundaryValues() for edge_cases / Large integer boundary values
// - createAppEdgeCasesDeeplyNestedStructure10Levels() for edge_cases / Deeply nested structure (10+ levels)
// - createAppEdgeCasesEmptyAndNullValueHandling() for edge_cases / Empty and null value handling
// - createAppEdgeCases16NegativeZeroHandling() for edge_cases / 16_negative_zero_handling
// - createAppQueryParamsStringValidationWithRegexSuccess() for query_params / String validation with regex - success
// - createAppQueryParams49IntegerGtConstraintSuccess() for query_params / 49_integer_gt_constraint_success
// - createAppQueryParamsEnumQueryParameterInvalidValue() for query_params / Enum query parameter - invalid value
// - createAppQueryParams68ArrayUniqueitemsSuccess() for query_params / 68_array_uniqueitems_success
// - createAppQueryParams47PatternValidationEmailSuccess() for query_params / 47_pattern_validation_email_success
// - createAppQueryParamsRequiredIntegerQueryParameterSuccess() for query_params / Required integer query parameter - success
// - createAppQueryParamsRequiredStringQueryParameterMissing() for query_params / Required string query parameter - missing
// - createAppQueryParams57BooleanEmptyStringCoercion() for query_params / 57_boolean_empty_string_coercion
// - createAppQueryParams52IntegerLeConstraintBoundary() for query_params / 52_integer_le_constraint_boundary
// - createAppQueryParamsListWithDefaultEmptyArrayNoValuesProvided() for query_params / List with default empty array - no values provided
// - createAppQueryParamsDateQueryParameterSuccess() for query_params / Date query parameter - success
// - createAppQueryParamsStringQueryParamWithMaxLengthConstraintFail() for query_params / String query param with max_length constraint - fail
// - createAppQueryParams45StringMinlengthValidationFailure() for query_params / 45_string_minlength_validation_failure
// - createAppQueryParamsIntegerWithDefaultValueOverride() for query_params / Integer with default value - override
// - createAppQueryParams67MultipleofConstraintFailure() for query_params / 67_multipleof_constraint_failure
// - createAppQueryParams58FormatEmailSuccess() for query_params / 58_format_email_success
// - createAppQueryParamsIntegerQueryParamWithGeConstraintBoundary() for query_params / Integer query param with ge constraint - boundary
// - createAppQueryParamsIntegerQueryParamWithGtConstraintValid() for query_params / Integer query param with gt constraint - valid
// - createAppQueryParamsRequiredIntegerQueryParameterInvalidType() for query_params / Required integer query parameter - invalid type
// - createAppQueryParamsRequiredIntegerQueryParameterFloatValue() for query_params / Required integer query parameter - float value
// - createAppQueryParamsQueryParameterWithUrlEncodedSpecialCharacters() for query_params / Query parameter with URL encoded special characters
// - createAppQueryParams59FormatEmailFailure() for query_params / 59_format_email_failure
// - createAppQueryParams43ScientificNotationFloat() for query_params / 43_scientific_notation_float
// - createAppQueryParams63FormatUriSuccess() for query_params / 63_format_uri_success
// - createAppQueryParamsBooleanQueryParameterNumeric1() for query_params / Boolean query parameter - numeric 1
// - createAppQueryParamsStringQueryParamWithMinLengthConstraintFail() for query_params / String query param with min_length constraint - fail
// - createAppQueryParamsOptionalStringQueryParameterProvided() for query_params / Optional string query parameter - provided
// - createAppQueryParamsListOfIntegersMultipleValues() for query_params / List of integers - multiple values
// - createAppQueryParamsIntegerQueryParamWithLtConstraintValid() for query_params / Integer query param with lt constraint - valid
// - createAppQueryParams42NegativeIntegerQueryParam() for query_params / 42_negative_integer_query_param
// - createAppQueryParams46StringMaxlengthValidationFailure() for query_params / 46_string_maxlength_validation_failure
// - createAppQueryParams56ArrayMaxitemsConstraintFailure() for query_params / 56_array_maxitems_constraint_failure
// - createAppQueryParamsStringQueryParamWithRegexPatternFail() for query_params / String query param with regex pattern - fail
// - createAppQueryParams44StringMinlengthValidationSuccess() for query_params / 44_string_minlength_validation_success
// - createAppQueryParams61FormatIpv4Failure() for query_params / 61_format_ipv4_failure
// - createAppQueryParams48PatternValidationEmailFailure() for query_params / 48_pattern_validation_email_failure
// - createAppQueryParamsRequiredIntegerQueryParameterMissing() for query_params / Required integer query parameter - missing
// - createAppQueryParamsQueryParameterWithSpecialCharactersUrlEncoding() for query_params / Query parameter with special characters - URL encoding
// - createAppQueryParamsListQueryParameterRequiredButMissing() for query_params / List query parameter - required but missing
// - createAppQueryParamsRequiredStringQueryParameterSuccess() for query_params / Required string query parameter - success
// - createAppQueryParams66MultipleofConstraintSuccess() for query_params / 66_multipleof_constraint_success
// - createAppQueryParams53IntegerLeConstraintFailure() for query_params / 53_integer_le_constraint_failure
// - createAppQueryParamsMultipleQueryParametersWithDifferentTypes() for query_params / Multiple query parameters with different types
// - createAppQueryParams71ArraySeparatorSemicolon() for query_params / 71_array_separator_semicolon
// - createAppQueryParams70ArraySeparatorPipe() for query_params / 70_array_separator_pipe
// - createAppQueryParamsIntegerWithDefaultValueNotProvided() for query_params / Integer with default value - not provided
// - createAppQueryParamsBooleanQueryParameterTrue() for query_params / Boolean query parameter - true
// - createAppQueryParamsIntegerQueryParamWithLeConstraintBoundary() for query_params / Integer query param with le constraint - boundary
// - createAppQueryParamsFloatQueryParamWithGeConstraintSuccess() for query_params / Float query param with ge constraint - success
// - createAppQueryParams51IntegerGeConstraintBoundary() for query_params / 51_integer_ge_constraint_boundary
// - createAppQueryParamsOptionalIntegerQueryParameterMissing() for query_params / Optional integer query parameter - missing
// - createAppQueryParams69ArrayUniqueitemsFailure() for query_params / 69_array_uniqueitems_failure
// - createAppQueryParams72ArraySeparatorSpace() for query_params / 72_array_separator_space
// - createAppQueryParamsStringValidationWithRegexFailure() for query_params / String validation with regex - failure
// - createAppQueryParams65FormatHostnameSuccess() for query_params / 65_format_hostname_success
// - createAppQueryParamsQueryParameterWithUrlEncodedSpace() for query_params / Query parameter with URL encoded space
// - createAppQueryParamsListOfStringsMultipleValues() for query_params / List of strings - multiple values
// - createAppQueryParamsOptionalQueryParameterWithDefaultValue() for query_params / Optional query parameter with default value
// - createAppQueryParams62FormatIpv6Success() for query_params / 62_format_ipv6_success
// - createAppQueryParamsArrayQueryParameterSingleValue() for query_params / Array query parameter - single value
// - createAppQueryParamsOptionalStringQueryParameterMissing() for query_params / Optional string query parameter - missing
// - createAppQueryParamsDatetimeQueryParameterSuccess() for query_params / Datetime query parameter - success
// - createAppQueryParamsUuidQueryParameterInvalidFormat() for query_params / UUID query parameter - invalid format
// - createAppQueryParamsArrayQueryParameterEmptyArray() for query_params / Array query parameter - empty array
// - createAppQueryParamsEnumQueryParameterSuccess() for query_params / Enum query parameter - success
// - createAppQueryParamsUuidQueryParameterSuccess() for query_params / UUID query parameter - success
// - createAppQueryParams50IntegerGtConstraintFailure() for query_params / 50_integer_gt_constraint_failure
// - createAppQueryParams64FormatUriFailure() for query_params / 64_format_uri_failure
// - createAppQueryParams54ArrayMinitemsConstraintSuccess() for query_params / 54_array_minitems_constraint_success
// - createAppQueryParams55ArrayMinitemsConstraintFailure() for query_params / 55_array_minitems_constraint_failure
// - createAppQueryParams60FormatIpv4Success() for query_params / 60_format_ipv4_success
// - createAppStaticFilesStaticFileServerReturnsTextFile() for static_files / Static file server returns text file
// - createAppStaticFilesStaticServerReturnsIndexHtmlForDirectory() for static_files / Static server returns index.html for directory
// - createAppJsonBodiesUuidFieldInvalidFormat() for json_bodies / UUID field - invalid format
// - createAppJsonBodies44ConstValidationFailure() for json_bodies / 44_const_validation_failure
// - createAppJsonBodiesBooleanFieldSuccess() for json_bodies / Boolean field - success
// - createAppJsonBodiesNumericLeValidationSuccess() for json_bodies / Numeric le validation - success
// - createAppJsonBodiesDeeplyNestedObjects() for json_bodies / Deeply nested objects
// - createAppJsonBodiesOptionalFieldsOmitted() for json_bodies / Optional fields - omitted
// - createAppJsonBodiesUuidFieldSuccess() for json_bodies / UUID field - success
// - createAppJsonBodiesDateFieldSuccess() for json_bodies / Date field - success
// - createAppJsonBodies47MaxpropertiesValidationFailure() for json_bodies / 47_maxproperties_validation_failure
// - createAppJsonBodies46MinpropertiesValidationFailure() for json_bodies / 46_minproperties_validation_failure
// - createAppJsonBodiesStringMinLengthValidationFail() for json_bodies / String min_length validation - fail
// - createAppJsonBodiesFieldTypeValidationInvalidType() for json_bodies / Field type validation - invalid type
// - createAppJsonBodies36OneofSchemaMultipleMatchFailure() for json_bodies / 36_oneof_schema_multiple_match_failure
// - createAppJsonBodiesNestedObjectSuccess() for json_bodies / Nested object - success
// - createAppJsonBodies41NotSchemaSuccess() for json_bodies / 41_not_schema_success
// - createAppJsonBodiesStringMaxLengthValidationFail() for json_bodies / String max_length validation - fail
// - createAppJsonBodies50DeepNesting4Levels() for json_bodies / 50_deep_nesting_4_levels
// - createAppJsonBodies48DependenciesValidationSuccess() for json_bodies / 48_dependencies_validation_success
// - createAppJsonBodiesPatchPartialUpdate() for json_bodies / PATCH partial update
// - createAppJsonBodies30NestedObjectMissingField() for json_bodies / 30_nested_object_missing_field
// - createAppJsonBodiesDatetimeFieldSuccess() for json_bodies / Datetime field - success
// - createAppJsonBodiesStringPatternValidationSuccess() for json_bodies / String pattern validation - success
// - createAppJsonBodiesExtraFieldsIgnoredNoAdditionalproperties() for json_bodies / Extra fields ignored (no additionalProperties)
// - createAppJsonBodies40AnyofSchemaFailure() for json_bodies / 40_anyof_schema_failure
// - createAppJsonBodies39AnyofSchemaMultipleMatchSuccess() for json_bodies / 39_anyof_schema_multiple_match_success
// - createAppJsonBodiesArrayOfPrimitiveValues() for json_bodies / Array of primitive values
// - createAppJsonBodiesNumericGeValidationFail() for json_bodies / Numeric ge validation - fail
// - createAppJsonBodies37OneofSchemaNoMatchFailure() for json_bodies / 37_oneof_schema_no_match_failure
// - createAppJsonBodiesEmptyArrayValidationFail() for json_bodies / Empty array validation - fail
// - createAppJsonBodies38AnyofSchemaSuccess() for json_bodies / 38_anyof_schema_success
// - createAppJsonBodiesEmptyJsonObject() for json_bodies / Empty JSON object
// - createAppJsonBodiesStringPatternValidationFail() for json_bodies / String pattern validation - fail
// - createAppJsonBodies49DependenciesValidationFailure() for json_bodies / 49_dependencies_validation_failure
// - createAppJsonBodiesSimpleJsonObjectSuccess() for json_bodies / Simple JSON object - success
// - createAppJsonBodiesRequiredFieldMissingValidationError() for json_bodies / Required field missing - validation error
// - createAppJsonBodies35OneofSchemaSuccess() for json_bodies / 35_oneof_schema_success
// - createAppJsonBodiesEnumFieldInvalidValue() for json_bodies / Enum field - invalid value
// - createAppJsonBodiesEnumFieldSuccess() for json_bodies / Enum field - success
// - createAppJsonBodies33AllofSchemaComposition() for json_bodies / 33_allof_schema_composition
// - createAppJsonBodies45MinpropertiesValidationSuccess() for json_bodies / 45_minproperties_validation_success
// - createAppJsonBodiesBodyWithQueryParameters() for json_bodies / Body with query parameters
// - createAppJsonBodies42NotSchemaFailure() for json_bodies / 42_not_schema_failure
// - createAppJsonBodies43ConstValidationSuccess() for json_bodies / 43_const_validation_success
// - createAppJsonBodies32SchemaRefDefinitions() for json_bodies / 32_schema_ref_definitions
// - createAppJsonBodies29NestedObjectValidationSuccess() for json_bodies / 29_nested_object_validation_success
// - createAppJsonBodies34AdditionalPropertiesFalse() for json_bodies / 34_additional_properties_false
// - createAppJsonBodiesNullValueForOptionalField() for json_bodies / Null value for optional field
// - createAppJsonBodies31NullablePropertyNullValue() for json_bodies / 31_nullable_property_null_value
// - createAppJsonBodiesArrayOfObjectsSuccess() for json_bodies / Array of objects - success
// - createAppSseNotifications() for asyncapi_sse / /notifications
// - createAppWebsocketChat() for asyncapi_websocket / /chat

export {
	UserJoinedMessageSchema,
	SystemAlertMessageSchema,
	ChatAckMessageSchema,
	StatusUpdateMessageSchema,
	NotificationBatchMessageSchema,
	ChatMessageMessageSchema,
	UserLeftMessageSchema,
	UserNotificationMessageSchema,
};
export type {
	UserJoinedMessage,
	SystemAlertMessage,
	ChatAckMessage,
	StatusUpdateMessage,
	NotificationBatchMessage,
	ChatMessageMessage,
	UserLeftMessage,
	UserNotificationMessage,
};
