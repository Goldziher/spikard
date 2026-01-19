/**
 * Generated E2E test application with per-fixture app factories.
 * @generated
 */

import { StreamingResponse, background, Spikard } from "@spikard/wasm";
import type {
	RouteMetadata,
	SpikardApp,
	ServerConfig,
} from "@spikard/wasm";
import { Buffer } from "node:buffer";
import { z } from "zod";

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

export type GrpcRequest = {
	serviceName: string;
	methodName: string;
	payload: Buffer;
	metadata?: Record<string, string>;
};
export type GrpcResponse = {
	payload: Buffer;
	metadata?: Record<string, string>;
	statusCode: string;
};

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
	if (typeof Buffer !== "undefined" && Buffer.isBuffer(message)) {
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

type HandlerContext = { signal?: unknown };
type AbortSignalLike = {
	aborted: boolean;
	addEventListener: (type: "abort", listener: () => void, options?: { once?: boolean } | boolean) => void;
	removeEventListener: (type: "abort", listener: () => void) => void;
};

function isAbortSignalLike(value: unknown): value is AbortSignalLike {
	if (!value || typeof value !== "object") return false;
	const candidate = value as AbortSignalLike;
	return (
		typeof candidate.aborted === "boolean" &&
		typeof candidate.addEventListener === "function" &&
		typeof candidate.removeEventListener === "function"
	);
}

function sleep(ms: number, signal?: AbortSignalLike): Promise<void> {
	return new Promise((resolve) => {
		const id = setTimeout(resolve, ms);
		if (!signal) return;
		const onAbort = () => {
			clearTimeout(id);
			signal.removeEventListener("abort", onAbort);
			resolve();
		};
		if (signal.aborted) {
			onAbort();
			return;
		}
		signal.addEventListener("abort", onAbort, { once: true });
	});
}

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

const NotificationBatchMessageSchema = z.array(z.object({
  message: z.string(),
  timestamp: z.string(),
  type: z.string(),
}));

type NotificationBatchMessage = {
  message: string;
  timestamp: string;
  type: string;
}[];

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

const BACKGROUND_STATE: Record<string, unknown[]> = {};

// Cleanup state tracking for DI fixtures
const CLEANUP_STATE: Record<string, string[]> = {};

/**
 * Handler for GET /rate-limit/basic
 */
async function rateLimitRateLimitBelowThresholdSucceeds(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"status":"ok","request":"under-limit"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppRateLimitRateLimitBelowThresholdSucceeds(): SpikardApp {
	const config: ServerConfig = {
		rateLimit: {
			perSecond: 5,
			burst: 5,
			ipBased: false
		}
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
			rate_limit_rate_limit_below_threshold_succeeds: rateLimitRateLimitBelowThresholdSucceeds
		},
		config,
	};
}


/**
 * Handler for GET /rate-limit/exceeded
 */
async function rateLimitRateLimitExceededReturns429(requestJson: string, _context?: HandlerContext): Promise<string> {
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
			ipBased: false
		}
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
			rate_limit_rate_limit_exceeded_returns_429: rateLimitRateLimitExceededReturns429
		},
		config,
	};
}


/**
 * Handler for GET /items/{item_id}
 */
async function validationErrorsInvalidUuidFormat(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const itemId = _params.item_id;
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"string","format":"uuid","source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_uuid_format: validationErrorsInvalidUuidFormat
		},
	};
}


/**
 * Handler for GET /items/
 */
async function validationErrorsInvalidBooleanValue(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = _params.q;
	const isActive = _params.is_active;
	if (q !== null && q !== undefined) {
		result.q = q;
	}
	if (isActive !== null && isActive !== undefined) {
		result.is_active = isActive;
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
		parameter_schema: {"type":"object","properties":{"q":{"type":"string","source":"query"},"is_active":{"type":"boolean","source":"query"}},"required":["q","is_active"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_boolean_value: validationErrorsInvalidBooleanValue
		},
	};
}


/**
 * Handler for GET /items/
 */
async function validationErrorsMissingRequiredQueryParameter(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = _params.q;
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
		parameter_schema: {"type":"object","properties":{"q":{"type":"string","source":"query"}},"required":["q"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_missing_required_query_parameter: validationErrorsMissingRequiredQueryParameter
		},
	};
}


/**
 * Handler for POST /items/
 */
async function validationErrorsArrayMaxItemsConstraintViolation(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"tags":{"type":"array","items":{"type":"string"},"maxItems":10}},"additionalProperties":false,"required":["name","price","tags"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_array_max_items_constraint_violation: validationErrorsArrayMaxItemsConstraintViolation
		},
	};
}


/**
 * Handler for GET /items/
 */
async function validationErrorsNumericConstraintViolationGtGreaterThan(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = _params.q;
	const price = _params.price;
	if (q !== null && q !== undefined) {
		result.q = q;
	}
	if (price !== null && price !== undefined) {
		result.price = price;
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
		parameter_schema: {"type":"object","properties":{"q":{"type":"string","source":"query"},"price":{"type":"number","exclusiveMinimum":0,"source":"query"}},"required":["q","price"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_numeric_constraint_violation_gt_greater_than: validationErrorsNumericConstraintViolationGtGreaterThan
		},
	};
}


/**
 * Handler for GET /items/
 */
async function validationErrorsStringRegexPatternMismatch(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = _params.q;
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
		parameter_schema: {"type":"object","properties":{"q":{"type":"string","pattern":"^[a-zA-Z0-9_-]+$","source":"query"}},"required":["q"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_string_regex_pattern_mismatch: validationErrorsStringRegexPatternMismatch
		},
	};
}


/**
 * Handler for GET /models/{model_name}
 */
async function validationErrorsInvalidEnumValue(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const modelName = _params.model_name;
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
		parameter_schema: {"type":"object","properties":{"model_name":{"type":"string","enum":["alexnet","resnet","lenet"],"source":"path"}},"required":["model_name"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_enum_value: validationErrorsInvalidEnumValue
		},
	};
}


/**
 * Handler for GET /items/
 */
async function validationErrorsStringMinLengthConstraintViolation(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = _params.q;
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
		parameter_schema: {"type":"object","properties":{"q":{"type":"string","minLength":3,"source":"query"}},"required":["q"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_string_min_length_constraint_violation: validationErrorsStringMinLengthConstraintViolation
		},
	};
}


/**
 * Handler for POST /items/
 */
async function validationErrorsMultipleValidationErrors(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string","minLength":3},"price":{"type":"integer","exclusiveMinimum":0},"quantity":{"type":"integer"}},"additionalProperties":false,"required":["name","price","quantity"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_multiple_validation_errors: validationErrorsMultipleValidationErrors
		},
	};
}


/**
 * Handler for GET /items/
 */
async function validationErrorsStringMaxLengthConstraintViolation(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = _params.q;
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
		parameter_schema: {"type":"object","properties":{"q":{"type":"string","maxLength":50,"source":"query"}},"required":["q"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_string_max_length_constraint_violation: validationErrorsStringMaxLengthConstraintViolation
		},
	};
}


/**
 * Handler for POST /items/
 */
async function validationErrorsNestedObjectValidationError(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"seller":{"type":"object","properties":{"name":{"type":"string","minLength":3},"address":{"type":"object","properties":{"city":{"type":"string","minLength":3},"zip_code":{"type":"string","minLength":5}},"additionalProperties":false,"required":["city","zip_code"]}},"additionalProperties":false,"required":["name","address"]}},"additionalProperties":false,"required":["name","price","seller"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_nested_object_validation_error: validationErrorsNestedObjectValidationError
		},
	};
}


/**
 * Handler for POST /profiles
 */
async function validationErrors10NestedErrorPath(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["profile"],"properties":{"profile":{"type":"object","required":["contact"],"properties":{"contact":{"type":"object","required":["email"],"properties":{"email":{"type":"string","format":"email"}}}}}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_10_nested_error_path: validationErrors10NestedErrorPath
		},
	};
}


/**
 * Handler for POST /items/
 */
async function validationErrorsInvalidDatetimeFormat(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"created_at":{"type":"string","format":"date-time"}},"additionalProperties":false,"required":["name","price","created_at"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_datetime_format: validationErrorsInvalidDatetimeFormat
		},
	};
}


/**
 * Handler for POST /items/
 */
async function validationErrorsArrayItemValidationError(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"tags":{"type":"array","items":{"type":"string"}}},"additionalProperties":false,"required":["name","price","tags"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_array_item_validation_error: validationErrorsArrayItemValidationError
		},
	};
}


/**
 * Handler for POST /items/
 */
async function validationErrorsMissingRequiredBodyField(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"string"}},"additionalProperties":false,"required":["name","price"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_missing_required_body_field: validationErrorsMissingRequiredBodyField
		},
	};
}


/**
 * Handler for POST /items/
 */
async function validationErrorsBodyFieldTypeErrorStringForFloat(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"}},"additionalProperties":false,"required":["name","price"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_body_field_type_error_string_for_float: validationErrorsBodyFieldTypeErrorStringForFloat
		},
	};
}


/**
 * Handler for POST /items/
 */
async function validationErrorsMalformedJsonBody(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 400 };
	const responseBody = {"detail":"Invalid request format"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppValidationErrorsMalformedJsonBody(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_malformed_json_body",
		request_schema: {"type":"string"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_malformed_json_body: validationErrorsMalformedJsonBody
		},
	};
}


/**
 * Handler for GET /items/
 */
async function validationErrorsQueryParamTypeErrorStringProvidedForInt(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = _params.q;
	const skip = _params.skip;
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
		parameter_schema: {"type":"object","properties":{"q":{"type":"string","source":"query"},"skip":{"type":"integer","source":"query"}},"required":["q","skip"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_query_param_type_error_string_provided_for_int: validationErrorsQueryParamTypeErrorStringProvidedForInt
		},
	};
}


/**
 * Handler for GET /items/
 */
async function validationErrorsHeaderValidationError(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = _params.q;
	const xToken = _params["x-token"];
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
		parameter_schema: {"type":"object","properties":{"q":{"type":"string","source":"query"},"x-token":{"type":"string","source":"header"}},"required":["q","x-token"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_header_validation_error: validationErrorsHeaderValidationError
		},
	};
}


/**
 * Handler for POST /users
 */
async function validationErrors09MultipleValidationErrors(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["name","email","age"],"properties":{"name":{"type":"string","minLength":3},"email":{"type":"string","format":"email"},"age":{"type":"integer","minimum":18}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_09_multiple_validation_errors: validationErrors09MultipleValidationErrors
		},
	};
}


/**
 * Handler for GET /items/
 */
async function validationErrorsNumericConstraintViolationLeLessThanOrEqual(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const q = _params.q;
	const limit = _params.limit;
	if (q !== null && q !== undefined) {
		result.q = q;
	}
	if (limit !== null && limit !== undefined) {
		result.limit = limit;
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
		parameter_schema: {"type":"object","properties":{"q":{"type":"string","source":"query"},"limit":{"type":"integer","maximum":100,"source":"query"}},"required":["q","limit"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_numeric_constraint_violation_le_less_than_or_equal: validationErrorsNumericConstraintViolationLeLessThanOrEqual
		},
	};
}


/**
 * Handler for POST /items/
 */
async function validationErrorsArrayMinItemsConstraintViolation(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"tags":{"type":"array","items":{},"minItems":1}},"additionalProperties":false,"required":["name","price","tags"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_array_min_items_constraint_violation: validationErrorsArrayMinItemsConstraintViolation
		},
	};
}


/**
 * Handler for GET /api/protected
 */
async function authJwtMalformedTokenFormat(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {"type":"https://spikard.dev/errors/unauthorized","title":"Malformed JWT token","status":401,"detail":"Malformed JWT token: expected 3 parts separated by dots, found 2"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtMalformedTokenFormat(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
		secret: "test-secret-key-do-not-use-in-production",
		algorithm: "HS256"
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_jwt_malformed_token_format",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","source":"header","description":"JWT token in Bearer format"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_jwt_malformed_token_format: authJwtMalformedTokenFormat
		},
		config,
	};
}


/**
 * Handler for GET /api/protected
 */
async function authBearerTokenWithoutPrefix(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {"type":"https://spikard.dev/errors/unauthorized","title":"Invalid Authorization header format","status":401,"detail":"Authorization header must use Bearer scheme: 'Bearer <token>'"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthBearerTokenWithoutPrefix(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
		secret: "test-secret-key-do-not-use-in-production",
		algorithm: "HS256"
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_bearer_token_without_prefix",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","source":"header","description":"JWT token in Bearer format"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_bearer_token_without_prefix: authBearerTokenWithoutPrefix
		},
		config,
	};
}


/**
 * Handler for GET /protected/user
 */
async function authJwtAuthenticationValidToken(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Access granted","user_id":"user123"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtAuthenticationValidToken(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
		secret: "test-secret-key-do-not-use-in-production",
		algorithm: "HS256",
		audience: ["https://api.example.com"],
		issuer: "https://auth.example.com"
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_valid_token",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","source":"header","description":"JWT token in Bearer format"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_jwt_authentication_valid_token: authJwtAuthenticationValidToken
		},
		config,
	};
}


/**
 * Handler for GET /api/data
 */
async function authApiKeyRotationOldKeyStillValid(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"x-api-key-deprecated":"true"};
	const responseBody = {"message":"Access granted","data":"sensitive information"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthApiKeyRotationOldKeyStillValid(): SpikardApp {
	const config: ServerConfig = {
		apiKeyAuth: {
		keys: ["sk_test_old_123456","sk_test_new_789012"],
		headerName: "X-API-Key"
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_rotation_old_key_still_valid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"X-API-Key":{"type":"string","source":"header","description":"API key for authentication"}},"required":["X-API-Key"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_api_key_rotation_old_key_still_valid: authApiKeyRotationOldKeyStillValid
		},
		config,
	};
}


/**
 * Handler for GET /api/protected
 */
async function authJwtInvalidIssuer(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {"type":"https://spikard.dev/errors/unauthorized","title":"JWT validation failed","status":401,"detail":"Token issuer is invalid, expected 'https://auth.example.com'"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtInvalidIssuer(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
		secret: "test-secret-key-do-not-use-in-production",
		algorithm: "HS256",
		issuer: "https://auth.example.com",
		leeway: 0
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_jwt_invalid_issuer",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","source":"header","description":"JWT token in Bearer format"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_jwt_invalid_issuer: authJwtInvalidIssuer
		},
		config,
	};
}


/**
 * Handler for GET /api/protected
 */
async function authJwtWithMultipleAudiences(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Access granted","user_id":"user123"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtWithMultipleAudiences(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
		secret: "test-secret-key-do-not-use-in-production",
		algorithm: "HS256",
		audience: ["https://api.example.com"],
		issuer: "https://auth.example.com"
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_jwt_with_multiple_audiences",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","source":"header","description":"JWT token in Bearer format"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_jwt_with_multiple_audiences: authJwtWithMultipleAudiences
		},
		config,
	};
}


/**
 * Handler for GET /api/data
 */
async function authApiKeyInQueryParameter(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Access granted","data":"sensitive information"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthApiKeyInQueryParameter(): SpikardApp {
	const config: ServerConfig = {
		apiKeyAuth: {
		keys: ["sk_test_123456","sk_test_789012"],
		headerName: "X-API-Key"
		}
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
			auth_api_key_in_query_parameter: authApiKeyInQueryParameter
		},
		config,
	};
}


/**
 * Handler for GET /protected/user
 */
async function authJwtAuthenticationExpiredToken(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {"type":"https://spikard.dev/errors/unauthorized","title":"JWT validation failed","status":401,"detail":"Token has expired"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtAuthenticationExpiredToken(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
		secret: "test-secret-key-do-not-use-in-production",
		algorithm: "HS256"
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_expired_token",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","source":"header"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_jwt_authentication_expired_token: authJwtAuthenticationExpiredToken
		},
		config,
	};
}


/**
 * Handler for GET /api/data
 */
async function authApiKeyAuthenticationInvalidKey(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {"type":"https://spikard.dev/errors/unauthorized","title":"Invalid API key","status":401,"detail":"The provided API key is not valid"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthApiKeyAuthenticationInvalidKey(): SpikardApp {
	const config: ServerConfig = {
		apiKeyAuth: {
		keys: ["sk_test_123456","sk_test_789012"],
		headerName: "X-API-Key"
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_authentication_invalid_key",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"X-API-Key":{"type":"string","source":"header"}},"required":["X-API-Key"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_api_key_authentication_invalid_key: authApiKeyAuthenticationInvalidKey
		},
		config,
	};
}


/**
 * Handler for GET /api/protected
 */
async function authJwtNotBeforeClaimInFuture(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {"type":"https://spikard.dev/errors/unauthorized","title":"JWT validation failed","status":401,"detail":"JWT not valid yet, not before claim is in the future"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtNotBeforeClaimInFuture(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
		secret: "test-secret-key-do-not-use-in-production",
		algorithm: "HS256",
		leeway: 0
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/protected",
		handler_name: "auth_jwt_not_before_claim_in_future",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","source":"header","description":"JWT token in Bearer format"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_jwt_not_before_claim_in_future: authJwtNotBeforeClaimInFuture
		},
		config,
	};
}


/**
 * Handler for GET /api/data
 */
async function authMultipleAuthenticationSchemesJwtPrecedence(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Access granted","user_id":"user123","auth_method":"jwt"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthMultipleAuthenticationSchemesJwtPrecedence(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
		secret: "test-secret-key-do-not-use-in-production",
		algorithm: "HS256",
		audience: ["https://api.example.com"],
		issuer: "https://auth.example.com"
		},
		apiKeyAuth: {
		keys: ["sk_test_123456","sk_test_789012"],
		headerName: "X-API-Key"
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_multiple_authentication_schemes_jwt_precedence",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","source":"header","description":"JWT token in Bearer format"},"X-API-Key":{"type":"string","source":"header","description":"API key for authentication"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_multiple_authentication_schemes_jwt_precedence: authMultipleAuthenticationSchemesJwtPrecedence
		},
		config,
	};
}


/**
 * Handler for GET /api/admin
 */
async function authJwtMissingRequiredCustomClaims(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 403 };
	const responseBody = {"type":"https://spikard.dev/errors/forbidden","title":"Forbidden","status":403,"detail":"Required claims 'role' and 'permissions' missing from JWT"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtMissingRequiredCustomClaims(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
		secret: "test-secret-key-do-not-use-in-production",
		algorithm: "HS256",
		audience: ["https://api.example.com"],
		issuer: "https://auth.example.com"
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/admin",
		handler_name: "auth_jwt_missing_required_custom_claims",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","source":"header","description":"JWT token in Bearer format"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_jwt_missing_required_custom_claims: authJwtMissingRequiredCustomClaims
		},
		config,
	};
}


/**
 * Handler for GET /api/data
 */
async function authApiKeyAuthenticationValidKey(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Access granted","data":"sensitive information"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthApiKeyAuthenticationValidKey(): SpikardApp {
	const config: ServerConfig = {
		apiKeyAuth: {
		keys: ["sk_test_123456","sk_test_789012"],
		headerName: "X-API-Key"
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_authentication_valid_key",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"X-API-Key":{"type":"string","source":"header","description":"API key for authentication"}},"required":["X-API-Key"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_api_key_authentication_valid_key: authApiKeyAuthenticationValidKey
		},
		config,
	};
}


/**
 * Handler for GET /api/data
 */
async function authApiKeyWithCustomHeaderName(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Access granted","data":"sensitive information"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthApiKeyWithCustomHeaderName(): SpikardApp {
	const config: ServerConfig = {
		apiKeyAuth: {
		keys: ["sk_test_123456","sk_test_789012"],
		headerName: "X-API-Token"
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_with_custom_header_name",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"X-API-Token":{"type":"string","source":"header","description":"API token for authentication"}},"required":["X-API-Token"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_api_key_with_custom_header_name: authApiKeyWithCustomHeaderName
		},
		config,
	};
}


/**
 * Handler for GET /api/data
 */
async function authApiKeyAuthenticationMissingHeader(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {"type":"https://spikard.dev/errors/unauthorized","title":"Missing API key","status":401,"detail":"Expected 'X-API-Key' header or 'api_key' query parameter with valid API key"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthApiKeyAuthenticationMissingHeader(): SpikardApp {
	const config: ServerConfig = {
		apiKeyAuth: {
		keys: ["sk_test_123456","sk_test_789012"],
		headerName: "X-API-Key"
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "auth_api_key_authentication_missing_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_api_key_authentication_missing_header: authApiKeyAuthenticationMissingHeader
		},
		config,
	};
}


/**
 * Handler for GET /protected/user
 */
async function authJwtAuthenticationInvalidSignature(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {"type":"https://spikard.dev/errors/unauthorized","title":"JWT validation failed","status":401,"detail":"Token signature is invalid"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtAuthenticationInvalidSignature(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
		secret: "test-secret-key-do-not-use-in-production",
		algorithm: "HS256"
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_invalid_signature",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","source":"header"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_jwt_authentication_invalid_signature: authJwtAuthenticationInvalidSignature
		},
		config,
	};
}


/**
 * Handler for GET /protected/user
 */
async function authJwtAuthenticationMissingAuthorizationHeader(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {"type":"https://spikard.dev/errors/unauthorized","title":"Missing or invalid Authorization header","status":401,"detail":"Expected 'Authorization: Bearer <token>'"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtAuthenticationMissingAuthorizationHeader(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
		secret: "test-secret-key-do-not-use-in-production",
		algorithm: "HS256"
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_missing_authorization_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_jwt_authentication_missing_authorization_header: authJwtAuthenticationMissingAuthorizationHeader
		},
		config,
	};
}


/**
 * Handler for GET /protected/user
 */
async function authJwtAuthenticationInvalidAudience(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {"type":"https://spikard.dev/errors/unauthorized","title":"JWT validation failed","status":401,"detail":"Token audience is invalid"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppAuthJwtAuthenticationInvalidAudience(): SpikardApp {
	const config: ServerConfig = {
		jwtAuth: {
		secret: "test-secret-key-do-not-use-in-production",
		algorithm: "HS256",
		audience: ["https://api.example.com"]
		}
	};

	const route: RouteMetadata = {
		method: "GET",
		path: "/protected/user",
		handler_name: "auth_jwt_authentication_invalid_audience",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","source":"header"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			auth_jwt_authentication_invalid_audience: authJwtAuthenticationInvalidAudience
		},
		config,
	};
}


/**
 * Handler for POST /messages
 */
async function edgeCases19EmojiInStrings(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = {"text":"Hello 👋 World 🌍"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases19EmojiInStrings(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/messages",
		handler_name: "edge_cases_19_emoji_in_strings",
		request_schema: {"type":"object","required":["text"],"properties":{"text":{"type":"string","minLength":1,"maxLength":100}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_19_emoji_in_strings: edgeCases19EmojiInStrings
		},
	};
}


/**
 * Handler for GET /search
 */
async function edgeCases12PercentEncodedSpecialChars(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"term":"hi there"};
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
		parameter_schema: {"type":"object","properties":{"term":{"type":"string","source":"query"}},"required":["term"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_12_percent_encoded_special_chars: edgeCases12PercentEncodedSpecialChars
		},
	};
}


/**
 * Handler for POST /strings/
 */
async function edgeCasesSpecialStringValuesAndEscaping(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"empty_string":"","whitespace":"   ","tabs_newlines":"line1\n\tline2\r\nline3","quotes":"He said \"hello\" and 'goodbye'","backslashes":"C:\\\\Users\\\\Path","unicode_escapes":"Hello","special_chars":"!@#$%^&*()_+-=[]{}|;':\",./<>?"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCasesSpecialStringValuesAndEscaping(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/strings/",
		handler_name: "edge_cases_special_string_values_and_escaping",
		request_schema: {"type":"object","properties":{"empty_string":{"type":"string"},"whitespace":{"type":"string"},"tabs_newlines":{"type":"string"},"quotes":{"type":"string"},"backslashes":{"type":"string"},"unicode_escapes":{"type":"string"},"special_chars":{"type":"string"}},"additionalProperties":false,"required":["empty_string","whitespace","tabs_newlines","quotes","backslashes","unicode_escapes","special_chars"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_special_string_values_and_escaping: edgeCasesSpecialStringValuesAndEscaping
		},
	};
}


/**
 * Handler for POST /calculate
 */
async function edgeCases15FloatPrecisionPreservation(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = {"value":3.141592653589793};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases15FloatPrecisionPreservation(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/calculate",
		handler_name: "edge_cases_15_float_precision_preservation",
		request_schema: {"type":"object","required":["value"],"properties":{"value":{"type":"number"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_15_float_precision_preservation: edgeCases15FloatPrecisionPreservation
		},
	};
}


/**
 * Handler for GET /items
 */
async function edgeCases13EmptyStringQueryParamPreserved(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"filter":""};
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
		parameter_schema: {"type":"object","properties":{"filter":{"type":"string","source":"query"}},"required":["filter"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_13_empty_string_query_param_preserved: edgeCases13EmptyStringQueryParamPreserved
		},
	};
}


/**
 * Handler for POST /items
 */
async function edgeCases24ArrayWithHoles(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"items":["first","third","sixth"]};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases24ArrayWithHoles(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items",
		handler_name: "edge_cases_24_array_with_holes",
		request_schema: {"type":"object","required":["items"],"properties":{"items":{"type":"array","items":{"type":"string"}}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_24_array_with_holes: edgeCases24ArrayWithHoles
		},
	};
}


/**
 * Handler for POST /calculate
 */
async function edgeCases21ScientificNotationNumber(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = {"value":123000};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases21ScientificNotationNumber(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/calculate",
		handler_name: "edge_cases_21_scientific_notation_number",
		request_schema: {"type":"object","required":["value"],"properties":{"value":{"type":"number","minimum":0}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_21_scientific_notation_number: edgeCases21ScientificNotationNumber
		},
	};
}


/**
 * Handler for POST /calculations/
 */
async function edgeCasesFloatPrecisionAndRounding(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"sum":0.30000000000000004,"precise_value":3.141592653589793,"very_small":1e-10,"very_large":1.7976931348623157e+308};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCasesFloatPrecisionAndRounding(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/calculations/",
		handler_name: "edge_cases_float_precision_and_rounding",
		request_schema: {"type":"object","properties":{"value1":{"type":"number"},"value2":{"type":"number"},"expected_sum":{"type":"number"},"precise_value":{"type":"number"},"very_small":{"type":"number"},"very_large":{"type":"number"}},"additionalProperties":false,"required":["value1","value2","expected_sum","precise_value","very_small","very_large"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_float_precision_and_rounding: edgeCasesFloatPrecisionAndRounding
		},
	};
}


/**
 * Handler for POST /items/
 */
async function edgeCasesUnicodeAndEmojiHandling(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"id":1,"name":"Coffee Shop ☕","description":"Best café in München 🇩🇪","tags":["食べ物","音楽","💰"],"emoji_reactions":"👍❤️😂🎉"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCasesUnicodeAndEmojiHandling(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "edge_cases_unicode_and_emoji_handling",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"description":{"type":"string"},"tags":{"type":"array","items":{"type":"string"}},"emoji_reactions":{"type":"string"}},"additionalProperties":false,"required":["name","description","tags","emoji_reactions"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_unicode_and_emoji_handling: edgeCasesUnicodeAndEmojiHandling
		},
	};
}


/**
 * Handler for POST /text
 */
async function edgeCases17ExtremelyLongString(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["content"],"properties":{"content":{"type":"string","maxLength":10000}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_17_extremely_long_string: edgeCases17ExtremelyLongString
		},
	};
}


/**
 * Handler for GET /search
 */
async function edgeCases11Utf8QueryParameter(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"term":"café"};
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
		parameter_schema: {"type":"object","properties":{"term":{"type":"string","source":"query"}},"required":["term"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_11_utf8_query_parameter: edgeCases11Utf8QueryParameter
		},
	};
}


/**
 * Handler for POST /users
 */
async function edgeCases18UnicodeNormalization(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = {"name":"café"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases18UnicodeNormalization(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "edge_cases_18_unicode_normalization",
		request_schema: {"type":"object","required":["name"],"properties":{"name":{"type":"string","minLength":1}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_18_unicode_normalization: edgeCases18UnicodeNormalization
		},
	};
}


/**
 * Handler for POST /files
 */
async function edgeCases20NullByteInString(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["filename"],"properties":{"filename":{"type":"string","pattern":"^[^\\x00]+$"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_20_null_byte_in_string: edgeCases20NullByteInString
		},
	};
}


/**
 * Handler for POST /data
 */
async function edgeCases23DeeplyNestedJsonLimit(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 400 };
	const responseBody = {"error":"Request body exceeds maximum nesting depth of 32"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases23DeeplyNestedJsonLimit(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "edge_cases_23_deeply_nested_json_limit",
		request_schema: {"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_23_deeply_nested_json_limit: edgeCases23DeeplyNestedJsonLimit
		},
	};
}


/**
 * Handler for GET /items
 */
async function edgeCases14LargeIntegerBoundary(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"id":9007199254740991};
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
		parameter_schema: {"type":"object","properties":{"id":{"type":"integer","source":"query"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_14_large_integer_boundary: edgeCases14LargeIntegerBoundary
		},
	};
}


/**
 * Handler for GET /data
 */
async function edgeCases22LeadingZerosInteger(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"value":123};
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
		parameter_schema: {"type":"object","properties":{"value":{"type":"integer","annotation":"int","source":"query"}},"required":["value"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_22_leading_zeros_integer: edgeCases22LeadingZerosInteger
		},
	};
}


/**
 * Handler for POST /numbers/
 */
async function edgeCasesLargeIntegerBoundaryValues(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"max_safe_int":9007199254740991,"large_int":"9223372036854775807","negative_large":"-9223372036854775808"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCasesLargeIntegerBoundaryValues(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/numbers/",
		handler_name: "edge_cases_large_integer_boundary_values",
		request_schema: {"type":"object","properties":{"max_safe_int":{"type":"integer"},"large_int":{"type":"integer"},"negative_large":{"type":"integer"}},"additionalProperties":false,"required":["max_safe_int","large_int","negative_large"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_large_integer_boundary_values: edgeCasesLargeIntegerBoundaryValues
		},
	};
}


/**
 * Handler for POST /nested/
 */
async function edgeCasesDeeplyNestedStructure10Levels(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Processed deeply nested structure","max_depth":10,"value_found":"deep"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCasesDeeplyNestedStructure10Levels(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/nested/",
		handler_name: "edge_cases_deeply_nested_structure_10_levels",
		request_schema: {"type":"object","properties":{"level1":{"type":"object","properties":{"level2":{"type":"object","properties":{"level3":{"type":"object","properties":{"level4":{"type":"object","properties":{"level5":{"type":"object","properties":{"level6":{"type":"object","properties":{"level7":{"type":"object","properties":{"level8":{"type":"object","properties":{"level9":{"type":"object","properties":{"level10":{"type":"object","properties":{"value":{"type":"string"},"depth":{"type":"integer"}},"additionalProperties":false,"required":["value","depth"]}},"additionalProperties":false,"required":["level10"]}},"additionalProperties":false,"required":["level9"]}},"additionalProperties":false,"required":["level8"]}},"additionalProperties":false,"required":["level7"]}},"additionalProperties":false,"required":["level6"]}},"additionalProperties":false,"required":["level5"]}},"additionalProperties":false,"required":["level4"]}},"additionalProperties":false,"required":["level3"]}},"additionalProperties":false,"required":["level2"]}},"additionalProperties":false,"required":["level1"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_deeply_nested_structure_10_levels: edgeCasesDeeplyNestedStructure10Levels
		},
	};
}


/**
 * Handler for POST /nulls/
 */
async function edgeCasesEmptyAndNullValueHandling(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"explicit_null_is_null":true,"empty_string_length":0,"empty_array_length":0,"empty_object_keys":0,"zero_is_falsy":true,"false_is_false":true};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCasesEmptyAndNullValueHandling(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/nulls/",
		handler_name: "edge_cases_empty_and_null_value_handling",
		request_schema: {"type":"object","properties":{"explicit_null":{"type":"null"},"empty_string":{"type":"string"},"empty_array":{"type":"array","items":{}},"empty_object":{"type":"object","properties":{},"additionalProperties":false},"zero_number":{"type":"integer"},"false_boolean":{"type":"boolean"}},"additionalProperties":false,"required":["explicit_null","empty_string","empty_array","empty_object","zero_number","false_boolean"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_empty_and_null_value_handling: edgeCasesEmptyAndNullValueHandling
		},
	};
}


/**
 * Handler for POST /data
 */
async function edgeCases16NegativeZeroHandling(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = {"offset":0};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppEdgeCases16NegativeZeroHandling(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "edge_cases_16_negative_zero_handling",
		request_schema: {"type":"object","required":["offset"],"properties":{"offset":{"type":"number"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_16_negative_zero_handling: edgeCases16NegativeZeroHandling
		},
	};
}


/**
 * Handler for GET /items/
 */
async function queryParamsStringValidationWithRegexSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"item_query":"fixedquery"};
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
		parameter_schema: {"type":"object","properties":{"item_query":{"type":"string","annotation":"str","pattern":"^fixedquery$","source":"query"}},"required":["item_query"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_string_validation_with_regex_success: queryParamsStringValidationWithRegexSuccess
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams49IntegerGtConstraintSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"limit":5};
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
		parameter_schema: {"type":"object","properties":{"limit":{"type":"integer","exclusiveMinimum":0,"source":"query"}},"required":["limit"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_49_integer_gt_constraint_success: queryParams49IntegerGtConstraintSuccess
		},
	};
}


/**
 * Handler for GET /query/enum
 */
async function queryParamsEnumQueryParameterInvalidValue(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const model = _params.model;
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
		parameter_schema: {"type":"object","properties":{"model":{"type":"string","annotation":"str","enum":["alexnet","resnet","lenet"],"source":"query"}},"required":["model"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_enum_query_parameter_invalid_value: queryParamsEnumQueryParameterInvalidValue
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams68ArrayUniqueitemsSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"ids":[1,2,3,4]};
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
		parameter_schema: {"type":"object","properties":{"ids":{"type":"array","items":{"type":"integer"},"uniqueItems":true,"source":"query"}},"required":["ids"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_68_array_uniqueitems_success: queryParams68ArrayUniqueitemsSuccess
		},
	};
}


/**
 * Handler for GET /subscribe
 */
async function queryParams47PatternValidationEmailSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"email":"user@example.com"};
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
		parameter_schema: {"type":"object","properties":{"email":{"type":"string","pattern":"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$","source":"query"}},"required":["email"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_47_pattern_validation_email_success: queryParams47PatternValidationEmailSuccess
		},
	};
}


/**
 * Handler for GET /query/int
 */
async function queryParamsRequiredIntegerQueryParameterSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"query":{"type":"integer","annotation":"int","source":"query"}},"required":["query"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_success: queryParamsRequiredIntegerQueryParameterSuccess
		},
	};
}


/**
 * Handler for GET /query
 */
async function queryParamsRequiredStringQueryParameterMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const query = _params.query;
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
		parameter_schema: {"type":"object","properties":{"query":{"type":"string","annotation":"str","source":"query"}},"required":["query"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_string_query_parameter_missing: queryParamsRequiredStringQueryParameterMissing
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams57BooleanEmptyStringCoercion(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"active":false};
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
		parameter_schema: {"type":"object","properties":{"active":{"type":"boolean","source":"query"}},"required":["active"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_57_boolean_empty_string_coercion: queryParams57BooleanEmptyStringCoercion
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams52IntegerLeConstraintBoundary(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"limit":100};
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
		parameter_schema: {"type":"object","properties":{"limit":{"type":"integer","maximum":100,"source":"query"}},"required":["limit"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_52_integer_le_constraint_boundary: queryParams52IntegerLeConstraintBoundary
		},
	};
}


/**
 * Handler for GET /query/list-default
 */
async function queryParamsListWithDefaultEmptyArrayNoValuesProvided(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"tags":{"type":"array","annotation":"list[str]","items":{"type":"string"},"source":"query","default":[]}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_list_with_default_empty_array_no_values_provided: queryParamsListWithDefaultEmptyArrayNoValuesProvided
		},
	};
}


/**
 * Handler for GET /query/date
 */
async function queryParamsDateQueryParameterSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"event_date":"2024-01-15"};
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
		parameter_schema: {"type":"object","properties":{"event_date":{"type":"string","annotation":"str","format":"date","source":"query"}},"required":["event_date"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_date_query_parameter_success: queryParamsDateQueryParameterSuccess
		},
	};
}


/**
 * Handler for GET /query/str-max-length
 */
async function queryParamsStringQueryParamWithMaxLengthConstraintFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const name = _params.name;
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
		parameter_schema: {"type":"object","properties":{"name":{"type":"string","annotation":"str","maxLength":10,"source":"query"}},"required":["name"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_string_query_param_with_max_length_constraint_fail: queryParamsStringQueryParamWithMaxLengthConstraintFail
		},
	};
}


/**
 * Handler for GET /search
 */
async function queryParams45StringMinlengthValidationFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const term = _params.term;
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
		parameter_schema: {"type":"object","properties":{"term":{"type":"string","minLength":3,"source":"query"}},"required":["term"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_45_string_minlength_validation_failure: queryParams45StringMinlengthValidationFailure
		},
	};
}


/**
 * Handler for GET /query/int/default
 */
async function queryParamsIntegerWithDefaultValueOverride(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"query":{"type":"integer","annotation":"int","source":"query","default":10}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_with_default_value_override: queryParamsIntegerWithDefaultValueOverride
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams67MultipleofConstraintFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const quantity = _params.quantity;
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
		parameter_schema: {"type":"object","properties":{"quantity":{"type":"integer","multipleOf":5,"source":"query"}},"required":["quantity"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_67_multipleof_constraint_failure: queryParams67MultipleofConstraintFailure
		},
	};
}


/**
 * Handler for GET /subscribe
 */
async function queryParams58FormatEmailSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"email":"user@example.com"};
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
		parameter_schema: {"type":"object","properties":{"email":{"type":"string","format":"email","source":"query"}},"required":["email"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_58_format_email_success: queryParams58FormatEmailSuccess
		},
	};
}


/**
 * Handler for GET /query/int-ge
 */
async function queryParamsIntegerQueryParamWithGeConstraintBoundary(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"value":10};
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
		parameter_schema: {"type":"object","properties":{"value":{"type":"integer","annotation":"int","minimum":10,"source":"query"}},"required":["value"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_query_param_with_ge_constraint_boundary: queryParamsIntegerQueryParamWithGeConstraintBoundary
		},
	};
}


/**
 * Handler for GET /query/int-gt
 */
async function queryParamsIntegerQueryParamWithGtConstraintValid(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"value":1};
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
		parameter_schema: {"type":"object","properties":{"value":{"type":"integer","annotation":"int","exclusiveMinimum":0,"source":"query"}},"required":["value"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_query_param_with_gt_constraint_valid: queryParamsIntegerQueryParamWithGtConstraintValid
		},
	};
}


/**
 * Handler for GET /query/int
 */
async function queryParamsRequiredIntegerQueryParameterInvalidType(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const query = _params.query;
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
		parameter_schema: {"type":"object","properties":{"query":{"type":"integer","annotation":"int","source":"query"}},"required":["query"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_invalid_type: queryParamsRequiredIntegerQueryParameterInvalidType
		},
	};
}


/**
 * Handler for GET /query/int
 */
async function queryParamsRequiredIntegerQueryParameterFloatValue(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const query = _params.query;
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
		parameter_schema: {"type":"object","properties":{"query":{"type":"integer","annotation":"int","source":"query"}},"required":["query"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_float_value: queryParamsRequiredIntegerQueryParameterFloatValue
		},
	};
}


/**
 * Handler for GET /query/basic
 */
async function queryParamsQueryParameterWithUrlEncodedSpecialCharacters(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"test&value=123"};
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
		parameter_schema: {"type":"object","properties":{"name":{"type":"string","annotation":"str","source":"query"}},"required":["name"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_query_parameter_with_url_encoded_special_characters: queryParamsQueryParameterWithUrlEncodedSpecialCharacters
		},
	};
}


/**
 * Handler for GET /subscribe
 */
async function queryParams59FormatEmailFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const email = _params.email;
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
		parameter_schema: {"type":"object","properties":{"email":{"type":"string","format":"email","source":"query"}},"required":["email"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_59_format_email_failure: queryParams59FormatEmailFailure
		},
	};
}


/**
 * Handler for GET /stats
 */
async function queryParams43ScientificNotationFloat(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"threshold":0.0015};
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
		parameter_schema: {"type":"object","properties":{"threshold":{"type":"number","annotation":"float","source":"query"}},"required":["threshold"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_43_scientific_notation_float: queryParams43ScientificNotationFloat
		},
	};
}


/**
 * Handler for GET /redirect
 */
async function queryParams63FormatUriSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"url":"https://example.com/path?query=value"};
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
		parameter_schema: {"type":"object","properties":{"url":{"type":"string","format":"uri","source":"query"}},"required":["url"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_63_format_uri_success: queryParams63FormatUriSuccess
		},
	};
}


/**
 * Handler for GET /query/bool
 */
async function queryParamsBooleanQueryParameterNumeric1(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"flag":true};
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
		parameter_schema: {"type":"object","properties":{"flag":{"type":"boolean","annotation":"bool","source":"query"}},"required":["flag"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_boolean_query_parameter_numeric_1: queryParamsBooleanQueryParameterNumeric1
		},
	};
}


/**
 * Handler for GET /query/str-min-length
 */
async function queryParamsStringQueryParamWithMinLengthConstraintFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const name = _params.name;
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
		parameter_schema: {"type":"object","properties":{"name":{"type":"string","annotation":"str","minLength":3,"source":"query"}},"required":["name"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_string_query_param_with_min_length_constraint_fail: queryParamsStringQueryParamWithMinLengthConstraintFail
		},
	};
}


/**
 * Handler for GET /query/optional
 */
async function queryParamsOptionalStringQueryParameterProvided(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"query":{"type":"string","annotation":"str","source":"query"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_optional_string_query_parameter_provided: queryParamsOptionalStringQueryParameterProvided
		},
	};
}


/**
 * Handler for GET /query/list
 */
async function queryParamsListOfIntegersMultipleValues(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = [1,2];
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
		parameter_schema: {"type":"object","properties":{"device_ids":{"type":"array","annotation":"list[int]","items":{"type":"integer"},"source":"query"}},"required":["device_ids"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_list_of_integers_multiple_values: queryParamsListOfIntegersMultipleValues
		},
	};
}


/**
 * Handler for GET /query/int-lt
 */
async function queryParamsIntegerQueryParamWithLtConstraintValid(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"value":49};
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
		parameter_schema: {"type":"object","properties":{"value":{"type":"integer","annotation":"int","exclusiveMaximum":50,"source":"query"}},"required":["value"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_query_param_with_lt_constraint_valid: queryParamsIntegerQueryParamWithLtConstraintValid
		},
	};
}


/**
 * Handler for GET /items/negative
 */
async function queryParams42NegativeIntegerQueryParam(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"offset":-10};
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
		parameter_schema: {"type":"object","properties":{"offset":{"type":"integer","annotation":"int","source":"query"}},"required":["offset"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_42_negative_integer_query_param: queryParams42NegativeIntegerQueryParam
		},
	};
}


/**
 * Handler for GET /search
 */
async function queryParams46StringMaxlengthValidationFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const term = _params.term;
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
		parameter_schema: {"type":"object","properties":{"term":{"type":"string","maxLength":10,"source":"query"}},"required":["term"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_46_string_maxlength_validation_failure: queryParams46StringMaxlengthValidationFailure
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams56ArrayMaxitemsConstraintFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const tags = _params.tags;
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
		parameter_schema: {"type":"object","properties":{"tags":{"type":"array","items":{"type":"string"},"maxItems":5,"source":"query"}},"required":["tags"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_56_array_maxitems_constraint_failure: queryParams56ArrayMaxitemsConstraintFailure
		},
	};
}


/**
 * Handler for GET /query/pattern
 */
async function queryParamsStringQueryParamWithRegexPatternFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const code = _params.code;
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
		parameter_schema: {"type":"object","properties":{"code":{"type":"string","annotation":"str","pattern":"^[0-9]{3,}$","source":"query"}},"required":["code"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_string_query_param_with_regex_pattern_fail: queryParamsStringQueryParamWithRegexPatternFail
		},
	};
}


/**
 * Handler for GET /search
 */
async function queryParams44StringMinlengthValidationSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"term":"foo"};
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
		parameter_schema: {"type":"object","properties":{"term":{"type":"string","minLength":3,"source":"query"}},"required":["term"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_44_string_minlength_validation_success: queryParams44StringMinlengthValidationSuccess
		},
	};
}


/**
 * Handler for GET /network
 */
async function queryParams61FormatIpv4Failure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const ip = _params.ip;
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
		parameter_schema: {"type":"object","properties":{"ip":{"type":"string","format":"ipv4","source":"query"}},"required":["ip"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_61_format_ipv4_failure: queryParams61FormatIpv4Failure
		},
	};
}


/**
 * Handler for GET /subscribe
 */
async function queryParams48PatternValidationEmailFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const email = _params.email;
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
		parameter_schema: {"type":"object","properties":{"email":{"type":"string","pattern":"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$","source":"query"}},"required":["email"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_48_pattern_validation_email_failure: queryParams48PatternValidationEmailFailure
		},
	};
}


/**
 * Handler for GET /query/int
 */
async function queryParamsRequiredIntegerQueryParameterMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const query = _params.query;
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
		parameter_schema: {"type":"object","properties":{"query":{"type":"integer","annotation":"int","source":"query"}},"required":["query"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_missing: queryParamsRequiredIntegerQueryParameterMissing
		},
	};
}


/**
 * Handler for GET /test
 */
async function queryParamsQueryParameterWithSpecialCharactersUrlEncoding(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"email":"x@test.com","special":"&@A.ac"};
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
		parameter_schema: {"type":"object","properties":{"email":{"type":"string","annotation":"str","source":"query"},"special":{"type":"string","annotation":"str","source":"query"}},"required":["email","special"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_query_parameter_with_special_characters_url_encoding: queryParamsQueryParameterWithSpecialCharactersUrlEncoding
		},
	};
}


/**
 * Handler for GET /query/list
 */
async function queryParamsListQueryParameterRequiredButMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const deviceIds = _params.device_ids;
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
		parameter_schema: {"type":"object","properties":{"device_ids":{"type":"array","annotation":"list[int]","items":{"type":"integer"},"source":"query"}},"required":["device_ids"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_list_query_parameter_required_but_missing: queryParamsListQueryParameterRequiredButMissing
		},
	};
}


/**
 * Handler for GET /query
 */
async function queryParamsRequiredStringQueryParameterSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"query":{"type":"string","annotation":"str","source":"query"}},"required":["query"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_string_query_parameter_success: queryParamsRequiredStringQueryParameterSuccess
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams66MultipleofConstraintSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"quantity":15};
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
		parameter_schema: {"type":"object","properties":{"quantity":{"type":"integer","multipleOf":5,"source":"query"}},"required":["quantity"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_66_multipleof_constraint_success: queryParams66MultipleofConstraintSuccess
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams53IntegerLeConstraintFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const limit = _params.limit;
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
		parameter_schema: {"type":"object","properties":{"limit":{"type":"integer","maximum":100,"source":"query"}},"required":["limit"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_53_integer_le_constraint_failure: queryParams53IntegerLeConstraintFailure
		},
	};
}


/**
 * Handler for GET /query/multi-type
 */
async function queryParamsMultipleQueryParametersWithDifferentTypes(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"john","age":30,"active":true,"score":95.5};
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
		parameter_schema: {"type":"object","properties":{"name":{"type":"string","annotation":"str","source":"query"},"age":{"type":"integer","annotation":"int","source":"query"},"active":{"type":"boolean","annotation":"bool","source":"query"},"score":{"type":"number","annotation":"float","source":"query"}},"required":["name","age","active","score"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_multiple_query_parameters_with_different_types: queryParamsMultipleQueryParametersWithDifferentTypes
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams71ArraySeparatorSemicolon(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"colors":["red","green","blue"]};
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
		parameter_schema: {"type":"object","properties":{"colors":{"type":"array","items":{"type":"string"},"separator":";","source":"query"}},"required":["colors"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_71_array_separator_semicolon: queryParams71ArraySeparatorSemicolon
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams70ArraySeparatorPipe(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"tags":["python","rust","typescript"]};
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
		parameter_schema: {"type":"object","properties":{"tags":{"type":"array","items":{"type":"string"},"separator":"|","source":"query"}},"required":["tags"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_70_array_separator_pipe: queryParams70ArraySeparatorPipe
		},
	};
}


/**
 * Handler for GET /query/int/default
 */
async function queryParamsIntegerWithDefaultValueNotProvided(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"query":{"type":"integer","annotation":"int","source":"query","default":10}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_with_default_value_not_provided: queryParamsIntegerWithDefaultValueNotProvided
		},
	};
}


/**
 * Handler for GET /query/bool
 */
async function queryParamsBooleanQueryParameterTrue(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"flag":true};
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
		parameter_schema: {"type":"object","properties":{"flag":{"type":"boolean","annotation":"bool","source":"query"}},"required":["flag"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_boolean_query_parameter_true: queryParamsBooleanQueryParameterTrue
		},
	};
}


/**
 * Handler for GET /query/int-le
 */
async function queryParamsIntegerQueryParamWithLeConstraintBoundary(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"value":100};
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
		parameter_schema: {"type":"object","properties":{"value":{"type":"integer","annotation":"int","maximum":100,"source":"query"}},"required":["value"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_query_param_with_le_constraint_boundary: queryParamsIntegerQueryParamWithLeConstraintBoundary
		},
	};
}


/**
 * Handler for GET /query/float-ge
 */
async function queryParamsFloatQueryParamWithGeConstraintSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"price":0.01};
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
		parameter_schema: {"type":"object","properties":{"price":{"type":"number","annotation":"float","minimum":0.01,"source":"query"}},"required":["price"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_float_query_param_with_ge_constraint_success: queryParamsFloatQueryParamWithGeConstraintSuccess
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams51IntegerGeConstraintBoundary(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"offset":0};
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
		parameter_schema: {"type":"object","properties":{"offset":{"type":"integer","minimum":0,"source":"query"}},"required":["offset"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_51_integer_ge_constraint_boundary: queryParams51IntegerGeConstraintBoundary
		},
	};
}


/**
 * Handler for GET /query/int/optional
 */
async function queryParamsOptionalIntegerQueryParameterMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"query":{"type":"integer","annotation":"int","source":"query"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_optional_integer_query_parameter_missing: queryParamsOptionalIntegerQueryParameterMissing
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams69ArrayUniqueitemsFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const ids = _params.ids;
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
		parameter_schema: {"type":"object","properties":{"ids":{"type":"array","items":{"type":"integer"},"uniqueItems":true,"source":"query"}},"required":["ids"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_69_array_uniqueitems_failure: queryParams69ArrayUniqueitemsFailure
		},
	};
}


/**
 * Handler for GET /search
 */
async function queryParams72ArraySeparatorSpace(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"keywords":["rust","web","framework"]};
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
		parameter_schema: {"type":"object","properties":{"keywords":{"type":"array","items":{"type":"string"},"separator":" ","source":"query"}},"required":["keywords"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_72_array_separator_space: queryParams72ArraySeparatorSpace
		},
	};
}


/**
 * Handler for GET /items/
 */
async function queryParamsStringValidationWithRegexFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const itemQuery = _params.item_query;
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
		parameter_schema: {"type":"object","properties":{"item_query":{"type":"string","annotation":"str","pattern":"^fixedquery$","source":"query"}},"required":["item_query"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_string_validation_with_regex_failure: queryParamsStringValidationWithRegexFailure
		},
	};
}


/**
 * Handler for GET /dns
 */
async function queryParams65FormatHostnameSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"host":"api.example.com"};
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
		parameter_schema: {"type":"object","properties":{"host":{"type":"string","format":"hostname","source":"query"}},"required":["host"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_65_format_hostname_success: queryParams65FormatHostnameSuccess
		},
	};
}


/**
 * Handler for GET /query/basic
 */
async function queryParamsQueryParameterWithUrlEncodedSpace(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"hello world"};
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
		parameter_schema: {"type":"object","properties":{"name":{"type":"string","annotation":"str","source":"query"}},"required":["name"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_query_parameter_with_url_encoded_space: queryParamsQueryParameterWithUrlEncodedSpace
		},
	};
}


/**
 * Handler for GET /items/
 */
async function queryParamsListOfStringsMultipleValues(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"q":["foo","bar"]};
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
		parameter_schema: {"type":"object","properties":{"q":{"type":"array","annotation":"list[str]","items":{"type":"string"},"source":"query"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_list_of_strings_multiple_values: queryParamsListOfStringsMultipleValues
		},
	};
}


/**
 * Handler for GET /query/optional-default
 */
async function queryParamsOptionalQueryParameterWithDefaultValue(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"limit":10};
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
		parameter_schema: {"type":"object","properties":{"limit":{"type":"integer","annotation":"int","source":"query","default":10}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_optional_query_parameter_with_default_value: queryParamsOptionalQueryParameterWithDefaultValue
		},
	};
}


/**
 * Handler for GET /network/ipv6
 */
async function queryParams62FormatIpv6Success(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"ip":"2001:0db8:85a3:0000:0000:8a2e:0370:7334"};
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
		parameter_schema: {"type":"object","properties":{"ip":{"type":"string","format":"ipv6","source":"query"}},"required":["ip"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_62_format_ipv6_success: queryParams62FormatIpv6Success
		},
	};
}


/**
 * Handler for GET /query/list-default
 */
async function queryParamsArrayQueryParameterSingleValue(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"tags":{"type":"array","annotation":"list[str]","items":{"type":"string"},"source":"query","default":[]}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_array_query_parameter_single_value: queryParamsArrayQueryParameterSingleValue
		},
	};
}


/**
 * Handler for GET /query/optional
 */
async function queryParamsOptionalStringQueryParameterMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"query":{"type":"string","annotation":"str","source":"query"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_optional_string_query_parameter_missing: queryParamsOptionalStringQueryParameterMissing
		},
	};
}


/**
 * Handler for GET /query/datetime
 */
async function queryParamsDatetimeQueryParameterSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"timestamp":"2024-01-15T10:30:00Z"};
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
		parameter_schema: {"type":"object","properties":{"timestamp":{"type":"string","annotation":"str","format":"date-time","source":"query"}},"required":["timestamp"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_datetime_query_parameter_success: queryParamsDatetimeQueryParameterSuccess
		},
	};
}


/**
 * Handler for GET /query/uuid
 */
async function queryParamsUuidQueryParameterInvalidFormat(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const itemId = _params.item_id;
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"string","annotation":"str","format":"uuid","source":"query"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_uuid_query_parameter_invalid_format: queryParamsUuidQueryParameterInvalidFormat
		},
	};
}


/**
 * Handler for GET /query/list-default
 */
async function queryParamsArrayQueryParameterEmptyArray(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"tags":{"type":"array","annotation":"list[str]","items":{"type":"string"},"source":"query","default":[]}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_array_query_parameter_empty_array: queryParamsArrayQueryParameterEmptyArray
		},
	};
}


/**
 * Handler for GET /query/enum
 */
async function queryParamsEnumQueryParameterSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"model":"alexnet"};
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
		parameter_schema: {"type":"object","properties":{"model":{"type":"string","annotation":"str","enum":["alexnet","resnet","lenet"],"source":"query"}},"required":["model"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_enum_query_parameter_success: queryParamsEnumQueryParameterSuccess
		},
	};
}


/**
 * Handler for GET /query/uuid
 */
async function queryParamsUuidQueryParameterSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"item_id":"c892496f-b1fd-4b91-bdb8-b46f92df1716"};
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"string","annotation":"str","format":"uuid","source":"query"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_uuid_query_parameter_success: queryParamsUuidQueryParameterSuccess
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams50IntegerGtConstraintFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const limit = _params.limit;
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
		parameter_schema: {"type":"object","properties":{"limit":{"type":"integer","exclusiveMinimum":0,"source":"query"}},"required":["limit"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_50_integer_gt_constraint_failure: queryParams50IntegerGtConstraintFailure
		},
	};
}


/**
 * Handler for GET /redirect
 */
async function queryParams64FormatUriFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const url = _params.url;
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
		parameter_schema: {"type":"object","properties":{"url":{"type":"string","format":"uri","source":"query"}},"required":["url"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_64_format_uri_failure: queryParams64FormatUriFailure
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams54ArrayMinitemsConstraintSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"ids":[1,2,3]};
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
		parameter_schema: {"type":"object","properties":{"ids":{"type":"array","items":{"type":"integer"},"minItems":2,"source":"query"}},"required":["ids"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_54_array_minitems_constraint_success: queryParams54ArrayMinitemsConstraintSuccess
		},
	};
}


/**
 * Handler for GET /items
 */
async function queryParams55ArrayMinitemsConstraintFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const ids = _params.ids;
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
		parameter_schema: {"type":"object","properties":{"ids":{"type":"array","items":{"type":"integer"},"minItems":2,"source":"query"}},"required":["ids"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_55_array_minitems_constraint_failure: queryParams55ArrayMinitemsConstraintFailure
		},
	};
}


/**
 * Handler for GET /network
 */
async function queryParams60FormatIpv4Success(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"ip":"192.168.1.1"};
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
		parameter_schema: {"type":"object","properties":{"ip":{"type":"string","format":"ipv4","source":"query"}},"required":["ip"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_60_format_ipv4_success: queryParams60FormatIpv4Success
		},
	};
}


/**
 * Handler for POST /slow-endpoint
 */
async function statusCodes408RequestTimeout(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 408 };
	response.headers = {"connection":"close"};
	const responseBody = {"detail":"Request timeout"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes408RequestTimeout(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/slow-endpoint",
		handler_name: "status_codes_408_request_timeout",
		request_schema: {"type":"object","properties":{"data":{"type":"string"}},"additionalProperties":false,"required":["data"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_408_request_timeout: statusCodes408RequestTimeout
		},
	};
}


/**
 * Handler for GET /status-test/{code}
 */
async function statusCodes404NotFoundResourceNotFound(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 404 };
	const responseBody = {"detail":"Item not found"};
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
		parameter_schema: {"type":"object","properties":{"code":{"type":"string","source":"path"}},"required":["code"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_404_not_found_resource_not_found: statusCodes404NotFoundResourceNotFound
		},
	};
}


/**
 * Handler for GET /health
 */
async function statusCodes503ServiceUnavailableServerOverload(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 503 };
	response.headers = {"retry-after":"120"};
	const responseBody = {"detail":"Service temporarily unavailable"};
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
			status_codes_503_service_unavailable_server_overload: statusCodes503ServiceUnavailableServerOverload
		},
	};
}


/**
 * Handler for POST /items/
 */
async function statusCodes422UnprocessableEntityValidationError(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"price":{"type":"string"},"name":{"type":"string"}},"additionalProperties":false,"required":["price","name"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_422_unprocessable_entity_validation_error: statusCodes422UnprocessableEntityValidationError
		},
	};
}


/**
 * Handler for GET /temp-redirect
 */
async function statusCodes302FoundTemporaryRedirect(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 302 };
	response.headers = {"location":"/target-path"};
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
			status_codes_302_found_temporary_redirect: statusCodes302FoundTemporaryRedirect
		},
	};
}


/**
 * Handler for GET /status-test/{code}
 */
async function statusCodes304NotModifiedCachedContentValid(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"code":{"type":"string","source":"path"},"If-None-Match":{"type":"string","source":"header"}},"required":["code"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_304_not_modified_cached_content_valid: statusCodes304NotModifiedCachedContentValid
		},
	};
}


/**
 * Handler for POST /items/
 */
async function statusCodes400BadRequestInvalidRequest(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 400 };
	const responseBody = {"detail":"Invalid request format"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes400BadRequestInvalidRequest(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "status_codes_400_bad_request_invalid_request",
		request_schema: {"type":"string"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_400_bad_request_invalid_request: statusCodes400BadRequestInvalidRequest
		},
	};
}


/**
 * Handler for TRACE /data
 */
async function statusCodes22501NotImplemented(requestJson: string, _context?: HandlerContext): Promise<string> {
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
			status_codes_22_501_not_implemented: statusCodes22501NotImplemented
		},
	};
}


/**
 * Handler for DELETE /status-test/{code}
 */
async function statusCodes204NoContentSuccessWithNoBody(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"code":{"type":"string","source":"path"}},"required":["code"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_204_no_content_success_with_no_body: statusCodes204NoContentSuccessWithNoBody
		},
	};
}


/**
 * Handler for GET /old-path
 */
async function statusCodes301MovedPermanentlyPermanentRedirect(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 301 };
	response.headers = {"location":"/new-path"};
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
			status_codes_301_moved_permanently_permanent_redirect: statusCodes301MovedPermanentlyPermanentRedirect
		},
	};
}


/**
 * Handler for POST /items/
 */
async function statusCodes201CreatedResourceCreated(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = {"id":1,"name":"New Item"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes201CreatedResourceCreated(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "status_codes_201_created_resource_created",
		request_schema: {"type":"object","properties":{"name":{"type":"string"}},"additionalProperties":false,"required":["name"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_201_created_resource_created: statusCodes201CreatedResourceCreated
		},
	};
}


/**
 * Handler for POST /tasks/
 */
async function statusCodes202AcceptedRequestAcceptedForProcessing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 202 };
	const responseBody = {"message":"Task accepted for processing","task_id":"abc123"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes202AcceptedRequestAcceptedForProcessing(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/tasks/",
		handler_name: "status_codes_202_accepted_request_accepted_for_processing",
		request_schema: {"type":"object","properties":{"task":{"type":"string"}},"additionalProperties":false,"required":["task"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_202_accepted_request_accepted_for_processing: statusCodes202AcceptedRequestAcceptedForProcessing
		},
	};
}


/**
 * Handler for POST /redirect-post
 */
async function statusCodes307TemporaryRedirectMethodPreserved(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 307 };
	response.headers = {"location":"/target-post"};
	response.body = null;
	return JSON.stringify(response);
}

export function createAppStatusCodes307TemporaryRedirectMethodPreserved(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/redirect-post",
		handler_name: "status_codes_307_temporary_redirect_method_preserved",
		request_schema: {"type":"object","properties":{},"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_307_temporary_redirect_method_preserved: statusCodes307TemporaryRedirectMethodPreserved
		},
	};
}


/**
 * Handler for GET /error
 */
async function statusCodes500InternalServerErrorServerError(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 500 };
	const responseBody = {"type":"https://spikard.dev/errors/internal-server-error","title":"Internal Server Error","status":500,"detail":"Internal server error"};
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
			status_codes_500_internal_server_error_server_error: statusCodes500InternalServerErrorServerError
		},
	};
}


/**
 * Handler for GET /data
 */
async function statusCodes20414UriTooLong(requestJson: string, _context?: HandlerContext): Promise<string> {
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
			status_codes_20_414_uri_too_long: statusCodes20414UriTooLong
		},
	};
}


/**
 * Handler for GET /users/me
 */
async function statusCodes401UnauthorizedMissingAuthentication(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	response.headers = {"www-authenticate":"Bearer"};
	const responseBody = {"detail":"Not authenticated"};
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
			status_codes_401_unauthorized_missing_authentication: statusCodes401UnauthorizedMissingAuthentication
		},
	};
}


/**
 * Handler for GET /data
 */
async function statusCodes23503ServiceUnavailable(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 503 };
	response.headers = {"retry-after":"60"};
	const responseBody = {"error":"Service Unavailable","message":"The service is temporarily unavailable. Please try again later."};
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
			status_codes_23_503_service_unavailable: statusCodes23503ServiceUnavailable
		},
	};
}


/**
 * Handler for POST /upload
 */
async function statusCodes19413PayloadTooLarge(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 413 };
	const responseBody = {"error":"Payload Too Large","message":"Request body size exceeds maximum allowed size of 1024 bytes"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppStatusCodes19413PayloadTooLarge(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "status_codes_19_413_payload_too_large",
		request_schema: {"type":"object","properties":{"data":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_19_413_payload_too_large: statusCodes19413PayloadTooLarge
		},
	};
}


/**
 * Handler for GET /admin/users
 */
async function statusCodes403ForbiddenInsufficientPermissions(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 403 };
	const responseBody = {"detail":"Not enough permissions"};
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
			status_codes_403_forbidden_insufficient_permissions: statusCodes403ForbiddenInsufficientPermissions
		},
	};
}


/**
 * Handler for GET /data
 */
async function statusCodes21431RequestHeaderFieldsTooLarge(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 431 };
	const responseBody = {"error":"Request Header Fields Too Large","message":"Request headers exceed maximum allowed size of 8192 bytes"};
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
		parameter_schema: {"type":"object","properties":{"X-Large-Header":{"type":"string","source":"header"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_21_431_request_header_fields_too_large: statusCodes21431RequestHeaderFieldsTooLarge
		},
	};
}


/**
 * Handler for GET /api/resource
 */
async function statusCodes429TooManyRequests(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 429 };
	response.headers = {"x-ratelimit-limit":"100","x-ratelimit-reset":"1609459200","retry-after":"60","x-ratelimit-remaining":"0"};
	const responseBody = {"detail":"Rate limit exceeded. Try again in 60 seconds."};
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
			status_codes_429_too_many_requests: statusCodes429TooManyRequests
		},
	};
}


/**
 * Handler for GET /status-test/{code}
 */
async function statusCodes200OkSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"id":1,"name":"Item 1"};
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
		parameter_schema: {"type":"object","properties":{"code":{"type":"string","source":"path"}},"required":["code"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_200_ok_success: statusCodes200OkSuccess
		},
	};
}


/**
 * Handler for GET /files/document.pdf
 */
async function statusCodes206PartialContent(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 206 };
	response.headers = {"content-length":"1024","accept-ranges":"bytes","content-range":"bytes 0-1023/5000","content-type":"application/pdf"};
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
			status_codes_206_partial_content: statusCodes206PartialContent
		},
	};
}


/**
 * Handler for POST /
 */
async function multipartMultipleValuesForSameFieldName(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"files":[{"filename":"file1.txt","size":10,"content":"first file","content_type":"text/plain"},{"filename":"file2.txt","size":11,"content":"second file","content_type":"text/plain"}],"tags":["python","rust","web"]};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartMultipleValuesForSameFieldName(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_multiple_values_for_same_field_name",
		request_schema: {"type":"object","properties":{"files":{"type":"array","items":{"type":"string","format":"binary"}},"tags":{"type":"array","items":{"type":"string"}}},"additionalProperties":false,"required":["files"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_multiple_values_for_same_field_name: multipartMultipleValuesForSameFieldName
		},
	};
}


/**
 * Handler for POST /upload
 */
async function multipart19FileMimeSpoofingPngAsJpeg(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{}},
		file_params: {"image":{"required":true,"content_type":["image/jpeg"],"validate_magic_numbers":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_19_file_mime_spoofing_png_as_jpeg: multipart19FileMimeSpoofingPngAsJpeg
		},
	};
}


/**
 * Handler for POST /upload
 */
async function multipart20FileMimeSpoofingJpegAsPng(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{}},
		file_params: {"image":{"required":true,"content_type":["image/png"],"validate_magic_numbers":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_20_file_mime_spoofing_jpeg_as_png: multipart20FileMimeSpoofingJpegAsPng
		},
	};
}


/**
 * Handler for POST /upload
 */
async function multipart21FilePdfMagicNumberSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{}},
		file_params: {"document":{"required":true,"content_type":["application/pdf"],"validate_magic_numbers":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_21_file_pdf_magic_number_success: multipart21FilePdfMagicNumberSuccess
		},
	};
}


/**
 * Handler for POST /files/images-only
 */
async function multipartContentTypeValidationInvalidType(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"file":{"type":"string","format":"binary"}},"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{}},
		file_params: {"file":{"required":true,"content_type":["image/jpeg","image/png","image/gif"]}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_content_type_validation_invalid_type: multipartContentTypeValidationInvalidType
		},
	};
}


/**
 * Handler for POST /files/document
 */
async function multipartPdfFileUpload(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"filename":"report.pdf","content_type":"application/pdf","size":16};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartPdfFileUpload(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/document",
		handler_name: "multipart_pdf_file_upload",
		request_schema: {"type":"object","properties":{"document":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["document"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_pdf_file_upload: multipartPdfFileUpload
		},
	};
}


/**
 * Handler for POST /files/list
 */
async function multipartFileListUploadArrayOfFiles(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"filenames":["file1.txt","file2.txt"],"total_size":35};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartFileListUploadArrayOfFiles(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/list",
		handler_name: "multipart_file_list_upload_array_of_files",
		request_schema: {"type":"object","properties":{"files":{"type":"array","items":{"type":"string","format":"binary"}}},"additionalProperties":false,"required":["files"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_file_list_upload_array_of_files: multipartFileListUploadArrayOfFiles
		},
	};
}


/**
 * Handler for POST /files/optional
 */
async function multipartOptionalFileUploadProvided(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"filename":"optional.txt","content_type":"text/plain","size":21};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartOptionalFileUploadProvided(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/optional",
		handler_name: "multipart_optional_file_upload_provided",
		request_schema: {"type":"object","properties":{"file":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["file"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_optional_file_upload_provided: multipartOptionalFileUploadProvided
		},
	};
}


/**
 * Handler for POST /files/validated
 */
async function multipartFileSizeValidationTooLarge(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 413 };
	const responseBody = {"detail":"File too large. Maximum size is 1MB"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartFileSizeValidationTooLarge(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/validated",
		handler_name: "multipart_file_size_validation_too_large",
		request_schema: {"type":"object","properties":{"file":{"type":"string","format":"binary"}},"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_file_size_validation_too_large: multipartFileSizeValidationTooLarge
		},
	};
}


/**
 * Handler for POST /
 */
async function multipartMixedFilesAndFormData(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"file":{"filename":"upload.txt","size":14,"content":"file data here","content_type":"text/plain"},"username":"testuser","age":"25","active":"true"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartMixedFilesAndFormData(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_mixed_files_and_form_data",
		request_schema: {"type":"object","properties":{"file":{"type":"string","format":"binary"},"username":{"type":"string"},"age":{"type":"string"},"active":{"type":"string"}},"additionalProperties":false,"required":["file"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_mixed_files_and_form_data: multipartMixedFilesAndFormData
		},
	};
}


/**
 * Handler for POST /
 */
async function multipartSimpleFileUpload(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"test":{"filename":"test.txt","size":14,"content":"<file content>","content_type":"text/plain"}};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartSimpleFileUpload(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_simple_file_upload",
		request_schema: {"type":"object","properties":{"test":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["test"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_simple_file_upload: multipartSimpleFileUpload
		},
	};
}


/**
 * Handler for POST /files/upload
 */
async function multipartEmptyFileUpload(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"filename":"empty.txt","size":0};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartEmptyFileUpload(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/upload",
		handler_name: "multipart_empty_file_upload",
		request_schema: {"type":"object","properties":{"file":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["file"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_empty_file_upload: multipartEmptyFileUpload
		},
	};
}


/**
 * Handler for POST /files/optional
 */
async function multipartOptionalFileUploadMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"file":null};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartOptionalFileUploadMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/optional",
		handler_name: "multipart_optional_file_upload_missing",
		request_schema: {"type":"object","properties":{},"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_optional_file_upload_missing: multipartOptionalFileUploadMissing
		},
	};
}


/**
 * Handler for POST /
 */
async function multipartFileUploadWithoutFilename(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"test1":"<file1 content>"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartFileUploadWithoutFilename(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_file_upload_without_filename",
		request_schema: {"type":"object","properties":{"test1":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["test1"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_file_upload_without_filename: multipartFileUploadWithoutFilename
		},
	};
}


/**
 * Handler for POST /upload
 */
async function multipart18FileMagicNumberJpegSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{}},
		file_params: {"image":{"required":true,"content_type":["image/jpeg"],"validate_magic_numbers":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_18_file_magic_number_jpeg_success: multipart18FileMagicNumberJpegSuccess
		},
	};
}


/**
 * Handler for POST /upload
 */
async function multipart22FileEmptyBuffer(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{}},
		file_params: {"file":{"required":true,"validate_magic_numbers":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_22_file_empty_buffer: multipart22FileEmptyBuffer
		},
	};
}


/**
 * Handler for POST /upload
 */
async function multipart17FileMagicNumberPngSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{}},
		file_params: {"image":{"required":true,"content_type":["image/png"],"validate_magic_numbers":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_17_file_magic_number_png_success: multipart17FileMagicNumberPngSuccess
		},
	};
}


/**
 * Handler for POST /
 */
async function multipartFormDataWithoutFiles(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"some":"data"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartFormDataWithoutFiles(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_form_data_without_files",
		request_schema: {"type":"object","properties":{"some":{"type":"string"}},"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_form_data_without_files: multipartFormDataWithoutFiles
		},
	};
}


/**
 * Handler for POST /
 */
async function multipartMultipleFileUploads(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"test1":{"filename":"test1.txt","size":15,"content":"<file1 content>","content_type":"text/plain"},"test2":{"filename":"test2.txt","size":15,"content":"<file2 content>","content_type":"text/plain"}};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartMultipleFileUploads(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_multiple_file_uploads",
		request_schema: {"type":"object","properties":{"test1":{"type":"string","format":"binary"},"test2":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["test1","test2"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_multiple_file_uploads: multipartMultipleFileUploads
		},
	};
}


/**
 * Handler for POST /
 */
async function multipartFileUploadWithCustomHeaders(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"test2":{"filename":"test2.txt","size":15,"content":"<file2 content>","content_type":"text/plain","headers":[["content-disposition","form-data; name=\"test2\"; filename=\"test2.txt\""],["content-type","text/plain"],["x-custom","f2"]]}};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartFileUploadWithCustomHeaders(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_file_upload_with_custom_headers",
		request_schema: {"type":"object","properties":{"test2":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["test2"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_file_upload_with_custom_headers: multipartFileUploadWithCustomHeaders
		},
	};
}


/**
 * Handler for POST /files/required
 */
async function multipartRequiredFileUploadMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"file":{"type":"string","format":"binary"}},"required":["file"],"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_required_file_upload_missing: multipartRequiredFileUploadMissing
		},
	};
}


/**
 * Handler for POST /files/image
 */
async function multipartImageFileUpload(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"filename":"photo.jpg","content_type":"image/jpeg","size":22};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppMultipartImageFileUpload(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/image",
		handler_name: "multipart_image_file_upload",
		request_schema: {"type":"object","properties":{"image":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["image"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_image_file_upload: multipartImageFileUpload
		},
	};
}


/**
 * Handler for OPTIONS /items/
 */
async function httpMethodsOptionsCorsPreflightRequest(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"access-control-allow-methods":"GET, POST, PUT, DELETE, OPTIONS","access-control-allow-headers":"Content-Type","access-control-max-age":"86400","access-control-allow-origin":"https://example.com"};
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
			http_methods_options_cors_preflight_request: httpMethodsOptionsCorsPreflightRequest
		},
	};
}


/**
 * Handler for DELETE /items/{id}
 */
async function httpMethodsDeleteRemoveResource(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_delete_remove_resource: httpMethodsDeleteRemoveResource
		},
	};
}


/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutCreateResourceIfDoesnTExist(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"id":999,"name":"New Item","price":49.99};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHttpMethodsPutCreateResourceIfDoesnTExist(): SpikardApp {
	const route: RouteMetadata = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_create_resource_if_doesn_t_exist",
		request_schema: {"type":"object","properties":{"id":{"type":"integer"},"name":{"type":"string"},"price":{"type":"number"}},"required":["id","name","price"]},
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_create_resource_if_doesn_t_exist: httpMethodsPutCreateResourceIfDoesnTExist
		},
	};
}


/**
 * Handler for PATCH /items/{id}
 */
async function httpMethodsPatchUpdateMultipleFields(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"id":1,"name":"Updated Name","price":89.99,"in_stock":false};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHttpMethodsPatchUpdateMultipleFields(): SpikardApp {
	const route: RouteMetadata = {
		method: "PATCH",
		path: "/items/{id}",
		handler_name: "http_methods_patch_update_multiple_fields",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"in_stock":{"type":"boolean"}},"required":["in_stock","name","price"]},
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_patch_update_multiple_fields: httpMethodsPatchUpdateMultipleFields
		},
	};
}


/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutValidationError(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const id = _params.id;
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","required":["id","name","price"],"properties":{"id":{"type":"integer"},"name":{"type":"string","minLength":3},"price":{"type":"number","exclusiveMinimum":0}}},
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_validation_error: httpMethodsPutValidationError
		},
	};
}


/**
 * Handler for HEAD /items/{id}
 */
async function httpMethodsHeadGetMetadataWithoutBody(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"content-type":"application/json","content-length":"85"};
	const result: Record<string, unknown> = {};
	const id = _params.id;
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
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_head_get_metadata_without_body: httpMethodsHeadGetMetadataWithoutBody
		},
	};
}


/**
 * Handler for DELETE /items/{id}
 */
async function httpMethodsDeleteWithResponseBody(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"id":1,"name":"Deleted Item","message":"Item deleted successfully"};
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
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_delete_with_response_body: httpMethodsDeleteWithResponseBody
		},
	};
}


/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutMissingRequiredField(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const id = _params.id;
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"id":{"type":"integer"},"name":{"type":"string"},"price":{"type":"string"}},"required":["price"]},
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_missing_required_field: httpMethodsPutMissingRequiredField
		},
	};
}


/**
 * Handler for PATCH /items/{id}
 */
async function httpMethodsPatchPartialUpdate(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"id":1,"name":"Existing Item","price":79.99,"in_stock":true};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHttpMethodsPatchPartialUpdate(): SpikardApp {
	const route: RouteMetadata = {
		method: "PATCH",
		path: "/items/{id}",
		handler_name: "http_methods_patch_partial_update",
		request_schema: {"type":"object","properties":{"price":{"type":"number"}},"required":["price"]},
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_patch_partial_update: httpMethodsPatchPartialUpdate
		},
	};
}


/**
 * Handler for DELETE /items/{id}
 */
async function httpMethodsDeleteResourceNotFound(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_delete_resource_not_found: httpMethodsDeleteResourceNotFound
		},
	};
}


/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutIdempotentOperation(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"id":1,"name":"Fixed Name","price":50.0};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHttpMethodsPutIdempotentOperation(): SpikardApp {
	const route: RouteMetadata = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_idempotent_operation",
		request_schema: {"type":"object","properties":{"id":{"type":"integer"},"name":{"type":"string"},"price":{"type":"number"}},"required":["id","name","price"]},
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_idempotent_operation: httpMethodsPutIdempotentOperation
		},
	};
}


/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutCompleteResourceReplacement(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"id":1,"name":"Updated Item","description":"Completely replaced","price":99.99,"in_stock":true};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHttpMethodsPutCompleteResourceReplacement(): SpikardApp {
	const route: RouteMetadata = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_complete_resource_replacement",
		request_schema: {"type":"object","properties":{"id":{"type":"integer"},"name":{"type":"string"},"description":{"type":"string"},"price":{"type":"number"},"in_stock":{"type":"boolean"}},"required":["description","id","in_stock","name","price"]},
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_complete_resource_replacement: httpMethodsPutCompleteResourceReplacement
		},
	};
}


/**
 * Handler for GET /compression/skip
 */
async function compressionCompressionPayloadBelowMinSizeIsNotCompressed(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Small payload","payload":"tiny"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCompressionCompressionPayloadBelowMinSizeIsNotCompressed(): SpikardApp {
	const config: ServerConfig = {
		compression: {
			gzip: true,
			brotli: false,
			minSize: 4096,
			quality: 6
		}
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
			compression_compression_payload_below_min_size_is_not_compressed: compressionCompressionPayloadBelowMinSizeIsNotCompressed
		},
		config,
	};
}


/**
 * Handler for GET /compression/gzip
 */
async function compressionCompressionGzipApplied(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"vary":"Accept-Encoding"};
	const responseBody = {"message":"Compressed payload","payload":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCompressionCompressionGzipApplied(): SpikardApp {
	const config: ServerConfig = {
		compression: {
			gzip: true,
			brotli: false,
			minSize: 0,
			quality: 4
		}
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
			compression_compression_gzip_applied: compressionCompressionGzipApplied
		},
		config,
	};
}


/**
 * Handler for POST /login/
 */
async function urlEncodedSimpleFormSubmissionSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"username":"johndoe"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedSimpleFormSubmissionSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/login/",
		handler_name: "url_encoded_simple_form_submission_success",
		request_schema: {"type":"object","required":["username","password"],"properties":{"username":{"type":"string"},"password":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_simple_form_submission_success: urlEncodedSimpleFormSubmissionSuccess
		},
	};
}


/**
 * Handler for POST /data
 */
async function urlEncoded15SpecialCharactersFieldNames(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = {"user-name":"JohnDoe","contact.email":"john@example.com"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncoded15SpecialCharactersFieldNames(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "url_encoded_15_special_characters_field_names",
		request_schema: {"type":"object","properties":{"user-name":{"type":"string"},"contact.email":{"type":"string","format":"email"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_15_special_characters_field_names: urlEncoded15SpecialCharactersFieldNames
		},
	};
}


/**
 * Handler for POST /form/validated
 */
async function urlEncodedPatternValidationFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["username"],"properties":{"username":{"type":"string","pattern":"^[a-z0-9_]+$"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_pattern_validation_fail: urlEncodedPatternValidationFail
		},
	};
}


/**
 * Handler for POST /settings
 */
async function urlEncoded22AdditionalPropertiesStrictFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["theme"],"properties":{"theme":{"type":"string","enum":["light","dark"]}},"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_22_additional_properties_strict_failure: urlEncoded22AdditionalPropertiesStrictFailure
		},
	};
}


/**
 * Handler for POST /accounts
 */
async function urlEncoded17PatternValidationFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["account_id"],"properties":{"account_id":{"type":"string","pattern":"^ACC-[0-9]{6}$"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_17_pattern_validation_failure: urlEncoded17PatternValidationFailure
		},
	};
}


/**
 * Handler for POST /subscribe
 */
async function urlEncoded20FormatEmailValidationFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["email"],"properties":{"email":{"type":"string","format":"email"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_20_format_email_validation_failure: urlEncoded20FormatEmailValidationFailure
		},
	};
}


/**
 * Handler for POST /form/tags
 */
async function urlEncodedMultipleValuesForSameField(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"tags":["python","fastapi","web"]};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedMultipleValuesForSameField(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/tags",
		handler_name: "url_encoded_multiple_values_for_same_field",
		request_schema: {"type":"object","required":["tags"],"properties":{"tags":{"type":"array","items":{"type":"string"}}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_multiple_values_for_same_field: urlEncodedMultipleValuesForSameField
		},
	};
}


/**
 * Handler for POST /login/
 */
async function urlEncodedRequiredFieldMissingValidationError(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["username","password"],"properties":{"username":{"type":"string"},"password":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_required_field_missing_validation_error: urlEncodedRequiredFieldMissingValidationError
		},
	};
}


/**
 * Handler for POST /register
 */
async function urlEncoded13ArrayFieldSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = {"tags":["python","rust","typescript"]};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncoded13ArrayFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/register",
		handler_name: "url_encoded_13_array_field_success",
		request_schema: {"type":"object","required":["tags"],"properties":{"tags":{"type":"array","items":{"type":"string"},"minItems":1}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_13_array_field_success: urlEncoded13ArrayFieldSuccess
		},
	};
}


/**
 * Handler for POST /form/
 */
async function urlEncodedNumericFieldTypeConversion(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"username":"johndoe","age":30};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedNumericFieldTypeConversion(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_numeric_field_type_conversion",
		request_schema: {"type":"object","required":["username"],"properties":{"username":{"type":"string"},"age":{"type":"integer"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_numeric_field_type_conversion: urlEncodedNumericFieldTypeConversion
		},
	};
}


/**
 * Handler for POST /form/
 */
async function urlEncodedSpecialCharactersEncoding(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"John Doe","description":"Test & Development"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedSpecialCharactersEncoding(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_special_characters_encoding",
		request_schema: {"type":"object","required":["name"],"properties":{"name":{"type":"string"},"description":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_special_characters_encoding: urlEncodedSpecialCharactersEncoding
		},
	};
}


/**
 * Handler for POST /form/
 */
async function urlEncodedBooleanFieldConversion(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"username":"johndoe","subscribe":true};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedBooleanFieldConversion(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_boolean_field_conversion",
		request_schema: {"type":"object","required":["username"],"properties":{"username":{"type":"string"},"subscribe":{"type":"boolean"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_boolean_field_conversion: urlEncodedBooleanFieldConversion
		},
	};
}


/**
 * Handler for POST /form/
 */
async function urlEncodedEmptyStringValue(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"username":"johndoe","description":""};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedEmptyStringValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_empty_string_value",
		request_schema: {"type":"object","required":["username"],"properties":{"username":{"type":"string"},"description":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_empty_string_value: urlEncodedEmptyStringValue
		},
	};
}


/**
 * Handler for POST /token
 */
async function urlEncodedOauth2PasswordGrantFlow(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"access_token":"johndoe","token_type":"bearer"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedOauth2PasswordGrantFlow(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/token",
		handler_name: "url_encoded_oauth2_password_grant_flow",
		request_schema: {"type":"object","required":["username","password","grant_type"],"properties":{"username":{"type":"string"},"password":{"type":"string"},"grant_type":{"type":"string"},"scope":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_oauth2_password_grant_flow: urlEncodedOauth2PasswordGrantFlow
		},
	};
}


/**
 * Handler for POST /tags
 */
async function urlEncoded19ArrayMinitemsValidationFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["tags"],"properties":{"tags":{"type":"array","items":{"type":"string"},"minItems":2}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_19_array_minitems_validation_failure: urlEncoded19ArrayMinitemsValidationFailure
		},
	};
}


/**
 * Handler for POST /register/
 */
async function urlEncodedOptionalFieldMissingSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"username":"johndoe","email":null};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncodedOptionalFieldMissingSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/register/",
		handler_name: "url_encoded_optional_field_missing_success",
		request_schema: {"type":"object","required":["username","password"],"properties":{"username":{"type":"string"},"password":{"type":"string"},"email":{"type":["string","null"],"format":"email"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_optional_field_missing_success: urlEncodedOptionalFieldMissingSuccess
		},
	};
}


/**
 * Handler for POST /profile
 */
async function urlEncoded14NestedObjectBracketNotation(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = {"user":{"name":"John Doe","email":"john@example.com","age":30}};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppUrlEncoded14NestedObjectBracketNotation(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/profile",
		handler_name: "url_encoded_14_nested_object_bracket_notation",
		request_schema: {"type":"object","required":["user"],"properties":{"user":{"type":"object","required":["name","email"],"properties":{"name":{"type":"string","minLength":1},"email":{"type":"string","format":"email"},"age":{"type":"integer","minimum":0}}}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_14_nested_object_bracket_notation: urlEncoded14NestedObjectBracketNotation
		},
	};
}


/**
 * Handler for POST /form/validated
 */
async function urlEncodedStringMaxLengthValidationFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["username"],"properties":{"username":{"type":"string","maxLength":20}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_string_max_length_validation_fail: urlEncodedStringMaxLengthValidationFail
		},
	};
}


/**
 * Handler for POST /products
 */
async function urlEncoded18IntegerMinimumValidationFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["quantity"],"properties":{"quantity":{"type":"integer","minimum":1}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_18_integer_minimum_validation_failure: urlEncoded18IntegerMinimumValidationFailure
		},
	};
}


/**
 * Handler for POST /products
 */
async function urlEncoded21IntegerTypeCoercionFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["price"],"properties":{"price":{"type":"integer"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_21_integer_type_coercion_failure: urlEncoded21IntegerTypeCoercionFailure
		},
	};
}


/**
 * Handler for POST /users
 */
async function urlEncoded16MinlengthValidationFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["username"],"properties":{"username":{"type":"string","minLength":3}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_16_minlength_validation_failure: urlEncoded16MinlengthValidationFailure
		},
	};
}


/**
 * Handler for POST /form/validated
 */
async function urlEncodedStringMinLengthValidationFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["username"],"properties":{"username":{"type":"string","minLength":3}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_string_min_length_validation_fail: urlEncodedStringMinLengthValidationFail
		},
	};
}


/**
 * Handler for POST /background/events
 */
async function backgroundBackgroundEventLoggingSecondPayload(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 202 };
	response.headers = {"content-type":"application/json"};
	BACKGROUND_STATE["background_background_event_logging_second_payload"] = BACKGROUND_STATE["background_background_event_logging_second_payload"] ?? [];
	const state = BACKGROUND_STATE["background_background_event_logging_second_payload"] as unknown[];
	const value = _body && typeof _body === "object" ? _body.event : undefined;
	if (value === undefined || value === null) {
		throw new Error("background task requires request body value");
	}
	void Promise.resolve().then(() => void state.push(value));
	response.body = null;
	return JSON.stringify(response);
}


async function backgroundBackgroundEventLoggingSecondPayloadBackgroundState(): Promise<string> {
	const state = BACKGROUND_STATE["background_background_event_logging_second_payload"] ?? [];
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/json" };
	response.body = { "events": state };
	return JSON.stringify(response);
}

export function createAppBackgroundBackgroundEventLoggingSecondPayload(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/background/events",
		handler_name: "background_background_event_logging_second_payload",
		request_schema: {"type":"object","properties":{"event":{"type":"string"}},"required":["event"],"additionalProperties":false},
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
			background_background_event_logging_second_payload_background_state: backgroundBackgroundEventLoggingSecondPayloadBackgroundState
		},
	};
}


/**
 * Handler for POST /background/events
 */
async function backgroundBackgroundEventLogging(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 202 };
	response.headers = {"content-type":"application/json"};
	BACKGROUND_STATE["background_background_event_logging"] = BACKGROUND_STATE["background_background_event_logging"] ?? [];
	const state = BACKGROUND_STATE["background_background_event_logging"] as unknown[];
	const value = _body && typeof _body === "object" ? _body.event : undefined;
	if (value === undefined || value === null) {
		throw new Error("background task requires request body value");
	}
	void Promise.resolve().then(() => void state.push(value));
	response.body = null;
	return JSON.stringify(response);
}


async function backgroundBackgroundEventLoggingBackgroundState(): Promise<string> {
	const state = BACKGROUND_STATE["background_background_event_logging"] ?? [];
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/json" };
	response.body = { "events": state };
	return JSON.stringify(response);
}

export function createAppBackgroundBackgroundEventLogging(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/background/events",
		handler_name: "background_background_event_logging",
		request_schema: {"type":"object","properties":{"event":{"type":"string"}},"required":["event"],"additionalProperties":false},
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
			background_background_event_logging_background_state: backgroundBackgroundEventLoggingBackgroundState
		},
	};
}


/**
 * Handler for GET /request-id/preserved
 */
async function requestIdRequestIdHeaderIsPreserved(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"x-request-id":"trace-123"};
	const responseBody = {"status":"preserved","echo":"trace-123"};
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
			request_id_request_id_header_is_preserved: requestIdRequestIdHeaderIsPreserved
		},
	};
}


/**
 * Handler for GET /request-id/disabled
 */
async function requestIdRequestIdMiddlewareCanBeDisabled(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"status":"no-request-id"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppRequestIdRequestIdMiddlewareCanBeDisabled(): SpikardApp {
	const config: ServerConfig = {
		enableRequestId: false
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
			request_id_request_id_middleware_can_be_disabled: requestIdRequestIdMiddlewareCanBeDisabled
		},
		config,
	};
}


/**
 * Handler for GET /request-id/generated
 */
async function requestIdRequestIdIsGeneratedWhenNotProvided(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"x-request-id":"00000000-0000-4000-8000-000000000000"};
	const responseBody = {"status":"generated"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppRequestIdRequestIdIsGeneratedWhenNotProvided(): SpikardApp {
	const config: ServerConfig = {
		enableRequestId: true
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
			request_id_request_id_is_generated_when_not_provided: requestIdRequestIdIsGeneratedWhenNotProvided
		},
		config,
	};
}


async function lifecycleHooksOnresponseSecurityHeadersSecurityHeadersOnResponse0(response: HookResponse): Promise<HookResponse> {
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
async function lifecycleHooksOnresponseSecurityHeaders(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"strict-transport-security":"max-age=31536000; includeSubDomains","x-xss-protection":"1; mode=block","x-frame-options":"DENY","x-content-type-options":"nosniff"};
	const responseBody = {"message":"Response with security headers"};
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
			lifecycle_hooks_onresponse_security_headers: lifecycleHooksOnresponseSecurityHeaders
		},
	lifecycleHooks: {
		onResponse: [lifecycleHooksOnresponseSecurityHeadersSecurityHeadersOnResponse0]
	},
	};
}


async function lifecycleHooksPrehandlerAuthenticationFailedShortCircuitAuthenticatorPreHandler0(_request: HookRequest): Promise<HookResult> {
	// preHandler hook: authenticator - Short circuits with 401
	return {
		statusCode: 401,
		body: {
			error: "Unauthorized",
			message: "Invalid or expired authentication token"
		}
	};
}



/**
 * Handler for GET /api/protected-resource-fail
 */
async function lifecycleHooksPrehandlerAuthenticationFailedShortCircuit(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 401 };
	const responseBody = {"error":"Unauthorized","message":"Invalid or expired authentication token"};
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
			lifecycle_hooks_prehandler_authentication_failed_short_circuit: lifecycleHooksPrehandlerAuthenticationFailedShortCircuit
		},
	lifecycleHooks: {
		preHandler: [lifecycleHooksPrehandlerAuthenticationFailedShortCircuitAuthenticatorPreHandler0]
	},
	};
}


async function lifecycleHooksPrehandlerAuthorizationCheckAuthenticatorPreHandler0(request: HookRequest): Promise<HookResult> {
	// Mock preHandler hook: authenticator
	return request;
}

async function lifecycleHooksPrehandlerAuthorizationCheckAuthorizerPreHandler1(request: HookRequest): Promise<HookResult> {
	// Mock preHandler hook: authorizer
	return request;
}



/**
 * Handler for GET /api/admin-only
 */
async function lifecycleHooksPrehandlerAuthorizationCheck(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Admin access granted","user_id":"admin-456","role":"admin"};
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
			lifecycle_hooks_prehandler_authorization_check: lifecycleHooksPrehandlerAuthorizationCheck
		},
	lifecycleHooks: {
		preHandler: [lifecycleHooksPrehandlerAuthorizationCheckAuthenticatorPreHandler0, lifecycleHooksPrehandlerAuthorizationCheckAuthorizerPreHandler1]
	},
	};
}


async function lifecycleHooksPrehandlerAuthenticationSuccessAuthenticatorPreHandler0(request: HookRequest): Promise<HookResult> {
	// Mock preHandler hook: authenticator
	return request;
}



/**
 * Handler for GET /api/protected-resource
 */
async function lifecycleHooksPrehandlerAuthenticationSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Access granted","user_id":"user-123","authenticated":true};
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
			lifecycle_hooks_prehandler_authentication_success: lifecycleHooksPrehandlerAuthenticationSuccess
		},
	lifecycleHooks: {
		preHandler: [lifecycleHooksPrehandlerAuthenticationSuccessAuthenticatorPreHandler0]
	},
	};
}


async function lifecycleHooksPrevalidationRateLimitExceededShortCircuitRateLimiterPreValidation0(_request: HookRequest): Promise<HookResult> {
	// preValidation hook: rate_limiter - Short circuits with 429
	return {
		statusCode: 429,
		body: {
			error: "Rate limit exceeded",
			message: "Too many requests, please try again later"
		},
		headers: {
			"Retry-After": "60"
		}
	};
}



/**
 * Handler for POST /api/test-rate-limit-exceeded
 */
async function lifecycleHooksPrevalidationRateLimitExceededShortCircuit(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 429 };
	response.headers = {"retry-after":"60"};
	const responseBody = {"error":"Rate limit exceeded","message":"Too many requests, please try again later"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppLifecycleHooksPrevalidationRateLimitExceededShortCircuit(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/api/test-rate-limit-exceeded",
		handler_name: "lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit",
		request_schema: {"type":"object","properties":{"data":{"type":"string"}},"required":["data"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit: lifecycleHooksPrevalidationRateLimitExceededShortCircuit
		},
	lifecycleHooks: {
		preValidation: [lifecycleHooksPrevalidationRateLimitExceededShortCircuitRateLimiterPreValidation0]
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
async function lifecycleHooksOnerrorErrorLogging(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 500 };
	response.headers = {"content-type":"application/json"};
	const responseBody = {"error":"Internal Server Error","message":"An unexpected error occurred","error_id":".*"};
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
			lifecycle_hooks_onerror_error_logging: lifecycleHooksOnerrorErrorLogging
		},
	lifecycleHooks: {
		onError: [lifecycleHooksOnerrorErrorLoggingErrorLoggerOnError0, lifecycleHooksOnerrorErrorLoggingErrorFormatterOnError1]
	},
	};
}


async function lifecycleHooksMultipleHooksAllPhasesRequestLoggerOnRequest0(request: HookRequest): Promise<HookResult> {
	// Mock onRequest hook: request_logger
	return request;
}

async function lifecycleHooksMultipleHooksAllPhasesRequestIdGeneratorOnRequest1(request: HookRequest): Promise<HookResult> {
	// Mock onRequest hook: request_id_generator
	return request;
}

async function lifecycleHooksMultipleHooksAllPhasesRateLimiterPreValidation0(request: HookRequest): Promise<HookResult> {
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

async function lifecycleHooksMultipleHooksAllPhasesSecurityHeadersOnResponse0(response: HookResponse): Promise<HookResponse> {
	// onResponse hook: security_headers - Adds security headers
	if (!response.headers) response.headers = {};
	response.headers["X-Content-Type-Options"] = "nosniff";
	response.headers["X-Frame-Options"] = "DENY";
	response.headers["X-XSS-Protection"] = "1; mode=block";
	response.headers["Strict-Transport-Security"] = "max-age=31536000; includeSubDomains";
	return response;
}

async function lifecycleHooksMultipleHooksAllPhasesResponseTimerOnResponse1(response: HookResponse): Promise<HookResponse> {
	// onResponse hook: response_timer - Adds timing header
	if (!response.headers) response.headers = {};
	response.headers["X-Response-Time"] = "0ms";
	return response;
}

async function lifecycleHooksMultipleHooksAllPhasesAuditLoggerOnResponse2(response: HookResponse): Promise<HookResponse> {
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
async function lifecycleHooksMultipleHooksAllPhases(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"x-response-time":".*ms","x-frame-options":"DENY","x-content-type-options":"nosniff","x-request-id":".*"};
	const responseBody = {"message":"Action completed successfully","user_id":"user-123","action":"update_profile","request_id":".*"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppLifecycleHooksMultipleHooksAllPhases(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/api/full-lifecycle",
		handler_name: "lifecycle_hooks_multiple_hooks_all_phases",
		request_schema: {"type":"object","properties":{"user_id":{"type":"string"},"action":{"type":"string"}},"required":["user_id","action"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_multiple_hooks_all_phases: lifecycleHooksMultipleHooksAllPhases
		},
	lifecycleHooks: {
		onRequest: [lifecycleHooksMultipleHooksAllPhasesRequestLoggerOnRequest0, lifecycleHooksMultipleHooksAllPhasesRequestIdGeneratorOnRequest1],
		preValidation: [lifecycleHooksMultipleHooksAllPhasesRateLimiterPreValidation0],
		preHandler: [lifecycleHooksMultipleHooksAllPhasesAuthenticatorPreHandler0, lifecycleHooksMultipleHooksAllPhasesAuthorizerPreHandler1],
		onResponse: [lifecycleHooksMultipleHooksAllPhasesSecurityHeadersOnResponse0, lifecycleHooksMultipleHooksAllPhasesResponseTimerOnResponse1, lifecycleHooksMultipleHooksAllPhasesAuditLoggerOnResponse2],
		onError: [lifecycleHooksMultipleHooksAllPhasesErrorLoggerOnError0]
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
async function lifecycleHooksHookExecutionOrder(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Hooks executed in order","execution_order":["first_hook","second_hook","third_hook"]};
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
			lifecycle_hooks_hook_execution_order: lifecycleHooksHookExecutionOrder
		},
	lifecycleHooks: {
		onRequest: [lifecycleHooksHookExecutionOrderFirstHookOnRequest0, lifecycleHooksHookExecutionOrderSecondHookOnRequest1, lifecycleHooksHookExecutionOrderThirdHookOnRequest2]
	},
	};
}


async function lifecycleHooksOnresponseResponseTimingStartTimerOnRequest0(request: HookRequest): Promise<HookResult> {
	// Mock onRequest hook: start_timer
	return request;
}

async function lifecycleHooksOnresponseResponseTimingResponseTimerOnResponse0(response: HookResponse): Promise<HookResponse> {
	// onResponse hook: response_timer - Adds timing header
	if (!response.headers) response.headers = {};
	response.headers["X-Response-Time"] = "0ms";
	return response;
}



/**
 * Handler for GET /api/test-timing
 */
async function lifecycleHooksOnresponseResponseTiming(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"x-response-time":".*ms"};
	const responseBody = {"message":"Response with timing info"};
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
			lifecycle_hooks_onresponse_response_timing: lifecycleHooksOnresponseResponseTiming
		},
	lifecycleHooks: {
		onRequest: [lifecycleHooksOnresponseResponseTimingStartTimerOnRequest0],
		onResponse: [lifecycleHooksOnresponseResponseTimingResponseTimerOnResponse0]
	},
	};
}


async function lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuitAuthenticatorPreHandler0(_request: HookRequest): Promise<HookResult> {
	// preHandler hook: authenticator - Short circuits with 403
	return {
		statusCode: 403,
		body: {
			error: "Forbidden",
			message: "Admin role required for this endpoint"
		}
	};
}

async function lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuitAuthorizerPreHandler1(_request: HookRequest): Promise<HookResult> {
	// preHandler hook: authorizer - Short circuits with 403
	return {
		statusCode: 403,
		body: {
			error: "Forbidden",
			message: "Admin role required for this endpoint"
		}
	};
}



/**
 * Handler for GET /api/admin-only-forbidden
 */
async function lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuit(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 403 };
	const responseBody = {"error":"Forbidden","message":"Admin role required for this endpoint"};
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
			lifecycle_hooks_prehandler_authorization_forbidden_short_circuit: lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuit
		},
	lifecycleHooks: {
		preHandler: [lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuitAuthenticatorPreHandler0, lifecycleHooksPrehandlerAuthorizationForbiddenShortCircuitAuthorizerPreHandler1]
	},
	};
}


async function lifecycleHooksOnrequestRequestLoggingRequestLoggerOnRequest0(request: HookRequest): Promise<HookResult> {
	// Mock onRequest hook: request_logger
	return request;
}

async function lifecycleHooksOnrequestRequestLoggingRequestIdGeneratorOnRequest1(request: HookRequest): Promise<HookResult> {
	// Mock onRequest hook: request_id_generator
	return request;
}



/**
 * Handler for GET /api/test-on-request
 */
async function lifecycleHooksOnrequestRequestLogging(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"x-request-id":".*"};
	const responseBody = {"message":"onRequest hooks executed","request_logged":true,"has_request_id":true};
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
			lifecycle_hooks_onrequest_request_logging: lifecycleHooksOnrequestRequestLogging
		},
	lifecycleHooks: {
		onRequest: [lifecycleHooksOnrequestRequestLoggingRequestLoggerOnRequest0, lifecycleHooksOnrequestRequestLoggingRequestIdGeneratorOnRequest1]
	},
	};
}


async function lifecycleHooksPrevalidationRateLimitingRateLimiterPreValidation0(request: HookRequest): Promise<HookResult> {
	// Mock preValidation hook: rate_limiter
	return request;
}



/**
 * Handler for POST /api/test-rate-limit
 */
async function lifecycleHooksPrevalidationRateLimiting(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Request accepted","rate_limit_checked":true};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppLifecycleHooksPrevalidationRateLimiting(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/api/test-rate-limit",
		handler_name: "lifecycle_hooks_prevalidation_rate_limiting",
		request_schema: {"type":"object","properties":{"data":{"type":"string"}},"required":["data"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			lifecycle_hooks_prevalidation_rate_limiting: lifecycleHooksPrevalidationRateLimiting
		},
	lifecycleHooks: {
		preValidation: [lifecycleHooksPrevalidationRateLimitingRateLimiterPreValidation0]
	},
	};
}


/**
 * Handler for GET /timeouts/slow
 */
async function requestTimeoutRequestExceedsTimeout(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 408 };
	const signal = isAbortSignalLike(_context?.signal) ? _context?.signal : undefined;
	await sleep(1500, signal);
	response.body = null;
	return JSON.stringify(response);
}

export function createAppRequestTimeoutRequestExceedsTimeout(): SpikardApp {
	const config: ServerConfig = {
		requestTimeout: 1
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
			request_timeout_request_exceeds_timeout: requestTimeoutRequestExceedsTimeout
		},
		config,
	};
}


/**
 * Handler for GET /timeouts/fast
 */
async function requestTimeoutRequestCompletesBeforeTimeout(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const signal = isAbortSignalLike(_context?.signal) ? _context?.signal : undefined;
	await sleep(100, signal);
	const responseBody = {"status":"ok","duration":"fast"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppRequestTimeoutRequestCompletesBeforeTimeout(): SpikardApp {
	const config: ServerConfig = {
		requestTimeout: 2
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
			request_timeout_request_completes_before_timeout: requestTimeoutRequestCompletesBeforeTimeout
		},
		config,
	};
}


/**
 * Handler for POST /body-limit/under
 */
async function bodyLimitsBodyUnderLimitSucceeds(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"accepted":true,"note":"small"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppBodyLimitsBodyUnderLimitSucceeds(): SpikardApp {
	const config: ServerConfig = {
		maxBodySize: 64
	};

	const route: RouteMetadata = {
		method: "POST",
		path: "/body-limit/under",
		handler_name: "body_limits_body_under_limit_succeeds",
		request_schema: {"type":"object","properties":{"note":{"type":"string"}},"required":["note"],"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			body_limits_body_under_limit_succeeds: bodyLimitsBodyUnderLimitSucceeds
		},
		config,
	};
}


/**
 * Handler for POST /body-limit/over
 */
async function bodyLimitsBodyOverLimitReturns413(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 413 };
	response.body = null;
	return JSON.stringify(response);
}

export function createAppBodyLimitsBodyOverLimitReturns413(): SpikardApp {
	const config: ServerConfig = {
		maxBodySize: 64
	};

	const route: RouteMetadata = {
		method: "POST",
		path: "/body-limit/over",
		handler_name: "body_limits_body_over_limit_returns_413",
		request_schema: {"type":"object","properties":{"note":{"type":"string"}},"required":["note"],"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			body_limits_body_over_limit_returns_413: bodyLimitsBodyOverLimitReturns413
		},
		config,
	};
}


/**
 * Handler for GET /stream/json-lines
 */
async function streamingStreamJsonLines(_requestJson: string): Promise<StreamingResponse> {
	const stream = async function* () {
		yield "{\"index\":0,\"payload\":\"alpha\"}\\n";
		yield "{\"index\":1,\"payload\":\"beta\"}\\n";
		yield "{\"index\":2,\"payload\":\"gamma\"}\\n";
	};

	return new StreamingResponse(stream(), {
		statusCode: 200,
		headers: {"content-type":"application/x-ndjson"}
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
			streaming_stream_json_lines: streamingStreamJsonLines
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
		headers: {"content-type":"application/octet-stream"}
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
			streaming_binary_log_download: streamingBinaryLogDownload
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
		headers: {"content-type":"text/csv"}
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
			streaming_chunked_csv_export: streamingChunkedCsvExport
		},
	};
}




export function createAppStaticFilesStaticFileServerReturnsTextFile(): SpikardApp {
	const config: ServerConfig = {
		staticFiles: [
			{
				directory: new URL("./static_assets/static_files_static_file_server_returns_text_file/public_0", import.meta.url).pathname,
				routePrefix: "/public",
				cacheControl: "public, max-age=60",
			}
		]
	};

	return {
		routes: [],
		handlers: {
		},
		config,
	};
}




export function createAppStaticFilesStaticServerReturnsIndexHtmlForDirectory(): SpikardApp {
	const config: ServerConfig = {
		staticFiles: [
			{
				directory: new URL("./static_assets/static_files_static_server_returns_index_html_for_directory/app_0", import.meta.url).pathname,
				routePrefix: "/app",
			}
		]
	};

	return {
		routes: [],
		handlers: {
		},
		config,
	};
}


/**
 * Handler for GET /headers/pattern
 */
async function headersHeaderRegexValidationSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"x_request_id":"12345"};
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
		parameter_schema: {"type":"object","properties":{"X-Request-Id":{"type":"string","annotation":"str","pattern":"^[0-9]{3,}$","source":"header"}},"required":["X-Request-Id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_regex_validation_success: headersHeaderRegexValidationSuccess
		},
	};
}


/**
 * Handler for GET /api/data
 */
async function headers33ApiKeyHeaderValid(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const result: Record<string, unknown> = {};
	const xAPIKey = _params["X-API-Key"];
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
		parameter_schema: {"type":"object","properties":{"X-API-Key":{"type":"string","pattern":"^[a-f0-9]{32}$","source":"header"}},"required":["X-API-Key"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_33_api_key_header_valid: headers33ApiKeyHeaderValid
		},
	};
}


/**
 * Handler for GET /headers/content-type
 */
async function headersContentTypeHeaderApplicationJson(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"content_type":"application/json"};
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
		parameter_schema: {"type":"object","properties":{"Content-Type":{"type":"string","annotation":"str","source":"header"}},"required":["Content-Type"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_content_type_header_application_json: headersContentTypeHeaderApplicationJson
		},
	};
}


/**
 * Handler for GET /headers/accept-language
 */
async function headersAcceptLanguageHeader(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"accept_language":"en-US,en;q=0.9"};
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
		parameter_schema: {"type":"object","properties":{"Accept-Language":{"type":"string","annotation":"str","source":"header"}},"required":["Accept-Language"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_accept_language_header: headersAcceptLanguageHeader
		},
	};
}


/**
 * Handler for GET /users/me
 */
async function headersXApiKeyRequiredHeaderSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"username":"secret"};
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
		parameter_schema: {"type":"object","properties":{"key":{"type":"string","annotation":"str","source":"header"}},"required":["key"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_x_api_key_required_header_success: headersXApiKeyRequiredHeaderSuccess
		},
	};
}


/**
 * Handler for GET /headers/max-length
 */
async function headersHeaderValidationMaxLengthConstraintFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const xSessionId = _params["X-Session-Id"];
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
		parameter_schema: {"type":"object","properties":{"X-Session-Id":{"type":"string","annotation":"str","maxLength":20,"source":"header"}},"required":["X-Session-Id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_validation_max_length_constraint_fail: headersHeaderValidationMaxLengthConstraintFail
		},
	};
}


/**
 * Handler for GET /users/me
 */
async function headersXApiKeyRequiredHeaderMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const xAPIKey = _params["X-API-Key"];
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
		parameter_schema: {"type":"object","properties":{"X-API-Key":{"type":"string","annotation":"str","source":"header"}},"required":["X-API-Key"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_x_api_key_required_header_missing: headersXApiKeyRequiredHeaderMissing
		},
	};
}


/**
 * Handler for GET /headers/origin
 */
async function headersOriginHeader(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"origin":"https://example.com"};
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
		parameter_schema: {"type":"object","properties":{"Origin":{"type":"string","annotation":"str","source":"header"}},"required":["Origin"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_origin_header: headersOriginHeader
		},
	};
}


/**
 * Handler for GET /items/
 */
async function headersUserAgentHeaderDefaultValue(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"User-Agent":"testclient"};
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
		parameter_schema: {"type":"object","properties":{"User-Agent":{"type":"string","annotation":"str","source":"header","default":"testclient"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_user_agent_header_default_value: headersUserAgentHeaderDefaultValue
		},
	};
}


/**
 * Handler for GET /protected
 */
async function headers32BearerTokenMissingPrefix(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const authorization = _params.Authorization;
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
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","pattern":"^Bearer [A-Za-z0-9-._~+/]+=*$","source":"header"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_32_bearer_token_missing_prefix: headers32BearerTokenMissingPrefix
		},
	};
}


/**
 * Handler for GET /items/
 */
async function headersOptionalHeaderWithNoneDefaultMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"strange_header":null};
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
		parameter_schema: {"type":"object","properties":{"strange-header":{"type":"string","annotation":"str","source":"header","default":null}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_optional_header_with_none_default_missing: headersOptionalHeaderWithNoneDefaultMissing
		},
	};
}


/**
 * Handler for GET /headers/pattern
 */
async function headersHeaderRegexValidationFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const xRequestId = _params["X-Request-Id"];
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
		parameter_schema: {"type":"object","properties":{"X-Request-Id":{"type":"string","annotation":"str","pattern":"^[0-9]{3,}$","source":"header"}},"required":["X-Request-Id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_regex_validation_fail: headersHeaderRegexValidationFail
		},
	};
}


/**
 * Handler for GET /protected
 */
async function headers31BearerTokenFormatInvalid(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const authorization = _params.Authorization;
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
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","pattern":"^Bearer [A-Za-z0-9-._~+/]+=*$","source":"header"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_31_bearer_token_format_invalid: headers31BearerTokenFormatInvalid
		},
	};
}


/**
 * Handler for GET /users/me
 */
async function headersXApiKeyOptionalHeaderSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"msg":"Hello secret"};
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
		parameter_schema: {"type":"object","properties":{"key":{"type":"string","annotation":"str","source":"header"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_x_api_key_optional_header_success: headersXApiKeyOptionalHeaderSuccess
		},
	};
}


/**
 * Handler for GET /users/me
 */
async function headersAuthorizationHeaderSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"scheme":"Digest","credentials":"foobar"};
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
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","annotation":"str","source":"header"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_authorization_header_success: headersAuthorizationHeaderSuccess
		},
	};
}


/**
 * Handler for GET /protected
 */
async function headers30BearerTokenFormatValid(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const result: Record<string, unknown> = {};
	const authorization = _params.Authorization;
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
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","pattern":"^Bearer [A-Za-z0-9-._~+/]+=*$","source":"header"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_30_bearer_token_format_valid: headers30BearerTokenFormatValid
		},
	};
}


/**
 * Handler for GET /users/me
 */
async function headersAuthorizationHeaderMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const authorization = _params.Authorization;
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
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","annotation":"str","source":"header"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_authorization_header_missing: headersAuthorizationHeaderMissing
		},
	};
}


/**
 * Handler for GET /headers/accept
 */
async function headersAcceptHeaderJson(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"accept":"application/json"};
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
		parameter_schema: {"type":"object","properties":{"Accept":{"type":"string","annotation":"str","source":"header"}},"required":["Accept"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_accept_header_json: headersAcceptHeaderJson
		},
	};
}


/**
 * Handler for GET /headers/accept-encoding
 */
async function headersAcceptEncodingHeader(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"accept_encoding":"gzip, deflate, br"};
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
		parameter_schema: {"type":"object","properties":{"Accept-Encoding":{"type":"string","annotation":"str","source":"header"}},"required":["Accept-Encoding"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_accept_encoding_header: headersAcceptEncodingHeader
		},
	};
}


/**
 * Handler for GET /users/me
 */
async function headersAuthorizationHeaderWrongScheme(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const authorization = _params.Authorization;
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
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","annotation":"str","source":"header","pattern":"^Digest .+"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_authorization_header_wrong_scheme: headersAuthorizationHeaderWrongScheme
		},
	};
}


/**
 * Handler for GET /headers/validated
 */
async function headersHeaderValidationMinLengthConstraint(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const xToken = _params["X-Token"];
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
		parameter_schema: {"type":"object","properties":{"X-Token":{"type":"string","annotation":"str","minLength":3,"source":"header"}},"required":["X-Token"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_validation_min_length_constraint: headersHeaderValidationMinLengthConstraint
		},
	};
}


/**
 * Handler for GET /headers/basic-auth
 */
async function headersBasicAuthenticationSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"username":"username","password":"password"};
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
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","annotation":"str","source":"header"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_basic_authentication_success: headersBasicAuthenticationSuccess
		},
	};
}


/**
 * Handler for GET /headers/bearer-auth
 */
async function headersBearerTokenAuthenticationMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const authorization = _params.Authorization;
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
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","annotation":"str","source":"header","pattern":"^Bearer .+"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_bearer_token_authentication_missing: headersBearerTokenAuthenticationMissing
		},
	};
}


/**
 * Handler for GET /users/me
 */
async function headersXApiKeyOptionalHeaderMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"msg":"Hello World"};
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
		parameter_schema: {"type":"object","properties":{"key":{"type":"string","annotation":"str","source":"header"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_x_api_key_optional_header_missing: headersXApiKeyOptionalHeaderMissing
		},
	};
}


/**
 * Handler for GET /items/
 */
async function headersMultipleHeaderValuesXToken(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"X-Token values":["foo","bar"]};
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
		parameter_schema: {"type":"object","properties":{"x-token":{"type":"string","annotation":"str","source":"header"}},"required":["x-token"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_multiple_header_values_x_token: headersMultipleHeaderValuesXToken
		},
	};
}


/**
 * Handler for GET /headers/multiple
 */
async function headersMultipleCustomHeaders(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"x_request_id":"req-12345","x_client_version":"1.2.3","x_trace_id":"trace-abc"};
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
		parameter_schema: {"type":"object","properties":{"X-Request-Id":{"type":"string","annotation":"str","source":"header"},"X-Client-Version":{"type":"string","annotation":"str","source":"header"},"X-Trace-Id":{"type":"string","annotation":"str","source":"header"}},"required":["X-Request-Id","X-Client-Version","X-Trace-Id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_multiple_custom_headers: headersMultipleCustomHeaders
		},
	};
}


/**
 * Handler for GET /api/data
 */
async function headers34ApiKeyHeaderInvalid(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const xAPIKey = _params["X-API-Key"];
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
		parameter_schema: {"type":"object","properties":{"X-API-Key":{"type":"string","pattern":"^[a-f0-9]{32}$","source":"header"}},"required":["X-API-Key"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_34_api_key_header_invalid: headers34ApiKeyHeaderInvalid
		},
	};
}


/**
 * Handler for GET /headers/bearer-auth
 */
async function headersBearerTokenAuthenticationSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"token":"valid_token_123"};
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
		parameter_schema: {"type":"object","properties":{"Authorization":{"type":"string","annotation":"str","source":"header"}},"required":["Authorization"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_bearer_token_authentication_success: headersBearerTokenAuthenticationSuccess
		},
	};
}


/**
 * Handler for GET /headers/host
 */
async function headersHostHeader(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"host":"example.com:8080"};
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
		parameter_schema: {"type":"object","properties":{"Host":{"type":"string","annotation":"str","source":"header"}},"required":["Host"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_host_header: headersHostHeader
		},
	};
}


/**
 * Handler for GET /headers/referer
 */
async function headersRefererHeader(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"referer":"https://example.com/page"};
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
		parameter_schema: {"type":"object","properties":{"Referer":{"type":"string","annotation":"str","source":"header"}},"required":["Referer"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_referer_header: headersRefererHeader
		},
	};
}


/**
 * Handler for GET /headers/underscore
 */
async function headersHeaderWithUnderscoreConversionExplicit(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"x_token":"secret123"};
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
		parameter_schema: {"type":"object","properties":{"X-Token":{"type":"string","annotation":"str","source":"header"}},"required":["X-Token"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_with_underscore_conversion_explicit: headersHeaderWithUnderscoreConversionExplicit
		},
	};
}


/**
 * Handler for POST /echo
 */
async function headersHeaderCaseInsensitivityAccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"content_type_lower":"application/json","content_type_upper":"application/json","content_type_mixed":"application/json"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppHeadersHeaderCaseInsensitivityAccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/echo",
		handler_name: "headers_header_case_insensitivity_access",
		request_schema: {"type":"object","properties":{"test":{"type":"string"}},"additionalProperties":false,"required":["test"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_case_insensitivity_access: headersHeaderCaseInsensitivityAccess
		},
	};
}


/**
 * Handler for GET /items/
 */
async function headersUserAgentHeaderCustomValue(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"User-Agent":"Mozilla/5.0 Custom Browser"};
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
		parameter_schema: {"type":"object","properties":{"User-Agent":{"type":"string","annotation":"str","source":"header"}},"required":["User-Agent"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_user_agent_header_custom_value: headersUserAgentHeaderCustomValue
		},
	};
}


/**
 * Handler for GET /path/bool/{item_id}
 */
async function pathParamsBooleanPathParameterTrue(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"item_id":true};
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"boolean","source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_boolean_path_parameter_true: pathParamsBooleanPathParameterTrue
		},
	};
}


/**
 * Handler for GET /prices/{amount}
 */
async function pathParams29DecimalPathParamSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"amount":"19.99"};
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
		parameter_schema: {"type":"object","properties":{"amount":{"type":"string","format":"decimal","source":"path"}},"required":["amount"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_29_decimal_path_param_success: pathParams29DecimalPathParamSuccess
		},
	};
}


/**
 * Handler for GET /path/param-lt-gt/{item_id}
 */
async function pathParamsIntegerPathParameterWithCombinedLtAndGtConstraintsSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"item_id":2};
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"integer","exclusiveMinimum":1,"exclusiveMaximum":3,"source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success: pathParamsIntegerPathParameterWithCombinedLtAndGtConstraintsSuccess
		},
	};
}


/**
 * Handler for GET /repos/{owner}/{repo}
 */
async function pathParams33StringPatternPathSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"owner":"spikard-labs","repo":"spikard-http"};
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
		parameter_schema: {"type":"object","properties":{"owner":{"type":"string","pattern":"^[a-zA-Z0-9-]+$","source":"path"},"repo":{"type":"string","pattern":"^[a-zA-Z0-9-_]+$","source":"path"}},"required":["owner","repo"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_33_string_pattern_path_success: pathParams33StringPatternPathSuccess
		},
	};
}


/**
 * Handler for GET /users/{username}
 */
async function pathParams31StringMinlengthPathFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const username = _params.username;
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
		parameter_schema: {"type":"object","properties":{"username":{"type":"string","minLength":3,"source":"path"}},"required":["username"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_31_string_minlength_path_failure: pathParams31StringMinlengthPathFailure
		},
	};
}


/**
 * Handler for GET /offset/{value}
 */
async function pathParams35NegativeIntegerPathParam(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"value":-100};
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
		parameter_schema: {"type":"object","properties":{"value":{"type":"integer","source":"path"}},"required":["value"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_35_negative_integer_path_param: pathParams35NegativeIntegerPathParam
		},
	};
}


/**
 * Handler for GET /models/{model_name}
 */
async function pathParamsEnumPathParameterInvalidValue(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const modelName = _params.model_name;
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
		parameter_schema: {"type":"object","properties":{"model_name":{"type":"string","enum":["alexnet","resnet","lenet"],"source":"path"}},"required":["model_name"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_enum_path_parameter_invalid_value: pathParamsEnumPathParameterInvalidValue
		},
	};
}


/**
 * Handler for GET /bookings/{timestamp}
 */
async function pathParams27DatetimeFormatPathParamSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"timestamp":"2025-10-30T14:30:00Z"};
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
		parameter_schema: {"type":"object","properties":{"timestamp":{"type":"string","format":"date-time","source":"path"}},"required":["timestamp"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_27_datetime_format_path_param_success: pathParams27DatetimeFormatPathParamSuccess
		},
	};
}


/**
 * Handler for GET /events/{date}
 */
async function pathParams25DateFormatInvalidFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const date = _params.date;
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
		parameter_schema: {"type":"object","properties":{"date":{"type":"string","format":"date","source":"path"}},"required":["date"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_25_date_format_invalid_failure: pathParams25DateFormatInvalidFailure
		},
	};
}


/**
 * Handler for GET /path/param-lt/{item_id}
 */
async function pathParamsIntegerPathParameterWithLtConstraintSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"item_id":2};
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"integer","exclusiveMaximum":3,"source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_lt_constraint_success: pathParamsIntegerPathParameterWithLtConstraintSuccess
		},
	};
}


/**
 * Handler for GET /path/param-gt/{item_id}
 */
async function pathParamsIntegerPathParameterWithGtConstraintSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"item_id":42};
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"integer","exclusiveMinimum":3,"source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_gt_constraint_success: pathParamsIntegerPathParameterWithGtConstraintSuccess
		},
	};
}


/**
 * Handler for GET /delays/{duration}
 */
async function pathParams28DurationFormatPathParamSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"duration":"P1DT2H30M"};
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
		parameter_schema: {"type":"object","properties":{"duration":{"type":"string","format":"duration","source":"path"}},"required":["duration"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_28_duration_format_path_param_success: pathParams28DurationFormatPathParamSuccess
		},
	};
}


/**
 * Handler for GET /type-syntax/items-count/{count:int}
 */
async function pathParamsPathParameterTypeSyntaxWithOverride(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"count":"50"};
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
		parameter_schema: {"type":"object","properties":{"count":{"type":"integer","minimum":1,"maximum":100,"source":"path"}},"required":["count"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_path_parameter_type_syntax_with_override: pathParamsPathParameterTypeSyntaxWithOverride
		},
	};
}


/**
 * Handler for GET /items/{id}
 */
async function pathParams20UuidV3PathParamSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"id":"e8b5a51d-11c8-3310-a6ab-367563f20686"};
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
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","format":"uuid","uuidVersion":"3","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_20_uuid_v3_path_param_success: pathParams20UuidV3PathParamSuccess
		},
	};
}


/**
 * Handler for GET /path/int/{item_id}
 */
async function pathParamsIntegerPathParameterInvalidString(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const itemId = _params.item_id;
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"integer","source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_invalid_string: pathParamsIntegerPathParameterInvalidString
		},
	};
}


/**
 * Handler for GET /users/{username}
 */
async function pathParams30StringMinlengthPathSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"username":"alice"};
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
		parameter_schema: {"type":"object","properties":{"username":{"type":"string","minLength":3,"source":"path"}},"required":["username"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_30_string_minlength_path_success: pathParams30StringMinlengthPathSuccess
		},
	};
}


/**
 * Handler for GET /path/param-le/{item_id}
 */
async function pathParamsIntegerPathParameterWithLeConstraintSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"item_id":3};
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"integer","maximum":3,"source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_le_constraint_success: pathParamsIntegerPathParameterWithLeConstraintSuccess
		},
	};
}


/**
 * Handler for GET /type-syntax/items/{id:uuid}
 */
async function pathParamsPathParameterTypeSyntaxInvalidUuid(requestJson: string, _context?: HandlerContext): Promise<string> {
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
			path_params_path_parameter_type_syntax_invalid_uuid: pathParamsPathParameterTypeSyntaxInvalidUuid
		},
	};
}


/**
 * Handler for GET /files/{file_path:path}
 */
async function pathParamsPathTypeParameterFilePath(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"file_path":"home/johndoe/myfile.txt"};
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
		parameter_schema: {"type":"object","properties":{"file_path":{"type":"string","source":"path"}},"required":["file_path"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_path_type_parameter_file_path: pathParamsPathTypeParameterFilePath
		},
	};
}


/**
 * Handler for GET /type-syntax/items/{id:uuid}
 */
async function pathParamsPathParameterWithTypeSyntaxUuid(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"id":"550e8400-e29b-41d4-a716-446655440000"};
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
			path_params_path_parameter_with_type_syntax_uuid: pathParamsPathParameterWithTypeSyntaxUuid
		},
	};
}


/**
 * Handler for GET /users/{username}
 */
async function pathParams32StringMaxlengthPathFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const username = _params.username;
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
		parameter_schema: {"type":"object","properties":{"username":{"type":"string","maxLength":20,"source":"path"}},"required":["username"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_32_string_maxlength_path_failure: pathParams32StringMaxlengthPathFailure
		},
	};
}


/**
 * Handler for GET /path/int/{item_id}
 */
async function pathParamsIntegerPathParameterSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"item_id":42};
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"integer","source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_success: pathParamsIntegerPathParameterSuccess
		},
	};
}


/**
 * Handler for GET /repos/{owner}
 */
async function pathParams34StringPatternPathFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const owner = _params.owner;
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
		parameter_schema: {"type":"object","properties":{"owner":{"type":"string","pattern":"^[a-zA-Z0-9-]+$","source":"path"}},"required":["owner"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_34_string_pattern_path_failure: pathParams34StringPatternPathFailure
		},
	};
}


/**
 * Handler for GET /items/{id}
 */
async function pathParams21UuidV5PathParamSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"id":"630eb68f-e0fa-5ecc-887a-7c7a62614681"};
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
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","format":"uuid","uuidVersion":"5","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_21_uuid_v5_path_param_success: pathParams21UuidV5PathParamSuccess
		},
	};
}


/**
 * Handler for GET /path/param-maxlength/{item_id}
 */
async function pathParamsStringPathParameterWithMaxLengthFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const itemId = _params.item_id;
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"string","maxLength":3,"source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_string_path_parameter_with_max_length_failure: pathParamsStringPathParameterWithMaxLengthFailure
		},
	};
}


/**
 * Handler for GET /path/param-minlength/{item_id}
 */
async function pathParamsStringPathParameterWithMinLengthFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const itemId = _params.item_id;
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"string","minLength":3,"source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_string_path_parameter_with_min_length_failure: pathParamsStringPathParameterWithMinLengthFailure
		},
	};
}


/**
 * Handler for GET /{version}/{service_id}/{user_id}/{order_id}
 */
async function pathParamsMultiplePathParametersSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"version":1.0,"service_id":1,"user_id":"abc","order_id":"c892496f-b1fd-4b91-bdb8-b46f92df1716"};
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
		parameter_schema: {"type":"object","properties":{"version":{"type":"number","source":"path"},"service_id":{"type":"integer","source":"path"},"user_id":{"type":"string","source":"path"},"order_id":{"type":"string","format":"uuid","source":"path"}},"required":["version","service_id","user_id","order_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_multiple_path_parameters_success: pathParamsMultiplePathParametersSuccess
		},
	};
}


/**
 * Handler for GET /date/{date_param}
 */
async function pathParamsDatePathParameterSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"date_param":"2023-07-15"};
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
		parameter_schema: {"type":"object","properties":{"date_param":{"type":"string","format":"date","source":"path"}},"required":["date_param"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_date_path_parameter_success: pathParamsDatePathParameterSuccess
		},
	};
}


/**
 * Handler for GET /path/param-gt/{item_id}
 */
async function pathParamsIntegerPathParameterWithGtConstraintFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const itemId = _params.item_id;
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"integer","exclusiveMinimum":3,"source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_gt_constraint_failure: pathParamsIntegerPathParameterWithGtConstraintFailure
		},
	};
}


/**
 * Handler for GET /events/{date}
 */
async function pathParams24DateFormatPathParamSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"date":"2025-10-30"};
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
		parameter_schema: {"type":"object","properties":{"date":{"type":"string","format":"date","source":"path"}},"required":["date"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_24_date_format_path_param_success: pathParams24DateFormatPathParamSuccess
		},
	};
}


/**
 * Handler for GET /path/float/{item_id}
 */
async function pathParamsFloatPathParameterSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"item_id":42.5};
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"number","source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_float_path_parameter_success: pathParamsFloatPathParameterSuccess
		},
	};
}


/**
 * Handler for GET /type-syntax/users/{user_id:int}
 */
async function pathParamsPathParameterWithTypeSyntaxInteger(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"user_id":"42"};
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
			path_params_path_parameter_with_type_syntax_integer: pathParamsPathParameterWithTypeSyntaxInteger
		},
	};
}


/**
 * Handler for GET /path/str/{item_id}
 */
async function pathParamsStringPathParameterSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"item_id":"foobar"};
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"string","source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_string_path_parameter_success: pathParamsStringPathParameterSuccess
		},
	};
}


/**
 * Handler for GET /items/{item_id}
 */
async function pathParamsUuidPathParameterSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"item_id":"ec38df32-ceda-4cfa-9b4a-1aeb94ad551a"};
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"string","format":"uuid","source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_uuid_path_parameter_success: pathParamsUuidPathParameterSuccess
		},
	};
}


/**
 * Handler for GET /path/param-ge/{item_id}
 */
async function pathParamsIntegerPathParameterWithGeConstraintSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"item_id":3};
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"integer","minimum":3,"source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_ge_constraint_success: pathParamsIntegerPathParameterWithGeConstraintSuccess
		},
	};
}


/**
 * Handler for GET /models/{model_name}
 */
async function pathParamsEnumPathParameterSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"model_name":"alexnet"};
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
		parameter_schema: {"type":"object","properties":{"model_name":{"type":"string","enum":["alexnet","lenet","resnet"],"source":"path"}},"required":["model_name"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_enum_path_parameter_success: pathParamsEnumPathParameterSuccess
		},
	};
}


/**
 * Handler for GET /path/bool/{item_id}
 */
async function pathParamsBooleanPathParameterNumeric1(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"item_id":true};
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
		parameter_schema: {"type":"object","properties":{"item_id":{"type":"boolean","source":"path"}},"required":["item_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_boolean_path_parameter_numeric_1: pathParamsBooleanPathParameterNumeric1
		},
	};
}


/**
 * Handler for POST /items/
 */
async function contentTypes415UnsupportedMediaType(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 415 };
	const responseBody = {"type":"https://spikard.dev/errors/unsupported-media-type","title":"Unsupported Media Type","status":415,"detail":"Unsupported media type"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes415UnsupportedMediaType(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "content_types_415_unsupported_media_type",
		request_schema: {"type":"string"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_415_unsupported_media_type: contentTypes415UnsupportedMediaType
		},
	};
}


/**
 * Handler for GET /xml
 */
async function contentTypesXmlResponseApplicationXml(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"content-type":"application/xml"};
	const responseBody = "<?xml version=\"1.0\"?><item><name>Item</name><price>42.0</price></item>";
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
			content_types_xml_response_application_xml: contentTypesXmlResponseApplicationXml
		},
	};
}


/**
 * Handler for POST /data
 */
async function contentTypes14ContentTypeCaseInsensitive(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = {"name":"test"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes14ContentTypeCaseInsensitive(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_14_content_type_case_insensitive",
		request_schema: {"type":"object","required":["name"],"properties":{"name":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_14_content_type_case_insensitive: contentTypes14ContentTypeCaseInsensitive
		},
	};
}


/**
 * Handler for GET /items/unicode
 */
async function contentTypesJsonWithUtf8Charset(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"content-type":"application/json; charset=utf-8"};
	const responseBody = {"name":"Café","emoji":"☕"};
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
			content_types_json_with_utf_8_charset: contentTypesJsonWithUtf8Charset
		},
	};
}


/**
 * Handler for POST /data
 */
async function contentTypes16TextPlainNotAccepted(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 415 };
	const responseBody = {"type":"https://spikard.dev/errors/unsupported-media-type","title":"Unsupported Media Type","status":415,"detail":"Unsupported media type"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes16TextPlainNotAccepted(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_16_text_plain_not_accepted",
		request_schema: {"type":"object","required":["data"],"properties":{"data":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_16_text_plain_not_accepted: contentTypes16TextPlainNotAccepted
		},
	};
}


/**
 * Handler for GET /download/document.pdf
 */
async function contentTypesPdfResponseApplicationPdf(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"content-type":"application/pdf","content-disposition":"attachment; filename=document.pdf"};
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
			content_types_pdf_response_application_pdf: contentTypesPdfResponseApplicationPdf
		},
	};
}


/**
 * Handler for POST /data
 */
async function contentTypes20ContentLengthMismatch(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 400 };
	const responseBody = {"type":"https://spikard.dev/errors/content-length-mismatch","title":"Content-Length header mismatch","status":400,"detail":"Content-Length header does not match actual body size"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes20ContentLengthMismatch(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_20_content_length_mismatch",
		request_schema: {"type":"object","properties":{"value":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"Content-Length":{"type":"string","source":"header"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_20_content_length_mismatch: contentTypes20ContentLengthMismatch
		},
	};
}


/**
 * Handler for POST /api/v1/resource
 */
async function contentTypes17VendorJsonAccepted(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = {"data":"value"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes17VendorJsonAccepted(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/api/v1/resource",
		handler_name: "content_types_17_vendor_json_accepted",
		request_schema: {"type":"object","required":["data"],"properties":{"data":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_17_vendor_json_accepted: contentTypes17VendorJsonAccepted
		},
	};
}


/**
 * Handler for POST /data
 */
async function contentTypes13JsonWithCharsetUtf16(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 415 };
	const responseBody = {"type":"https://spikard.dev/errors/unsupported-charset","title":"Unsupported Charset","status":415,"detail":"Unsupported charset 'utf-16' for JSON. Only UTF-8 is supported."};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes13JsonWithCharsetUtf16(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_13_json_with_charset_utf16",
		request_schema: {"type":"object","properties":{"value":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_13_json_with_charset_utf16: contentTypes13JsonWithCharsetUtf16
		},
	};
}


/**
 * Handler for GET /items/json
 */
async function contentTypesJsonResponseApplicationJson(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"content-type":"application/json"};
	const responseBody = {"name":"Item","price":42.0};
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
			content_types_json_response_application_json: contentTypesJsonResponseApplicationJson
		},
	};
}


/**
 * Handler for POST /upload
 */
async function contentTypes15MultipartBoundaryRequired(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 400 };
	const responseBody = {"error":"multipart/form-data requires 'boundary' parameter"};
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
		parameter_schema: {"type":"object","properties":{}},
		file_params: {"document":{"required":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_15_multipart_boundary_required: contentTypes15MultipartBoundaryRequired
		},
	};
}


/**
 * Handler for GET /accept-test/{id}
 */
async function contentTypesContentNegotiationAcceptHeader(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"content-type":"application/json"};
	const responseBody = {"id":1,"name":"Item"};
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
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_content_negotiation_accept_header: contentTypesContentNegotiationAcceptHeader
		},
	};
}


/**
 * Handler for GET /html
 */
async function contentTypesHtmlResponseTextHtml(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"content-type":"text/html; charset=utf-8"};
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
			content_types_html_response_text_html: contentTypesHtmlResponseTextHtml
		},
	};
}


/**
 * Handler for GET /images/photo.jpg
 */
async function contentTypesJpegImageResponseImageJpeg(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"content-type":"image/jpeg"};
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
			content_types_jpeg_image_response_image_jpeg: contentTypesJpegImageResponseImageJpeg
		},
	};
}


/**
 * Handler for POST /data
 */
async function contentTypes19MissingContentTypeDefaultJson(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = {"name":"test"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes19MissingContentTypeDefaultJson(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_19_missing_content_type_default_json",
		request_schema: {"type":"object","required":["name"],"properties":{"name":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_19_missing_content_type_default_json: contentTypes19MissingContentTypeDefaultJson
		},
	};
}


/**
 * Handler for GET /images/logo.png
 */
async function contentTypesPngImageResponseImagePng(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"content-type":"image/png"};
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
			content_types_png_image_response_image_png: contentTypesPngImageResponseImagePng
		},
	};
}


/**
 * Handler for GET /text
 */
async function contentTypesPlainTextResponseTextPlain(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"content-type":"text/plain; charset=utf-8"};
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
			content_types_plain_text_response_text_plain: contentTypesPlainTextResponseTextPlain
		},
	};
}


/**
 * Handler for POST /data
 */
async function contentTypes18ContentTypeWithMultipleParams(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 201 };
	const responseBody = {"value":"test"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppContentTypes18ContentTypeWithMultipleParams(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_18_content_type_with_multiple_params",
		request_schema: {"type":"object","properties":{"value":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_18_content_type_with_multiple_params: contentTypes18ContentTypeWithMultipleParams
		},
	};
}


/**
 * Handler for GET /export/data.csv
 */
async function contentTypesCsvResponseTextCsv(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"content-type":"text/csv; charset=utf-8","content-disposition":"attachment; filename=data.csv"};
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
			content_types_csv_response_text_csv: contentTypesCsvResponseTextCsv
		},
	};
}


/**
 * Handler for GET /download/file.bin
 */
async function contentTypesBinaryResponseApplicationOctetStream(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"content-type":"application/octet-stream","content-disposition":"attachment; filename=file.bin"};
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
			content_types_binary_response_application_octet_stream: contentTypesBinaryResponseApplicationOctetStream
		},
	};
}


/**
 * Handler for OPTIONS /api/data
 */
async function cors07CorsPreflightHeaderNotAllowed(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"Origin":{"type":"string","source":"header"},"Access-Control-Request-Method":{"type":"string","source":"header"},"Access-Control-Request-Headers":{"type":"string","source":"header"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_07_cors_preflight_header_not_allowed: cors07CorsPreflightHeaderNotAllowed
		},
	};
}


/**
 * Handler for GET /api/cached-resource
 */
async function corsCorsVaryHeaderForProperCaching(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"vary":"Origin","cache-control":"public, max-age=3600","access-control-allow-origin":"https://app.example.com"};
	const responseBody = {"data":"cacheable resource"};
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
			cors_cors_vary_header_for_proper_caching: corsCorsVaryHeaderForProperCaching
		},
	};
}


/**
 * Handler for OPTIONS /api/resource/123
 */
async function corsCorsPreflightForPutMethod(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 204 };
	response.headers = {"access-control-allow-origin":"https://app.example.com","access-control-max-age":"3600","vary":"Origin","access-control-allow-methods":"GET, POST, PUT, PATCH, DELETE","access-control-allow-headers":"Content-Type, X-Custom-Header"};
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
			cors_cors_preflight_for_put_method: corsCorsPreflightForPutMethod
		},
	};
}


/**
 * Handler for OPTIONS /api/resource/456
 */
async function corsCorsPreflightForDeleteMethod(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 204 };
	response.headers = {"access-control-allow-methods":"GET, POST, PUT, PATCH, DELETE","access-control-allow-origin":"https://app.example.com","vary":"Origin","access-control-max-age":"3600"};
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
			cors_cors_preflight_for_delete_method: corsCorsPreflightForDeleteMethod
		},
	};
}


/**
 * Handler for GET /api/data
 */
async function corsCorsMultipleAllowedOrigins(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"vary":"Origin","access-control-allow-origin":"https://admin.example.com"};
	const responseBody = {"data":"resource data"};
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
			cors_cors_multiple_allowed_origins: corsCorsMultipleAllowedOrigins
		},
	};
}


/**
 * Handler for OPTIONS /items/
 */
async function corsCorsPreflightRequest(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"access-control-allow-methods":"GET, POST, PUT, DELETE, OPTIONS","access-control-allow-origin":"https://example.com","access-control-allow-headers":"Content-Type, X-Custom-Header","access-control-max-age":"600"};
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
			cors_cors_preflight_request: corsCorsPreflightRequest
		},
	};
}


/**
 * Handler for GET /api/user/profile
 */
async function corsCorsWithCredentials(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"vary":"Origin","access-control-allow-origin":"https://app.example.com","access-control-allow-credentials":"true"};
	const responseBody = {"username":"john"};
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
			cors_cors_with_credentials: corsCorsWithCredentials
		},
	};
}


/**
 * Handler for GET /api/data
 */
async function corsCorsRegexPatternMatchingForOrigins(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"vary":"Origin","access-control-allow-origin":"https://subdomain.example.com"};
	const responseBody = {"data":"resource data"};
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
			cors_cors_regex_pattern_matching_for_origins: corsCorsRegexPatternMatchingForOrigins
		},
	};
}


/**
 * Handler for OPTIONS /api/data
 */
async function cors08CorsMaxAge(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 204 };
	response.headers = {"access-control-allow-methods":"POST","access-control-max-age":"3600","access-control-allow-headers":"Content-Type","access-control-allow-origin":"https://example.com"};
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
		parameter_schema: {"type":"object","properties":{"Origin":{"type":"string","source":"header"},"Access-Control-Request-Method":{"type":"string","source":"header"},"Access-Control-Request-Headers":{"type":"string","source":"header"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_08_cors_max_age: cors08CorsMaxAge
		},
	};
}


/**
 * Handler for GET /api/data
 */
async function cors10CorsOriginNull(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 403 };
	const responseBody = {"error":"Origin 'null' is not allowed"};
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
		parameter_schema: {"type":"object","properties":{"Origin":{"type":"string","source":"header"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_10_cors_origin_null: cors10CorsOriginNull
		},
	};
}


/**
 * Handler for GET /public/data
 */
async function corsCorsWildcardOrigin(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"access-control-allow-origin":"*"};
	const responseBody = {"data":"public"};
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
			cors_cors_wildcard_origin: corsCorsWildcardOrigin
		},
	};
}


/**
 * Handler for POST /api/form
 */
async function corsCorsSafelistedHeadersWithoutPreflight(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"access-control-allow-origin":"https://app.example.com","vary":"Origin"};
	const responseBody = {"message":"Success"};
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
			cors_cors_safelisted_headers_without_preflight: corsCorsSafelistedHeadersWithoutPreflight
		},
	};
}


/**
 * Handler for OPTIONS /api/local-resource
 */
async function corsCorsPrivateNetworkAccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 204 };
	response.headers = {"access-control-allow-origin":"https://public.example.com","access-control-allow-methods":"GET, POST","access-control-allow-private-network":"true","vary":"Origin"};
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
			cors_cors_private_network_access: corsCorsPrivateNetworkAccess
		},
	};
}


/**
 * Handler for GET /api/data
 */
async function corsCorsOriginCaseSensitivity(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"vary":"Origin"};
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
			cors_cors_origin_case_sensitivity: corsCorsOriginCaseSensitivity
		},
	};
}


/**
 * Handler for GET /items/
 */
async function corsCorsRequestBlocked(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 403 };
	const responseBody = {"detail":"CORS request from origin 'https://malicious-site.com' not allowed"};
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
		parameter_schema: {"type":"object","properties":{"Origin":{"type":"string","source":"header"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_cors_request_blocked: corsCorsRequestBlocked
		},
	};
}


/**
 * Handler for GET /items/
 */
async function corsSimpleCorsRequest(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"vary":"Origin","access-control-allow-origin":"https://example.com"};
	const responseBody = {"items":[]};
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
			cors_simple_cors_request: corsSimpleCorsRequest
		},
	};
}


/**
 * Handler for GET /api/data
 */
async function cors09CorsExposeHeaders(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"access-control-allow-origin":"https://example.com","access-control-expose-headers":"X-Total-Count, X-Request-Id","x-request-id":"abc123","x-total-count":"42"};
	const result: Record<string, unknown> = {};
	const origin = _params.Origin;
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
		parameter_schema: {"type":"object","properties":{"Origin":{"type":"string","source":"header"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_09_cors_expose_headers: cors09CorsExposeHeaders
		},
	};
}


/**
 * Handler for OPTIONS /api/data
 */
async function cors06CorsPreflightMethodNotAllowed(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		parameter_schema: {"type":"object","properties":{"Origin":{"type":"string","source":"header"},"Access-Control-Request-Method":{"type":"string","source":"header"},"Access-Control-Request-Headers":{"type":"string","source":"header"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_06_cors_preflight_method_not_allowed: cors06CorsPreflightMethodNotAllowed
		},
	};
}


/**
 * Handler for GET /data
 */
async function cookies25CookieSamesiteLax(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const result: Record<string, unknown> = {};
	const tracking = _params.tracking;
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
		parameter_schema: {"type":"object","properties":{"tracking":{"type":"string","source":"cookie","samesite":"Lax"}},"required":["tracking"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_25_cookie_samesite_lax: cookies25CookieSamesiteLax
		},
	};
}


/**
 * Handler for GET /items/
 */
async function cookiesOptionalCookieParameterSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"ads_id":"abc123"};
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
		parameter_schema: {"type":"object","properties":{"ads_id":{"type":"string","source":"cookie"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_optional_cookie_parameter_success: cookiesOptionalCookieParameterSuccess
		},
	};
}


/**
 * Handler for GET /cookies/pattern
 */
async function cookiesCookieRegexPatternValidationFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const trackingId = _params.tracking_id;
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
		parameter_schema: {"type":"object","properties":{"tracking_id":{"type":"string","pattern":"^[A-Z0-9]{8}$","source":"cookie"}},"required":["tracking_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_regex_pattern_validation_fail: cookiesCookieRegexPatternValidationFail
		},
	};
}


/**
 * Handler for POST /cookies/session
 */
async function cookiesResponseSessionCookieNoMaxAge(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Session cookie set"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseSessionCookieNoMaxAge(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/session",
		handler_name: "cookies_response_session_cookie_no_max_age",
		request_schema: {"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_session_cookie_no_max_age: cookiesResponseSessionCookieNoMaxAge
		},
	};
}


/**
 * Handler for GET /secure
 */
async function cookies27CookieHttponlyFlag(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const result: Record<string, unknown> = {};
	const session = _params.session;
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
		parameter_schema: {"type":"object","properties":{"session":{"type":"string","source":"cookie","httponly":true}},"required":["session"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_27_cookie_httponly_flag: cookies27CookieHttponlyFlag
		},
	};
}


/**
 * Handler for GET /cookie/set
 */
async function cookiesResponseCookieWithAttributes(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Cookie set"};
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
			cookies_response_cookie_with_attributes: cookiesResponseCookieWithAttributes
		},
	};
}


/**
 * Handler for GET /secure
 */
async function cookies24CookieSamesiteStrict(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const result: Record<string, unknown> = {};
	const sessionId = _params.session_id;
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
		parameter_schema: {"type":"object","properties":{"session_id":{"type":"string","source":"cookie","samesite":"Strict"}},"required":["session_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_24_cookie_samesite_strict: cookies24CookieSamesiteStrict
		},
	};
}


/**
 * Handler for GET /users/me
 */
async function cookiesApikeyCookieAuthenticationSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"username":"secret"};
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
		parameter_schema: {"type":"object","properties":{"key":{"type":"string","source":"cookie"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_apikey_cookie_authentication_success: cookiesApikeyCookieAuthenticationSuccess
		},
	};
}


/**
 * Handler for GET /cookies/min-length
 */
async function cookiesCookieValidationMinLengthConstraintSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"token":"abc"};
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
		parameter_schema: {"type":"object","properties":{"token":{"type":"string","source":"cookie"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_validation_min_length_constraint_success: cookiesCookieValidationMinLengthConstraintSuccess
		},
	};
}


/**
 * Handler for GET /items/
 */
async function cookiesCookieValidationMinLengthFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const trackingId = _params.tracking_id;
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
		parameter_schema: {"type":"object","properties":{"tracking_id":{"type":"string","minLength":3,"source":"cookie"}},"required":["tracking_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_validation_min_length_failure: cookiesCookieValidationMinLengthFailure
		},
	};
}


/**
 * Handler for GET /cookies/validated
 */
async function cookiesCookieValidationMaxLengthConstraintFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const sessionId = _params.session_id;
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
		parameter_schema: {"type":"object","properties":{"session_id":{"type":"string","maxLength":20,"source":"cookie"}},"required":["session_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_validation_max_length_constraint_fail: cookiesCookieValidationMaxLengthConstraintFail
		},
	};
}


/**
 * Handler for GET /items/cookies
 */
async function cookiesRequiredCookieMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const sessionId = _params.session_id;
	const fatebookTracker = _params.fatebook_tracker;
	if (sessionId !== null && sessionId !== undefined) {
		result.session_id = sessionId;
	}
	if (fatebookTracker !== null && fatebookTracker !== undefined) {
		result.fatebook_tracker = fatebookTracker;
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
		parameter_schema: {"type":"object","properties":{"session_id":{"type":"string","source":"cookie"},"fatebook_tracker":{"type":"string","source":"cookie"}},"required":["session_id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_required_cookie_missing: cookiesRequiredCookieMissing
		},
	};
}


/**
 * Handler for GET /items/
 */
async function cookiesOptionalCookieParameterMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"ads_id":null};
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
		parameter_schema: {"type":"object","properties":{"ads_id":{"type":"string","source":"cookie"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_optional_cookie_parameter_missing: cookiesOptionalCookieParameterMissing
		},
	};
}


/**
 * Handler for GET /users/me/auth
 */
async function cookiesApikeyCookieAuthenticationMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	const key = _params.key;
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
		parameter_schema: {"type":"object","properties":{"key":{"type":"string","source":"cookie"}},"required":["key"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_apikey_cookie_authentication_missing: cookiesApikeyCookieAuthenticationMissing
		},
	};
}


/**
 * Handler for POST /cookies/multiple
 */
async function cookiesResponseMultipleCookies(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Multiple cookies set"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseMultipleCookies(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/multiple",
		handler_name: "cookies_response_multiple_cookies",
		request_schema: {"type":"object","properties":{"user":{"type":"string"},"session":{"type":"string"}},"required":["user","session"],"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_multiple_cookies: cookiesResponseMultipleCookies
		},
	};
}


/**
 * Handler for POST /cookies/samesite-lax
 */
async function cookiesResponseCookieWithSamesiteLax(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Cookie set with SameSite=Lax"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseCookieWithSamesiteLax(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/samesite-lax",
		handler_name: "cookies_response_cookie_with_samesite_lax",
		request_schema: {"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_samesite_lax: cookiesResponseCookieWithSamesiteLax
		},
	};
}


/**
 * Handler for POST /cookies/delete
 */
async function cookiesResponseDeleteCookie(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Cookie deleted"};
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
		parameter_schema: {"type":"object","properties":{"session":{"type":"string","source":"cookie"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_delete_cookie: cookiesResponseDeleteCookie
		},
	};
}


/**
 * Handler for POST /cookies/set-with-path
 */
async function cookiesResponseCookieWithPathAttribute(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Cookie set with path"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseCookieWithPathAttribute(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/set-with-path",
		handler_name: "cookies_response_cookie_with_path_attribute",
		request_schema: {"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_path_attribute: cookiesResponseCookieWithPathAttribute
		},
	};
}


/**
 * Handler for GET /users/me
 */
async function cookiesOptionalApikeyCookieMissing(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"msg":"Create an account first"};
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
		parameter_schema: {"type":"object","properties":{"key":{"type":"string","source":"cookie"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_optional_apikey_cookie_missing: cookiesOptionalApikeyCookieMissing
		},
	};
}


/**
 * Handler for POST /cookies/samesite-strict
 */
async function cookiesResponseCookieWithSamesiteStrict(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Cookie set with SameSite=Strict"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseCookieWithSamesiteStrict(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/samesite-strict",
		handler_name: "cookies_response_cookie_with_samesite_strict",
		request_schema: {"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_samesite_strict: cookiesResponseCookieWithSamesiteStrict
		},
	};
}


/**
 * Handler for POST /cookies/samesite-none
 */
async function cookiesResponseCookieWithSamesiteNone(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Cookie set with SameSite=None"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseCookieWithSamesiteNone(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/samesite-none",
		handler_name: "cookies_response_cookie_with_samesite_none",
		request_schema: {"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_samesite_none: cookiesResponseCookieWithSamesiteNone
		},
	};
}


/**
 * Handler for GET /cookies/pattern
 */
async function cookiesCookieRegexPatternValidationSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"tracking_id":"ABC12345"};
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
		parameter_schema: {"type":"object","properties":{"tracking_id":{"type":"string","source":"cookie"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_regex_pattern_validation_success: cookiesCookieRegexPatternValidationSuccess
		},
	};
}


/**
 * Handler for POST /cookie/
 */
async function cookiesResponseSetCookieBasic(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Come to the dark side, we have cookies"};
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
			cookies_response_set_cookie_basic: cookiesResponseSetCookieBasic
		},
	};
}


/**
 * Handler for GET /items/
 */
async function cookiesMultipleCookiesSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"session_id":"session123","fatebook_tracker":"tracker456","googall_tracker":"ga789"};
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
		parameter_schema: {"type":"object","properties":{"session_id":{"type":"string","source":"cookie"},"fatebook_tracker":{"type":"string","source":"cookie"},"googall_tracker":{"type":"string","source":"cookie"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_multiple_cookies_success: cookiesMultipleCookiesSuccess
		},
	};
}


/**
 * Handler for GET /secure
 */
async function cookies26CookieSecureFlag(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const result: Record<string, unknown> = {};
	const authToken = _params.auth_token;
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
		parameter_schema: {"type":"object","properties":{"auth_token":{"type":"string","source":"cookie","secure":true}},"required":["auth_token"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_26_cookie_secure_flag: cookies26CookieSecureFlag
		},
	};
}


/**
 * Handler for POST /cookies/set-with-domain
 */
async function cookiesResponseCookieWithDomainAttribute(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"message":"Cookie set with domain"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppCookiesResponseCookieWithDomainAttribute(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/set-with-domain",
		handler_name: "cookies_response_cookie_with_domain_attribute",
		request_schema: {"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_domain_attribute: cookiesResponseCookieWithDomainAttribute
		},
	};
}


/**
 * Handler for GET /api/override-test
 */
async function diRouteLevelDependencyOverrideSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _api_key_validator = request.dependencies?.api_key_validator ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"mode":"test","strict":false};
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
		handler_dependencies: ["api_key_validator"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
			di_route_level_dependency_override_success: diRouteLevelDependencyOverrideSuccess
		};
	return app;
}


function _diCircularDependencyDetectionErrorServiceA(serviceB): unknown {
	// Factory for service_a
	void serviceB;
	return { _factory: "service_a", _random: Math.random() };
}

function _diCircularDependencyDetectionErrorServiceB(serviceA): unknown {
	// Factory for service_b
	void serviceA;
	return { _factory: "service_b", _random: Math.random() };
}

/**
 * Handler for GET /api/circular
 */
async function diCircularDependencyDetectionError(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _service_a = request.dependencies?.service_a ?? null;
	const response: HandlerResponse = { status: 500 };
	const responseBody = {"type":"https://spikard.dev/errors/dependency-error","title":"Dependency Resolution Failed","status":500,"detail":"Circular dependency detected","errors":[{"type":"circular_dependency","msg":"Circular dependency detected in dependency graph","cycle":["service_a","service_b","service_a"]}]};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiCircularDependencyDetectionError(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/circular",
		handler_name: "di_circular_dependency_detection_error",
		handler_dependencies: ["service_a"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			di_circular_dependency_detection_error: diCircularDependencyDetectionError
		},
	};
}


function _diFactoryDependencySuccessTimestampGenerator(): unknown {
	// Factory for timestamp_generator
	return { _factory: "timestamp_generator", _random: Math.random() };
}

/**
 * Handler for GET /api/timestamp
 */
async function diFactoryDependencySuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _timestamp_generator = request.dependencies?.timestamp_generator ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"timestamp":"<<present>>"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiFactoryDependencySuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("timestampGenerator", _diFactoryDependencySuccessTimestampGenerator);

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/timestamp",
		handler_name: "di_factory_dependency_success",
		handler_dependencies: ["timestamp_generator"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
			di_factory_dependency_success: diFactoryDependencySuccess
		};
	return app;
}


/**
 * Handler for GET /api/config
 */
async function diValueDependencyInjectionSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _app_name = request.dependencies?.app_name ?? null;
	const _version = request.dependencies?.version ?? null;
	const _max_connections = request.dependencies?.max_connections ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"app_name":"SpikardApp","version":"1.0.0","max_connections":100};
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
		handler_dependencies: ["app_name", "version", "max_connections"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
			di_value_dependency_injection_success: diValueDependencyInjectionSuccess
		};
	return app;
}


/**
 * Handler for GET /api/node-destructure
 */
async function diNodeJsObjectDestructuringInjectionSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _db = request.dependencies?.db ?? null;
	const _logger = request.dependencies?.logger ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"db_name":"PostgreSQL","log_level":"info"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiNodeJsObjectDestructuringInjectionSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("db", { name: "PostgreSQL", connected: true });
	app.provide("logger", { level: "info", enabled: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/node-destructure",
		handler_name: "di_node_js_object_destructuring_injection_success",
		handler_dependencies: ["db", "logger"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
			di_node_js_object_destructuring_injection_success: diNodeJsObjectDestructuringInjectionSuccess
		};
	return app;
}


function _diNestedDependencies3LevelsSuccessAuthService(dbPool, cache): unknown {
	// Factory for auth_service
	void dbPool;
	void cache;
	return { _factory: "auth_service", _random: Math.random() };
}

async function _diNestedDependencies3LevelsSuccessCache(config): Promise<unknown> {
	// Async factory for cache
	void config;
	// Simulate async cache connection
	return { ready: true, cacheId: Math.random().toString() };
}

async function _diNestedDependencies3LevelsSuccessDbPool(config): Promise<unknown> {
	// Async factory for db_pool
	void config;
	// Simulate async DB connection
	return { connected: true, poolId: Math.random().toString() };
}

/**
 * Handler for GET /api/auth-status
 */
async function diNestedDependencies3LevelsSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _auth_service = request.dependencies?.auth_service ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"auth_enabled":true,"has_db":true,"has_cache":true};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiNestedDependencies3LevelsSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("config", { db_url: "postgresql://localhost/mydb", cache_ttl: 300 });
	app.provide("cache", _diNestedDependencies3LevelsSuccessCache, { cacheable: true });
	app.provide("dbPool", _diNestedDependencies3LevelsSuccessDbPool, { cacheable: true });
	app.provide("authService", _diNestedDependencies3LevelsSuccessAuthService, { cacheable: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/auth-status",
		handler_name: "di_nested_dependencies_3_levels_success",
		handler_dependencies: ["auth_service"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
			di_nested_dependencies_3_levels_success: diNestedDependencies3LevelsSuccess
		};
	return app;
}


/**
 * Handler for GET /api/type-mismatch
 */
async function diTypeMismatchInDependencyResolutionError(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _config = request.dependencies?.config ?? null;
	const response: HandlerResponse = { status: 500 };
	const responseBody = {"type":"https://spikard.dev/errors/dependency-error","title":"Dependency Resolution Failed","status":500,"detail":"Dependency type mismatch","errors":[{"type":"type_mismatch","msg":"Dependency 'config' type mismatch: expected object, got string","dependency_key":"config","expected_type":"object","actual_type":"string"}]};
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
		handler_dependencies: ["config"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
			di_type_mismatch_in_dependency_resolution_error: diTypeMismatchInDependencyResolutionError
		};
	return app;
}


/**
 * Handler for GET /api/missing-dep
 */
async function diMissingDependencyError(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _non_existent_service = request.dependencies?.non_existent_service ?? null;
	const response: HandlerResponse = { status: 500 };
	const responseBody = {"type":"https://spikard.dev/errors/dependency-error","title":"Dependency Resolution Failed","status":500,"detail":"Required dependency not found","errors":[{"type":"missing_dependency","msg":"Dependency 'non_existent_service' is not registered","dependency_key":"non_existent_service"}]};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiMissingDependencyError(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/missing-dep",
		handler_name: "di_missing_dependency_error",
		handler_dependencies: ["non_existent_service"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			di_missing_dependency_error: diMissingDependencyError
		},
	};
}


/**
 * Handler for GET /api/python-name-inject
 */
async function diPythonParameterNameBasedInjectionSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _db_pool = request.dependencies?.db_pool ?? null;
	const _cache = request.dependencies?.cache ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"db_status":"connected","cache_status":"ready"};
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
		handler_dependencies: ["db_pool", "cache"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
			di_python_parameter_name_based_injection_success: diPythonParameterNameBasedInjectionSuccess
		};
	return app;
}


async function diDependencyInjectionInLifecycleHooksSuccessLogRequestOnRequest0(request: HookRequest): Promise<HookResult> {
	// Mock onRequest hook: log_request
	return request;
}

async function diDependencyInjectionInLifecycleHooksSuccessAuthCheckPreHandler0(request: HookRequest): Promise<HookResult> {
	// Mock preHandler hook: auth_check
	return request;
}



/**
 * Handler for GET /api/hook-di-test
 */
async function diDependencyInjectionInLifecycleHooksSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	response.headers = {"x-auth-mode":"strict","x-log-level":"debug"};
	const responseBody = {"authenticated":true,"logged":true};
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
			di_dependency_injection_in_lifecycle_hooks_success: diDependencyInjectionInLifecycleHooksSuccess
		};
	app.lifecycleHooks = {
		onRequest: [diDependencyInjectionInLifecycleHooksSuccessLogRequestOnRequest0],
		preHandler: [diDependencyInjectionInLifecycleHooksSuccessAuthCheckPreHandler0]
	};
	return app;
}


/**
 * Handler for GET /api/ruby-kwargs
 */
async function diRubyKeywordArgumentInjectionSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _db_pool = request.dependencies?.db_pool ?? null;
	const _session = request.dependencies?.session ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"adapter":"postgresql","user_id":42};
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
		handler_dependencies: ["db_pool", "session"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
			di_ruby_keyword_argument_injection_success: diRubyKeywordArgumentInjectionSuccess
		};
	return app;
}


async function* _diMultipleDependenciesWithCleanupSuccessCacheConnection(): AsyncGenerator<unknown, void, unknown> {
	// Factory for cache_connection with cleanup
	// Initialize cleanup state
	CLEANUP_STATE["di_multiple_dependencies_with_cleanup_success"] = CLEANUP_STATE["di_multiple_dependencies_with_cleanup_success"] || [];
	CLEANUP_STATE["di_multiple_dependencies_with_cleanup_success"].push("cache_opened");
	// Create resource
	const resource = { id: "00000000-0000-0000-0000-00000000002d", opened: true };
	try {
		yield resource;
	} finally {
		// Cleanup resource
		CLEANUP_STATE["di_multiple_dependencies_with_cleanup_success"].push("cache_closed");
	}
}

async function* _diMultipleDependenciesWithCleanupSuccessSession(dbConnection, cacheConnection): AsyncGenerator<unknown, void, unknown> {
	// Factory for session with cleanup
	void dbConnection;
	void cacheConnection;
	// Initialize cleanup state
	CLEANUP_STATE["di_multiple_dependencies_with_cleanup_success"] = CLEANUP_STATE["di_multiple_dependencies_with_cleanup_success"] || [];
	CLEANUP_STATE["di_multiple_dependencies_with_cleanup_success"].push("session_opened");
	// Create resource
	const resource = { id: "00000000-0000-0000-0000-00000000002d", opened: true };
	try {
		yield resource;
	} finally {
		// Cleanup resource
		CLEANUP_STATE["di_multiple_dependencies_with_cleanup_success"].push("session_closed");
	}
}

async function* _diMultipleDependenciesWithCleanupSuccessDbConnection(): AsyncGenerator<unknown, void, unknown> {
	// Factory for db_connection with cleanup
	// Initialize cleanup state
	CLEANUP_STATE["di_multiple_dependencies_with_cleanup_success"] = CLEANUP_STATE["di_multiple_dependencies_with_cleanup_success"] || [];
	CLEANUP_STATE["di_multiple_dependencies_with_cleanup_success"].push("db_opened");
	// Create resource
	const resource = { id: "00000000-0000-0000-0000-00000000002d", opened: true };
	try {
		yield resource;
	} finally {
		// Cleanup resource
		CLEANUP_STATE["di_multiple_dependencies_with_cleanup_success"].push("db_closed");
	}
}

/**
 * Handler for GET /api/multi-cleanup-test
 */
async function diMultipleDependenciesWithCleanupSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _session = request.dependencies?.session ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"session_active":true};
	response.body = responseBody;
	return JSON.stringify(response);
}

async function diMultipleDependenciesWithCleanupSuccessBackgroundState(): Promise<string> {
	const state = CLEANUP_STATE["di_multiple_dependencies_with_cleanup_success"] ?? [];
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/json" };
	response.body = { "cleanup_order": state };
	return JSON.stringify(response);
}

async function diMultipleDependenciesWithCleanupSuccessCleanupState(): Promise<string> {
	// Return cleanup events
	const cleanupEvents = CLEANUP_STATE["di_multiple_dependencies_with_cleanup_success"] || [];
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/json" };
	response.body = { cleanup_events: cleanupEvents };
	return JSON.stringify(response);
}

export function createAppDiMultipleDependenciesWithCleanupSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("cacheConnection", _diMultipleDependenciesWithCleanupSuccessCacheConnection, { cacheable: true });
	app.provide("dbConnection", _diMultipleDependenciesWithCleanupSuccessDbConnection, { cacheable: true });
	app.provide("session", _diMultipleDependenciesWithCleanupSuccessSession, { cacheable: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/multi-cleanup-test",
		handler_name: "di_multiple_dependencies_with_cleanup_success",
		handler_dependencies: ["session"],
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
			di_multiple_dependencies_with_cleanup_success_background_state: diMultipleDependenciesWithCleanupSuccessBackgroundState,
			di_multiple_dependencies_with_cleanup_success_cleanup_state: diMultipleDependenciesWithCleanupSuccessCleanupState
		};
	return app;
}


function _diMixedSingletonAndPerRequestCachingSuccessRequestContext(dbPool): unknown {
	// Factory for request_context
	void dbPool;
	return { _factory: "request_context", _random: Math.random() };
}

function _diMixedSingletonAndPerRequestCachingSuccessDbPool(appConfig): unknown {
	// Factory for db_pool
	void appConfig;
	return { _factory: "db_pool", _random: Math.random() };
}

/**
 * Handler for GET /api/mixed-caching
 */
async function diMixedSingletonAndPerRequestCachingSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _app_config = request.dependencies?.app_config ?? null;
	const _db_pool = request.dependencies?.db_pool ?? null;
	const _request_context = request.dependencies?.request_context ?? null;
	const response: HandlerResponse = { status: 200 };
	const poolKey = "di_mixed_singleton_and_per_request_caching_success_pool";
	const ctxKey = "di_mixed_singleton_and_per_request_caching_success_ctx_counter";
	const pool = (BACKGROUND_STATE[poolKey] as { pool_id: string } | undefined) ?? { pool_id: "00000000-0000-0000-0000-000000000063" };
	BACKGROUND_STATE[poolKey] = pool;
	const ctxCount = (BACKGROUND_STATE[ctxKey] as number | undefined) ?? 0;
	BACKGROUND_STATE[ctxKey] = ctxCount + 1;
	const context_id = `context-${ctxCount + 1}`;
	response.body = { app_name: "MyApp", pool_id: pool.pool_id, context_id };
	return JSON.stringify(response);
}

export function createAppDiMixedSingletonAndPerRequestCachingSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("appConfig", { app_name: "MyApp", version: "2.0" });
	app.provide("dbPool", _diMixedSingletonAndPerRequestCachingSuccessDbPool, { singleton: true });
	app.provide("requestContext", _diMixedSingletonAndPerRequestCachingSuccessRequestContext, { cacheable: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/mixed-caching",
		handler_name: "di_mixed_singleton_and_per_request_caching_success",
		handler_dependencies: ["app_config", "db_pool", "request_context"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
			di_mixed_singleton_and_per_request_caching_success: diMixedSingletonAndPerRequestCachingSuccess
		};
	return app;
}


async function* _diResourceCleanupAfterRequestSuccessDbSession(): AsyncGenerator<unknown, void, unknown> {
	// Factory for db_session with cleanup
	// Initialize cleanup state
	CLEANUP_STATE["di_resource_cleanup_after_request_success"] = CLEANUP_STATE["di_resource_cleanup_after_request_success"] || [];
	CLEANUP_STATE["di_resource_cleanup_after_request_success"].push("session_opened");
	// Create resource
	const resource = { id: "00000000-0000-0000-0000-000000000029", opened: true };
	try {
		yield resource;
	} finally {
		// Cleanup resource
		CLEANUP_STATE["di_resource_cleanup_after_request_success"].push("session_closed");
	}
}

/**
 * Handler for GET /api/cleanup-test
 */
async function diResourceCleanupAfterRequestSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _db_session = request.dependencies?.db_session ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"session_id":"<<uuid>>","status":"completed"};
	response.body = responseBody;
	return JSON.stringify(response);
}

async function diResourceCleanupAfterRequestSuccessCleanupState(): Promise<string> {
	// Return cleanup events
	const cleanupEvents = CLEANUP_STATE["di_resource_cleanup_after_request_success"] || [];
	const response: HandlerResponse = { status: 200 };
	response.headers = { "content-type": "application/json" };
	response.body = { cleanup_events: cleanupEvents };
	return JSON.stringify(response);
}

export function createAppDiResourceCleanupAfterRequestSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("dbSession", _diResourceCleanupAfterRequestSuccessDbSession, { cacheable: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/cleanup-test",
		handler_name: "di_resource_cleanup_after_request_success",
		handler_dependencies: ["db_session"],
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

	app.routes = [route, cleanupRoute];
	app.handlers = {
			di_resource_cleanup_after_request_success: diResourceCleanupAfterRequestSuccess,
			di_resource_cleanup_after_request_success_cleanup_state: diResourceCleanupAfterRequestSuccessCleanupState
		};
	return app;
}


/**
 * Handler for GET /api/python-type-inject
 */
async function diPythonTypeAnnotationBasedInjectionSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _database_pool = request.dependencies?.database_pool ?? null;
	const _cache_client = request.dependencies?.cache_client ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"pool_type":"PostgreSQL","cache_type":"Redis"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiPythonTypeAnnotationBasedInjectionSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("cacheClient", { cache_type: "Redis", ttl: 300 });
	app.provide("databasePool", { pool_type: "PostgreSQL", max_connections: 20 });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/python-type-inject",
		handler_name: "di_python_type_annotation_based_injection_success",
		handler_dependencies: ["database_pool", "cache_client"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
			di_python_type_annotation_based_injection_success: diPythonTypeAnnotationBasedInjectionSuccess
		};
	return app;
}


function _diPerRequestDependencyCachingSuccessRequestIdGenerator(): unknown {
	// Factory for request_id_generator
	return { _factory: "request_id_generator", _random: Math.random() };
}

/**
 * Handler for GET /api/request-id
 */
async function diPerRequestDependencyCachingSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _request_id_generator = request.dependencies?.request_id_generator ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"first_id":"<<uuid>>","second_id":"<<same_as:first_id>>"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiPerRequestDependencyCachingSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("requestIdGenerator", _diPerRequestDependencyCachingSuccessRequestIdGenerator, { cacheable: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/request-id",
		handler_name: "di_per_request_dependency_caching_success",
		handler_dependencies: ["request_id_generator", "request_id_generator"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
			di_per_request_dependency_caching_success: diPerRequestDependencyCachingSuccess
		};
	return app;
}


function _diSingletonDependencyCachingSuccessAppCounter(): unknown {
	// Factory for app_counter
	return { _factory: "app_counter", _random: Math.random() };
}

/**
 * Handler for GET /api/app-counter
 */
async function diSingletonDependencyCachingSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _app_counter = request.dependencies?.app_counter ?? null;
	const response: HandlerResponse = { status: 200 };
	const stateKey = "di_singleton_dependency_caching_success_counter";
	const existing = BACKGROUND_STATE[stateKey] as { counter_id: string; count: number } | undefined;
	const counter = existing ?? { counter_id: "00000000-0000-0000-0000-000000000063", count: 0 };
	counter.count += 1;
	BACKGROUND_STATE[stateKey] = counter;
	response.body = { counter_id: counter.counter_id, count: counter.count };
	return JSON.stringify(response);
}

export function createAppDiSingletonDependencyCachingSuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("appCounter", _diSingletonDependencyCachingSuccessAppCounter, { singleton: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/app-counter",
		handler_name: "di_singleton_dependency_caching_success",
		handler_dependencies: ["app_counter"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
			di_singleton_dependency_caching_success: diSingletonDependencyCachingSuccess
		};
	return app;
}


async function _diAsyncFactoryDependencySuccessDbPool(): Promise<unknown> {
	// Async factory for db_pool
	// Simulate async DB connection
	return { connected: true, poolId: Math.random().toString() };
}

/**
 * Handler for GET /api/db-status
 */
async function diAsyncFactoryDependencySuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const _db_pool = request.dependencies?.db_pool ?? null;
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"pool_status":"connected","max_size":10};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppDiAsyncFactoryDependencySuccess(): SpikardApp {
	const app = new Spikard();

	app.provide("dbPool", _diAsyncFactoryDependencySuccessDbPool, { cacheable: true });

	const route: RouteMetadata = {
		method: "GET",
		path: "/api/db-status",
		handler_name: "di_async_factory_dependency_success",
		handler_dependencies: ["db_pool"],
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	app.routes = [route];
	app.handlers = {
			di_async_factory_dependency_success: diAsyncFactoryDependencySuccess
		};
	return app;
}


/**
 * Handler for POST /items/
 */
async function jsonBodiesUuidFieldInvalidFormat(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"item_id":{"type":"string","format":"uuid"}},"additionalProperties":false,"required":["name","item_id"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_uuid_field_invalid_format: jsonBodiesUuidFieldInvalidFormat
		},
	};
}


/**
 * Handler for POST /api/v1/data
 */
async function jsonBodies44ConstValidationFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["version","data"],"properties":{"version":{"type":"string","const":"1.0"},"data":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_44_const_validation_failure: jsonBodies44ConstValidationFailure
		},
	};
}


/**
 * Handler for POST /items/
 */
async function jsonBodiesBooleanFieldSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Item","price":42.0,"in_stock":true};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesBooleanFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_boolean_field_success",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"in_stock":{"type":"boolean"}},"additionalProperties":false,"required":["name","price","in_stock"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_boolean_field_success: jsonBodiesBooleanFieldSuccess
		},
	};
}


/**
 * Handler for POST /items/validated
 */
async function jsonBodiesNumericLeValidationSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Item","price":100.0};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesNumericLeValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_numeric_le_validation_success",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"}},"additionalProperties":false,"required":["name","price"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_numeric_le_validation_success: jsonBodiesNumericLeValidationSuccess
		},
	};
}


/**
 * Handler for POST /items/nested
 */
async function jsonBodiesDeeplyNestedObjects(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Product","price":100.0,"seller":{"name":"John Doe","address":{"street":"123 Main St","city":"Springfield","country":{"name":"USA","code":"US"}}}};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesDeeplyNestedObjects(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/nested",
		handler_name: "json_bodies_deeply_nested_objects",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"seller":{"type":"object","properties":{"name":{"type":"string"},"address":{"type":"object","properties":{"street":{"type":"string"},"city":{"type":"string"},"country":{"type":"object","properties":{"name":{"type":"string"},"code":{"type":"string"}},"additionalProperties":false,"required":["name","code"]}},"additionalProperties":false,"required":["street","city","country"]}},"additionalProperties":false,"required":["name","address"]}},"additionalProperties":false,"required":["name","price","seller"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_deeply_nested_objects: jsonBodiesDeeplyNestedObjects
		},
	};
}


/**
 * Handler for POST /items/
 */
async function jsonBodiesOptionalFieldsOmitted(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Foo","price":35.4,"description":null,"tax":null};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesOptionalFieldsOmitted(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_optional_fields_omitted",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"}},"additionalProperties":false,"required":["name","price"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_optional_fields_omitted: jsonBodiesOptionalFieldsOmitted
		},
	};
}


/**
 * Handler for POST /items/
 */
async function jsonBodiesUuidFieldSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Item","item_id":"c892496f-b1fd-4b91-bdb8-b46f92df1716"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesUuidFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_uuid_field_success",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"item_id":{"type":"string","format":"uuid"}},"additionalProperties":false,"required":["name","item_id"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_uuid_field_success: jsonBodiesUuidFieldSuccess
		},
	};
}


/**
 * Handler for POST /events/
 */
async function jsonBodiesDateFieldSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Conference","event_date":"2024-03-15"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesDateFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/events/",
		handler_name: "json_bodies_date_field_success",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"event_date":{"type":"string"}},"additionalProperties":false,"required":["name","event_date"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_date_field_success: jsonBodiesDateFieldSuccess
		},
	};
}


/**
 * Handler for POST /config
 */
async function jsonBodies47MaxpropertiesValidationFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","maxProperties":3},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_47_maxproperties_validation_failure: jsonBodies47MaxpropertiesValidationFailure
		},
	};
}


/**
 * Handler for POST /config
 */
async function jsonBodies46MinpropertiesValidationFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","minProperties":2},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_46_minproperties_validation_failure: jsonBodies46MinpropertiesValidationFailure
		},
	};
}


/**
 * Handler for POST /items/validated
 */
async function jsonBodiesStringMinLengthValidationFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string","minLength":3},"price":{"type":"number"}},"additionalProperties":false,"required":["name","price"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_string_min_length_validation_fail: jsonBodiesStringMinLengthValidationFail
		},
	};
}


/**
 * Handler for POST /items/
 */
async function jsonBodiesFieldTypeValidationInvalidType(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"description":{"type":"string"},"price":{"type":"number"},"tax":{"type":"number"}},"additionalProperties":false,"required":["name","description","price","tax"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_field_type_validation_invalid_type: jsonBodiesFieldTypeValidationInvalidType
		},
	};
}


/**
 * Handler for POST /payment
 */
async function jsonBodies36OneofSchemaMultipleMatchFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"oneOf":[{"type":"object","required":["credit_card"],"properties":{"credit_card":{"type":"string","pattern":"^[0-9]{16}$"}}},{"type":"object","required":["paypal_email"],"properties":{"paypal_email":{"type":"string","format":"email"}}}]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_36_oneof_schema_multiple_match_failure: jsonBodies36OneofSchemaMultipleMatchFailure
		},
	};
}


/**
 * Handler for POST /items/nested
 */
async function jsonBodiesNestedObjectSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Foo","price":42.0,"image":{"url":"https://example.com/image.jpg","name":"Product Image"}};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesNestedObjectSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/nested",
		handler_name: "json_bodies_nested_object_success",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"image":{"type":"object","properties":{"url":{"type":"string"},"name":{"type":"string"}},"additionalProperties":false,"required":["url","name"]}},"additionalProperties":false,"required":["name","price","image"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_nested_object_success: jsonBodiesNestedObjectSuccess
		},
	};
}


/**
 * Handler for POST /users
 */
async function jsonBodies41NotSchemaSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		request_schema: {"type":"object","required":["username"],"properties":{"username":{"type":"string","not":{"enum":["admin","root","system"]}}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_41_not_schema_success: jsonBodies41NotSchemaSuccess
		},
	};
}


/**
 * Handler for POST /items/validated
 */
async function jsonBodiesStringMaxLengthValidationFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string","maxLength":50},"price":{"type":"number"}},"additionalProperties":false,"required":["name","price"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_string_max_length_validation_fail: jsonBodiesStringMaxLengthValidationFail
		},
	};
}


/**
 * Handler for POST /data
 */
async function jsonBodies50DeepNesting4Levels(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		request_schema: {"type":"object","required":["user"],"properties":{"user":{"type":"object","required":["profile"],"properties":{"profile":{"type":"object","required":["contact"],"properties":{"contact":{"type":"object","required":["address"],"properties":{"address":{"type":"object","required":["street"],"properties":{"street":{"type":"string"}}}}}}}}}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_50_deep_nesting_4_levels: jsonBodies50DeepNesting4Levels
		},
	};
}


/**
 * Handler for POST /billing
 */
async function jsonBodies48DependenciesValidationSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"credit_card":{"type":"string"},"billing_address":{"type":"string"}},"dependencies":{"credit_card":["billing_address"]}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_48_dependencies_validation_success: jsonBodies48DependenciesValidationSuccess
		},
	};
}


/**
 * Handler for PATCH /items/{id}
 */
async function jsonBodiesPatchPartialUpdate(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Original Item","price":45.0,"description":"Original description"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesPatchPartialUpdate(): SpikardApp {
	const route: RouteMetadata = {
		method: "PATCH",
		path: "/items/{id}",
		handler_name: "json_bodies_patch_partial_update",
		request_schema: {"type":"object","properties":{"price":{"type":"number"}},"required":["price"]},
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"id":{"type":"string","source":"path"}},"required":["id"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_patch_partial_update: jsonBodiesPatchPartialUpdate
		},
	};
}


/**
 * Handler for POST /users
 */
async function jsonBodies30NestedObjectMissingField(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["profile"],"properties":{"profile":{"type":"object","required":["name","email"],"properties":{"name":{"type":"string","minLength":1},"email":{"type":"string","format":"email"}}}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_30_nested_object_missing_field: jsonBodies30NestedObjectMissingField
		},
	};
}


/**
 * Handler for POST /events/
 */
async function jsonBodiesDatetimeFieldSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Meeting","created_at":"2024-03-15T10:30:00Z"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesDatetimeFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/events/",
		handler_name: "json_bodies_datetime_field_success",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"created_at":{"type":"string","format":"date-time"}},"additionalProperties":false,"required":["name","created_at"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_datetime_field_success: jsonBodiesDatetimeFieldSuccess
		},
	};
}


/**
 * Handler for POST /items/validated
 */
async function jsonBodiesStringPatternValidationSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Item","sku":"ABC1234"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesStringPatternValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_string_pattern_validation_success",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"sku":{"type":"string"}},"additionalProperties":false,"required":["name","sku"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_string_pattern_validation_success: jsonBodiesStringPatternValidationSuccess
		},
	};
}


/**
 * Handler for POST /items/
 */
async function jsonBodiesExtraFieldsIgnoredNoAdditionalproperties(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Item","price":42.0};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesExtraFieldsIgnoredNoAdditionalproperties(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_extra_fields_ignored_no_additionalproperties",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"extra_field":{"type":"string"},"another_extra":{"type":"integer"}},"additionalProperties":false,"required":["name","price","extra_field","another_extra"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_extra_fields_ignored_no_additionalproperties: jsonBodiesExtraFieldsIgnoredNoAdditionalproperties
		},
	};
}


/**
 * Handler for POST /contact
 */
async function jsonBodies40AnyofSchemaFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["name"],"properties":{"name":{"type":"string"},"email":{"type":"string","format":"email"},"phone":{"type":"string"}},"anyOf":[{"required":["email"]},{"required":["phone"]}]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_40_anyof_schema_failure: jsonBodies40AnyofSchemaFailure
		},
	};
}


/**
 * Handler for POST /contact
 */
async function jsonBodies39AnyofSchemaMultipleMatchSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		request_schema: {"type":"object","required":["name"],"properties":{"name":{"type":"string"},"email":{"type":"string","format":"email"},"phone":{"type":"string"}},"anyOf":[{"required":["email"]},{"required":["phone"]}]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_39_anyof_schema_multiple_match_success: jsonBodies39AnyofSchemaMultipleMatchSuccess
		},
	};
}


/**
 * Handler for POST /items/
 */
async function jsonBodiesArrayOfPrimitiveValues(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Product","tags":["electronics","gadget","new"],"ratings":[4.5,4.8,5.0,4.2]};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesArrayOfPrimitiveValues(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_array_of_primitive_values",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"tags":{"type":"array","items":{"type":"string"}},"ratings":{"type":"array","items":{"type":"number"}}},"additionalProperties":false,"required":["name","tags","ratings"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_array_of_primitive_values: jsonBodiesArrayOfPrimitiveValues
		},
	};
}


/**
 * Handler for POST /items/validated
 */
async function jsonBodiesNumericGeValidationFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number","minimum":1}},"additionalProperties":false,"required":["name","price"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_numeric_ge_validation_fail: jsonBodiesNumericGeValidationFail
		},
	};
}


/**
 * Handler for POST /payment
 */
async function jsonBodies37OneofSchemaNoMatchFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"oneOf":[{"type":"object","required":["credit_card"],"properties":{"credit_card":{"type":"string","pattern":"^[0-9]{16}$"}}},{"type":"object","required":["paypal_email"],"properties":{"paypal_email":{"type":"string","format":"email"}}}]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_37_oneof_schema_no_match_failure: jsonBodies37OneofSchemaNoMatchFailure
		},
	};
}


/**
 * Handler for POST /items/list-validated
 */
async function jsonBodiesEmptyArrayValidationFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"tags":{"type":"array","items":{},"minItems":1}},"additionalProperties":false,"required":["name","tags"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_empty_array_validation_fail: jsonBodiesEmptyArrayValidationFail
		},
	};
}


/**
 * Handler for POST /contact
 */
async function jsonBodies38AnyofSchemaSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		request_schema: {"type":"object","required":["name"],"properties":{"name":{"type":"string"}},"anyOf":[{"required":["email"]},{"required":["phone"]}]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_38_anyof_schema_success: jsonBodies38AnyofSchemaSuccess
		},
	};
}


/**
 * Handler for POST /items/optional-all
 */
async function jsonBodiesEmptyJsonObject(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":null,"description":null,"price":null,"tax":null};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesEmptyJsonObject(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/optional-all",
		handler_name: "json_bodies_empty_json_object",
		request_schema: {"type":"object","properties":{},"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_empty_json_object: jsonBodiesEmptyJsonObject
		},
	};
}


/**
 * Handler for POST /items/validated
 */
async function jsonBodiesStringPatternValidationFail(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"sku":{"type":"string","pattern":"^[A-Z]{3}[0-9]{4}$"}},"additionalProperties":false,"required":["name","sku"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_string_pattern_validation_fail: jsonBodiesStringPatternValidationFail
		},
	};
}


/**
 * Handler for POST /billing
 */
async function jsonBodies49DependenciesValidationFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"credit_card":{"type":"string"},"billing_address":{"type":"string"}},"dependencies":{"credit_card":["billing_address"]}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_49_dependencies_validation_failure: jsonBodies49DependenciesValidationFailure
		},
	};
}


/**
 * Handler for POST /items/
 */
async function jsonBodiesSimpleJsonObjectSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Foo","description":"A very nice Item","price":35.4,"tax":3.2};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesSimpleJsonObjectSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_simple_json_object_success",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"description":{"type":"string"},"price":{"type":"number"},"tax":{"type":"number"}},"additionalProperties":false,"required":["name","description","price","tax"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_simple_json_object_success: jsonBodiesSimpleJsonObjectSuccess
		},
	};
}


/**
 * Handler for POST /items/
 */
async function jsonBodiesRequiredFieldMissingValidationError(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"description":{"type":"string"},"price":{"type":"number"},"name":{"type":"string"}},"additionalProperties":false,"required":["description","price","name"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_required_field_missing_validation_error: jsonBodiesRequiredFieldMissingValidationError
		},
	};
}


/**
 * Handler for POST /payment
 */
async function jsonBodies35OneofSchemaSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		request_schema: {"oneOf":[{"type":"object","required":["credit_card"],"properties":{"credit_card":{"type":"string","pattern":"^[0-9]{16}$"}}},{"type":"object","required":["paypal_email"],"properties":{"paypal_email":{"type":"string","format":"email"}}}]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_35_oneof_schema_success: jsonBodies35OneofSchemaSuccess
		},
	};
}


/**
 * Handler for POST /items/
 */
async function jsonBodiesEnumFieldInvalidValue(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"category":{"type":"string","enum":["electronics","clothing","books"]}},"additionalProperties":false,"required":["name","category"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_enum_field_invalid_value: jsonBodiesEnumFieldInvalidValue
		},
	};
}


/**
 * Handler for POST /items/
 */
async function jsonBodiesEnumFieldSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Item","category":"electronics"};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesEnumFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_enum_field_success",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"category":{"type":"string"}},"additionalProperties":false,"required":["name","category"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_enum_field_success: jsonBodiesEnumFieldSuccess
		},
	};
}


/**
 * Handler for POST /items
 */
async function jsonBodies33AllofSchemaComposition(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		request_schema: {"allOf":[{"type":"object","required":["name"],"properties":{"name":{"type":"string"}}},{"type":"object","required":["price"],"properties":{"price":{"type":"number","minimum":0}}}]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_33_allof_schema_composition: jsonBodies33AllofSchemaComposition
		},
	};
}


/**
 * Handler for POST /config
 */
async function jsonBodies45MinpropertiesValidationSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		request_schema: {"type":"object","minProperties":2},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_45_minproperties_validation_success: jsonBodies45MinpropertiesValidationSuccess
		},
	};
}


/**
 * Handler for POST /items/
 */
async function jsonBodiesBodyWithQueryParameters(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"item":{"name":"Item","price":42.0},"limit":10};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesBodyWithQueryParameters(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_body_with_query_parameters",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"}},"additionalProperties":false,"required":["name","price"]},
		response_schema: undefined,
		parameter_schema: {"type":"object","properties":{"limit":{"type":"integer","source":"query"}},"required":["limit"]},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_body_with_query_parameters: jsonBodiesBodyWithQueryParameters
		},
	};
}


/**
 * Handler for POST /users
 */
async function jsonBodies42NotSchemaFailure(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["username"],"properties":{"username":{"type":"string","not":{"enum":["admin","root","system"]}}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_42_not_schema_failure: jsonBodies42NotSchemaFailure
		},
	};
}


/**
 * Handler for POST /api/v1/data
 */
async function jsonBodies43ConstValidationSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		request_schema: {"type":"object","required":["version","data"],"properties":{"version":{"type":"string","const":"1.0"},"data":{"type":"string"}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_43_const_validation_success: jsonBodies43ConstValidationSuccess
		},
	};
}


/**
 * Handler for POST /products
 */
async function jsonBodies32SchemaRefDefinitions(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		request_schema: {"type":"object","required":["product"],"properties":{"product":{"$ref":"#/definitions/Product"}},"definitions":{"Product":{"type":"object","required":["name","price"],"properties":{"name":{"type":"string"},"price":{"type":"number","minimum":0}}}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_32_schema_ref_definitions: jsonBodies32SchemaRefDefinitions
		},
	};
}


/**
 * Handler for POST /users
 */
async function jsonBodies29NestedObjectValidationSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		request_schema: {"type":"object","required":["profile"],"properties":{"profile":{"type":"object","required":["name","email"],"properties":{"name":{"type":"string","minLength":1},"email":{"type":"string","format":"email"}}}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_29_nested_object_validation_success: jsonBodies29NestedObjectValidationSuccess
		},
	};
}


/**
 * Handler for POST /users
 */
async function jsonBodies34AdditionalPropertiesFalse(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 422 };
	const result: Record<string, unknown> = {};
	if (_body !== null && _body !== undefined) {
		if (typeof _body === "object") {
			Object.assign(result, _body);
		} else {
			result.body = _body;
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
		request_schema: {"type":"object","required":["name"],"properties":{"name":{"type":"string"},"email":{"type":"string"}},"additionalProperties":false},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_34_additional_properties_false: jsonBodies34AdditionalPropertiesFalse
		},
	};
}


/**
 * Handler for POST /items/
 */
async function jsonBodiesNullValueForOptionalField(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Item","price":42.0,"description":null,"tax":null};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesNullValueForOptionalField(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_null_value_for_optional_field",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"description":{"type":"null"},"tax":{"type":"null"}},"additionalProperties":false,"required":["name","price","description","tax"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_null_value_for_optional_field: jsonBodiesNullValueForOptionalField
		},
	};
}


/**
 * Handler for POST /users
 */
async function jsonBodies31NullablePropertyNullValue(requestJson: string, _context?: HandlerContext): Promise<string> {
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
		request_schema: {"type":"object","required":["name"],"properties":{"name":{"type":"string"},"description":{"type":["string","null"]}}},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_31_nullable_property_null_value: jsonBodies31NullablePropertyNullValue
		},
	};
}


/**
 * Handler for POST /items/list
 */
async function jsonBodiesArrayOfObjectsSuccess(requestJson: string, _context?: HandlerContext): Promise<string> {
	const request = JSON.parse(requestJson);
	const _body = request.body ?? null;
	const _params = request.params ?? {};
	const response: HandlerResponse = { status: 200 };
	const responseBody = {"name":"Product Bundle","tags":["electronics","gadget"],"images":[{"url":"https://example.com/img1.jpg","name":"Front"},{"url":"https://example.com/img2.jpg","name":"Back"}]};
	response.body = responseBody;
	return JSON.stringify(response);
}

export function createAppJsonBodiesArrayOfObjectsSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/list",
		handler_name: "json_bodies_array_of_objects_success",
		request_schema: {"type":"object","properties":{"name":{"type":"string"},"tags":{"type":"array","items":{"type":"string"}},"images":{"type":"array","items":{"type":"object","properties":{"url":{"type":"string"},"name":{"type":"string"}},"additionalProperties":false,"required":["url","name"]}}},"additionalProperties":false,"required":["name","tags","images"]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_array_of_objects_success: jsonBodiesArrayOfObjectsSuccess
		},
	};
}




async function sseHandlerNotifications(_requestJson: string): Promise<StreamingResponse> {
	const events = [
    SystemAlertMessageSchema.parse({ level: "critical", message: "Database connection pool exhausted", source: "database-service", timestamp: "2024-01-15T10:30:00Z", type: "system_alert" }),
    NotificationBatchMessageSchema.parse([{ message: "example_message", timestamp: "2024-01-15T10:30:00Z", type: "example_type" }, { message: "example_message", timestamp: "2024-01-15T10:30:00Z", type: "example_type" }]),
    UserNotificationMessageSchema.parse({ body: "You have received a new direct message", priority: "high", timestamp: "2024-01-15T10:30:00Z", title: "New message from John", type: "user_notification", userId: "user_12345" }),
    StatusUpdateMessageSchema.parse({ message: "All systems operational", metadata: { region: "us-east-1", uptime: 99.99 }, service: "payment-gateway", status: "operational", timestamp: "2024-01-15T10:30:00Z", type: "status_update" })
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


type GraphqlFixture = {
	name: string;
	query: string;
	variables: unknown | null;
	operationName: string | null;
	statusCode: number;
	data: unknown | null;
	errors: unknown[] | null;
};

function graphqlVariablesKey(value: unknown): string {
	return JSON.stringify(value ?? null);
}

function getHeaderValue(headers: Record<string, string> | undefined, name: string): string | null {
	if (!headers) return null;
	const target = name.toLowerCase();
	for (const [key, value] of Object.entries(headers)) {
		if (key.toLowerCase() === target) return value;
	}
	return null;
}

function findGraphqlFixture(fixtures: GraphqlFixture[], body: unknown, headers: Record<string, string> | undefined): GraphqlFixture | null {
	const fixtureName = getHeaderValue(headers, "x-spikard-fixture");
	if (fixtureName) {
		const byName = fixtures.find((fixture) => fixture.name === fixtureName);
		if (byName) return byName;
	}
	if (!body || typeof body !== "object") return null;
	const payload = body as { query?: string; variables?: unknown; operationName?: string | null };
	const query = payload.query ?? "";
	const variables = payload.variables ?? null;
	const operationName = payload.operationName ?? null;
	return (
		fixtures.find(
			(fixture) =>
				fixture.query === query &&
				graphqlVariablesKey(fixture.variables) === graphqlVariablesKey(variables) &&
				fixture.operationName === operationName,
		) ?? null
	);
}

const graphqlMutationFixtures: GraphqlFixture[] = [
	{ name: "create_resource", query: `mutation CreateUser($input: CreateUserInput!) {\n  createUser(input: $input) {\n    id\n    name\n    email\n    role\n    createdAt\n  }\n}`, variables: { input: { name: "John Doe", email: "john@example.com", role: "admin" } }, operationName: "CreateUser", statusCode: 200, data: { createUser: { id: "user-123", name: "John Doe", email: "john@example.com", role: "admin", createdAt: "2025-12-27T10:30:00Z" } }, errors: null },
	{ name: "custom_scalar_invalid", query: `mutation CreateContact($input: CreateContactInput!) {\n  createContact(input: $input) {\n    id\n    name\n    email\n    website\n    phone\n    createdAt\n  }\n}`, variables: { input: { name: "Invalid Contact", email: "not-an-email", website: "not a url", phone: "123" } }, operationName: "CreateContact", statusCode: 422, data: null, errors: [{ message: "Email must be a valid email address", locations: [{ line: 1, column: 30 }], path: null, extensions: { field: "email", received_value: "not-an-email", expected_format: "RFC 5322 compliant email address", kind: "CUSTOM_SCALAR_COERCION_ERROR", code: "INVALID_EMAIL_FORMAT" } }, { message: "URL must start with http:// or https://", locations: [{ line: 1, column: 30 }], path: null, extensions: { field: "website", received_value: "not a url", expected_format: "Valid absolute URL with http:// or https:// scheme", kind: "CUSTOM_SCALAR_COERCION_ERROR", code: "INVALID_URL_FORMAT" } }, { message: "PhoneNumber must be a valid E.164 format", locations: [{ line: 1, column: 30 }], path: null, extensions: { field: "phone", received_value: "123", expected_format: "E.164 format: +1-555-123-4567 or +1555123456", kind: "CUSTOM_SCALAR_COERCION_ERROR", code: "INVALID_PHONE_FORMAT" } }] },
	{ name: "custom_scalar_validation", query: `mutation CreateContact($input: CreateContactInput!) {\n  createContact(input: $input) {\n    id\n    name\n    email\n    website\n    phone\n    createdAt\n  }\n}`, variables: { input: { name: "Alice Johnson", email: "alice.johnson@example.com", website: "https://example.com", phone: "+1-555-123-4567" } }, operationName: "CreateContact", statusCode: 200, data: { createContact: { id: "contact-001", name: "Alice Johnson", email: "alice.johnson@example.com", website: "https://example.com", phone: "+1-555-123-4567", createdAt: "2025-12-27T14:30:00Z" } }, errors: null },
	{ name: "delete_resource", query: `mutation DeleteUser($id: ID!) {\n  deleteUser(id: $id) {\n    success\n    message\n    deletedId\n  }\n}`, variables: { id: "user-123" }, operationName: "DeleteUser", statusCode: 200, data: { deleteUser: { success: true, message: "User successfully deleted", deletedId: "user-123" } }, errors: null },
	{ name: "dynamic_authorization", query: `mutation ApprovePost($postId: String!) {\n  approvePost(id: $postId) {\n    success\n    postId\n    status\n  }\n}`, variables: { postId: "post123" }, operationName: null, statusCode: 403, data: null, errors: [{ message: "Only post author or admin can approve posts", locations: null, path: null, extensions: null }] },
	{ name: "file_upload_multipart_spec", query: `mutation UploadDocument($title: String!, $files: [Upload!]!) {\n  uploadDocument(title: $title, files: $files) {\n    id\n    title\n    files {\n      id\n      filename\n      mimetype\n      size\n    }\n    uploadedAt\n  }\n}`, variables: { title: "Important Documents", files: [null, null] }, operationName: "UploadDocument", statusCode: 200, data: { uploadDocument: { id: "doc-001", title: "Important Documents", files: [{ id: "file-006", filename: "contract.pdf", mimetype: "application/pdf", size: 88 }, { id: "file-007", filename: "summary.txt", mimetype: "text/plain", size: 65 }], uploadedAt: "2025-12-27T14:30:00Z" } }, errors: null },
	{ name: "file_upload_validation_size", query: `mutation Upload($file: Upload!) {\n  singleUpload(file: $file) {\n    id\n    filename\n    mimetype\n    size\n  }\n}`, variables: { file: null }, operationName: "Upload", statusCode: 400, data: null, errors: [{ message: "File too large", locations: null, path: null, extensions: { code: "FILE_TOO_LARGE", maxSize: 10485760, uploadedSize: 15728640 } }] },
	{ name: "file_upload_validation_type", query: `mutation UploadImage($file: Upload!) {\n  uploadImage(file: $file) {\n    id\n    filename\n    mimetype\n    width\n    height\n  }\n}`, variables: { file: null }, operationName: "UploadImage", statusCode: 400, data: null, errors: [{ message: "Invalid file type", locations: null, path: null, extensions: { code: "INVALID_FILE_TYPE", expected: ["image/jpeg", "image/png", "image/gif", "image/webp"], received: "text/plain" } }] },
	{ name: "file_upload_with_variables", query: `mutation UploadProfile($userId: ID!, $file: Upload!) {\n  uploadProfilePicture(userId: $userId, file: $file) {\n    id\n    name\n    email\n    profilePicture {\n      id\n      filename\n      mimetype\n      size\n    }\n  }\n}`, variables: { userId: "user-123", file: null }, operationName: "UploadProfile", statusCode: 200, data: { uploadProfilePicture: { id: "user-123", name: "John Doe", email: "john@example.com", profilePicture: { id: "file-005", filename: "profile.jpg", mimetype: "image/jpeg", size: 24568 } } }, errors: null },
	{ name: "multiple_files_upload", query: `mutation MultipleUpload($files: [Upload!]!) {\n  multipleUpload(files: $files) {\n    id\n    filename\n    mimetype\n    size\n  }\n}`, variables: { files: [null, null, null] }, operationName: "MultipleUpload", statusCode: 200, data: { multipleUpload: [{ id: "file-002", filename: "document.pdf", mimetype: "application/pdf", size: 32 }, { id: "file-003", filename: "image.png", mimetype: "image/png", size: 24 }, { id: "file-004", filename: "data.csv", mimetype: "text/csv", size: 68 }] }, errors: null },
	{ name: "mutation_permission_check", query: `mutation DeleteUser($userId: String!) {\n  deleteUser(id: $userId) {\n    success\n    message\n  }\n}`, variables: { userId: "user123" }, operationName: null, statusCode: 403, data: null, errors: [{ message: "Missing required permission: DELETE", locations: null, path: null, extensions: null }] },
	{ name: "required_fields", query: `mutation Register($input: UserRegistrationInput!) {\n  registerUser(input: $input) {\n    success\n    userId\n    message\n  }\n}`, variables: { input: { username: "johndoe", email: "john@example.com" } }, operationName: null, statusCode: 400, data: null, errors: [{ message: "Field \"UserRegistrationInput.password\" of required type \"String!\" was not provided.", locations: [{ line: 1, column: 21 }], path: null, extensions: { fieldName: "password", typeName: "UserRegistrationInput", kind: "MISSING_REQUIRED_FIELD" } }] },
	{ name: "single_file_upload", query: `mutation Upload($file: Upload!) {\n  singleUpload(file: $file) {\n    id\n    filename\n    mimetype\n    size\n  }\n}`, variables: { file: null }, operationName: "Upload", statusCode: 200, data: { singleUpload: { id: "file-001", filename: "test.txt", mimetype: "text/plain", size: 39 } }, errors: null },
	{ name: "update_resource", query: `mutation UpdateUser($id: ID!, $input: UpdateUserInput!) {\n  updateUser(id: $id, input: $input) {\n    id\n    name\n    email\n    role\n    updatedAt\n  }\n}`, variables: { id: "user-123", input: { email: "john.doe@example.com", role: "editor" } }, operationName: "UpdateUser", statusCode: 200, data: { updateUser: { id: "user-123", name: "John Doe", email: "john.doe@example.com", role: "editor", updatedAt: "2025-12-27T11:45:00Z" } }, errors: null },
	{ name: "validation_directive", query: `mutation CreateUser($input: CreateUserInput!) {\n  createUser(input: $input) {\n    id\n    name\n    bio\n  }\n}`, variables: { input: { name: "a", bio: null } }, operationName: null, statusCode: 422, data: null, errors: [{ message: "Validation error on input field 'name': String length must be between 3 and 50 characters (provided: 1)", locations: null, path: ["createUser"], extensions: { code: "BAD_USER_INPUT", field: "name", constraint: "length", min: 3, max: 50, value_length: 1 } }] },
	{ name: "validation_error", query: `mutation CreatePost($input: CreatePostInput!) {\n  createPost(input: $input) {\n    id\n    title\n    content\n    tags\n    createdAt\n  }\n}`, variables: { input: { title: "My Post", content: "This is a post" } }, operationName: null, statusCode: 400, data: null, errors: [{ message: "Field \"CreatePostInput.tags\" of required type \"[String!]!\" was not provided.", locations: [{ line: 1, column: 30 }], path: null, extensions: { provided: { title: "My Post", content: "This is a post" }, missing_fields: ["tags"] } }] },
];

async function graphqlMutation(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const fixture = findGraphqlFixture(graphqlMutationFixtures, request.body ?? null, request.headers ?? undefined);
	const response: HandlerResponse = { status: fixture?.statusCode ?? 500 };
	response.body = fixture
		? { data: fixture.data ?? null, errors: fixture.errors ?? null }
		: { errors: [{ message: "GraphQL fixture not found" }] };
	return JSON.stringify(response);
}

export function createAppGraphqlMutation(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/graphql",
		handler_name: "graphql_mutation",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			graphql_mutation: graphqlMutation,
		},
	};
}
const graphqlQueryFixtures: GraphqlFixture[] = [
	{ name: "api_key_invalid", query: `query {\n  secureData\n}`, variables: null, operationName: null, statusCode: 401, data: null, errors: [{ message: "Invalid API key", locations: null, path: null, extensions: null }] },
	{ name: "api_key_valid", query: `query {\n  secureData\n}`, variables: null, operationName: null, statusCode: 200, data: { secureData: "Protected data from API key authentication" }, errors: null },
	{ name: "cache_directive", query: `query GetUser($id: ID!) {\n  user(id: $id) {\n    id\n    name\n    email\n  }\n}`, variables: { id: "1" }, operationName: null, statusCode: 200, data: { user: { id: "1", name: "Alice Smith", email: "alice@example.com" } }, errors: null },
	{ name: "complex_query", query: `query ComplexSearch($searchTerm: String!, $userLimit: Int!, $postLimit: Int!) {\n  search(term: $searchTerm) {\n    total\n    users(limit: $userLimit) {\n      id\n      name\n      email\n      profile {\n        bio\n        avatar\n        joinedAt\n      }\n      recentPosts: posts(limit: 3) {\n        id\n        title\n        likes\n      }\n      followerCount: followers(limit: 100) {\n        id\n      }\n    }\n    posts(limit: $postLimit) {\n      id\n      title\n      content\n      likes\n      author {\n        id\n        name\n        profile {\n          avatar\n        }\n      }\n      topComments: comments(limit: 5) {\n        id\n        text\n        likes\n        author {\n          id\n          name\n        }\n      }\n    }\n  }\n}`, variables: { searchTerm: "graphql", userLimit: 5, postLimit: 10 }, operationName: "ComplexSearch", statusCode: 200, data: { search: { total: 42, users: [{ id: "user-1", name: "GraphQL Expert", email: "expert@example.com", profile: { bio: "GraphQL enthusiast and API designer", avatar: "https://example.com/avatars/expert.jpg", joinedAt: "2024-01-15T08:30:00Z" }, recentPosts: [{ id: "post-101", title: "GraphQL Best Practices", likes: 234 }, { id: "post-102", title: "Schema Design Patterns", likes: 189 }, { id: "post-103", title: "Performance Optimization", likes: 156 }], followerCount: [{ id: "user-2" }, { id: "user-3" }] }, { id: "user-2", name: "API Developer", email: "developer@example.com", profile: { bio: "Building scalable APIs", avatar: "https://example.com/avatars/developer.jpg", joinedAt: "2024-02-20T10:15:00Z" }, recentPosts: [{ id: "post-201", title: "GraphQL vs REST", likes: 145 }], followerCount: [{ id: "user-1" }] }], posts: [{ id: "post-101", title: "GraphQL Best Practices", content: "A comprehensive guide to GraphQL best practices and patterns...", likes: 234, author: { id: "user-1", name: "GraphQL Expert", profile: { avatar: "https://example.com/avatars/expert.jpg" } }, topComments: [{ id: "comment-1", text: "Great post, very helpful!", likes: 45, author: { id: "user-2", name: "API Developer" } }, { id: "comment-2", text: "Could you elaborate on caching?", likes: 32, author: { id: "user-3", name: "Data Scientist" } }] }, { id: "post-102", title: "Schema Design Patterns", content: "Exploring common patterns for designing GraphQL schemas...", likes: 189, author: { id: "user-1", name: "GraphQL Expert", profile: { avatar: "https://example.com/avatars/expert.jpg" } }, topComments: [{ id: "comment-3", text: "Excellent breakdown", likes: 28, author: { id: "user-4", name: "Backend Engineer" } }] }] } }, errors: null },
	{ name: "cross_subgraph_query", query: `query {\n  user(id: \"usr-42\") {\n    id\n    name\n    email\n    orders {\n      id\n      orderId\n      total\n      status\n      createdAt\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { user: { id: "usr-42", name: "Emma Wilson", email: "emma@example.com", orders: [{ id: "order-101", orderId: "ORD-2024-001", total: 149.99, status: "DELIVERED", createdAt: "2024-01-15T10:30:00Z" }, { id: "order-102", orderId: "ORD-2024-002", total: 89.5, status: "PROCESSING", createdAt: "2024-12-20T14:22:00Z" }] } }, errors: null },
	{ name: "custom_auth_directive", query: `query {\n  publicData\n  secretData\n  moderatorData\n}`, variables: null, operationName: null, statusCode: 200, data: { publicData: "Anyone can see this", secretData: null, moderatorData: null }, errors: [{ message: "Unauthorized: User role USER cannot access ADMIN field", locations: null, path: ["secretData"], extensions: { code: "FORBIDDEN", required_role: "ADMIN", user_role: "USER" } }, { message: "Unauthorized: User role USER cannot access MODERATOR field", locations: null, path: ["moderatorData"], extensions: { code: "FORBIDDEN", required_role: "MODERATOR", user_role: "USER" } }] },
	{ name: "dataloader_batch_users", query: `query GetUsers($ids: [ID!]!) {\n  users(ids: $ids) {\n    id\n    name\n    email\n    age\n  }\n}`, variables: { ids: ["1", "2", "3"] }, operationName: null, statusCode: 200, data: { users: [{ id: "1", name: "Alice Johnson", email: "alice@example.com", age: 28 }, { id: "2", name: "Bob Smith", email: "bob@example.com", age: 34 }, { id: "3", name: "Carol Davis", email: "carol@example.com", age: 26 }] }, errors: null },
	{ name: "dataloader_cache_hit", query: `query {\n  user1: user(id: \"1\") {\n    id\n    name\n    email\n  }\n  user2: user(id: \"1\") {\n    id\n    name\n    username\n  }\n  user3: user(id: \"2\") {\n    id\n    name\n    email\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { user1: { id: "1", name: "Alice Smith", email: "alice@example.com" }, user2: { id: "1", name: "Alice Smith", username: "alice_smith" }, user3: { id: "2", name: "Bob Johnson", email: "bob@example.com" } }, errors: null },
	{ name: "dataloader_custom_key", query: `query GetProduct($slug: String!) {\n  productBySlug(slug: $slug) {\n    id\n    name\n    slug\n    price\n    category\n    description\n  }\n}`, variables: { slug: "laptop-pro-2025" }, operationName: null, statusCode: 200, data: { productBySlug: { id: "prod-1", name: "Professional Laptop", slug: "laptop-pro-2025", price: 1299.99, category: "electronics", description: "High-performance laptop for professionals" } }, errors: null },
	{ name: "dataloader_error_handling", query: `query GetUsers($ids: [ID!]!) {\n  users(ids: $ids) {\n    id\n    name\n    email\n  }\n}`, variables: { ids: ["1", "999", "2"] }, operationName: null, statusCode: 200, data: { users: [{ id: "1", name: "Alice Johnson", email: "alice@example.com" }, null, { id: "2", name: "Bob Smith", email: "bob@example.com" }] }, errors: [{ message: "User not found with id '999'", locations: null, path: ["users", 1], extensions: { code: "NOT_FOUND", id: "999" } }] },
	{ name: "dataloader_n_plus_one_prevention", query: `query {\n  posts {\n    id\n    title\n    content\n    author {\n      id\n      name\n      email\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { posts: [{ id: "post-1", title: "GraphQL Basics", content: "Introduction to GraphQL...", author: { id: "user-1", name: "Alice Johnson", email: "alice@example.com" } }, { id: "post-2", title: "DataLoader Patterns", content: "Optimizing GraphQL queries...", author: { id: "user-2", name: "Bob Smith", email: "bob@example.com" } }, { id: "post-3", title: "Advanced GraphQL", content: "Custom directives and more...", author: { id: "user-1", name: "Alice Johnson", email: "alice@example.com" } }] }, errors: null },
	{ name: "dataloader_nested_batching", query: `query {\n  posts {\n    id\n    title\n    comments {\n      id\n      text\n      author {\n        id\n        name\n        email\n      }\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { posts: [{ id: "post-1", title: "GraphQL Introduction", comments: [{ id: "comment-1", text: "Great article!", author: { id: "user-1", name: "Alice Johnson", email: "alice@example.com" } }, { id: "comment-2", text: "Very helpful", author: { id: "user-2", name: "Bob Smith", email: "bob@example.com" } }] }, { id: "post-2", title: "Advanced Patterns", comments: [{ id: "comment-3", text: "Excellent explanation", author: { id: "user-1", name: "Alice Johnson", email: "alice@example.com" } }] }] }, errors: null },
	{ name: "dataloader_priming", query: `query {\n  userList {\n    id\n    name\n    email\n    role\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { userList: [{ id: "user-1", name: "Alice Johnson", email: "alice@example.com", role: "admin" }, { id: "user-2", name: "Bob Smith", email: "bob@example.com", role: "user" }, { id: "user-3", name: "Carol Davis", email: "carol@example.com", role: "moderator" }] }, errors: null },
	{ name: "dataloader_with_variables", query: `query GetPosts($ids: [ID!]!) {\n  posts(ids: $ids) {\n    id\n    title\n    slug\n    publishedAt\n    tags\n  }\n}`, variables: { ids: ["1", "2", "3"] }, operationName: null, statusCode: 200, data: { posts: [{ id: "1", title: "Getting Started with GraphQL", slug: "getting-started-graphql", publishedAt: "2025-01-10T08:00:00Z", tags: ["graphql", "tutorial"] }, { id: "2", title: "Mastering DataLoader", slug: "mastering-dataloader", publishedAt: "2025-01-15T10:30:00Z", tags: ["dataloader", "performance", "optimization"] }, { id: "3", title: "GraphQL Best Practices", slug: "graphql-best-practices", publishedAt: "2025-01-20T14:45:00Z", tags: ["graphql", "best-practices", "patterns"] }] }, errors: null },
	{ name: "datetime_scalar", query: `query GetEvents($since: DateTime, $until: DateTime) {\n  events(since: $since, until: $until) {\n    id\n    title\n    scheduledAt\n    completedAt\n  }\n}`, variables: { since: "2025-01-01T00:00:00Z", until: "2025-12-31T23:59:59Z" }, operationName: null, statusCode: 200, data: { events: [{ id: "event-1", title: "Conference", scheduledAt: "2025-06-15T09:00:00Z", completedAt: "2025-06-15T17:00:00Z" }, { id: "event-2", title: "Hackathon", scheduledAt: "2025-08-20T10:00:00Z", completedAt: null }] }, errors: null },
	{ name: "deeply_nested_query", query: `query GetUserDeepNested($userId: String!) {\n  user(id: $userId) {\n    id\n    name\n    profile {\n      bio\n      settings {\n        preferences {\n          theme\n          language\n          timezone {\n            name\n            offset\n          }\n        }\n        notifications {\n          email\n          push\n        }\n      }\n    }\n  }\n}`, variables: { userId: "user-deep-001" }, operationName: null, statusCode: 200, data: { user: { id: "user-deep-001", name: "Alice Cooper", profile: { bio: "DevOps engineer passionate about scalability", settings: { preferences: { theme: "dark", language: "en-US", timezone: { name: "America/Los_Angeles", offset: -480 } }, notifications: { email: true, push: false } } } } }, errors: null },
	{ name: "deprecated_field", query: `query {\n  oldField\n  newField\n}`, variables: null, operationName: null, statusCode: 200, data: { oldField: "legacy value", newField: "modern value" }, errors: null },
	{ name: "entity_resolution_basic", query: `query {\n  _entities(representations: [{__typename: \"User\", id: \"1\"}]) {\n    ... on User {\n      id\n      name\n      email\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { _entities: [{ id: "1", name: "Alice Johnson", email: "alice@example.com" }] }, errors: null },
	{ name: "entity_with_compound_key", query: `query {\n  _entities(representations: [{__typename: \"Product\", sku: \"ABC123\", category: \"electronics\"}]) {\n    ... on Product {\n      sku\n      category\n      name\n      description\n      price\n      stock\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { _entities: [{ sku: "ABC123", category: "electronics", name: "Wireless Headphones", description: "Premium noise-cancelling wireless headphones", price: 199.99, stock: 45 }] }, errors: null },
	{ name: "entity_with_key", query: `query {\n  _entities(representations: [{__typename: \"User\", id: \"42\"}]) {\n    ... on User {\n      id\n      name\n      username\n      profile {\n        bio\n        avatar\n        joinDate\n      }\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { _entities: [{ id: "42", name: "Bob Smith", username: "bobsmith", profile: { bio: "Software engineer and open source enthusiast", avatar: "https://example.com/avatars/bob.jpg", joinDate: "2020-03-15" } }] }, errors: null },
	{ name: "external_field", query: `query {\n  _entities(representations: [{__typename: \"Parcel\", id: \"parcel-x1\", weight: 2.5, dimensions: \"10x8x6\"}]) {\n    ... on Parcel {\n      id\n      weight\n      dimensions\n      label\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { _entities: [{ id: "parcel-x1", weight: 2.5, dimensions: "10x8x6", label: "SMALL_PACKAGE_2.5KG" }] }, errors: null },
	{ name: "federation_error_missing_entity", query: `query {\n  _entities(representations: [{__typename: \"Customer\", id: \"999999\"}]) {\n    ... on Customer {\n      id\n      firstName\n      lastName\n      email\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { _entities: [null] }, errors: null },
	{ name: "federation_type_mismatch", query: `query {\n  _entities(representations: [{__typename: \"InvalidType\", id: \"1\"}]) {\n    ... on Article {\n      id\n      title\n      content\n      author\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 400, data: null, errors: [{ message: "Unknown type 'InvalidType' in entity representation", locations: null, path: null, extensions: { code: "FEDERATION_ERROR" } }] },
	{ name: "field_error", query: `query GetUser($id: ID!) {\n  user(id: $id) {\n    id\n    name\n    invalidField\n  }\n}`, variables: { id: "user-123" }, operationName: null, statusCode: 400, data: null, errors: [{ message: "Cannot query field \"invalidField\" on type \"User\". Did you mean \"id\", \"name\", or \"email\"?", locations: [{ line: 4, column: 5 }], path: ["user", "invalidField"], extensions: null }] },
	{ name: "field_level_permissions", query: `query {\n  user(id: \"user123\") {\n    id\n    email\n    privateData\n  }\n}`, variables: { userId: "user123" }, operationName: null, statusCode: 200, data: null, errors: [{ message: "Field 'privateData' requires elevated permissions", locations: null, path: ["user", "privateData"], extensions: null }] },
	{ name: "full_schema_introspection", query: ``, variables: null, operationName: null, statusCode: 200, data: { __schema: { queryType: { name: "Query" }, mutationType: { name: "Mutation" }, subscriptionType: null, types: [{ kind: "SCALAR", name: "DateTime", description: "ISO 8601 DateTime scalar", fields: null, inputFields: null, interfaces: null, enumValues: null, possibleTypes: null }, { kind: "SCALAR", name: "UUID", description: "UUID scalar type", fields: null, inputFields: null, interfaces: null, enumValues: null, possibleTypes: null }, { kind: "OBJECT", name: "Query", description: "Root query type", fields: [{ name: "hello", description: "Greeting message", args: [], type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "String" } }, isDeprecated: false, deprecationReason: null }, { name: "version", description: "API version", args: [], type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "String" } }, isDeprecated: false, deprecationReason: null }, { name: "user", description: "Get user by ID", args: [{ name: "id", description: "User ID", type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "UUID" } }, defaultValue: null }], type: { kind: "OBJECT", name: "User" }, isDeprecated: false, deprecationReason: null }, { name: "users", description: "Get all users with pagination", args: [{ name: "limit", description: "Maximum number of results", type: { kind: "SCALAR", name: "Int" }, defaultValue: "10" }, { name: "offset", description: "Number of results to skip", type: { kind: "SCALAR", name: "Int" }, defaultValue: "0" }], type: { kind: "NON_NULL", name: null, ofType: { kind: "LIST", name: null, ofType: { kind: "NON_NULL", name: null, ofType: { kind: "OBJECT", name: "User" } } } }, isDeprecated: false, deprecationReason: null }], inputFields: null, interfaces: [], enumValues: null, possibleTypes: null }, { kind: "OBJECT", name: "Mutation", description: "Root mutation type", fields: [{ name: "createPost", description: "Create a new post", args: [{ name: "input", description: "Post creation input", type: { kind: "NON_NULL", name: null, ofType: { kind: "INPUT_OBJECT", name: "CreatePostInput" } }, defaultValue: null }], type: { kind: "NON_NULL", name: null, ofType: { kind: "OBJECT", name: "Post" } }, isDeprecated: false, deprecationReason: null }, { name: "updateUser", description: "Update user information", args: [{ name: "id", description: "User ID", type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "UUID" } }, defaultValue: null }, { name: "name", description: "New user name", type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "String" } }, defaultValue: null }], type: { kind: "NON_NULL", name: null, ofType: { kind: "OBJECT", name: "User" } }, isDeprecated: false, deprecationReason: null }, { name: "deletePost", description: "Delete a post", args: [{ name: "id", description: "Post ID", type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "UUID" } }, defaultValue: null }], type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "Boolean" } }, isDeprecated: false, deprecationReason: null }], inputFields: null, interfaces: [], enumValues: null, possibleTypes: null }, { kind: "OBJECT", name: "User", description: "User entity", fields: [{ name: "id", description: "Unique identifier", args: [], type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "UUID" } }, isDeprecated: false, deprecationReason: null }, { name: "name", description: "User's full name", args: [], type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "String" } }, isDeprecated: false, deprecationReason: null }, { name: "email", description: "User's email address", args: [], type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "String" } }, isDeprecated: false, deprecationReason: null }, { name: "createdAt", description: "Creation timestamp", args: [], type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "DateTime" } }, isDeprecated: false, deprecationReason: null }, { name: "posts", description: "User's posts", args: [], type: { kind: "NON_NULL", name: null, ofType: { kind: "LIST", name: null, ofType: { kind: "NON_NULL", name: null, ofType: { kind: "OBJECT", name: "Post" } } } }, isDeprecated: false, deprecationReason: null }], inputFields: null, interfaces: [], enumValues: null, possibleTypes: null }, { kind: "OBJECT", name: "Post", description: "Blog post entity", fields: [{ name: "id", description: "Unique identifier", args: [], type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "UUID" } }, isDeprecated: false, deprecationReason: null }, { name: "title", description: "Post title", args: [], type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "String" } }, isDeprecated: false, deprecationReason: null }, { name: "content", description: "Post content", args: [], type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "String" } }, isDeprecated: false, deprecationReason: null }, { name: "authorId", description: "Author's ID", args: [], type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "UUID" } }, isDeprecated: false, deprecationReason: null }, { name: "author", description: "Post author", args: [], type: { kind: "NON_NULL", name: null, ofType: { kind: "OBJECT", name: "User" } }, isDeprecated: false, deprecationReason: null }, { name: "createdAt", description: "Creation timestamp", args: [], type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "DateTime" } }, isDeprecated: false, deprecationReason: null }, { name: "updatedAt", description: "Last update timestamp", args: [], type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "DateTime" } }, isDeprecated: false, deprecationReason: null }], inputFields: null, interfaces: [], enumValues: null, possibleTypes: null }, { kind: "INPUT_OBJECT", name: "CreatePostInput", description: "Input for creating posts", fields: null, inputFields: [{ name: "title", description: "Post title", type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "String" } }, defaultValue: null }, { name: "content", description: "Post content", type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "String" } }, defaultValue: null }, { name: "authorId", description: "Author ID", type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "UUID" } }, defaultValue: null }], interfaces: null, enumValues: null, possibleTypes: null }], directives: [{ name: "skip", description: "Directs the executor to skip this field or fragment when the `if` argument is true.", locations: ["FIELD", "FRAGMENT_SPREAD", "INLINE_FRAGMENT"], args: [{ name: "if", description: "Skipped when true", type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "Boolean" } }, defaultValue: null }] }, { name: "include", description: "Directs the executor to include this field or fragment when the `if` argument is true.", locations: ["FIELD", "FRAGMENT_SPREAD", "INLINE_FRAGMENT"], args: [{ name: "if", description: "Included when true", type: { kind: "NON_NULL", name: null, ofType: { kind: "SCALAR", name: "Boolean" } }, defaultValue: null }] }, { name: "deprecated", description: "Marks an element of a GraphQL schema as no longer supported.", locations: ["FIELD_DEFINITION", "ENUM_VALUE"], args: [{ name: "reason", description: "Explains why this element was deprecated", type: { kind: "SCALAR", name: "String" }, defaultValue: "No longer supported" }] }] } }, errors: null },
	{ name: "inaccessible_directive", query: `query {\n  user(id: \"user-42\") {\n    id\n    name\n    email\n    internalScore\n    publicStatus\n  }\n}`, variables: null, operationName: null, statusCode: 400, data: null, errors: [{ message: "Cannot query field 'internalScore' on type 'User'. This field is @inaccessible and not available in the public schema.", locations: null, path: null, extensions: { code: "GRAPHQL_VALIDATION_FAILED", specifiedBy: "https://spec.apollographql.com/core/v0.3#sec-Query-Root-Type" } }] },
	{ name: "introspection_disabled", query: ``, variables: null, operationName: null, statusCode: 400, data: null, errors: [{ message: "Introspection is disabled", locations: null, path: null, extensions: { code: "INTROSPECTION_DISABLED", introspection_enabled: false } }] },
	{ name: "invalid_types", query: `query SearchUsers($limit: Int!, $offset: Int) {\n  searchUsers(limit: $limit, offset: $offset) {\n    id\n    name\n    email\n  }\n}`, variables: { limit: "not_an_integer", offset: 10 }, operationName: null, statusCode: 400, data: null, errors: [{ message: "Variable \"$limit\" of type \"Int!\" was provided invalid value.", locations: [{ line: 1, column: 16 }], path: null, extensions: { value: "not_an_integer", problems: [{ path: [], explanation: "Expected value to be an integer, but received: \"not_an_integer\"" }] } }] },
	{ name: "json_scalar", query: `query GetConfig {\n  configuration {\n    id\n    name\n    settings\n    metadata\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { configuration: { id: "config-1", name: "Production Config", settings: { timeout: 30000, retries: 3, features: { caching: true, compression: true, tracing: false }, endpoints: ["https://api.example.com", "https://api-backup.example.com"] }, metadata: { version: "1.0.0", environment: "production", lastUpdated: "2025-12-27T10:00:00Z", author: "DevOps Team" } } }, errors: null },
	{ name: "jwt_expired", query: `query {\n  currentUser {\n    id\n    email\n  }\n}`, variables: null, operationName: null, statusCode: 401, data: null, errors: [{ message: "Token expired", locations: null, path: null, extensions: null }] },
	{ name: "jwt_invalid_signature", query: `query {\n  currentUser {\n    id\n    email\n  }\n}`, variables: null, operationName: null, statusCode: 401, data: null, errors: [{ message: "Invalid token signature", locations: null, path: null, extensions: null }] },
	{ name: "jwt_valid", query: `query {\n  currentUser {\n    id\n    email\n    name\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { currentUser: { id: "user123", email: "john@example.com", name: "John Doe" } }, errors: null },
	{ name: "multiple_auth_methods", query: `query {\n  currentUser {\n    id\n    email\n    authMethod\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { currentUser: { id: "user123", email: "john@example.com", authMethod: "jwt" } }, errors: null },
	{ name: "nested_objects", query: `query GetUser($userId: String!) {\n  user(id: $userId) {\n    id\n    name\n    email\n    profile {\n      bio\n      location\n    }\n  }\n}`, variables: { userId: "550e8400-e29b-41d4-a716-446655440000" }, operationName: null, statusCode: 200, data: { user: { id: "550e8400-e29b-41d4-a716-446655440000", name: "Alice Johnson", email: "alice@example.com", profile: { bio: "Software engineer and open source enthusiast", location: "San Francisco, CA" } } }, errors: null },
	{ name: "no_authentication", query: `query {\n  protectedQuery\n}`, variables: null, operationName: null, statusCode: 401, data: null, errors: [{ message: "Authentication required", locations: null, path: null, extensions: null }] },
	{ name: "override_directive", query: `query {\n  user(id: \"user-789\") {\n    id\n    username\n    email\n    profile {\n      bio\n      joinDate\n      location\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { user: { id: "user-789", username: "johndoe", email: "john.doe@example.com", profile: { bio: "Software developer and tech enthusiast", joinDate: "2021-06-12", location: "San Francisco, CA" } } }, errors: null },
	{ name: "permission_chain", query: `query {\n  dashboard {\n    id\n    publicMetrics {\n      pageViews\n      uniqueVisitors\n    }\n    privateMetrics {\n      pageViews\n      uniqueVisitors\n    }\n    adminSettings {\n      apiKey\n      webhookUrl\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: null, errors: [{ message: "Insufficient permissions to access privateMetrics", locations: null, path: ["dashboard", "privateMetrics"], extensions: null }, { message: "Insufficient permissions to access adminSettings", locations: null, path: ["dashboard", "adminSettings"], extensions: null }] },
	{ name: "persisted_query_allowlist", query: ``, variables: {  }, operationName: null, statusCode: 403, data: null, errors: [{ message: "Query not in allowlist", locations: null, path: null, extensions: { code: "QUERY_NOT_WHITELISTED", hash: "999999999999999999999999999999999999999999999999999999999999999999" } }] },
	{ name: "persisted_query_automatic_persisted", query: ``, variables: { q: "GraphQL" }, operationName: null, statusCode: 400, data: null, errors: [{ message: "PersistedQueryNotFound", locations: null, path: null, extensions: { code: "PERSISTED_QUERY_NOT_FOUND" } }] },
	{ name: "persisted_query_hash_mismatch", query: `query GetUser($id: ID!) {\n  user(id: $id) {\n    id\n    name\n    email\n  }\n}`, variables: { id: "user-999" }, operationName: "GetUser", statusCode: 400, data: null, errors: [{ message: "Hash mismatch", locations: null, path: null, extensions: { code: "PERSISTED_QUERY_HASH_MISMATCH", expected_hash: "4a5b4c8d9e2f1a3b5c7d9e1f3a5b7c9d1e3f5a7b9c1d3e5f7a9b1c3d5f7a", provided_hash: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff" } }] },
	{ name: "persisted_query_hit", query: ``, variables: { id: "user-123" }, operationName: null, statusCode: 200, data: { user: { id: "user-123", name: "Alice Smith", email: "alice@example.com" } }, errors: null },
	{ name: "persisted_query_miss", query: ``, variables: { id: "user-456" }, operationName: null, statusCode: 400, data: null, errors: [{ message: "PersistedQueryNotFound", locations: null, path: null, extensions: { code: "PERSISTED_QUERY_NOT_FOUND", hash: "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890" } }] },
	{ name: "persisted_query_registration", query: `query GetUserPosts($userId: ID!) {\n  posts(userId: $userId) {\n    id\n    title\n    content\n    author {\n      id\n      name\n    }\n  }\n}`, variables: { userId: "user-789" }, operationName: "GetUserPosts", statusCode: 200, data: { posts: [{ id: "post-1", title: "GraphQL Best Practices", content: "Understanding GraphQL query optimization...", author: { id: "user-789", name: "Bob Johnson" } }, { id: "post-2", title: "Persisted Queries Guide", content: "How to implement persisted queries for performance...", author: { id: "user-789", name: "Bob Johnson" } }] }, errors: null },
	{ name: "provides_directive", query: `query {\n  _entities(representations: [{__typename: \"Post\", id: \"post-123\"}]) {\n    ... on Post {\n      id\n      title\n      content\n      reviews {\n        id\n        rating\n        text\n        author {\n          id\n          name\n        }\n      }\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { _entities: [{ id: "post-123", title: "Getting Started with GraphQL Federation", content: "Learn how to build scalable microservices...", reviews: [{ id: "rev-001", rating: 5, text: "Excellent post!", author: { id: "user-1", name: "Charlie Brown" } }, { id: "rev-002", rating: 4, text: "Very helpful", author: { id: "user-2", name: "Diana Prince" } }] }] }, errors: null },
	{ name: "query_batching", query: ``, variables: null, operationName: null, statusCode: 200, data: [{ user: { id: "user-1", name: "Alice Johnson", email: "alice@example.com" } }, { user: { id: "user-2", name: "Bob Smith", email: "bob@example.com" } }, { post: { id: "post-1", title: "GraphQL Performance Tips", author_id: "user-1" } }], errors: null },
	{ name: "rate_limit_directive", query: `query {\n  expensiveQuery\n}`, variables: null, operationName: null, statusCode: 200, data: { expensiveQuery: "Result from expensive computation" }, errors: null },
	{ name: "requires_directive", query: `query {\n  _entities(representations: [{__typename: \"Shipment\", id: \"ship-001\", weight: 5.5, destination: \"NYC\"}]) {\n    ... on Shipment {\n      id\n      weight\n      destination\n      shippingEstimate\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { _entities: [{ id: "ship-001", weight: 5.5, destination: "NYC", shippingEstimate: 24.75 }] }, errors: null },
	{ name: "resource_owner_allowed", query: `query GetUserProfile($userId: String!) {\n  user(id: $userId) {\n    id\n    profile {\n      bio\n      website\n      joinDate\n    }\n  }\n}`, variables: { userId: "user123" }, operationName: null, statusCode: 200, data: { user: { id: "user123", profile: { bio: "Software engineer from San Francisco", website: "https://example.com", joinDate: "2020-01-15" } } }, errors: null },
	{ name: "resource_owner_denied", query: `query GetUserProfile($userId: String!) {\n  user(id: $userId) {\n    id\n    profile {\n      bio\n      website\n    }\n  }\n}`, variables: { userId: "user456" }, operationName: null, statusCode: 403, data: null, errors: [{ message: "Not authorized to access this resource", locations: null, path: null, extensions: null }] },
	{ name: "response_streaming", query: `query GetUserWithDeferred($userId: String!) {\n  user(id: $userId) {\n    id\n    name\n    email\n    ...DeferredPosts @defer(label: \"userPosts\")\n    ...DeferredFollowers @defer(label: \"userFollowers\")\n  }\n}\n\nfragment DeferredPosts on User {\n  posts @stream(initialCount: 1, label: \"postsStream\") {\n    id\n    title\n    published_at\n  }\n}\n\nfragment DeferredFollowers on User {\n  followers @stream(initialCount: 2, label: \"followersStream\") {\n    id\n    name\n  }\n}`, variables: { userId: "user-123" }, operationName: null, statusCode: 200, data: null, errors: null },
	{ name: "role_admin_allowed", query: `query {\n  adminPanel {\n    stats {\n      totalUsers\n      activeUsers\n      totalRevenue\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { adminPanel: { stats: { totalUsers: 1250, activeUsers: 856, totalRevenue: 125000.5 } } }, errors: null },
	{ name: "role_user_denied", query: `query {\n  adminPanel {\n    stats {\n      totalUsers\n      activeUsers\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 403, data: null, errors: [{ message: "Insufficient permissions to access adminPanel", locations: null, path: null, extensions: null }] },
	{ name: "session_cookie_valid", query: `query {\n  userProfile {\n    id\n    username\n    email\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { userProfile: { id: "user456", username: "alice_smith", email: "alice@example.com" } }, errors: null },
	{ name: "shareable_directive", query: `query {\n  _entities(representations: [{__typename: \"Product\", id: \"prod-001\"}]) {\n    ... on Product {\n      id\n      name\n      description\n      category\n      price\n    }\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { _entities: [{ id: "prod-001", name: "Wireless Headphones", description: "Premium noise-canceling headphones with 30-hour battery life", category: "Electronics", price: 199.99 }] }, errors: null },
	{ name: "simple_field", query: `query {\n  hello\n}`, variables: null, operationName: null, statusCode: 200, data: { hello: "Hello, World!" }, errors: null },
	{ name: "subgraph_introspection", query: `query {\n  _service {\n    sdl\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { _service: { sdl: "type Account @key(fields: \"accountId\") {\n  accountId: ID!\n  accountName: String!\n  tier: String!\n  createdAt: String!\n}\n\ntype Query {\n  account(accountId: ID!): Account\n}" } }, errors: null },
	{ name: "syntax_error", query: `query {\n  user(id: \"123\n}`, variables: null, operationName: null, statusCode: 400, data: null, errors: [{ message: "Syntax Error in GraphQL query at line 2, column 17: Unterminated string.", locations: [{ line: 2, column: 17 }], path: null, extensions: null }] },
	{ name: "transform_directive", query: `query {\n  message @uppercase\n  title @uppercase\n}`, variables: null, operationName: null, statusCode: 200, data: { message: "HELLO FROM GRAPHQL", title: "WELCOME TO SPIKARD" }, errors: null },
	{ name: "type_error", query: `query GetPost($id: ID!) {\n  post(id: $id) {\n    id\n    title\n    content\n  }\n}`, variables: { id: true }, operationName: null, statusCode: 400, data: null, errors: [{ message: "Variable \"$id\" of type \"ID!\" was provided invalid value.", locations: [{ line: 1, column: 24 }], path: null, extensions: { value: true, problems: [{ path: [], explanation: "Expected value to be a string, but received: true" }] } }] },
	{ name: "uuid_scalar", query: `query GetResource($id: UUID!) {\n  resource(id: $id) {\n    id\n    parentId\n    name\n    ownerId\n    relatedIds\n  }\n}`, variables: { id: "550e8400-e29b-41d4-a716-446655440000" }, operationName: null, statusCode: 200, data: { resource: { id: "550e8400-e29b-41d4-a716-446655440000", parentId: "6ba7b810-9dad-11d1-80b4-00c04fd430c8", name: "Primary Resource", ownerId: "6ba7b811-9dad-11d1-80b4-00c04fd430c8", relatedIds: ["6ba7b812-9dad-11d1-80b4-00c04fd430c8", "6ba7b814-9dad-11d1-80b4-00c04fd430c8"] } }, errors: null },
	{ name: "with_arguments", query: `query Greet($name: String!) {\n  greet(name: $name)\n}`, variables: { name: "Alice" }, operationName: "Greet", statusCode: 200, data: { greet: "Hello, Alice!" }, errors: null },
];

async function graphqlQuery(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const fixture = findGraphqlFixture(graphqlQueryFixtures, request.body ?? null, request.headers ?? undefined);
	const response: HandlerResponse = { status: fixture?.statusCode ?? 500 };
	response.body = fixture
		? { data: fixture.data ?? null, errors: fixture.errors ?? null }
		: { errors: [{ message: "GraphQL fixture not found" }] };
	return JSON.stringify(response);
}

export function createAppGraphqlQuery(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/graphql",
		handler_name: "graphql_query",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			graphql_query: graphqlQuery,
		},
	};
}
const graphqlSubscriptionFixtures: GraphqlFixture[] = [
	{ name: "filtered_subscription", query: `subscription OnOrderUpdated($status: OrderStatus) {\n  orderUpdated(status: $status) {\n    id\n    orderId\n    status\n    amount\n    updatedAt\n  }\n}`, variables: { status: "SHIPPED" }, operationName: "OnOrderUpdated", statusCode: 200, data: { orderUpdated: { id: "order-456", orderId: "ORD-2025-00123", status: "SHIPPED", amount: 149.99, updatedAt: "2025-12-27T11:30:00Z" } }, errors: null },
	{ name: "simple_subscription", query: `subscription {\n  messageAdded {\n    id\n    text\n    timestamp\n  }\n}`, variables: null, operationName: null, statusCode: 200, data: { messageAdded: { id: "msg-1", text: "Hello, WebSocket!", timestamp: "2025-12-27T10:00:00Z" } }, errors: null },
	{ name: "subscription_authentication", query: `subscription {\n  privateMessages {\n    id\n    from\n    content\n    isPrivate\n  }\n}`, variables: null, operationName: null, statusCode: 401, data: null, errors: null },
	{ name: "subscription_connection_params", query: `subscription {\n  secureStream {\n    id\n    data\n    timestamp\n  }\n}`, variables: null, operationName: null, statusCode: 101, data: { secureStream: { id: "stream-1", data: "Connection established", timestamp: "2025-12-27T14:00:00Z" } }, errors: null },
	{ name: "subscription_error", query: `subscription {\n  invalidSubscription {\n    id\n    data\n  }\n}`, variables: null, operationName: null, statusCode: 400, data: null, errors: null },
	{ name: "subscription_multiple_fields", query: `subscription MultiStream {\n  messageAdded {\n    id\n    text\n    author\n  }\n  userOnline {\n    userId\n    username\n    isOnline\n    lastSeen\n  }\n}`, variables: null, operationName: "MultiStream", statusCode: 200, data: { messageAdded: { id: "msg-101", text: "Hey everyone!", author: "alice" }, userOnline: { userId: "user-42", username: "bob", isOnline: true, lastSeen: "2025-12-27T13:00:00Z" } }, errors: null },
	{ name: "subscription_rate_limited", query: `subscription OnStockUpdate($symbol: String!) {\n  stockTicker(symbol: $symbol) {\n    id\n    symbol\n    price\n    change\n    changePercent\n    timestamp\n    volume\n  }\n}`, variables: { symbol: "AAPL" }, operationName: "OnStockUpdate", statusCode: 200, data: { stockTicker: { id: "stock-aapl-1", symbol: "AAPL", price: 238.45, change: 2.15, changePercent: 0.91, timestamp: "2025-12-27T17:00:00Z", volume: 52345678 } }, errors: null },
	{ name: "subscription_unsubscribe", query: `subscription OnTick {\n  ticker {\n    id\n    symbol\n    price\n    timestamp\n  }\n}`, variables: null, operationName: "OnTick", statusCode: 200, data: { ticker: { id: "tick-1", symbol: "AAPL", price: 195.45, timestamp: "2025-12-27T15:00:00Z" } }, errors: null },
	{ name: "subscription_with_auth_middleware", query: `subscription {\n  privateNotifications {\n    id\n    userId\n    type\n    message\n    priority\n    createdAt\n  }\n}`, variables: null, operationName: null, statusCode: 101, data: { privateNotifications: { id: "notif-456", userId: "user123", type: "ALERT", message: "Your subscription is about to expire", priority: "HIGH", createdAt: "2025-12-27T16:20:00Z" } }, errors: null },
	{ name: "subscription_with_filtering", query: `subscription OnPostUpdated($authorId: ID!, $statuses: [PostStatus!]!, $tagFilter: String, $scoreThreshold: Int) {\n  postUpdated(filter: {\n    authorId: $authorId\n    status: $statuses\n    tags_contains: $tagFilter\n    minScore: $scoreThreshold\n  }) {\n    id\n    title\n    authorId\n    content\n    status\n    tags\n    score\n    updatedAt\n  }\n}`, variables: { authorId: "123", statuses: ["PUBLISHED", "DRAFT"], tagFilter: "graphql", scoreThreshold: 50 }, operationName: "OnPostUpdated", statusCode: 200, data: { postUpdated: { id: "post-789", title: "Advanced GraphQL Patterns", authorId: "123", content: "A comprehensive guide to GraphQL subscriptions with advanced filtering techniques...", status: "PUBLISHED", tags: ["graphql", "subscriptions", "real-time"], score: 95, updatedAt: "2025-12-27T15:45:00Z" } }, errors: null },
	{ name: "subscription_with_variables", query: `subscription OnUserActivity($userId: ID!) {\n  userActivity(userId: $userId) {\n    id\n    userId\n    action\n    description\n    timestamp\n  }\n}`, variables: { userId: "user123" }, operationName: "OnUserActivity", statusCode: 200, data: { userActivity: { id: "event-789", userId: "user123", action: "LOGIN", description: "User logged in from browser", timestamp: "2025-12-27T12:15:00Z" } }, errors: null },
];

async function graphqlSubscription(requestJson: string): Promise<string> {
	const request = JSON.parse(requestJson);
	const fixture = findGraphqlFixture(graphqlSubscriptionFixtures, request.body ?? null, request.headers ?? undefined);
	const response: HandlerResponse = { status: fixture?.statusCode ?? 500 };
	response.body = fixture
		? { data: fixture.data ?? null, errors: fixture.errors ?? null }
		: { errors: [{ message: "GraphQL fixture not found" }] };
	return JSON.stringify(response);
}

export function createAppGraphqlSubscription(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/graphql",
		handler_name: "graphql_subscription",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			graphql_subscription: graphqlSubscription,
		},
	};
}


export async function handleGrpcFullAuthorizationContextWithRoleBasedAccessControl(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests complete authorization context including user roles, permissions, and resource-level access control.
  const responsePayload = Buffer.from(JSON.stringify({ authorized: true, message: "Access granted with admin privileges" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // x-user-id: "user-admin-001"
  // x-user-permissions: "read,write,delete"
  // content-type: "application/grpc"
  // authorization: "Bearer token123"
  // x-user-roles: "admin,editor"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcClientStreamingRpc(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests client streaming where client sends multiple messages. Covers streaming request aggregation patterns.
  const responsePayload = Buffer.from(JSON.stringify({ file_id: "file-12345", total_bytes: 57, status: "COMPLETED", checksum: "d8e8fca2dc0f896fd7cb4cb0031ba249" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcGrpcOkStatus0SuccessfulResponse(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests successful gRPC response with OK status code. Validates basic request-response completion.
  const responsePayload = Buffer.from(JSON.stringify({ request_id: "status-ok-001", status: "success" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcCustomAuthenticationSchemeHeader(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests custom authentication header scheme. Validates that custom auth headers are properly extracted and validated.
  const responsePayload = Buffer.from(JSON.stringify({ success: true }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // x-custom-auth: "CustomScheme token_value_123"
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcGrpcDataLossStatus15(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests DATA_LOSS gRPC status code. Returned when unrecoverable data loss or corruption occurred.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "DATA_LOSS"
  };
}

export async function handleGrpcMapFieldHandlingMapStringMessage(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests protobuf map fields with string keys and message values. Validates proper key-value pair serialization and access patterns.
  const responsePayload = Buffer.from(JSON.stringify({ id: "map-test-001", map_count: 3, keys: ["key1", "key2", "key3"] }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcWellKnownWrapperTypesStringvalueInt32valueEtc(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests usage of google.protobuf wrapper types (StringValue, Int32Value, BoolValue) for nullable scalar types. Validates proper null/present distinction.
  const responsePayload = Buffer.from(JSON.stringify({ id: "wrapper-test-001", name_present: true, name_value: "Test Name", count_present: true, count_value: 42 }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcGoogleProtobufAnyTypeUsage(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests usage of google.protobuf.Any for storing arbitrary message types. Validates type URL encoding and message packing.
  const responsePayload = Buffer.from(JSON.stringify({ request_id: "any-test-001", type_name: "example.v1.Container", success: true }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcGrpcUnauthenticatedStatus16AuthRequired(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests UNAUTHENTICATED gRPC status code. Returned when the request lacks valid authentication credentials.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "UNAUTHENTICATED"
  };
}

export async function handleGrpcGrpcPermissionDeniedStatus7(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests PERMISSION_DENIED gRPC status code. Returned when the caller does not have sufficient permissions.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "PERMISSION_DENIED"
  };
}

export async function handleGrpcGrpcFailedPreconditionStatus9(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests FAILED_PRECONDITION gRPC status code. Returned when the RPC failed because the system is not in the required state.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "FAILED_PRECONDITION"
  };
}

export async function handleGrpcSpecialCharactersUnicodeAndEmojiInStrings(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests handling of unicode characters, emojis, and special characters in protobuf string fields. Validates proper UTF-8 encoding/decoding.
  const responsePayload = Buffer.from(JSON.stringify({ echo: "Hello 世界 Привет שלום مرحبا" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcGrpcAbortedStatus10(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests ABORTED gRPC status code. Returned when an operation was aborted, typically due to a concurrency issue like conflict.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "ABORTED"
  };
}

export async function handleGrpcDeeplyNestedLargeStructure(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests deeply nested protobuf messages with complex hierarchies. Validates that nested message serialization handles proper field numbering and recursive structures.
  const responsePayload = Buffer.from(JSON.stringify({ success: true, person: { name: "John Doe", address: { street: "123 Main St", city: "Springfield" } } }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcGrpcUnavailableStatus14ServiceUnavailable(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests UNAVAILABLE gRPC status code. Returned when the service is temporarily unavailable.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "UNAVAILABLE"
  };
}

export async function handleGrpcTimestampAndDurationWellKnownTypes(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests usage of google.protobuf.Timestamp and Duration types. Validates RFC 3339 timestamp serialization and duration calculations.
  const responsePayload = Buffer.from(JSON.stringify({ event_id: "event-001", processed_at: "2024-01-15T10:31:45.123Z", processing_time_ms: 1000 }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcGrpcDeadlineExceededStatus4(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests DEADLINE_EXCEEDED gRPC status code. Returned when the RPC does not complete within the specified time limit.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "DEADLINE_EXCEEDED"
  };
}

export async function handleGrpcChunkedFileUploadWithClientStreaming(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests client streaming RPC for chunked file uploads. Validates that multiple message chunks are properly accumulated and processed by the server.
  const responsePayload = Buffer.from(JSON.stringify({ file_id: "chunked-upload-test", total_chunks: 5, total_size: 102400, upload_status: "completed" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcClientStreamingWithLargeBatchRequests(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests client streaming RPC with large batch requests. Validates server accumulation of multiple large client messages.
  const responsePayload = Buffer.from(JSON.stringify({ batch_id: "batch-large-001", items_processed: 100, total_bytes: 5242880 }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcMutualTlsMetadataSimulation(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests mutual TLS authentication by validating client certificate metadata. Simulates mTLS handshake verification.
  const responsePayload = Buffer.from(JSON.stringify({ verified: true, client_cn: "client.example.com" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // x-client-cert-cn: "client.example.com"
  // x-client-cert-fingerprint: "AB:CD:EF:12:34:56:78:90"
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcRepeatedFieldsArrays(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests arrays/repeated fields for primitive types and messages. Covers repeated field serialization and deserialization.
  const responsePayload = Buffer.from(JSON.stringify({ id: 789, title: "Getting Started with gRPC", content: "This is a comprehensive guide to gRPC...", tags: [{ id: 1, name: "gRPC" }, { id: 2, name: "Protocol Buffers" }, { id: 3, name: "RPC" }], categories: ["tutorial", "programming", "networking"] }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcGrpcNotFoundStatus5(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests NOT_FOUND gRPC status code. Returned when a requested resource (e.g., user, file) does not exist.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "NOT_FOUND"
  };
}

export async function handleGrpcProto3DefaultValueBehavior(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests how proto3 handles implicit default values. When fields are omitted from the request, response should reflect appropriate defaults.
  const responsePayload = Buffer.from(JSON.stringify({ id: 1, name: "", active: false, has_id: true }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcMaximumFieldNumber536870911(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests protobuf messages using the maximum allowed field number (536870911). Validates proper field number encoding in varint format.
  const responsePayload = Buffer.from(JSON.stringify({ id: 42, received_max: "Testing maximum field number" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcLargeBinaryDataInBytesField(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests handling of large binary data in protobuf bytes fields. Validates proper base64 encoding/decoding and preservation of binary integrity.
  const responsePayload = Buffer.from(JSON.stringify({ file_id: "binary-large-001", bytes_received: 512000 }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcCircuitBreakerTriggeredUnavailableWithMetadata(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests UNAVAILABLE status code with circuit breaker metadata. Indicates service degradation and when to retry.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "UNAVAILABLE"
  };
}

export async function handleGrpcServerStreamingRpc(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming where the server sends multiple responses. Covers streaming response patterns.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcGrpcResourceExhaustedStatus8(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests RESOURCE_EXHAUSTED gRPC status code. Returned when the server has run out of resources (disk space, memory, connections, etc.).
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "RESOURCE_EXHAUSTED"
  };
}

export async function handleGrpcLarge10mbMessagePayload(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests handling of 10MB protobuf messages. Validates high-capacity transfers, memory efficiency, and absence of stream fragmentation issues.
  const responsePayload = Buffer.from(JSON.stringify({ id: "bulk-10mb-transfer", status: "received" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcGrpcInvalidArgumentStatus3(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests INVALID_ARGUMENT gRPC status code. Indicates that the client provided an invalid or malformed argument.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "INVALID_ARGUMENT"
  };
}

export async function handleGrpcGrpcOutOfRangeStatus11(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests OUT_OF_RANGE gRPC status code. Returned when a value is outside the acceptable range.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OUT_OF_RANGE"
  };
}

export async function handleGrpcGrpcUnimplementedStatus12(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests UNIMPLEMENTED gRPC status code. Returned when the server does not implement the requested RPC method.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "UNIMPLEMENTED"
  };
}

export async function handleGrpcLargeRepeatedFieldWith10000Items(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests handling of repeated fields containing thousands of elements. Validates efficient serialization and deserialization of large arrays without memory bloat.
  const responsePayload = Buffer.from(JSON.stringify({ series_id: "metrics-large-series", point_count: 10000, min_value: 10.5, max_value: 99.9 }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcSimpleUnaryRpcGetuser(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests basic unary gRPC call with scalar types (int32, string). Covers fundamental request-response pattern.
  const responsePayload = Buffer.from(JSON.stringify({ id: 123, name: "Alice Johnson", email: "alice@example.com" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // authorization: "Bearer test-token"
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcJwtBearerTokenAuthentication(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests JWT authentication via gRPC metadata. Validates that JWT tokens are properly extracted and validated from authorization header.
  const responsePayload = Buffer.from(JSON.stringify({ user_id: "user-123", action: "read" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"
  // authorization: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyLTEyMyIsImlhdCI6MTUxNjIzOTAyMn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcBidirectionalStreamingFilterValidMessages(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests bidirectional streaming RPC where server filters out invalid messages during streaming.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcBidirectionalStreamingPingPongPairs(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests bidirectional streaming RPC with request-response pairs (ping-pong pattern).
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcBidirectionalStreamingChatConversation(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests bidirectional streaming RPC simulating a chat-like service with alternating messages.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcBidirectionalStreamingAsyncProcessingWithDelays(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests bidirectional streaming RPC with asynchronous message processing.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcBidirectionalStreamingBothStreamsEmpty(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests bidirectional streaming RPC where both request and response streams are empty.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcBidirectionalStreamingTransformToUppercase(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests bidirectional streaming RPC where server transforms incoming messages to uppercase.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcBidirectionalStreamingEcho5Messages(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests bidirectional streaming RPC where client sends 5 messages and expects them echoed back in the same order.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcBidirectionalStreamingLargeStreams(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests bidirectional streaming RPC with 50+ messages in both directions.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcBidirectionalStreamingErrorMidStream(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests bidirectional streaming RPC where server returns error after processing some messages.
  const responsePayload = Buffer.from(JSON.stringify("Error after processing 2 messages"));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "INTERNAL"
  };
}

export async function handleGrpcBidirectionalStreamingEmptyRequestStream(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests bidirectional streaming RPC with empty request stream but server sends response.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcServerStreamingUnicodeAndSpecialCharacters(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC with messages containing unicode characters, emoji, special symbols, and multi-byte UTF-8 sequences. Validates proper encoding/decoding across the streaming pipeline.
  const responsePayload = Buffer.from(JSON.stringify("Unicode stream completed successfully"));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"
  // encoding: "utf-8"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcServerStreamingEmptyStream(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC that returns an empty stream. The server opens the stream but sends no messages before completing successfully.
  const responsePayload = Buffer.from(JSON.stringify("Stream completed with no messages"));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcServerStreamingTimeoutScenario(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC that exceeds the deadline/timeout. The server starts streaming but doesn't complete before the client-imposed timeout expires. Validates proper timeout handling and stream cancellation.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // grpc-timeout: "1000m"
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "DEADLINE_EXCEEDED"
  };
}

export async function handleGrpcServerStreaming10Messages(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC that returns a normal stream of 10 messages. Validates message ordering and complete stream delivery.
  const responsePayload = Buffer.from(JSON.stringify("10 messages streamed successfully"));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcServerStreamingNestedObjectMessages(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC with complex nested message structures. Validates proper serialization and deserialization of deeply nested protobuf objects in streaming context.
  const responsePayload = Buffer.from(JSON.stringify("3 people with nested objects streamed successfully"));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcServerStreamingSingleMessage(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC that returns exactly one message. Verifies that single-message streams are properly handled and distinguished from unary responses.
  const responsePayload = Buffer.from(JSON.stringify("Stream completed with one message"));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcServerStreamingMidStreamError(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC that sends 5 messages successfully, then encounters an error before completing the stream. Validates partial stream delivery and error handling.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "INTERNAL"
  };
}

export async function handleGrpcServerStreamingWithMetadataAndTrailers(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC with gRPC metadata headers and trailers. Validates that metadata is accessible before streaming begins and trailers are delivered after stream completion.
  const responsePayload = Buffer.from(JSON.stringify("Stream completed with metadata and trailers"));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"
  // x-request-id: "metadata-stream-001"
  // x-client-version: "1.0.0"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcServerStreaming1mbMessages(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC with large message payloads (approximately 1MB each). Validates that the streaming framework can handle large individual messages without truncation or memory issues.
  const responsePayload = Buffer.from(JSON.stringify("3 large messages streamed successfully"));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcServerStreamingRapid100MessageStream(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC with 100 messages sent in rapid succession. Validates backpressure handling, buffering, and delivery of high-volume message streams without loss or corruption.
  const responsePayload = Buffer.from(JSON.stringify("100 messages streamed successfully in sequence"));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcErrorHandlingResourceNotFound(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests NOT_FOUND gRPC status code. Returned when the requested resource does not exist. Validates unary RPC requesting non-existent resource.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "NOT_FOUND"
  };
}

export async function handleGrpcErrorHandlingPermissionDeniedClientStreaming(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests client streaming RPC accessing unauthorized resource. Expects PERMISSION_DENIED status when client sends restricted access level requests. Demonstrates permission validation on streaming upload operations.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "PERMISSION_DENIED"
  };
}

export async function handleGrpcErrorHandlingUnimplementedMethod(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests unary RPC calling an unimplemented method. Validates that UNIMPLEMENTED status is returned when the server does not support the requested RPC method. This fixture ensures proper error handling for feature requests that are not yet available in the current server implementation.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "UNIMPLEMENTED"
  };
}

export async function handleGrpcErrorHandlingUnauthenticatedServerStreamingRequest(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC without required auth metadata. Expects UNAUTHENTICATED status when authorization header is missing.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "UNAUTHENTICATED"
  };
}

export async function handleGrpcErrorHandlingResourceExhausted(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests bidirectional streaming RPC exceeding rate limits. Expects RESOURCE_EXHAUSTED status when client attempts to send 100 messages in rapid succession, exceeding the 100 requests/second rate limit threshold.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "RESOURCE_EXHAUSTED"
  };
}

export async function handleGrpcErrorHandlingDeadlineExceeded(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC that exceeds deadline. Expects DEADLINE_EXCEEDED status when RPC time exceeds configured timeout.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "DEADLINE_EXCEEDED"
  };
}

export async function handleGrpcErrorHandlingStreamErrorMidTransmission(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC that errors after yielding 3 messages. The stream opens successfully and delivers 3 messages before encountering an INTERNAL error. Validates that partial stream data is delivered before the error is signaled.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "INTERNAL"
  };
}

export async function handleGrpcErrorHandlingInvalidRequestPayload(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC with invalid request payload. Validates that INVALID_ARGUMENT status is returned when required field is missing from the request message. The server should reject the malformed payload before beginning the stream.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "INVALID_ARGUMENT"
  };
}

export async function handleGrpcClientStreaming10MessagesSum(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests client streaming RPC where client sends 10 integer values. Server sums all values and returns result.
  const responsePayload = Buffer.from(JSON.stringify({ sequence_id: "seq-001", count: 10, sum: 550, status: "COMPLETE" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcClientStreamingLargeBatch100Messages(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests client streaming RPC with 100 messages in the stream. Validates performance with large batch aggregation.
  const responsePayload = Buffer.from(JSON.stringify({ batch_id: "batch-large-001", total_items: 100, total_value: 5050, average_value: 50.5, status: "PROCESSED" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcClientStreamingEarlyStreamClose(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests client streaming RPC where client closes stream after sending 3 messages instead of the expected 5. Server should gracefully handle partial stream.
  const responsePayload = Buffer.from(JSON.stringify({ session_id: "sess-early-001", received_chunks: 3, expected_chunks: 5, status: "INCOMPLETE" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcClientStreamingMessageSizeLimitExceeded(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests client streaming RPC where one message exceeds the max_message_size limit. Server rejects the oversized message and terminates the stream.
  const responsePayload = Buffer.from(JSON.stringify({ message_id: "payload-002", processed_count: 1, status: "FAILED", error_detail: "Message payload size 10240 exceeds maximum allowed size 4096" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // grpc-max-message-size: "4096"
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "RESOURCE_EXHAUSTED"
  };
}

export async function handleGrpcClientStreamingRapidHighFrequencyMessages(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests client streaming RPC with rapid-fire message delivery. Server handles 50 messages in quick succession and returns aggregated metrics.
  const responsePayload = Buffer.from(JSON.stringify({ event_id: "rapid-batch-001", event_count: 50, min_value: 0.1, max_value: 5.0, avg_value: 2.55, throughput_mps: 500.0, status: "PROCESSED" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcClientStreamingUnicodeStringAggregation(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests client streaming RPC with Unicode strings that are concatenated. Validates proper UTF-8 handling across multiple messages.
  const responsePayload = Buffer.from(JSON.stringify({ fragment_id: "unicode-001", result: "Hello, 世界! Привет 🌍", fragment_count: 4, total_length: 26, status: "CONCATENATED" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcClientStreamingMetadataPreservedInResponse(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests client streaming RPC where request metadata is forwarded to and preserved in the response. Validates metadata propagation through streaming pipeline.
  const responsePayload = Buffer.from(JSON.stringify({ request_id: "req-meta-001", processed_by: "grpc-handler-1", received_user_id: "user-789", message_count: 3, trace_id: "trace-abc456", status: "COMPLETE_WITH_METADATA" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // authorization: "Bearer token-xyz123"
  // custom-header: "custom-value"
  // x-user-id: "user-789"
  // x-trace-id: "trace-abc456"
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcClientStreamingSingleMessageAggregation(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests client streaming RPC where client sends a single message. Server acknowledges and returns aggregated result.
  const responsePayload = Buffer.from(JSON.stringify({ count: 1, total: 42, average: 42.0, status: "AGGREGATED" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcClientStreamingValidationFailureMidStream(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests client streaming RPC where a message fails validation in the middle of the stream. Server rejects the stream and returns error.
  const responsePayload = Buffer.from(JSON.stringify({ processed: 2, status: "VALIDATION_FAILED", error_message: "Invalid email format at message index 2: invalid-email" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "INVALID_ARGUMENT"
  };
}

export async function handleGrpcClientStreamingEmptyStreamReturnsDefault(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests client streaming RPC where client sends no messages (empty stream). Server gracefully handles empty input and returns default response.
  const responsePayload = Buffer.from(JSON.stringify({ request_id: "empty-stream-req", message_count: 0, result: "DEFAULT_RESULT", is_default: true }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcGrpcInternalStatus13ServerError(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests INTERNAL gRPC status code. Returned when an internal server error occurs.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "INTERNAL"
  };
}

export async function handleGrpcOptionalFields(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests optional field handling with presence semantics. Covers optional fields with and without values.
  const responsePayload = Buffer.from(JSON.stringify({ user_id: 42, username: "charlie_dev", bio: "Software engineer and gRPC enthusiast", updated_at: 1704067200000 }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcGrpcCancelledStatus1(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests CANCELLED gRPC status code. Returned when the RPC was cancelled by the client or server.
  const responsePayload = Buffer.from(JSON.stringify({ id: "cancel-001" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "CANCELLED"
  };
}

export async function handleGrpcBidirectionalStreamingRpc(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests bidirectional streaming where both client and server send multiple messages. Covers duplex communication patterns.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // authorization: "Bearer user-token"
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcEmptyMessageRequestAndResponse(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests handling of empty protobuf messages with no fields. Validates that the protocol correctly handles minimal payloads.
  const responsePayload = Buffer.from(JSON.stringify({  }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcGrpcUnknownStatus2(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests UNKNOWN gRPC status code. Used for errors that do not fit any other status code.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "UNKNOWN"
  };
}

export async function handleGrpcTimeoutWithRetryMetadata(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests DEADLINE_EXCEEDED status code with retry metadata in response trailers. Indicates whether client should retry.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "DEADLINE_EXCEEDED"
  };
}

export async function handleGrpcGrpcAlreadyExistsStatus6(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests ALREADY_EXISTS gRPC status code. Returned when trying to create a resource that already exists.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "ALREADY_EXISTS"
  };
}

export async function handleGrpcAllFieldsSetToZeroFalseEmptyValues(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests proto3 default value behavior when all fields are explicitly set to zero, false, empty string. Validates that zero values are transmitted correctly.
  const responsePayload = Buffer.from(JSON.stringify({ success: true, fields_received: 5 }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcOneofFieldHandling(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests oneof fields where only one field in the group can be set at a time. Validates proper mutual exclusivity and serialization.
  const responsePayload = Buffer.from(JSON.stringify({ received_type: "text_data", data_present: true }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcNestedMessages(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests nested message types with complex field hierarchies. Covers nested message definitions and serialization.
  const responsePayload = Buffer.from(JSON.stringify({ user_id: 456, name: "Bob Smith", email: "bob@example.com", address: { street: "123 Main St", city: "Springfield", zip_code: "12345" } }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcEnumTypes(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests enum definitions and serialization. Covers enum fields with named constants.
  const responsePayload = Buffer.from(JSON.stringify({ id: 1001, product_name: "Laptop", quantity: 2, status: "PENDING", priority: "HIGH" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcCorsRelatedMetadataHeaders(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests CORS-related metadata in gRPC calls. Validates origin validation and cross-origin request handling.
  const responsePayload = Buffer.from(JSON.stringify({ allowed: true }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // origin: "https://example.com"
  // content-type: "application/grpc"
  // access-control-request-method: "POST"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcErrorHandlingGrpcStatusCodes(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests gRPC error status codes and error responses. Covers NOT_FOUND, INVALID_ARGUMENT, INTERNAL, and other gRPC status codes.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "INVALID_ARGUMENT"
  };
}

export async function handleGrpcGrpcCompressionTestGzip(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests gRPC payload compression using gzip. Validates that compressed messages are properly decompressed and that header metadata indicates compression.
  const responsePayload = Buffer.from(JSON.stringify({ id: "compress-test-001", compressed: true }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // grpc-encoding: "gzip"
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcLarge1mbMessagePayload(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests handling of 1MB protobuf messages. Verifies that large payloads are properly serialized, transmitted, and deserialized without truncation or corruption.
  const responsePayload = Buffer.from(JSON.stringify({ request_id: "large-1mb-test-001", data_size: 1048576 }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcOauth2BearerTokenAuthentication(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests OAuth2 Bearer token authentication. Validates token validation and scope checking.
  const responsePayload = Buffer.from(JSON.stringify({ granted: true, token_info: "oauth2_token" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"
  // authorization: "Bearer ya29.a0AfH6SMBx..."

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcRateLimitingWithMetadataHeaders(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests gRPC rate limiting. Validates rate limit headers in response and proper 429 handling.
  const responsePayload = Buffer.from(JSON.stringify({ result: "success" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcApiKeyAuthentication(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests API key authentication via gRPC metadata. Validates that API keys are properly validated and associated with clients.
  const responsePayload = Buffer.from(JSON.stringify({ data: "resource_data", client_id: "client-api-001" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"
  // x-api-key: "sk_live_abc123def456"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcRequestIdForDistributedTracing(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests request ID header propagation for distributed tracing. Validates X-Request-ID generation and propagation.
  const responsePayload = Buffer.from(JSON.stringify({ request_id: "req-12345-67890" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"
  // x-request-id: "req-12345-67890"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcGrpcMetadataHeaders(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests gRPC metadata handling for request/response headers including authorization, tracing IDs, and custom headers.
  const responsePayload = Buffer.from(JSON.stringify({ request_id: "req-987654321", received_auth_header: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9", received_trace_id: "trace-abc123def456", received_custom_header: "custom-value", response_time_ms: 45 }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // x-trace-id: "trace-abc123def456"
  // authorization: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
  // content-type: "application/grpc"
  // x-custom-header: "custom-value"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcUserAgentAndClientInfoMetadata(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests User-Agent header handling and client identification. Validates proper user-agent parsing and logging.
  const responsePayload = Buffer.from(JSON.stringify({ client_type: "grpc-client", client_version: "1.2.3" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // user-agent: "grpc-client/1.2.3 (linux; amd64)"
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcBidirectionalStreamingWithLargePayloads(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests bidirectional streaming RPC with large messages in both directions. Validates concurrent read/write handling and proper message ordering.
  const responsePayload = Buffer.from(JSON.stringify({ message_id: "bi-large-001", sequence: 1, direction: "server-to-client" }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

export async function handleGrpcValidationErrorInvalidArgumentWithDetails(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests INVALID_ARGUMENT status code with detailed validation error information. Demonstrates how validation failures are communicated.
  const responsePayload = Buffer.from(JSON.stringify({}));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "INVALID_ARGUMENT"
  };
}

export async function handleGrpcServerStreamingWithLargeResponseData(request: GrpcRequest): Promise<GrpcResponse> {
  // Tests server streaming RPC that yields multiple large messages. Validates proper streaming protocol handling and backpressure management.
  const responsePayload = Buffer.from(JSON.stringify({ stream_id: "stream-large-001", chunk_number: 1, is_final: false }));
  const metadata: Record<string, string> = {};

  // Request metadata (headers)
  // content-type: "application/grpc"

  return {
    payload: responsePayload,
    metadata,
    statusCode: "OK"
  };
}

// App factory functions:
// - createAppRateLimitRateLimitBelowThresholdSucceeds() for rate_limit / Rate limit below threshold succeeds
// - createAppRateLimitRateLimitExceededReturns429() for rate_limit / Rate limit exceeded returns 429
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
// - createAppCompressionCompressionPayloadBelowMinSizeIsNotCompressed() for compression / Compression - payload below min_size is not compressed
// - createAppCompressionCompressionGzipApplied() for compression / Compression - gzip applied
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
// - createAppBackgroundBackgroundEventLoggingSecondPayload() for background / Background event logging - second payload
// - createAppBackgroundBackgroundEventLogging() for background / Background event logging
// - createAppRequestIdRequestIdHeaderIsPreserved() for request_id / Request ID header is preserved
// - createAppRequestIdRequestIdMiddlewareCanBeDisabled() for request_id / Request ID middleware can be disabled
// - createAppRequestIdRequestIdIsGeneratedWhenNotProvided() for request_id / Request ID is generated when not provided
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
// - createAppRequestTimeoutRequestExceedsTimeout() for request_timeout / Request exceeds timeout
// - createAppRequestTimeoutRequestCompletesBeforeTimeout() for request_timeout / Request completes before timeout
// - createAppBodyLimitsBodyUnderLimitSucceeds() for body_limits / Body under limit succeeds
// - createAppBodyLimitsBodyOverLimitReturns413() for body_limits / Body over limit returns 413
// - createAppStreamingStreamJsonLines() for streaming / Stream JSON lines
// - createAppStreamingBinaryLogDownload() for streaming / Binary log download
// - createAppStreamingChunkedCsvExport() for streaming / Chunked CSV export
// - createAppStaticFilesStaticFileServerReturnsTextFile() for static_files / Static file server returns text file
// - createAppStaticFilesStaticServerReturnsIndexHtmlForDirectory() for static_files / Static server returns index.html for directory
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
// - createAppGraphqlMutation() for graphql / mutation
// - createAppGraphqlQuery() for graphql / query
// - createAppGraphqlSubscription() for graphql / subscription

export {
	StatusUpdateMessageSchema,
	SystemAlertMessageSchema,
	UserNotificationMessageSchema,
	UserLeftMessageSchema,
	UserJoinedMessageSchema,
	ChatMessageMessageSchema,
	NotificationBatchMessageSchema,
	ChatAckMessageSchema,
};
export type {
	StatusUpdateMessage,
	SystemAlertMessage,
	UserNotificationMessage,
	UserLeftMessage,
	UserJoinedMessage,
	ChatMessageMessage,
	NotificationBatchMessage,
	ChatAckMessage,
};
