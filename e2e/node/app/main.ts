/**
 * Generated E2E test application with per-fixture app factories.
 * @generated
 */

import type { SpikardApp, RouteMetadata } from "@spikard/node";

/**
 * Handler for GET /items/{item_id}
 */
async function validationErrorsInvalidUuidFormat(itemId: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (itemId !== null && itemId !== undefined) {
		result["itemId"] = itemId;
	}
	return result;
}

export function createAppValidationErrorsInvalidUuidFormat(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/{item_id}",
		handler_name: "validation_errors_invalid_uuid_format",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"format":"uuid","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_uuid_format: validation_errors_invalid_uuid_format,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsInvalidBooleanValue(isActive: boolean, q: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (isActive !== null && isActive !== undefined) {
		result["isActive"] = isActive;
	}
	if (q !== null && q !== undefined) {
		result["q"] = q;
	}
	return result;
}

export function createAppValidationErrorsInvalidBooleanValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_invalid_boolean_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"is_active":{"type":"boolean"},"q":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_boolean_value: validation_errors_invalid_boolean_value,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsMissingRequiredQueryParameter(q: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (q !== null && q !== undefined) {
		result["q"] = q;
	}
	return result;
}

export function createAppValidationErrorsMissingRequiredQueryParameter(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_missing_required_query_parameter",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"q":{"required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_missing_required_query_parameter: validation_errors_missing_required_query_parameter,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsArrayMaxItemsConstraintViolation(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppValidationErrorsArrayMaxItemsConstraintViolation(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_array_max_items_constraint_violation",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"},"tags":{"items":{"type":"string"},"maxItems":10,"type":"array"}},"required":["name","price","tags"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_array_max_items_constraint_violation: validation_errors_array_max_items_constraint_violation,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsNumericConstraintViolationGtGreaterThan(price: number, q: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (price !== null && price !== undefined) {
		result["price"] = price;
	}
	if (q !== null && q !== undefined) {
		result["q"] = q;
	}
	return result;
}

export function createAppValidationErrorsNumericConstraintViolationGtGreaterThan(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_numeric_constraint_violation_gt_greater_than",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"price":{"exclusiveMinimum":0,"type":"number"},"q":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_numeric_constraint_violation_gt_greater_than: validation_errors_numeric_constraint_violation_gt_greater_than,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsStringRegexPatternMismatch(q: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (q !== null && q !== undefined) {
		result["q"] = q;
	}
	return result;
}

export function createAppValidationErrorsStringRegexPatternMismatch(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_string_regex_pattern_mismatch",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"q":{"pattern":"^[a-zA-Z0-9_-]+$","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_string_regex_pattern_mismatch: validation_errors_string_regex_pattern_mismatch,
		},
	};
}

/**
 * Handler for GET /models/{model_name}
 */
async function validationErrorsInvalidEnumValue(modelName: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (modelName !== null && modelName !== undefined) {
		result["modelName"] = modelName;
	}
	return result;
}

export function createAppValidationErrorsInvalidEnumValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/models/{model_name}",
		handler_name: "validation_errors_invalid_enum_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"model_name":{"enum":["alexnet","resnet","lenet"],"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_enum_value: validation_errors_invalid_enum_value,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsStringMinLengthConstraintViolation(q: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (q !== null && q !== undefined) {
		result["q"] = q;
	}
	return result;
}

export function createAppValidationErrorsStringMinLengthConstraintViolation(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_string_min_length_constraint_violation",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"q":{"minLength":3,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_string_min_length_constraint_violation: validation_errors_string_min_length_constraint_violation,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsMultipleValidationErrors(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppValidationErrorsMultipleValidationErrors(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_multiple_validation_errors",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"integer"},"quantity":{"type":"integer"}},"required":["name","price","quantity"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_multiple_validation_errors: validation_errors_multiple_validation_errors,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsStringMaxLengthConstraintViolation(q: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (q !== null && q !== undefined) {
		result["q"] = q;
	}
	return result;
}

export function createAppValidationErrorsStringMaxLengthConstraintViolation(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_string_max_length_constraint_violation",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"q":{"maxLength":50,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_string_max_length_constraint_violation: validation_errors_string_max_length_constraint_violation,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsNestedObjectValidationError(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppValidationErrorsNestedObjectValidationError(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_nested_object_validation_error",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"},"seller":{"additionalProperties":false,"properties":{"address":{"additionalProperties":false,"properties":{"city":{"minLength":3,"type":"string"},"zip_code":{"type":"string"}},"required":["city","zip_code"],"type":"object"},"name":{"type":"string"}},"required":["name","address"],"type":"object"}},"required":["name","price","seller"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_nested_object_validation_error: validation_errors_nested_object_validation_error,
		},
	};
}

/**
 * Handler for POST /profiles
 */
async function validationErrors10NestedErrorPath(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppValidationErrors10NestedErrorPath(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/profiles",
		handler_name: "validation_errors_10_nested_error_path",
		request_schema: {"properties":{"profile":{"properties":{"contact":{"properties":{"email":{"format":"email","type":"string"}},"required":["email"],"type":"object"}},"required":["contact"],"type":"object"}},"required":["profile"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_10_nested_error_path: validation_errors_10_nested_error_path,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsInvalidDatetimeFormat(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppValidationErrorsInvalidDatetimeFormat(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_invalid_datetime_format",
		request_schema: {"additionalProperties":false,"properties":{"created_at":{"format":"date-time","type":"string"},"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price","created_at"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_invalid_datetime_format: validation_errors_invalid_datetime_format,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsArrayItemValidationError(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppValidationErrorsArrayItemValidationError(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_array_item_validation_error",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"},"tags":{"items":{"type":"string"},"type":"array"}},"required":["name","price","tags"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_array_item_validation_error: validation_errors_array_item_validation_error,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsMissingRequiredBodyField(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppValidationErrorsMissingRequiredBodyField(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_missing_required_body_field",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"string"}},"required":["name","price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_missing_required_body_field: validation_errors_missing_required_body_field,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsBodyFieldTypeErrorStringForFloat(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppValidationErrorsBodyFieldTypeErrorStringForFloat(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_body_field_type_error_string_for_float",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_body_field_type_error_string_for_float: validation_errors_body_field_type_error_string_for_float,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsMalformedJsonBody(body: any): Promise<any> {
	return {"detail":"Invalid request format"};
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
			validation_errors_malformed_json_body: validation_errors_malformed_json_body,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsQueryParamTypeErrorStringProvidedForInt(q: string, skip: number): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (q !== null && q !== undefined) {
		result["q"] = q;
	}
	if (skip !== null && skip !== undefined) {
		result["skip"] = skip;
	}
	return result;
}

export function createAppValidationErrorsQueryParamTypeErrorStringProvidedForInt(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_query_param_type_error_string_provided_for_int",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"q":{"type":"string"},"skip":{"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_query_param_type_error_string_provided_for_int: validation_errors_query_param_type_error_string_provided_for_int,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsHeaderValidationError(q: string, xToken: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (q !== null && q !== undefined) {
		result["q"] = q;
	}
	if (xToken !== null && xToken !== undefined) {
		result["xToken"] = xToken;
	}
	return result;
}

export function createAppValidationErrorsHeaderValidationError(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_header_validation_error",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"x-token":{"required":true,"type":"string"}},"query":{"q":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_header_validation_error: validation_errors_header_validation_error,
		},
	};
}

/**
 * Handler for POST /users
 */
async function validationErrors09MultipleValidationErrors(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppValidationErrors09MultipleValidationErrors(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "validation_errors_09_multiple_validation_errors",
		request_schema: {"properties":{"age":{"minimum":18,"type":"integer"},"email":{"format":"email","type":"string"},"name":{"minLength":3,"type":"string"}},"required":["name","email","age"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_09_multiple_validation_errors: validation_errors_09_multiple_validation_errors,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function validationErrorsNumericConstraintViolationLeLessThanOrEqual(limit: number, q: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (limit !== null && limit !== undefined) {
		result["limit"] = limit;
	}
	if (q !== null && q !== undefined) {
		result["q"] = q;
	}
	return result;
}

export function createAppValidationErrorsNumericConstraintViolationLeLessThanOrEqual(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "validation_errors_numeric_constraint_violation_le_less_than_or_equal",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"limit":{"maximum":100,"type":"integer"},"q":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_numeric_constraint_violation_le_less_than_or_equal: validation_errors_numeric_constraint_violation_le_less_than_or_equal,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function validationErrorsArrayMinItemsConstraintViolation(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppValidationErrorsArrayMinItemsConstraintViolation(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "validation_errors_array_min_items_constraint_violation",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"},"tags":{"items":{},"minItems":1,"type":"array"}},"required":["name","price","tags"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			validation_errors_array_min_items_constraint_violation: validation_errors_array_min_items_constraint_violation,
		},
	};
}

/**
 * Handler for OPTIONS /api/data
 */
async function cors07CorsPreflightHeaderNotAllowed(accessControlRequestHeaders: string, accessControlRequestMethod: string, origin: string): Promise<any> {
	return { status: 403 };
}

export function createAppCors07CorsPreflightHeaderNotAllowed(): SpikardApp {
	const route: RouteMetadata = {
		method: "OPTIONS",
		path: "/api/data",
		handler_name: "cors_07_cors_preflight_header_not_allowed",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Access-Control-Request-Headers":{"optional":true,"type":"string"},"Access-Control-Request-Method":{"optional":true,"type":"string"},"Origin":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_07_cors_preflight_header_not_allowed: cors_07_cors_preflight_header_not_allowed,
		},
	};
}

/**
 * Handler for OPTIONS /items/
 */
async function corsCorsPreflightRequest(): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	return result;
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
			cors_cors_preflight_request: cors_cors_preflight_request,
		},
	};
}

/**
 * Handler for GET /api/user/profile
 */
async function corsCorsWithCredentials(): Promise<any> {
	return {"username":"john"};
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
			cors_cors_with_credentials: cors_cors_with_credentials,
		},
	};
}

/**
 * Handler for OPTIONS /api/data
 */
async function cors08CorsMaxAge(accessControlRequestHeaders: string, accessControlRequestMethod: string, origin: string): Promise<any> {
	return { status: 204 };
}

export function createAppCors08CorsMaxAge(): SpikardApp {
	const route: RouteMetadata = {
		method: "OPTIONS",
		path: "/api/data",
		handler_name: "cors_08_cors_max_age",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Access-Control-Request-Headers":{"optional":true,"type":"string"},"Access-Control-Request-Method":{"optional":true,"type":"string"},"Origin":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_08_cors_max_age: cors_08_cors_max_age,
		},
	};
}

/**
 * Handler for GET /api/data
 */
async function cors10CorsOriginNull(origin: string): Promise<any> {
	return {"error":"Origin 'null' is not allowed"};
}

export function createAppCors10CorsOriginNull(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "cors_10_cors_origin_null",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Origin":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_10_cors_origin_null: cors_10_cors_origin_null,
		},
	};
}

/**
 * Handler for GET /public/data
 */
async function corsCorsWildcardOrigin(): Promise<any> {
	return {"data":"public"};
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
			cors_cors_wildcard_origin: cors_cors_wildcard_origin,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function corsCorsRequestBlocked(origin: string): Promise<any> {
	return {"detail":"CORS request from origin 'https://malicious-site.com' not allowed"};
}

export function createAppCorsCorsRequestBlocked(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "cors_cors_request_blocked",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Origin":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_cors_request_blocked: cors_cors_request_blocked,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function corsSimpleCorsRequest(): Promise<any> {
	return {"items":[]};
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
			cors_simple_cors_request: cors_simple_cors_request,
		},
	};
}

/**
 * Handler for GET /api/data
 */
async function cors09CorsExposeHeaders(origin: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (origin !== null && origin !== undefined) {
		result["origin"] = origin;
	}
	return result;
}

export function createAppCors09CorsExposeHeaders(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "cors_09_cors_expose_headers",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Origin":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_09_cors_expose_headers: cors_09_cors_expose_headers,
		},
	};
}

/**
 * Handler for OPTIONS /api/data
 */
async function cors06CorsPreflightMethodNotAllowed(accessControlRequestHeaders: string, accessControlRequestMethod: string, origin: string): Promise<any> {
	return { status: 403 };
}

export function createAppCors06CorsPreflightMethodNotAllowed(): SpikardApp {
	const route: RouteMetadata = {
		method: "OPTIONS",
		path: "/api/data",
		handler_name: "cors_06_cors_preflight_method_not_allowed",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Access-Control-Request-Headers":{"optional":true,"type":"string"},"Access-Control-Request-Method":{"optional":true,"type":"string"},"Origin":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cors_06_cors_preflight_method_not_allowed: cors_06_cors_preflight_method_not_allowed,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesUuidFieldInvalidFormat(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodiesUuidFieldInvalidFormat(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_uuid_field_invalid_format",
		request_schema: {"additionalProperties":false,"properties":{"item_id":{"format":"uuid","type":"string"},"name":{"type":"string"}},"required":["name","item_id"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_uuid_field_invalid_format: json_bodies_uuid_field_invalid_format,
		},
	};
}

/**
 * Handler for POST /api/v1/data
 */
async function jsonBodies44ConstValidationFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodies44ConstValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/api/v1/data",
		handler_name: "json_bodies_44_const_validation_failure",
		request_schema: {"properties":{"data":{"type":"string"},"version":{"const":"1.0","type":"string"}},"required":["version","data"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_44_const_validation_failure: json_bodies_44_const_validation_failure,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesBooleanFieldSuccess(body: any): Promise<any> {
	return {"in_stock":true,"name":"Item","price":42.0};
}

export function createAppJsonBodiesBooleanFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_boolean_field_success",
		request_schema: {"additionalProperties":false,"properties":{"in_stock":{"type":"boolean"},"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price","in_stock"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_boolean_field_success: json_bodies_boolean_field_success,
		},
	};
}

/**
 * Handler for POST /items/validated
 */
async function jsonBodiesNumericLeValidationSuccess(body: any): Promise<any> {
	return {"name":"Item","price":100.0};
}

export function createAppJsonBodiesNumericLeValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_numeric_le_validation_success",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_numeric_le_validation_success: json_bodies_numeric_le_validation_success,
		},
	};
}

/**
 * Handler for POST /items/nested
 */
async function jsonBodiesDeeplyNestedObjects(body: any): Promise<any> {
	return {"name":"Product","price":100.0,"seller":{"address":{"city":"Springfield","country":{"code":"US","name":"USA"},"street":"123 Main St"},"name":"John Doe"}};
}

export function createAppJsonBodiesDeeplyNestedObjects(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/nested",
		handler_name: "json_bodies_deeply_nested_objects",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"},"seller":{"additionalProperties":false,"properties":{"address":{"additionalProperties":false,"properties":{"city":{"type":"string"},"country":{"additionalProperties":false,"properties":{"code":{"type":"string"},"name":{"type":"string"}},"required":["name","code"],"type":"object"},"street":{"type":"string"}},"required":["street","city","country"],"type":"object"},"name":{"type":"string"}},"required":["name","address"],"type":"object"}},"required":["name","price","seller"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_deeply_nested_objects: json_bodies_deeply_nested_objects,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesOptionalFieldsOmitted(body: any): Promise<any> {
	return {"description":null,"name":"Foo","price":35.4,"tax":null};
}

export function createAppJsonBodiesOptionalFieldsOmitted(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_optional_fields_omitted",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_optional_fields_omitted: json_bodies_optional_fields_omitted,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesUuidFieldSuccess(body: any): Promise<any> {
	return {"item_id":"c892496f-b1fd-4b91-bdb8-b46f92df1716","name":"Item"};
}

export function createAppJsonBodiesUuidFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_uuid_field_success",
		request_schema: {"additionalProperties":false,"properties":{"item_id":{"format":"uuid","type":"string"},"name":{"type":"string"}},"required":["name","item_id"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_uuid_field_success: json_bodies_uuid_field_success,
		},
	};
}

/**
 * Handler for POST /events/
 */
async function jsonBodiesDateFieldSuccess(body: any): Promise<any> {
	return {"event_date":"2024-03-15","name":"Conference"};
}

export function createAppJsonBodiesDateFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/events/",
		handler_name: "json_bodies_date_field_success",
		request_schema: {"additionalProperties":false,"properties":{"event_date":{"type":"string"},"name":{"type":"string"}},"required":["name","event_date"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_date_field_success: json_bodies_date_field_success,
		},
	};
}

/**
 * Handler for POST /config
 */
async function jsonBodies47MaxpropertiesValidationFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodies47MaxpropertiesValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/config",
		handler_name: "json_bodies_47_maxproperties_validation_failure",
		request_schema: {"maxProperties":3,"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_47_maxproperties_validation_failure: json_bodies_47_maxproperties_validation_failure,
		},
	};
}

/**
 * Handler for POST /config
 */
async function jsonBodies46MinpropertiesValidationFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodies46MinpropertiesValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/config",
		handler_name: "json_bodies_46_minproperties_validation_failure",
		request_schema: {"minProperties":2,"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_46_minproperties_validation_failure: json_bodies_46_minproperties_validation_failure,
		},
	};
}

/**
 * Handler for POST /items/validated
 */
async function jsonBodiesStringMinLengthValidationFail(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodiesStringMinLengthValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_string_min_length_validation_fail",
		request_schema: {"additionalProperties":false,"properties":{"name":{"minLength":3,"type":"string"},"price":{"type":"number"}},"required":["name","price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_string_min_length_validation_fail: json_bodies_string_min_length_validation_fail,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesFieldTypeValidationInvalidType(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodiesFieldTypeValidationInvalidType(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_field_type_validation_invalid_type",
		request_schema: {"additionalProperties":false,"properties":{"description":{"type":"string"},"name":{"type":"string"},"price":{"type":"number"},"tax":{"type":"number"}},"required":["name","description","price","tax"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_field_type_validation_invalid_type: json_bodies_field_type_validation_invalid_type,
		},
	};
}

/**
 * Handler for POST /payment
 */
async function jsonBodies36OneofSchemaMultipleMatchFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodies36OneofSchemaMultipleMatchFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/payment",
		handler_name: "json_bodies_36_oneof_schema_multiple_match_failure",
		request_schema: {"oneOf":[{"properties":{"credit_card":{"pattern":"^[0-9]{16}$","type":"string"}},"required":["credit_card"],"type":"object"},{"properties":{"paypal_email":{"format":"email","type":"string"}},"required":["paypal_email"],"type":"object"}]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_36_oneof_schema_multiple_match_failure: json_bodies_36_oneof_schema_multiple_match_failure,
		},
	};
}

/**
 * Handler for POST /items/nested
 */
async function jsonBodiesNestedObjectSuccess(body: any): Promise<any> {
	return {"image":{"name":"Product Image","url":"https://example.com/image.jpg"},"name":"Foo","price":42.0};
}

export function createAppJsonBodiesNestedObjectSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/nested",
		handler_name: "json_bodies_nested_object_success",
		request_schema: {"additionalProperties":false,"properties":{"image":{"additionalProperties":false,"properties":{"name":{"type":"string"},"url":{"type":"string"}},"required":["url","name"],"type":"object"},"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price","image"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_nested_object_success: json_bodies_nested_object_success,
		},
	};
}

/**
 * Handler for POST /users
 */
async function jsonBodies41NotSchemaSuccess(body: any): Promise<any> {
	return { status: 201 };
}

export function createAppJsonBodies41NotSchemaSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_41_not_schema_success",
		request_schema: {"properties":{"username":{"not":{"enum":["admin","root","system"]},"type":"string"}},"required":["username"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_41_not_schema_success: json_bodies_41_not_schema_success,
		},
	};
}

/**
 * Handler for POST /items/validated
 */
async function jsonBodiesStringMaxLengthValidationFail(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodiesStringMaxLengthValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_string_max_length_validation_fail",
		request_schema: {"additionalProperties":false,"properties":{"name":{"maxLength":50,"type":"string"},"price":{"type":"number"}},"required":["name","price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_string_max_length_validation_fail: json_bodies_string_max_length_validation_fail,
		},
	};
}

/**
 * Handler for POST /data
 */
async function jsonBodies50DeepNesting4Levels(body: any): Promise<any> {
	return { status: 201 };
}

export function createAppJsonBodies50DeepNesting4Levels(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "json_bodies_50_deep_nesting_4_levels",
		request_schema: {"properties":{"user":{"properties":{"profile":{"properties":{"contact":{"properties":{"address":{"properties":{"street":{"type":"string"}},"required":["street"],"type":"object"}},"required":["address"],"type":"object"}},"required":["contact"],"type":"object"}},"required":["profile"],"type":"object"}},"required":["user"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_50_deep_nesting_4_levels: json_bodies_50_deep_nesting_4_levels,
		},
	};
}

/**
 * Handler for POST /billing
 */
async function jsonBodies48DependenciesValidationSuccess(body: any): Promise<any> {
	return { status: 201 };
}

export function createAppJsonBodies48DependenciesValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/billing",
		handler_name: "json_bodies_48_dependencies_validation_success",
		request_schema: {"dependencies":{"credit_card":["billing_address"]},"properties":{"billing_address":{"type":"string"},"credit_card":{"type":"string"},"name":{"type":"string"}},"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_48_dependencies_validation_success: json_bodies_48_dependencies_validation_success,
		},
	};
}

/**
 * Handler for PATCH /items/{id}
 */
async function jsonBodiesPatchPartialUpdate(body: any, id: string): Promise<any> {
	return {"description":"Original description","name":"Original Item","price":45.0};
}

export function createAppJsonBodiesPatchPartialUpdate(): SpikardApp {
	const route: RouteMetadata = {
		method: "PATCH",
		path: "/items/{id}",
		handler_name: "json_bodies_patch_partial_update",
		request_schema: {"properties":{"price":{"type":"number"}},"required":["price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_patch_partial_update: json_bodies_patch_partial_update,
		},
	};
}

/**
 * Handler for POST /users
 */
async function jsonBodies30NestedObjectMissingField(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodies30NestedObjectMissingField(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_30_nested_object_missing_field",
		request_schema: {"properties":{"profile":{"properties":{"email":{"format":"email","type":"string"},"name":{"minLength":1,"type":"string"}},"required":["name","email"],"type":"object"}},"required":["profile"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_30_nested_object_missing_field: json_bodies_30_nested_object_missing_field,
		},
	};
}

/**
 * Handler for POST /events/
 */
async function jsonBodiesDatetimeFieldSuccess(body: any): Promise<any> {
	return {"created_at":"2024-03-15T10:30:00Z","name":"Meeting"};
}

export function createAppJsonBodiesDatetimeFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/events/",
		handler_name: "json_bodies_datetime_field_success",
		request_schema: {"additionalProperties":false,"properties":{"created_at":{"format":"date-time","type":"string"},"name":{"type":"string"}},"required":["name","created_at"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_datetime_field_success: json_bodies_datetime_field_success,
		},
	};
}

/**
 * Handler for POST /items/validated
 */
async function jsonBodiesStringPatternValidationSuccess(body: any): Promise<any> {
	return {"name":"Item","sku":"ABC1234"};
}

export function createAppJsonBodiesStringPatternValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_string_pattern_validation_success",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"sku":{"type":"string"}},"required":["name","sku"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_string_pattern_validation_success: json_bodies_string_pattern_validation_success,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesExtraFieldsIgnoredNoAdditionalproperties(body: any): Promise<any> {
	return {"name":"Item","price":42.0};
}

export function createAppJsonBodiesExtraFieldsIgnoredNoAdditionalproperties(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_extra_fields_ignored_no_additionalproperties",
		request_schema: {"additionalProperties":false,"properties":{"another_extra":{"type":"integer"},"extra_field":{"type":"string"},"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price","extra_field","another_extra"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_extra_fields_ignored_no_additionalproperties: json_bodies_extra_fields_ignored_no_additionalproperties,
		},
	};
}

/**
 * Handler for POST /contact
 */
async function jsonBodies40AnyofSchemaFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodies40AnyofSchemaFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/contact",
		handler_name: "json_bodies_40_anyof_schema_failure",
		request_schema: {"anyOf":[{"required":["email"]},{"required":["phone"]}],"properties":{"email":{"format":"email","type":"string"},"name":{"type":"string"},"phone":{"type":"string"}},"required":["name"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_40_anyof_schema_failure: json_bodies_40_anyof_schema_failure,
		},
	};
}

/**
 * Handler for POST /contact
 */
async function jsonBodies39AnyofSchemaMultipleMatchSuccess(body: any): Promise<any> {
	return { status: 201 };
}

export function createAppJsonBodies39AnyofSchemaMultipleMatchSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/contact",
		handler_name: "json_bodies_39_anyof_schema_multiple_match_success",
		request_schema: {"anyOf":[{"required":["email"]},{"required":["phone"]}],"properties":{"email":{"format":"email","type":"string"},"name":{"type":"string"},"phone":{"type":"string"}},"required":["name"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_39_anyof_schema_multiple_match_success: json_bodies_39_anyof_schema_multiple_match_success,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesArrayOfPrimitiveValues(body: any): Promise<any> {
	return {"name":"Product","ratings":[4.5,4.8,5.0,4.2],"tags":["electronics","gadget","new"]};
}

export function createAppJsonBodiesArrayOfPrimitiveValues(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_array_of_primitive_values",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"ratings":{"items":{"type":"number"},"type":"array"},"tags":{"items":{"type":"string"},"type":"array"}},"required":["name","tags","ratings"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_array_of_primitive_values: json_bodies_array_of_primitive_values,
		},
	};
}

/**
 * Handler for POST /items/validated
 */
async function jsonBodiesNumericGeValidationFail(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodiesNumericGeValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_numeric_ge_validation_fail",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"minimum":1,"type":"number"}},"required":["name","price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_numeric_ge_validation_fail: json_bodies_numeric_ge_validation_fail,
		},
	};
}

/**
 * Handler for POST /payment
 */
async function jsonBodies37OneofSchemaNoMatchFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodies37OneofSchemaNoMatchFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/payment",
		handler_name: "json_bodies_37_oneof_schema_no_match_failure",
		request_schema: {"oneOf":[{"properties":{"credit_card":{"pattern":"^[0-9]{16}$","type":"string"}},"required":["credit_card"],"type":"object"},{"properties":{"paypal_email":{"format":"email","type":"string"}},"required":["paypal_email"],"type":"object"}]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_37_oneof_schema_no_match_failure: json_bodies_37_oneof_schema_no_match_failure,
		},
	};
}

/**
 * Handler for POST /items/list-validated
 */
async function jsonBodiesEmptyArrayValidationFail(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodiesEmptyArrayValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/list-validated",
		handler_name: "json_bodies_empty_array_validation_fail",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"tags":{"items":{},"minItems":1,"type":"array"}},"required":["name","tags"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_empty_array_validation_fail: json_bodies_empty_array_validation_fail,
		},
	};
}

/**
 * Handler for POST /contact
 */
async function jsonBodies38AnyofSchemaSuccess(body: any): Promise<any> {
	return { status: 201 };
}

export function createAppJsonBodies38AnyofSchemaSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/contact",
		handler_name: "json_bodies_38_anyof_schema_success",
		request_schema: {"anyOf":[{"required":["email"]},{"required":["phone"]}],"properties":{"name":{"type":"string"}},"required":["name"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_38_anyof_schema_success: json_bodies_38_anyof_schema_success,
		},
	};
}

/**
 * Handler for POST /items/optional-all
 */
async function jsonBodiesEmptyJsonObject(body: any): Promise<any> {
	return {"description":null,"name":null,"price":null,"tax":null};
}

export function createAppJsonBodiesEmptyJsonObject(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/optional-all",
		handler_name: "json_bodies_empty_json_object",
		request_schema: {"additionalProperties":false,"properties":{},"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_empty_json_object: json_bodies_empty_json_object,
		},
	};
}

/**
 * Handler for POST /items/validated
 */
async function jsonBodiesStringPatternValidationFail(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodiesStringPatternValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/validated",
		handler_name: "json_bodies_string_pattern_validation_fail",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"sku":{"pattern":"^[A-Z]{3}[0-9]{4}$","type":"string"}},"required":["name","sku"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_string_pattern_validation_fail: json_bodies_string_pattern_validation_fail,
		},
	};
}

/**
 * Handler for POST /billing
 */
async function jsonBodies49DependenciesValidationFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodies49DependenciesValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/billing",
		handler_name: "json_bodies_49_dependencies_validation_failure",
		request_schema: {"dependencies":{"credit_card":["billing_address"]},"properties":{"billing_address":{"type":"string"},"credit_card":{"type":"string"},"name":{"type":"string"}},"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_49_dependencies_validation_failure: json_bodies_49_dependencies_validation_failure,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesSimpleJsonObjectSuccess(body: any): Promise<any> {
	return {"description":"A very nice Item","name":"Foo","price":35.4,"tax":3.2};
}

export function createAppJsonBodiesSimpleJsonObjectSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_simple_json_object_success",
		request_schema: {"additionalProperties":false,"properties":{"description":{"type":"string"},"name":{"type":"string"},"price":{"type":"number"},"tax":{"type":"number"}},"required":["name","description","price","tax"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_simple_json_object_success: json_bodies_simple_json_object_success,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesRequiredFieldMissingValidationError(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodiesRequiredFieldMissingValidationError(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_required_field_missing_validation_error",
		request_schema: {"additionalProperties":false,"properties":{"description":{"type":"string"},"name":{"type":"string"},"price":{"type":"number"}},"required":["description","price","name"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_required_field_missing_validation_error: json_bodies_required_field_missing_validation_error,
		},
	};
}

/**
 * Handler for POST /payment
 */
async function jsonBodies35OneofSchemaSuccess(body: any): Promise<any> {
	return { status: 201 };
}

export function createAppJsonBodies35OneofSchemaSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/payment",
		handler_name: "json_bodies_35_oneof_schema_success",
		request_schema: {"oneOf":[{"properties":{"credit_card":{"pattern":"^[0-9]{16}$","type":"string"}},"required":["credit_card"],"type":"object"},{"properties":{"paypal_email":{"format":"email","type":"string"}},"required":["paypal_email"],"type":"object"}]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_35_oneof_schema_success: json_bodies_35_oneof_schema_success,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesEnumFieldInvalidValue(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodiesEnumFieldInvalidValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_enum_field_invalid_value",
		request_schema: {"additionalProperties":false,"properties":{"category":{"enum":["electronics","clothing","books"],"type":"string"},"name":{"type":"string"}},"required":["name","category"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_enum_field_invalid_value: json_bodies_enum_field_invalid_value,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesEnumFieldSuccess(body: any): Promise<any> {
	return {"category":"electronics","name":"Item"};
}

export function createAppJsonBodiesEnumFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_enum_field_success",
		request_schema: {"additionalProperties":false,"properties":{"category":{"type":"string"},"name":{"type":"string"}},"required":["name","category"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_enum_field_success: json_bodies_enum_field_success,
		},
	};
}

/**
 * Handler for POST /items
 */
async function jsonBodies33AllofSchemaComposition(body: any): Promise<any> {
	return { status: 201 };
}

export function createAppJsonBodies33AllofSchemaComposition(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items",
		handler_name: "json_bodies_33_allof_schema_composition",
		request_schema: {"allOf":[{"properties":{"name":{"type":"string"}},"required":["name"],"type":"object"},{"properties":{"price":{"minimum":0,"type":"number"}},"required":["price"],"type":"object"}]},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_33_allof_schema_composition: json_bodies_33_allof_schema_composition,
		},
	};
}

/**
 * Handler for POST /config
 */
async function jsonBodies45MinpropertiesValidationSuccess(body: any): Promise<any> {
	return { status: 201 };
}

export function createAppJsonBodies45MinpropertiesValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/config",
		handler_name: "json_bodies_45_minproperties_validation_success",
		request_schema: {"minProperties":2,"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_45_minproperties_validation_success: json_bodies_45_minproperties_validation_success,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesBodyWithQueryParameters(body: any, limit: number): Promise<any> {
	return {"item":{"name":"Item","price":42.0},"limit":10};
}

export function createAppJsonBodiesBodyWithQueryParameters(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_body_with_query_parameters",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: {"query":{"limit":{"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_body_with_query_parameters: json_bodies_body_with_query_parameters,
		},
	};
}

/**
 * Handler for POST /users
 */
async function jsonBodies42NotSchemaFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodies42NotSchemaFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_42_not_schema_failure",
		request_schema: {"properties":{"username":{"not":{"enum":["admin","root","system"]},"type":"string"}},"required":["username"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_42_not_schema_failure: json_bodies_42_not_schema_failure,
		},
	};
}

/**
 * Handler for POST /api/v1/data
 */
async function jsonBodies43ConstValidationSuccess(body: any): Promise<any> {
	return { status: 201 };
}

export function createAppJsonBodies43ConstValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/api/v1/data",
		handler_name: "json_bodies_43_const_validation_success",
		request_schema: {"properties":{"data":{"type":"string"},"version":{"const":"1.0","type":"string"}},"required":["version","data"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_43_const_validation_success: json_bodies_43_const_validation_success,
		},
	};
}

/**
 * Handler for POST /products
 */
async function jsonBodies32SchemaRefDefinitions(body: any): Promise<any> {
	return { status: 201 };
}

export function createAppJsonBodies32SchemaRefDefinitions(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/products",
		handler_name: "json_bodies_32_schema_ref_definitions",
		request_schema: {"definitions":{"Product":{"properties":{"name":{"type":"string"},"price":{"minimum":0,"type":"number"}},"required":["name","price"],"type":"object"}},"properties":{"product":{"$ref":"#/definitions/Product"}},"required":["product"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_32_schema_ref_definitions: json_bodies_32_schema_ref_definitions,
		},
	};
}

/**
 * Handler for POST /users
 */
async function jsonBodies29NestedObjectValidationSuccess(body: any): Promise<any> {
	return { status: 201 };
}

export function createAppJsonBodies29NestedObjectValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_29_nested_object_validation_success",
		request_schema: {"properties":{"profile":{"properties":{"email":{"format":"email","type":"string"},"name":{"minLength":1,"type":"string"}},"required":["name","email"],"type":"object"}},"required":["profile"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_29_nested_object_validation_success: json_bodies_29_nested_object_validation_success,
		},
	};
}

/**
 * Handler for POST /users
 */
async function jsonBodies34AdditionalPropertiesFalse(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppJsonBodies34AdditionalPropertiesFalse(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_34_additional_properties_false",
		request_schema: {"additionalProperties":false,"properties":{"email":{"type":"string"},"name":{"type":"string"}},"required":["name"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_34_additional_properties_false: json_bodies_34_additional_properties_false,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function jsonBodiesNullValueForOptionalField(body: any): Promise<any> {
	return {"description":null,"name":"Item","price":42.0,"tax":null};
}

export function createAppJsonBodiesNullValueForOptionalField(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "json_bodies_null_value_for_optional_field",
		request_schema: {"additionalProperties":false,"properties":{"description":{"type":"null"},"name":{"type":"string"},"price":{"type":"number"},"tax":{"type":"null"}},"required":["name","price","description","tax"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_null_value_for_optional_field: json_bodies_null_value_for_optional_field,
		},
	};
}

/**
 * Handler for POST /users
 */
async function jsonBodies31NullablePropertyNullValue(body: any): Promise<any> {
	return { status: 201 };
}

export function createAppJsonBodies31NullablePropertyNullValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "json_bodies_31_nullable_property_null_value",
		request_schema: {"properties":{"description":{"type":["string","null"]},"name":{"type":"string"}},"required":["name"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_31_nullable_property_null_value: json_bodies_31_nullable_property_null_value,
		},
	};
}

/**
 * Handler for POST /items/list
 */
async function jsonBodiesArrayOfObjectsSuccess(body: any): Promise<any> {
	return {"images":[{"name":"Front","url":"https://example.com/img1.jpg"},{"name":"Back","url":"https://example.com/img2.jpg"}],"name":"Product Bundle","tags":["electronics","gadget"]};
}

export function createAppJsonBodiesArrayOfObjectsSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/list",
		handler_name: "json_bodies_array_of_objects_success",
		request_schema: {"additionalProperties":false,"properties":{"images":{"items":{"additionalProperties":false,"properties":{"name":{"type":"string"},"url":{"type":"string"}},"required":["url","name"],"type":"object"},"type":"array"},"name":{"type":"string"},"tags":{"items":{"type":"string"},"type":"array"}},"required":["name","tags","images"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			json_bodies_array_of_objects_success: json_bodies_array_of_objects_success,
		},
	};
}

/**
 * Handler for GET /data
 */
async function cookies25CookieSamesiteLax(tracking: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (tracking !== null && tracking !== undefined) {
		result["tracking"] = tracking;
	}
	return result;
}

export function createAppCookies25CookieSamesiteLax(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/data",
		handler_name: "cookies_25_cookie_samesite_lax",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"tracking":{"required":true,"samesite":"Lax","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_25_cookie_samesite_lax: cookies_25_cookie_samesite_lax,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function cookiesOptionalCookieParameterSuccess(adsId: string): Promise<any> {
	return {"ads_id":"abc123"};
}

export function createAppCookiesOptionalCookieParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "cookies_optional_cookie_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"ads_id":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_optional_cookie_parameter_success: cookies_optional_cookie_parameter_success,
		},
	};
}

/**
 * Handler for GET /cookies/pattern
 */
async function cookiesCookieRegexPatternValidationFail(trackingId: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (trackingId !== null && trackingId !== undefined) {
		result["trackingId"] = trackingId;
	}
	return result;
}

export function createAppCookiesCookieRegexPatternValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/cookies/pattern",
		handler_name: "cookies_cookie_regex_pattern_validation_fail",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"tracking_id":{"pattern":"^[A-Z0-9]{8}$","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_regex_pattern_validation_fail: cookies_cookie_regex_pattern_validation_fail,
		},
	};
}

/**
 * Handler for POST /cookies/session
 */
async function cookiesResponseSessionCookieNoMaxAge(body: any): Promise<any> {
	return {"message":"Session cookie set"};
}

export function createAppCookiesResponseSessionCookieNoMaxAge(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/session",
		handler_name: "cookies_response_session_cookie_no_max_age",
		request_schema: {"additionalProperties":false,"properties":{"value":{"type":"string"}},"required":["value"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_session_cookie_no_max_age: cookies_response_session_cookie_no_max_age,
		},
	};
}

/**
 * Handler for GET /secure
 */
async function cookies27CookieHttponlyFlag(session: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (session !== null && session !== undefined) {
		result["session"] = session;
	}
	return result;
}

export function createAppCookies27CookieHttponlyFlag(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/secure",
		handler_name: "cookies_27_cookie_httponly_flag",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"session":{"httponly":true,"required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_27_cookie_httponly_flag: cookies_27_cookie_httponly_flag,
		},
	};
}

/**
 * Handler for GET /cookie/set
 */
async function cookiesResponseCookieWithAttributes(): Promise<any> {
	return {"message":"Cookie set"};
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
			cookies_response_cookie_with_attributes: cookies_response_cookie_with_attributes,
		},
	};
}

/**
 * Handler for GET /secure
 */
async function cookies24CookieSamesiteStrict(sessionId: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (sessionId !== null && sessionId !== undefined) {
		result["sessionId"] = sessionId;
	}
	return result;
}

export function createAppCookies24CookieSamesiteStrict(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/secure",
		handler_name: "cookies_24_cookie_samesite_strict",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"session_id":{"required":true,"samesite":"Strict","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_24_cookie_samesite_strict: cookies_24_cookie_samesite_strict,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function cookiesApikeyCookieAuthenticationSuccess(key: string): Promise<any> {
	return {"username":"secret"};
}

export function createAppCookiesApikeyCookieAuthenticationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "cookies_apikey_cookie_authentication_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"key":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_apikey_cookie_authentication_success: cookies_apikey_cookie_authentication_success,
		},
	};
}

/**
 * Handler for GET /cookies/min-length
 */
async function cookiesCookieValidationMinLengthConstraintSuccess(token: string): Promise<any> {
	return {"token":"abc"};
}

export function createAppCookiesCookieValidationMinLengthConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/cookies/min-length",
		handler_name: "cookies_cookie_validation_min_length_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"token":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_validation_min_length_constraint_success: cookies_cookie_validation_min_length_constraint_success,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function cookiesCookieValidationMinLengthFailure(trackingId: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (trackingId !== null && trackingId !== undefined) {
		result["trackingId"] = trackingId;
	}
	return result;
}

export function createAppCookiesCookieValidationMinLengthFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "cookies_cookie_validation_min_length_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"tracking_id":{"minLength":3,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_validation_min_length_failure: cookies_cookie_validation_min_length_failure,
		},
	};
}

/**
 * Handler for GET /cookies/validated
 */
async function cookiesCookieValidationMaxLengthConstraintFail(sessionId: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (sessionId !== null && sessionId !== undefined) {
		result["sessionId"] = sessionId;
	}
	return result;
}

export function createAppCookiesCookieValidationMaxLengthConstraintFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/cookies/validated",
		handler_name: "cookies_cookie_validation_max_length_constraint_fail",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"session_id":{"maxLength":20,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_validation_max_length_constraint_fail: cookies_cookie_validation_max_length_constraint_fail,
		},
	};
}

/**
 * Handler for GET /items/cookies
 */
async function cookiesRequiredCookieMissing(fatebookTracker: string, sessionId: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (fatebookTracker !== null && fatebookTracker !== undefined) {
		result["fatebookTracker"] = fatebookTracker;
	}
	if (sessionId !== null && sessionId !== undefined) {
		result["sessionId"] = sessionId;
	}
	return result;
}

export function createAppCookiesRequiredCookieMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/cookies",
		handler_name: "cookies_required_cookie_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"fatebook_tracker":{"optional":true,"type":"string"},"session_id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_required_cookie_missing: cookies_required_cookie_missing,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function cookiesOptionalCookieParameterMissing(adsId: string): Promise<any> {
	return {"ads_id":null};
}

export function createAppCookiesOptionalCookieParameterMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "cookies_optional_cookie_parameter_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"ads_id":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_optional_cookie_parameter_missing: cookies_optional_cookie_parameter_missing,
		},
	};
}

/**
 * Handler for GET /users/me/auth
 */
async function cookiesApikeyCookieAuthenticationMissing(key: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (key !== null && key !== undefined) {
		result["key"] = key;
	}
	return result;
}

export function createAppCookiesApikeyCookieAuthenticationMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me/auth",
		handler_name: "cookies_apikey_cookie_authentication_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"key":{"required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_apikey_cookie_authentication_missing: cookies_apikey_cookie_authentication_missing,
		},
	};
}

/**
 * Handler for POST /cookies/multiple
 */
async function cookiesResponseMultipleCookies(body: any): Promise<any> {
	return {"message":"Multiple cookies set"};
}

export function createAppCookiesResponseMultipleCookies(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/multiple",
		handler_name: "cookies_response_multiple_cookies",
		request_schema: {"additionalProperties":false,"properties":{"session":{"type":"string"},"user":{"type":"string"}},"required":["user","session"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_multiple_cookies: cookies_response_multiple_cookies,
		},
	};
}

/**
 * Handler for POST /cookies/samesite-lax
 */
async function cookiesResponseCookieWithSamesiteLax(body: any): Promise<any> {
	return {"message":"Cookie set with SameSite=Lax"};
}

export function createAppCookiesResponseCookieWithSamesiteLax(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/samesite-lax",
		handler_name: "cookies_response_cookie_with_samesite_lax",
		request_schema: {"additionalProperties":false,"properties":{"value":{"type":"string"}},"required":["value"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_samesite_lax: cookies_response_cookie_with_samesite_lax,
		},
	};
}

/**
 * Handler for POST /cookies/delete
 */
async function cookiesResponseDeleteCookie(session: string): Promise<any> {
	return {"message":"Cookie deleted"};
}

export function createAppCookiesResponseDeleteCookie(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/delete",
		handler_name: "cookies_response_delete_cookie",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"session":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_delete_cookie: cookies_response_delete_cookie,
		},
	};
}

/**
 * Handler for POST /cookies/set-with-path
 */
async function cookiesResponseCookieWithPathAttribute(body: any): Promise<any> {
	return {"message":"Cookie set with path"};
}

export function createAppCookiesResponseCookieWithPathAttribute(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/set-with-path",
		handler_name: "cookies_response_cookie_with_path_attribute",
		request_schema: {"additionalProperties":false,"properties":{"value":{"type":"string"}},"required":["value"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_path_attribute: cookies_response_cookie_with_path_attribute,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function cookiesOptionalApikeyCookieMissing(key: string): Promise<any> {
	return {"msg":"Create an account first"};
}

export function createAppCookiesOptionalApikeyCookieMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "cookies_optional_apikey_cookie_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"key":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_optional_apikey_cookie_missing: cookies_optional_apikey_cookie_missing,
		},
	};
}

/**
 * Handler for POST /cookies/samesite-strict
 */
async function cookiesResponseCookieWithSamesiteStrict(body: any): Promise<any> {
	return {"message":"Cookie set with SameSite=Strict"};
}

export function createAppCookiesResponseCookieWithSamesiteStrict(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/samesite-strict",
		handler_name: "cookies_response_cookie_with_samesite_strict",
		request_schema: {"additionalProperties":false,"properties":{"value":{"type":"string"}},"required":["value"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_samesite_strict: cookies_response_cookie_with_samesite_strict,
		},
	};
}

/**
 * Handler for POST /cookies/samesite-none
 */
async function cookiesResponseCookieWithSamesiteNone(body: any): Promise<any> {
	return {"message":"Cookie set with SameSite=None"};
}

export function createAppCookiesResponseCookieWithSamesiteNone(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/samesite-none",
		handler_name: "cookies_response_cookie_with_samesite_none",
		request_schema: {"additionalProperties":false,"properties":{"value":{"type":"string"}},"required":["value"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_samesite_none: cookies_response_cookie_with_samesite_none,
		},
	};
}

/**
 * Handler for GET /cookies/pattern
 */
async function cookiesCookieRegexPatternValidationSuccess(trackingId: string): Promise<any> {
	return {"tracking_id":"ABC12345"};
}

export function createAppCookiesCookieRegexPatternValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/cookies/pattern",
		handler_name: "cookies_cookie_regex_pattern_validation_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"tracking_id":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_cookie_regex_pattern_validation_success: cookies_cookie_regex_pattern_validation_success,
		},
	};
}

/**
 * Handler for POST /cookie/
 */
async function cookiesResponseSetCookieBasic(): Promise<any> {
	return {"message":"Come to the dark side, we have cookies"};
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
			cookies_response_set_cookie_basic: cookies_response_set_cookie_basic,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function cookiesMultipleCookiesSuccess(fatebookTracker: string, googallTracker: string, sessionId: string): Promise<any> {
	return {"fatebook_tracker":"tracker456","googall_tracker":"ga789","session_id":"session123"};
}

export function createAppCookiesMultipleCookiesSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "cookies_multiple_cookies_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"fatebook_tracker":{"optional":true,"type":"string"},"googall_tracker":{"optional":true,"type":"string"},"session_id":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_multiple_cookies_success: cookies_multiple_cookies_success,
		},
	};
}

/**
 * Handler for GET /secure
 */
async function cookies26CookieSecureFlag(authToken: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (authToken !== null && authToken !== undefined) {
		result["authToken"] = authToken;
	}
	return result;
}

export function createAppCookies26CookieSecureFlag(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/secure",
		handler_name: "cookies_26_cookie_secure_flag",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"cookies":{"auth_token":{"required":true,"secure":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_26_cookie_secure_flag: cookies_26_cookie_secure_flag,
		},
	};
}

/**
 * Handler for POST /cookies/set-with-domain
 */
async function cookiesResponseCookieWithDomainAttribute(body: any): Promise<any> {
	return {"message":"Cookie set with domain"};
}

export function createAppCookiesResponseCookieWithDomainAttribute(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/cookies/set-with-domain",
		handler_name: "cookies_response_cookie_with_domain_attribute",
		request_schema: {"additionalProperties":false,"properties":{"value":{"type":"string"}},"required":["value"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			cookies_response_cookie_with_domain_attribute: cookies_response_cookie_with_domain_attribute,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function queryParamsStringValidationWithRegexSuccess(itemQuery: string): Promise<any> {
	return {"item_query":"fixedquery"};
}

export function createAppQueryParamsStringValidationWithRegexSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "query_params_string_validation_with_regex_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"item_query":{"annotation":"str","pattern":"^fixedquery$","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_string_validation_with_regex_success: query_params_string_validation_with_regex_success,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams49IntegerGtConstraintSuccess(limit: number): Promise<any> {
	return {"limit":5};
}

export function createAppQueryParams49IntegerGtConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_49_integer_gt_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"limit":{"exclusiveMinimum":0,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_49_integer_gt_constraint_success: query_params_49_integer_gt_constraint_success,
		},
	};
}

/**
 * Handler for GET /query/enum
 */
async function queryParamsEnumQueryParameterInvalidValue(model: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (model !== null && model !== undefined) {
		result["model"] = model;
	}
	return result;
}

export function createAppQueryParamsEnumQueryParameterInvalidValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/enum",
		handler_name: "query_params_enum_query_parameter_invalid_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"model":{"annotation":"str","enum":["alexnet","resnet","lenet"],"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_enum_query_parameter_invalid_value: query_params_enum_query_parameter_invalid_value,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams68ArrayUniqueitemsSuccess(ids: number[]): Promise<any> {
	return {"ids":[1,2,3,4]};
}

export function createAppQueryParams68ArrayUniqueitemsSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_68_array_uniqueitems_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"ids":{"items":{"type":"integer"},"type":"array","uniqueItems":true}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_68_array_uniqueitems_success: query_params_68_array_uniqueitems_success,
		},
	};
}

/**
 * Handler for GET /subscribe
 */
async function queryParams47PatternValidationEmailSuccess(email: string): Promise<any> {
	return {"email":"user@example.com"};
}

export function createAppQueryParams47PatternValidationEmailSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/subscribe",
		handler_name: "query_params_47_pattern_validation_email_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"email":{"pattern":"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_47_pattern_validation_email_success: query_params_47_pattern_validation_email_success,
		},
	};
}

/**
 * Handler for GET /query/int
 */
async function queryParamsRequiredIntegerQueryParameterSuccess(query: number): Promise<any> {
	return "foo bar 42";
}

export function createAppQueryParamsRequiredIntegerQueryParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int",
		handler_name: "query_params_required_integer_query_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"query":{"annotation":"int","type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_success: query_params_required_integer_query_parameter_success,
		},
	};
}

/**
 * Handler for GET /query
 */
async function queryParamsRequiredStringQueryParameterMissing(query: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (query !== null && query !== undefined) {
		result["query"] = query;
	}
	return result;
}

export function createAppQueryParamsRequiredStringQueryParameterMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query",
		handler_name: "query_params_required_string_query_parameter_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"query":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_string_query_parameter_missing: query_params_required_string_query_parameter_missing,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams57BooleanEmptyStringCoercion(active: boolean): Promise<any> {
	return {"active":false};
}

export function createAppQueryParams57BooleanEmptyStringCoercion(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_57_boolean_empty_string_coercion",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"active":{"type":"boolean"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_57_boolean_empty_string_coercion: query_params_57_boolean_empty_string_coercion,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams52IntegerLeConstraintBoundary(limit: number): Promise<any> {
	return {"limit":100};
}

export function createAppQueryParams52IntegerLeConstraintBoundary(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_52_integer_le_constraint_boundary",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"limit":{"maximum":100,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_52_integer_le_constraint_boundary: query_params_52_integer_le_constraint_boundary,
		},
	};
}

/**
 * Handler for GET /query/list-default
 */
async function queryParamsListWithDefaultEmptyArrayNoValuesProvided(tags: string[]): Promise<any> {
	return [];
}

export function createAppQueryParamsListWithDefaultEmptyArrayNoValuesProvided(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/list-default",
		handler_name: "query_params_list_with_default_empty_array_no_values_provided",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"tags":{"annotation":"list[str]","default":[],"items":{"type":"string"},"optional":true,"type":"array"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_list_with_default_empty_array_no_values_provided: query_params_list_with_default_empty_array_no_values_provided,
		},
	};
}

/**
 * Handler for GET /query/date
 */
async function queryParamsDateQueryParameterSuccess(eventDate: Date): Promise<any> {
	return {"event_date":"2024-01-15"};
}

export function createAppQueryParamsDateQueryParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/date",
		handler_name: "query_params_date_query_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"event_date":{"annotation":"str","format":"date","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_date_query_parameter_success: query_params_date_query_parameter_success,
		},
	};
}

/**
 * Handler for GET /query/str-max-length
 */
async function queryParamsStringQueryParamWithMaxLengthConstraintFail(name: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (name !== null && name !== undefined) {
		result["name"] = name;
	}
	return result;
}

export function createAppQueryParamsStringQueryParamWithMaxLengthConstraintFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/str-max-length",
		handler_name: "query_params_string_query_param_with_max_length_constraint_fail",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"name":{"annotation":"str","maxLength":10,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_string_query_param_with_max_length_constraint_fail: query_params_string_query_param_with_max_length_constraint_fail,
		},
	};
}

/**
 * Handler for GET /search
 */
async function queryParams45StringMinlengthValidationFailure(term: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (term !== null && term !== undefined) {
		result["term"] = term;
	}
	return result;
}

export function createAppQueryParams45StringMinlengthValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/search",
		handler_name: "query_params_45_string_minlength_validation_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"term":{"minLength":3,"required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_45_string_minlength_validation_failure: query_params_45_string_minlength_validation_failure,
		},
	};
}

/**
 * Handler for GET /query/int/default
 */
async function queryParamsIntegerWithDefaultValueOverride(query: number): Promise<any> {
	return "foo bar 50";
}

export function createAppQueryParamsIntegerWithDefaultValueOverride(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int/default",
		handler_name: "query_params_integer_with_default_value_override",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"query":{"annotation":"int","default":10,"optional":true,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_with_default_value_override: query_params_integer_with_default_value_override,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams67MultipleofConstraintFailure(quantity: number): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (quantity !== null && quantity !== undefined) {
		result["quantity"] = quantity;
	}
	return result;
}

export function createAppQueryParams67MultipleofConstraintFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_67_multipleof_constraint_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"quantity":{"multipleOf":5,"required":true,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_67_multipleof_constraint_failure: query_params_67_multipleof_constraint_failure,
		},
	};
}

/**
 * Handler for GET /subscribe
 */
async function queryParams58FormatEmailSuccess(email: string): Promise<any> {
	return {"email":"user@example.com"};
}

export function createAppQueryParams58FormatEmailSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/subscribe",
		handler_name: "query_params_58_format_email_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"email":{"format":"email","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_58_format_email_success: query_params_58_format_email_success,
		},
	};
}

/**
 * Handler for GET /query/int-ge
 */
async function queryParamsIntegerQueryParamWithGeConstraintBoundary(value: number): Promise<any> {
	return {"value":10};
}

export function createAppQueryParamsIntegerQueryParamWithGeConstraintBoundary(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int-ge",
		handler_name: "query_params_integer_query_param_with_ge_constraint_boundary",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"value":{"annotation":"int","minimum":10,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_query_param_with_ge_constraint_boundary: query_params_integer_query_param_with_ge_constraint_boundary,
		},
	};
}

/**
 * Handler for GET /query/int-gt
 */
async function queryParamsIntegerQueryParamWithGtConstraintValid(value: number): Promise<any> {
	return {"value":1};
}

export function createAppQueryParamsIntegerQueryParamWithGtConstraintValid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int-gt",
		handler_name: "query_params_integer_query_param_with_gt_constraint_valid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"value":{"annotation":"int","exclusiveMinimum":0,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_query_param_with_gt_constraint_valid: query_params_integer_query_param_with_gt_constraint_valid,
		},
	};
}

/**
 * Handler for GET /query/int
 */
async function queryParamsRequiredIntegerQueryParameterInvalidType(query: number): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (query !== null && query !== undefined) {
		result["query"] = query;
	}
	return result;
}

export function createAppQueryParamsRequiredIntegerQueryParameterInvalidType(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int",
		handler_name: "query_params_required_integer_query_parameter_invalid_type",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"query":{"annotation":"int","type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_invalid_type: query_params_required_integer_query_parameter_invalid_type,
		},
	};
}

/**
 * Handler for GET /query/int
 */
async function queryParamsRequiredIntegerQueryParameterFloatValue(query: number): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (query !== null && query !== undefined) {
		result["query"] = query;
	}
	return result;
}

export function createAppQueryParamsRequiredIntegerQueryParameterFloatValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int",
		handler_name: "query_params_required_integer_query_parameter_float_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"query":{"annotation":"int","type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_float_value: query_params_required_integer_query_parameter_float_value,
		},
	};
}

/**
 * Handler for GET /query/basic
 */
async function queryParamsQueryParameterWithUrlEncodedSpecialCharacters(name: string): Promise<any> {
	return {"name":"test&value=123"};
}

export function createAppQueryParamsQueryParameterWithUrlEncodedSpecialCharacters(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/basic",
		handler_name: "query_params_query_parameter_with_url_encoded_special_characters",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"name":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_query_parameter_with_url_encoded_special_characters: query_params_query_parameter_with_url_encoded_special_characters,
		},
	};
}

/**
 * Handler for GET /subscribe
 */
async function queryParams59FormatEmailFailure(email: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (email !== null && email !== undefined) {
		result["email"] = email;
	}
	return result;
}

export function createAppQueryParams59FormatEmailFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/subscribe",
		handler_name: "query_params_59_format_email_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"email":{"format":"email","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_59_format_email_failure: query_params_59_format_email_failure,
		},
	};
}

/**
 * Handler for GET /stats
 */
async function queryParams43ScientificNotationFloat(threshold: number): Promise<any> {
	return {"threshold":0.0015};
}

export function createAppQueryParams43ScientificNotationFloat(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/stats",
		handler_name: "query_params_43_scientific_notation_float",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"threshold":{"annotation":"float","type":"number"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_43_scientific_notation_float: query_params_43_scientific_notation_float,
		},
	};
}

/**
 * Handler for GET /redirect
 */
async function queryParams63FormatUriSuccess(url: string): Promise<any> {
	return {"url":"https://example.com/path?query=value"};
}

export function createAppQueryParams63FormatUriSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/redirect",
		handler_name: "query_params_63_format_uri_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"url":{"format":"uri","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_63_format_uri_success: query_params_63_format_uri_success,
		},
	};
}

/**
 * Handler for GET /query/bool
 */
async function queryParamsBooleanQueryParameterNumeric1(flag: boolean): Promise<any> {
	return {"flag":true};
}

export function createAppQueryParamsBooleanQueryParameterNumeric1(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/bool",
		handler_name: "query_params_boolean_query_parameter_numeric_1",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"flag":{"annotation":"bool","type":"boolean"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_boolean_query_parameter_numeric_1: query_params_boolean_query_parameter_numeric_1,
		},
	};
}

/**
 * Handler for GET /query/str-min-length
 */
async function queryParamsStringQueryParamWithMinLengthConstraintFail(name: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (name !== null && name !== undefined) {
		result["name"] = name;
	}
	return result;
}

export function createAppQueryParamsStringQueryParamWithMinLengthConstraintFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/str-min-length",
		handler_name: "query_params_string_query_param_with_min_length_constraint_fail",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"name":{"annotation":"str","minLength":3,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_string_query_param_with_min_length_constraint_fail: query_params_string_query_param_with_min_length_constraint_fail,
		},
	};
}

/**
 * Handler for GET /query/optional
 */
async function queryParamsOptionalStringQueryParameterProvided(query: string): Promise<any> {
	return "foo bar baz";
}

export function createAppQueryParamsOptionalStringQueryParameterProvided(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/optional",
		handler_name: "query_params_optional_string_query_parameter_provided",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"query":{"annotation":"str","optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_optional_string_query_parameter_provided: query_params_optional_string_query_parameter_provided,
		},
	};
}

/**
 * Handler for GET /query/list
 */
async function queryParamsListOfIntegersMultipleValues(deviceIds: number[]): Promise<any> {
	return [1,2];
}

export function createAppQueryParamsListOfIntegersMultipleValues(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/list",
		handler_name: "query_params_list_of_integers_multiple_values",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"device_ids":{"annotation":"list[int]","items":{"type":"integer"},"type":"array"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_list_of_integers_multiple_values: query_params_list_of_integers_multiple_values,
		},
	};
}

/**
 * Handler for GET /query/int-lt
 */
async function queryParamsIntegerQueryParamWithLtConstraintValid(value: number): Promise<any> {
	return {"value":49};
}

export function createAppQueryParamsIntegerQueryParamWithLtConstraintValid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int-lt",
		handler_name: "query_params_integer_query_param_with_lt_constraint_valid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"value":{"annotation":"int","exclusiveMaximum":50,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_query_param_with_lt_constraint_valid: query_params_integer_query_param_with_lt_constraint_valid,
		},
	};
}

/**
 * Handler for GET /items/negative
 */
async function queryParams42NegativeIntegerQueryParam(offset: number): Promise<any> {
	return {"offset":-10};
}

export function createAppQueryParams42NegativeIntegerQueryParam(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/negative",
		handler_name: "query_params_42_negative_integer_query_param",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"offset":{"annotation":"int","type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_42_negative_integer_query_param: query_params_42_negative_integer_query_param,
		},
	};
}

/**
 * Handler for GET /search
 */
async function queryParams46StringMaxlengthValidationFailure(term: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (term !== null && term !== undefined) {
		result["term"] = term;
	}
	return result;
}

export function createAppQueryParams46StringMaxlengthValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/search",
		handler_name: "query_params_46_string_maxlength_validation_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"term":{"maxLength":10,"required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_46_string_maxlength_validation_failure: query_params_46_string_maxlength_validation_failure,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams56ArrayMaxitemsConstraintFailure(tags: string[]): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (tags !== null && tags !== undefined) {
		result["tags"] = tags;
	}
	return result;
}

export function createAppQueryParams56ArrayMaxitemsConstraintFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_56_array_maxitems_constraint_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"tags":{"items":{"type":"string"},"maxItems":5,"required":true,"type":"array"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_56_array_maxitems_constraint_failure: query_params_56_array_maxitems_constraint_failure,
		},
	};
}

/**
 * Handler for GET /query/pattern
 */
async function queryParamsStringQueryParamWithRegexPatternFail(code: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (code !== null && code !== undefined) {
		result["code"] = code;
	}
	return result;
}

export function createAppQueryParamsStringQueryParamWithRegexPatternFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/pattern",
		handler_name: "query_params_string_query_param_with_regex_pattern_fail",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"code":{"annotation":"str","pattern":"^[0-9]{3,}$","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_string_query_param_with_regex_pattern_fail: query_params_string_query_param_with_regex_pattern_fail,
		},
	};
}

/**
 * Handler for GET /search
 */
async function queryParams44StringMinlengthValidationSuccess(term: string): Promise<any> {
	return {"term":"foo"};
}

export function createAppQueryParams44StringMinlengthValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/search",
		handler_name: "query_params_44_string_minlength_validation_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"term":{"minLength":3,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_44_string_minlength_validation_success: query_params_44_string_minlength_validation_success,
		},
	};
}

/**
 * Handler for GET /network
 */
async function queryParams61FormatIpv4Failure(ip: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (ip !== null && ip !== undefined) {
		result["ip"] = ip;
	}
	return result;
}

export function createAppQueryParams61FormatIpv4Failure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/network",
		handler_name: "query_params_61_format_ipv4_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"ip":{"format":"ipv4","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_61_format_ipv4_failure: query_params_61_format_ipv4_failure,
		},
	};
}

/**
 * Handler for GET /subscribe
 */
async function queryParams48PatternValidationEmailFailure(email: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (email !== null && email !== undefined) {
		result["email"] = email;
	}
	return result;
}

export function createAppQueryParams48PatternValidationEmailFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/subscribe",
		handler_name: "query_params_48_pattern_validation_email_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"email":{"pattern":"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_48_pattern_validation_email_failure: query_params_48_pattern_validation_email_failure,
		},
	};
}

/**
 * Handler for GET /query/int
 */
async function queryParamsRequiredIntegerQueryParameterMissing(query: number): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (query !== null && query !== undefined) {
		result["query"] = query;
	}
	return result;
}

export function createAppQueryParamsRequiredIntegerQueryParameterMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int",
		handler_name: "query_params_required_integer_query_parameter_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"query":{"annotation":"int","type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_integer_query_parameter_missing: query_params_required_integer_query_parameter_missing,
		},
	};
}

/**
 * Handler for GET /test
 */
async function queryParamsQueryParameterWithSpecialCharactersUrlEncoding(email: string, special: string): Promise<any> {
	return {"email":"x@test.com","special":"&@A.ac"};
}

export function createAppQueryParamsQueryParameterWithSpecialCharactersUrlEncoding(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/test",
		handler_name: "query_params_query_parameter_with_special_characters_url_encoding",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"email":{"annotation":"str","type":"string"},"special":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_query_parameter_with_special_characters_url_encoding: query_params_query_parameter_with_special_characters_url_encoding,
		},
	};
}

/**
 * Handler for GET /query/list
 */
async function queryParamsListQueryParameterRequiredButMissing(deviceIds: number[]): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (deviceIds !== null && deviceIds !== undefined) {
		result["deviceIds"] = deviceIds;
	}
	return result;
}

export function createAppQueryParamsListQueryParameterRequiredButMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/list",
		handler_name: "query_params_list_query_parameter_required_but_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"device_ids":{"annotation":"list[int]","items":{"type":"integer"},"type":"array"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_list_query_parameter_required_but_missing: query_params_list_query_parameter_required_but_missing,
		},
	};
}

/**
 * Handler for GET /query
 */
async function queryParamsRequiredStringQueryParameterSuccess(query: string): Promise<any> {
	return "foo bar baz";
}

export function createAppQueryParamsRequiredStringQueryParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query",
		handler_name: "query_params_required_string_query_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"query":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_required_string_query_parameter_success: query_params_required_string_query_parameter_success,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams66MultipleofConstraintSuccess(quantity: number): Promise<any> {
	return {"quantity":15};
}

export function createAppQueryParams66MultipleofConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_66_multipleof_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"quantity":{"multipleOf":5,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_66_multipleof_constraint_success: query_params_66_multipleof_constraint_success,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams53IntegerLeConstraintFailure(limit: number): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (limit !== null && limit !== undefined) {
		result["limit"] = limit;
	}
	return result;
}

export function createAppQueryParams53IntegerLeConstraintFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_53_integer_le_constraint_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"limit":{"maximum":100,"required":true,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_53_integer_le_constraint_failure: query_params_53_integer_le_constraint_failure,
		},
	};
}

/**
 * Handler for GET /query/multi-type
 */
async function queryParamsMultipleQueryParametersWithDifferentTypes(active: boolean, age: number, name: string, score: number): Promise<any> {
	return {"active":true,"age":30,"name":"john","score":95.5};
}

export function createAppQueryParamsMultipleQueryParametersWithDifferentTypes(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/multi-type",
		handler_name: "query_params_multiple_query_parameters_with_different_types",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"active":{"annotation":"bool","type":"boolean"},"age":{"annotation":"int","type":"integer"},"name":{"annotation":"str","type":"string"},"score":{"annotation":"float","type":"number"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_multiple_query_parameters_with_different_types: query_params_multiple_query_parameters_with_different_types,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams71ArraySeparatorSemicolon(colors: string[]): Promise<any> {
	return {"colors":["red","green","blue"]};
}

export function createAppQueryParams71ArraySeparatorSemicolon(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_71_array_separator_semicolon",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"colors":{"items":{"type":"string"},"separator":";","type":"array"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_71_array_separator_semicolon: query_params_71_array_separator_semicolon,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams70ArraySeparatorPipe(tags: string[]): Promise<any> {
	return {"tags":["python","rust","typescript"]};
}

export function createAppQueryParams70ArraySeparatorPipe(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_70_array_separator_pipe",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"tags":{"items":{"type":"string"},"separator":"|","type":"array"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_70_array_separator_pipe: query_params_70_array_separator_pipe,
		},
	};
}

/**
 * Handler for GET /query/int/default
 */
async function queryParamsIntegerWithDefaultValueNotProvided(query: number): Promise<any> {
	return "foo bar 10";
}

export function createAppQueryParamsIntegerWithDefaultValueNotProvided(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int/default",
		handler_name: "query_params_integer_with_default_value_not_provided",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"query":{"annotation":"int","default":10,"optional":true,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_with_default_value_not_provided: query_params_integer_with_default_value_not_provided,
		},
	};
}

/**
 * Handler for GET /query/bool
 */
async function queryParamsBooleanQueryParameterTrue(flag: boolean): Promise<any> {
	return {"flag":true};
}

export function createAppQueryParamsBooleanQueryParameterTrue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/bool",
		handler_name: "query_params_boolean_query_parameter_true",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"flag":{"annotation":"bool","type":"boolean"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_boolean_query_parameter_true: query_params_boolean_query_parameter_true,
		},
	};
}

/**
 * Handler for GET /query/int-le
 */
async function queryParamsIntegerQueryParamWithLeConstraintBoundary(value: number): Promise<any> {
	return {"value":100};
}

export function createAppQueryParamsIntegerQueryParamWithLeConstraintBoundary(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int-le",
		handler_name: "query_params_integer_query_param_with_le_constraint_boundary",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"value":{"annotation":"int","maximum":100,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_integer_query_param_with_le_constraint_boundary: query_params_integer_query_param_with_le_constraint_boundary,
		},
	};
}

/**
 * Handler for GET /query/float-ge
 */
async function queryParamsFloatQueryParamWithGeConstraintSuccess(price: number): Promise<any> {
	return {"price":0.01};
}

export function createAppQueryParamsFloatQueryParamWithGeConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/float-ge",
		handler_name: "query_params_float_query_param_with_ge_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"price":{"annotation":"float","minimum":0.01,"type":"number"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_float_query_param_with_ge_constraint_success: query_params_float_query_param_with_ge_constraint_success,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams51IntegerGeConstraintBoundary(offset: number): Promise<any> {
	return {"offset":0};
}

export function createAppQueryParams51IntegerGeConstraintBoundary(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_51_integer_ge_constraint_boundary",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"offset":{"minimum":0,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_51_integer_ge_constraint_boundary: query_params_51_integer_ge_constraint_boundary,
		},
	};
}

/**
 * Handler for GET /query/int/optional
 */
async function queryParamsOptionalIntegerQueryParameterMissing(query: number): Promise<any> {
	return "foo bar None";
}

export function createAppQueryParamsOptionalIntegerQueryParameterMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/int/optional",
		handler_name: "query_params_optional_integer_query_parameter_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"query":{"annotation":"int","optional":true,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_optional_integer_query_parameter_missing: query_params_optional_integer_query_parameter_missing,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams69ArrayUniqueitemsFailure(ids: number[]): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (ids !== null && ids !== undefined) {
		result["ids"] = ids;
	}
	return result;
}

export function createAppQueryParams69ArrayUniqueitemsFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_69_array_uniqueitems_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"ids":{"items":{"type":"integer"},"required":true,"type":"array","uniqueItems":true}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_69_array_uniqueitems_failure: query_params_69_array_uniqueitems_failure,
		},
	};
}

/**
 * Handler for GET /search
 */
async function queryParams72ArraySeparatorSpace(keywords: string[]): Promise<any> {
	return {"keywords":["rust","web","framework"]};
}

export function createAppQueryParams72ArraySeparatorSpace(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/search",
		handler_name: "query_params_72_array_separator_space",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"keywords":{"items":{"type":"string"},"separator":" ","type":"array"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_72_array_separator_space: query_params_72_array_separator_space,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function queryParamsStringValidationWithRegexFailure(itemQuery: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (itemQuery !== null && itemQuery !== undefined) {
		result["itemQuery"] = itemQuery;
	}
	return result;
}

export function createAppQueryParamsStringValidationWithRegexFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "query_params_string_validation_with_regex_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"item_query":{"annotation":"str","pattern":"^fixedquery$","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_string_validation_with_regex_failure: query_params_string_validation_with_regex_failure,
		},
	};
}

/**
 * Handler for GET /dns
 */
async function queryParams65FormatHostnameSuccess(host: string): Promise<any> {
	return {"host":"api.example.com"};
}

export function createAppQueryParams65FormatHostnameSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/dns",
		handler_name: "query_params_65_format_hostname_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"host":{"format":"hostname","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_65_format_hostname_success: query_params_65_format_hostname_success,
		},
	};
}

/**
 * Handler for GET /query/basic
 */
async function queryParamsQueryParameterWithUrlEncodedSpace(name: string): Promise<any> {
	return {"name":"hello world"};
}

export function createAppQueryParamsQueryParameterWithUrlEncodedSpace(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/basic",
		handler_name: "query_params_query_parameter_with_url_encoded_space",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"name":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_query_parameter_with_url_encoded_space: query_params_query_parameter_with_url_encoded_space,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function queryParamsListOfStringsMultipleValues(q: string[]): Promise<any> {
	return {"q":["foo","bar"]};
}

export function createAppQueryParamsListOfStringsMultipleValues(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "query_params_list_of_strings_multiple_values",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"q":{"annotation":"list[str]","items":{"type":"string"},"optional":true,"type":"array"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_list_of_strings_multiple_values: query_params_list_of_strings_multiple_values,
		},
	};
}

/**
 * Handler for GET /query/optional-default
 */
async function queryParamsOptionalQueryParameterWithDefaultValue(limit: number): Promise<any> {
	return {"limit":10};
}

export function createAppQueryParamsOptionalQueryParameterWithDefaultValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/optional-default",
		handler_name: "query_params_optional_query_parameter_with_default_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"limit":{"annotation":"int","default":10,"optional":true,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_optional_query_parameter_with_default_value: query_params_optional_query_parameter_with_default_value,
		},
	};
}

/**
 * Handler for GET /network/ipv6
 */
async function queryParams62FormatIpv6Success(ip: string): Promise<any> {
	return {"ip":"2001:0db8:85a3:0000:0000:8a2e:0370:7334"};
}

export function createAppQueryParams62FormatIpv6Success(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/network/ipv6",
		handler_name: "query_params_62_format_ipv6_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"ip":{"format":"ipv6","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_62_format_ipv6_success: query_params_62_format_ipv6_success,
		},
	};
}

/**
 * Handler for GET /query/list-default
 */
async function queryParamsArrayQueryParameterSingleValue(tags: string[]): Promise<any> {
	return ["apple"];
}

export function createAppQueryParamsArrayQueryParameterSingleValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/list-default",
		handler_name: "query_params_array_query_parameter_single_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"tags":{"annotation":"list[str]","default":[],"items":{"type":"string"},"optional":true,"type":"array"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_array_query_parameter_single_value: query_params_array_query_parameter_single_value,
		},
	};
}

/**
 * Handler for GET /query/optional
 */
async function queryParamsOptionalStringQueryParameterMissing(query: string): Promise<any> {
	return "foo bar None";
}

export function createAppQueryParamsOptionalStringQueryParameterMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/optional",
		handler_name: "query_params_optional_string_query_parameter_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"query":{"annotation":"str","optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_optional_string_query_parameter_missing: query_params_optional_string_query_parameter_missing,
		},
	};
}

/**
 * Handler for GET /query/datetime
 */
async function queryParamsDatetimeQueryParameterSuccess(timestamp: Date): Promise<any> {
	return {"timestamp":"2024-01-15T10:30:00Z"};
}

export function createAppQueryParamsDatetimeQueryParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/datetime",
		handler_name: "query_params_datetime_query_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"timestamp":{"annotation":"str","format":"date-time","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_datetime_query_parameter_success: query_params_datetime_query_parameter_success,
		},
	};
}

/**
 * Handler for GET /query/uuid
 */
async function queryParamsUuidQueryParameterInvalidFormat(itemId: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (itemId !== null && itemId !== undefined) {
		result["itemId"] = itemId;
	}
	return result;
}

export function createAppQueryParamsUuidQueryParameterInvalidFormat(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/uuid",
		handler_name: "query_params_uuid_query_parameter_invalid_format",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"item_id":{"annotation":"str","format":"uuid","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_uuid_query_parameter_invalid_format: query_params_uuid_query_parameter_invalid_format,
		},
	};
}

/**
 * Handler for GET /query/list-default
 */
async function queryParamsArrayQueryParameterEmptyArray(tags: string[]): Promise<any> {
	return [];
}

export function createAppQueryParamsArrayQueryParameterEmptyArray(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/list-default",
		handler_name: "query_params_array_query_parameter_empty_array",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"tags":{"annotation":"list[str]","default":[],"items":{"type":"string"},"optional":true,"type":"array"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_array_query_parameter_empty_array: query_params_array_query_parameter_empty_array,
		},
	};
}

/**
 * Handler for GET /query/enum
 */
async function queryParamsEnumQueryParameterSuccess(model: string): Promise<any> {
	return {"model":"alexnet"};
}

export function createAppQueryParamsEnumQueryParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/enum",
		handler_name: "query_params_enum_query_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"model":{"annotation":"str","enum":["alexnet","resnet","lenet"],"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_enum_query_parameter_success: query_params_enum_query_parameter_success,
		},
	};
}

/**
 * Handler for GET /query/uuid
 */
async function queryParamsUuidQueryParameterSuccess(itemId: string): Promise<any> {
	return {"item_id":"c892496f-b1fd-4b91-bdb8-b46f92df1716"};
}

export function createAppQueryParamsUuidQueryParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/query/uuid",
		handler_name: "query_params_uuid_query_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"item_id":{"annotation":"str","format":"uuid","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_uuid_query_parameter_success: query_params_uuid_query_parameter_success,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams50IntegerGtConstraintFailure(limit: number): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (limit !== null && limit !== undefined) {
		result["limit"] = limit;
	}
	return result;
}

export function createAppQueryParams50IntegerGtConstraintFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_50_integer_gt_constraint_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"limit":{"exclusiveMinimum":0,"required":true,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_50_integer_gt_constraint_failure: query_params_50_integer_gt_constraint_failure,
		},
	};
}

/**
 * Handler for GET /redirect
 */
async function queryParams64FormatUriFailure(url: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (url !== null && url !== undefined) {
		result["url"] = url;
	}
	return result;
}

export function createAppQueryParams64FormatUriFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/redirect",
		handler_name: "query_params_64_format_uri_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"url":{"format":"uri","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_64_format_uri_failure: query_params_64_format_uri_failure,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams54ArrayMinitemsConstraintSuccess(ids: number[]): Promise<any> {
	return {"ids":[1,2,3]};
}

export function createAppQueryParams54ArrayMinitemsConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_54_array_minitems_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"ids":{"items":{"type":"integer"},"minItems":2,"type":"array"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_54_array_minitems_constraint_success: query_params_54_array_minitems_constraint_success,
		},
	};
}

/**
 * Handler for GET /items
 */
async function queryParams55ArrayMinitemsConstraintFailure(ids: number[]): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (ids !== null && ids !== undefined) {
		result["ids"] = ids;
	}
	return result;
}

export function createAppQueryParams55ArrayMinitemsConstraintFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "query_params_55_array_minitems_constraint_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"ids":{"items":{"type":"integer"},"minItems":2,"required":true,"type":"array"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_55_array_minitems_constraint_failure: query_params_55_array_minitems_constraint_failure,
		},
	};
}

/**
 * Handler for GET /network
 */
async function queryParams60FormatIpv4Success(ip: string): Promise<any> {
	return {"ip":"192.168.1.1"};
}

export function createAppQueryParams60FormatIpv4Success(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/network",
		handler_name: "query_params_60_format_ipv4_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"ip":{"format":"ipv4","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			query_params_60_format_ipv4_success: query_params_60_format_ipv4_success,
		},
	};
}

/**
 * Handler for POST /slow-endpoint
 */
async function statusCodes408RequestTimeout(body: any): Promise<any> {
	return {"detail":"Request timeout"};
}

export function createAppStatusCodes408RequestTimeout(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/slow-endpoint",
		handler_name: "status_codes_408_request_timeout",
		request_schema: {"additionalProperties":false,"properties":{"data":{"type":"string"}},"required":["data"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_408_request_timeout: status_codes_408_request_timeout,
		},
	};
}

/**
 * Handler for GET /status-test/{code}
 */
async function statusCodes404NotFoundResourceNotFound(code: string): Promise<any> {
	return {"detail":"Item not found"};
}

export function createAppStatusCodes404NotFoundResourceNotFound(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/status-test/{code}",
		handler_name: "status_codes_404_not_found_resource_not_found",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"code":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_404_not_found_resource_not_found: status_codes_404_not_found_resource_not_found,
		},
	};
}

/**
 * Handler for GET /health
 */
async function statusCodes503ServiceUnavailableServerOverload(): Promise<any> {
	return {"detail":"Service temporarily unavailable"};
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
			status_codes_503_service_unavailable_server_overload: status_codes_503_service_unavailable_server_overload,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function statusCodes422UnprocessableEntityValidationError(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppStatusCodes422UnprocessableEntityValidationError(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "status_codes_422_unprocessable_entity_validation_error",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"string"}},"required":["price","name"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_422_unprocessable_entity_validation_error: status_codes_422_unprocessable_entity_validation_error,
		},
	};
}

/**
 * Handler for GET /temp-redirect
 */
async function statusCodes302FoundTemporaryRedirect(): Promise<any> {
	return { status: 302 };
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
			status_codes_302_found_temporary_redirect: status_codes_302_found_temporary_redirect,
		},
	};
}

/**
 * Handler for GET /status-test/{code}
 */
async function statusCodes304NotModifiedCachedContentValid(code: string, ifNoneMatch: string): Promise<any> {
	return { status: 304 };
}

export function createAppStatusCodes304NotModifiedCachedContentValid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/status-test/{code}",
		handler_name: "status_codes_304_not_modified_cached_content_valid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"If-None-Match":{"optional":true,"type":"string"}},"path":{"code":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_304_not_modified_cached_content_valid: status_codes_304_not_modified_cached_content_valid,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function statusCodes400BadRequestInvalidRequest(body: any): Promise<any> {
	return {"detail":"Invalid request format"};
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
			status_codes_400_bad_request_invalid_request: status_codes_400_bad_request_invalid_request,
		},
	};
}

/**
 * Handler for TRACE /data
 */
async function statusCodes22501NotImplemented(): Promise<any> {
	return { status: 405 };
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
			status_codes_22_501_not_implemented: status_codes_22_501_not_implemented,
		},
	};
}

/**
 * Handler for DELETE /status-test/{code}
 */
async function statusCodes204NoContentSuccessWithNoBody(code: string): Promise<any> {
	return { status: 204 };
}

export function createAppStatusCodes204NoContentSuccessWithNoBody(): SpikardApp {
	const route: RouteMetadata = {
		method: "DELETE",
		path: "/status-test/{code}",
		handler_name: "status_codes_204_no_content_success_with_no_body",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"code":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_204_no_content_success_with_no_body: status_codes_204_no_content_success_with_no_body,
		},
	};
}

/**
 * Handler for GET /old-path
 */
async function statusCodes301MovedPermanentlyPermanentRedirect(): Promise<any> {
	return { status: 301 };
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
			status_codes_301_moved_permanently_permanent_redirect: status_codes_301_moved_permanently_permanent_redirect,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function statusCodes201CreatedResourceCreated(body: any): Promise<any> {
	return {"id":1,"name":"New Item"};
}

export function createAppStatusCodes201CreatedResourceCreated(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "status_codes_201_created_resource_created",
		request_schema: {"additionalProperties":false,"properties":{"name":{"type":"string"}},"required":["name"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_201_created_resource_created: status_codes_201_created_resource_created,
		},
	};
}

/**
 * Handler for POST /tasks/
 */
async function statusCodes202AcceptedRequestAcceptedForProcessing(body: any): Promise<any> {
	return {"message":"Task accepted for processing","task_id":"abc123"};
}

export function createAppStatusCodes202AcceptedRequestAcceptedForProcessing(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/tasks/",
		handler_name: "status_codes_202_accepted_request_accepted_for_processing",
		request_schema: {"additionalProperties":false,"properties":{"task":{"type":"string"}},"required":["task"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_202_accepted_request_accepted_for_processing: status_codes_202_accepted_request_accepted_for_processing,
		},
	};
}

/**
 * Handler for POST /redirect-post
 */
async function statusCodes307TemporaryRedirectMethodPreserved(body: any): Promise<any> {
	return {};
}

export function createAppStatusCodes307TemporaryRedirectMethodPreserved(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/redirect-post",
		handler_name: "status_codes_307_temporary_redirect_method_preserved",
		request_schema: {"additionalProperties":false,"properties":{},"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_307_temporary_redirect_method_preserved: status_codes_307_temporary_redirect_method_preserved,
		},
	};
}

/**
 * Handler for GET /error
 */
async function statusCodes500InternalServerErrorServerError(): Promise<any> {
	return {"detail":"Internal server error","status":500,"title":"Internal Server Error","type":"https://spikard.dev/errors/internal-server-error"};
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
			status_codes_500_internal_server_error_server_error: status_codes_500_internal_server_error_server_error,
		},
	};
}

/**
 * Handler for GET /data
 */
async function statusCodes20414UriTooLong(): Promise<any> {
	return {};
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
			status_codes_20_414_uri_too_long: status_codes_20_414_uri_too_long,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function statusCodes401UnauthorizedMissingAuthentication(): Promise<any> {
	return {"detail":"Not authenticated"};
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
			status_codes_401_unauthorized_missing_authentication: status_codes_401_unauthorized_missing_authentication,
		},
	};
}

/**
 * Handler for GET /data
 */
async function statusCodes23503ServiceUnavailable(): Promise<any> {
	return {"error":"Service Unavailable","message":"The service is temporarily unavailable. Please try again later."};
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
			status_codes_23_503_service_unavailable: status_codes_23_503_service_unavailable,
		},
	};
}

/**
 * Handler for POST /upload
 */
async function statusCodes19413PayloadTooLarge(body: any): Promise<any> {
	return {"error":"Payload Too Large","message":"Request body size exceeds maximum allowed size of 1024 bytes"};
}

export function createAppStatusCodes19413PayloadTooLarge(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "status_codes_19_413_payload_too_large",
		request_schema: {"properties":{"data":{"type":"string"}},"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_19_413_payload_too_large: status_codes_19_413_payload_too_large,
		},
	};
}

/**
 * Handler for GET /admin/users
 */
async function statusCodes403ForbiddenInsufficientPermissions(): Promise<any> {
	return {"detail":"Not enough permissions"};
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
			status_codes_403_forbidden_insufficient_permissions: status_codes_403_forbidden_insufficient_permissions,
		},
	};
}

/**
 * Handler for GET /data
 */
async function statusCodes21431RequestHeaderFieldsTooLarge(xLargeHeader: string): Promise<any> {
	return {"error":"Request Header Fields Too Large","message":"Request headers exceed maximum allowed size of 8192 bytes"};
}

export function createAppStatusCodes21431RequestHeaderFieldsTooLarge(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/data",
		handler_name: "status_codes_21_431_request_header_fields_too_large",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"X-Large-Header":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_21_431_request_header_fields_too_large: status_codes_21_431_request_header_fields_too_large,
		},
	};
}

/**
 * Handler for GET /api/resource
 */
async function statusCodes429TooManyRequests(): Promise<any> {
	return {"detail":"Rate limit exceeded. Try again in 60 seconds."};
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
			status_codes_429_too_many_requests: status_codes_429_too_many_requests,
		},
	};
}

/**
 * Handler for GET /status-test/{code}
 */
async function statusCodes200OkSuccess(code: string): Promise<any> {
	return {"id":1,"name":"Item 1"};
}

export function createAppStatusCodes200OkSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/status-test/{code}",
		handler_name: "status_codes_200_ok_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"code":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			status_codes_200_ok_success: status_codes_200_ok_success,
		},
	};
}

/**
 * Handler for GET /files/document.pdf
 */
async function statusCodes206PartialContent(): Promise<any> {
	return "binary_data_1024_bytes";
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
			status_codes_206_partial_content: status_codes_206_partial_content,
		},
	};
}

/**
 * Handler for OPTIONS /items/
 */
async function httpMethodsOptionsCorsPreflightRequest(): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	return result;
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
			http_methods_options_cors_preflight_request: http_methods_options_cors_preflight_request,
		},
	};
}

/**
 * Handler for DELETE /items/{id}
 */
async function httpMethodsDeleteRemoveResource(id: string): Promise<any> {
	return {};
}

export function createAppHttpMethodsDeleteRemoveResource(): SpikardApp {
	const route: RouteMetadata = {
		method: "DELETE",
		path: "/items/{id}",
		handler_name: "http_methods_delete_remove_resource",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_delete_remove_resource: http_methods_delete_remove_resource,
		},
	};
}

/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutCreateResourceIfDoesnTExist(body: any, id: string): Promise<any> {
	return {"id":999,"name":"New Item","price":49.99};
}

export function createAppHttpMethodsPutCreateResourceIfDoesnTExist(): SpikardApp {
	const route: RouteMetadata = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_create_resource_if_doesn_t_exist",
		request_schema: {"properties":{"id":{"type":"integer"},"name":{"type":"string"},"price":{"type":"number"}},"required":["id","name","price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_create_resource_if_doesn_t_exist: http_methods_put_create_resource_if_doesn_t_exist,
		},
	};
}

/**
 * Handler for PATCH /items/{id}
 */
async function httpMethodsPatchUpdateMultipleFields(body: any, id: string): Promise<any> {
	return {"id":1,"in_stock":false,"name":"Updated Name","price":89.99};
}

export function createAppHttpMethodsPatchUpdateMultipleFields(): SpikardApp {
	const route: RouteMetadata = {
		method: "PATCH",
		path: "/items/{id}",
		handler_name: "http_methods_patch_update_multiple_fields",
		request_schema: {"properties":{"in_stock":{"type":"boolean"},"name":{"type":"string"},"price":{"type":"number"}},"required":["in_stock","name","price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_patch_update_multiple_fields: http_methods_patch_update_multiple_fields,
		},
	};
}

/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutValidationError(body: any, id: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	if (id !== null && id !== undefined) {
		result["id"] = id;
	}
	return result;
}

export function createAppHttpMethodsPutValidationError(): SpikardApp {
	const route: RouteMetadata = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_validation_error",
		request_schema: {"$schema":"https://json-schema.org/draft/2020-12/schema","properties":{"id":{"type":"integer"},"name":{"minLength":3,"type":"string"},"price":{"exclusiveMinimum":0,"type":"number"}},"required":["id","name","price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_validation_error: http_methods_put_validation_error,
		},
	};
}

/**
 * Handler for HEAD /items/{id}
 */
async function httpMethodsHeadGetMetadataWithoutBody(id: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (id !== null && id !== undefined) {
		result["id"] = id;
	}
	return result;
}

export function createAppHttpMethodsHeadGetMetadataWithoutBody(): SpikardApp {
	const route: RouteMetadata = {
		method: "HEAD",
		path: "/items/{id}",
		handler_name: "http_methods_head_get_metadata_without_body",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_head_get_metadata_without_body: http_methods_head_get_metadata_without_body,
		},
	};
}

/**
 * Handler for DELETE /items/{id}
 */
async function httpMethodsDeleteWithResponseBody(id: string): Promise<any> {
	return {"id":1,"message":"Item deleted successfully","name":"Deleted Item"};
}

export function createAppHttpMethodsDeleteWithResponseBody(): SpikardApp {
	const route: RouteMetadata = {
		method: "DELETE",
		path: "/items/{id}",
		handler_name: "http_methods_delete_with_response_body",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_delete_with_response_body: http_methods_delete_with_response_body,
		},
	};
}

/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutMissingRequiredField(body: any, id: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	if (id !== null && id !== undefined) {
		result["id"] = id;
	}
	return result;
}

export function createAppHttpMethodsPutMissingRequiredField(): SpikardApp {
	const route: RouteMetadata = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_missing_required_field",
		request_schema: {"properties":{"id":{"type":"integer"},"name":{"type":"string"},"price":{"type":"string"}},"required":["price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_missing_required_field: http_methods_put_missing_required_field,
		},
	};
}

/**
 * Handler for PATCH /items/{id}
 */
async function httpMethodsPatchPartialUpdate(body: any, id: string): Promise<any> {
	return {"id":1,"in_stock":true,"name":"Existing Item","price":79.99};
}

export function createAppHttpMethodsPatchPartialUpdate(): SpikardApp {
	const route: RouteMetadata = {
		method: "PATCH",
		path: "/items/{id}",
		handler_name: "http_methods_patch_partial_update",
		request_schema: {"properties":{"price":{"type":"number"}},"required":["price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_patch_partial_update: http_methods_patch_partial_update,
		},
	};
}

/**
 * Handler for DELETE /items/{id}
 */
async function httpMethodsDeleteResourceNotFound(id: string): Promise<any> {
	return {};
}

export function createAppHttpMethodsDeleteResourceNotFound(): SpikardApp {
	const route: RouteMetadata = {
		method: "DELETE",
		path: "/items/{id}",
		handler_name: "http_methods_delete_resource_not_found",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_delete_resource_not_found: http_methods_delete_resource_not_found,
		},
	};
}

/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutIdempotentOperation(body: any, id: string): Promise<any> {
	return {"id":1,"name":"Fixed Name","price":50.0};
}

export function createAppHttpMethodsPutIdempotentOperation(): SpikardApp {
	const route: RouteMetadata = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_idempotent_operation",
		request_schema: {"properties":{"id":{"type":"integer"},"name":{"type":"string"},"price":{"type":"number"}},"required":["id","name","price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_idempotent_operation: http_methods_put_idempotent_operation,
		},
	};
}

/**
 * Handler for PUT /items/{id}
 */
async function httpMethodsPutCompleteResourceReplacement(body: any, id: string): Promise<any> {
	return {"description":"Completely replaced","id":1,"in_stock":true,"name":"Updated Item","price":99.99};
}

export function createAppHttpMethodsPutCompleteResourceReplacement(): SpikardApp {
	const route: RouteMetadata = {
		method: "PUT",
		path: "/items/{id}",
		handler_name: "http_methods_put_complete_resource_replacement",
		request_schema: {"properties":{"description":{"type":"string"},"id":{"type":"integer"},"in_stock":{"type":"boolean"},"name":{"type":"string"},"price":{"type":"number"}},"required":["description","id","in_stock","name","price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			http_methods_put_complete_resource_replacement: http_methods_put_complete_resource_replacement,
		},
	};
}

/**
 * Handler for POST /messages
 */
async function edgeCases19EmojiInStrings(body: any): Promise<any> {
	return {"text":"Hello 👋 World 🌍"};
}

export function createAppEdgeCases19EmojiInStrings(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/messages",
		handler_name: "edge_cases_19_emoji_in_strings",
		request_schema: {"properties":{"text":{"maxLength":100,"minLength":1,"type":"string"}},"required":["text"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_19_emoji_in_strings: edge_cases_19_emoji_in_strings,
		},
	};
}

/**
 * Handler for GET /search
 */
async function edgeCases12PercentEncodedSpecialChars(term: string): Promise<any> {
	return {"term":"hi there"};
}

export function createAppEdgeCases12PercentEncodedSpecialChars(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/search",
		handler_name: "edge_cases_12_percent_encoded_special_chars",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"term":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_12_percent_encoded_special_chars: edge_cases_12_percent_encoded_special_chars,
		},
	};
}

/**
 * Handler for POST /strings/
 */
async function edgeCasesSpecialStringValuesAndEscaping(body: any): Promise<any> {
	return {"backslashes":"C:\\\\Users\\\\Path","empty_string":"","quotes":"He said \"hello\" and 'goodbye'","special_chars":"!@#$%^&*()_+-=[]{}|;':\",./<>?","tabs_newlines":"line1\n\tline2\r\nline3","unicode_escapes":"Hello","whitespace":"   "};
}

export function createAppEdgeCasesSpecialStringValuesAndEscaping(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/strings/",
		handler_name: "edge_cases_special_string_values_and_escaping",
		request_schema: {"additionalProperties":false,"properties":{"backslashes":{"type":"string"},"empty_string":{"type":"string"},"quotes":{"type":"string"},"special_chars":{"type":"string"},"tabs_newlines":{"type":"string"},"unicode_escapes":{"type":"string"},"whitespace":{"type":"string"}},"required":["empty_string","whitespace","tabs_newlines","quotes","backslashes","unicode_escapes","special_chars"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_special_string_values_and_escaping: edge_cases_special_string_values_and_escaping,
		},
	};
}

/**
 * Handler for POST /calculate
 */
async function edgeCases15FloatPrecisionPreservation(body: any): Promise<any> {
	return {"value":3.141592653589793};
}

export function createAppEdgeCases15FloatPrecisionPreservation(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/calculate",
		handler_name: "edge_cases_15_float_precision_preservation",
		request_schema: {"properties":{"value":{"type":"number"}},"required":["value"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_15_float_precision_preservation: edge_cases_15_float_precision_preservation,
		},
	};
}

/**
 * Handler for GET /items
 */
async function edgeCases13EmptyStringQueryParamPreserved(filter: string): Promise<any> {
	return {"filter":""};
}

export function createAppEdgeCases13EmptyStringQueryParamPreserved(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "edge_cases_13_empty_string_query_param_preserved",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"filter":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_13_empty_string_query_param_preserved: edge_cases_13_empty_string_query_param_preserved,
		},
	};
}

/**
 * Handler for POST /items
 */
async function edgeCases24ArrayWithHoles(body: any): Promise<any> {
	return {"items":["first","third","sixth"]};
}

export function createAppEdgeCases24ArrayWithHoles(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items",
		handler_name: "edge_cases_24_array_with_holes",
		request_schema: {"properties":{"items":{"items":{"type":"string"},"type":"array"}},"required":["items"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_24_array_with_holes: edge_cases_24_array_with_holes,
		},
	};
}

/**
 * Handler for POST /calculate
 */
async function edgeCases21ScientificNotationNumber(body: any): Promise<any> {
	return {"value":123000};
}

export function createAppEdgeCases21ScientificNotationNumber(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/calculate",
		handler_name: "edge_cases_21_scientific_notation_number",
		request_schema: {"properties":{"value":{"minimum":0,"type":"number"}},"required":["value"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_21_scientific_notation_number: edge_cases_21_scientific_notation_number,
		},
	};
}

/**
 * Handler for POST /calculations/
 */
async function edgeCasesFloatPrecisionAndRounding(body: any): Promise<any> {
	return {"precise_value":3.141592653589793,"sum":0.30000000000000004,"very_large":1.7976931348623157e308,"very_small":1e-10};
}

export function createAppEdgeCasesFloatPrecisionAndRounding(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/calculations/",
		handler_name: "edge_cases_float_precision_and_rounding",
		request_schema: {"additionalProperties":false,"properties":{"expected_sum":{"type":"number"},"precise_value":{"type":"number"},"value1":{"type":"number"},"value2":{"type":"number"},"very_large":{"type":"number"},"very_small":{"type":"number"}},"required":["value1","value2","expected_sum","precise_value","very_small","very_large"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_float_precision_and_rounding: edge_cases_float_precision_and_rounding,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function edgeCasesUnicodeAndEmojiHandling(body: any): Promise<any> {
	return {"description":"Best café in München 🇩🇪","emoji_reactions":"👍❤️😂🎉","id":1,"name":"Coffee Shop ☕","tags":["食べ物","音楽","💰"]};
}

export function createAppEdgeCasesUnicodeAndEmojiHandling(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/items/",
		handler_name: "edge_cases_unicode_and_emoji_handling",
		request_schema: {"additionalProperties":false,"properties":{"description":{"type":"string"},"emoji_reactions":{"type":"string"},"name":{"type":"string"},"tags":{"items":{"type":"string"},"type":"array"}},"required":["name","description","tags","emoji_reactions"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_unicode_and_emoji_handling: edge_cases_unicode_and_emoji_handling,
		},
	};
}

/**
 * Handler for POST /text
 */
async function edgeCases17ExtremelyLongString(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppEdgeCases17ExtremelyLongString(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/text",
		handler_name: "edge_cases_17_extremely_long_string",
		request_schema: {"properties":{"content":{"maxLength":10000,"type":"string"}},"required":["content"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_17_extremely_long_string: edge_cases_17_extremely_long_string,
		},
	};
}

/**
 * Handler for GET /search
 */
async function edgeCases11Utf8QueryParameter(term: string): Promise<any> {
	return {"term":"café"};
}

export function createAppEdgeCases11Utf8QueryParameter(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/search",
		handler_name: "edge_cases_11_utf8_query_parameter",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"term":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_11_utf8_query_parameter: edge_cases_11_utf8_query_parameter,
		},
	};
}

/**
 * Handler for POST /users
 */
async function edgeCases18UnicodeNormalization(body: any): Promise<any> {
	return {"name":"café"};
}

export function createAppEdgeCases18UnicodeNormalization(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "edge_cases_18_unicode_normalization",
		request_schema: {"properties":{"name":{"minLength":1,"type":"string"}},"required":["name"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_18_unicode_normalization: edge_cases_18_unicode_normalization,
		},
	};
}

/**
 * Handler for POST /files
 */
async function edgeCases20NullByteInString(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppEdgeCases20NullByteInString(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files",
		handler_name: "edge_cases_20_null_byte_in_string",
		request_schema: {"properties":{"filename":{"pattern":"^[^\\x00]+$","type":"string"}},"required":["filename"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_20_null_byte_in_string: edge_cases_20_null_byte_in_string,
		},
	};
}

/**
 * Handler for POST /data
 */
async function edgeCases23DeeplyNestedJsonLimit(body: any): Promise<any> {
	return {"error":"Request body exceeds maximum nesting depth of 32"};
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
			edge_cases_23_deeply_nested_json_limit: edge_cases_23_deeply_nested_json_limit,
		},
	};
}

/**
 * Handler for GET /items
 */
async function edgeCases14LargeIntegerBoundary(id: number): Promise<any> {
	return {"id":9007199254740991};
}

export function createAppEdgeCases14LargeIntegerBoundary(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items",
		handler_name: "edge_cases_14_large_integer_boundary",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"id":{"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_14_large_integer_boundary: edge_cases_14_large_integer_boundary,
		},
	};
}

/**
 * Handler for GET /data
 */
async function edgeCases22LeadingZerosInteger(value: number): Promise<any> {
	return {"value":123};
}

export function createAppEdgeCases22LeadingZerosInteger(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/data",
		handler_name: "edge_cases_22_leading_zeros_integer",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"query":{"value":{"annotation":"int","type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_22_leading_zeros_integer: edge_cases_22_leading_zeros_integer,
		},
	};
}

/**
 * Handler for POST /numbers/
 */
async function edgeCasesLargeIntegerBoundaryValues(body: any): Promise<any> {
	return {"large_int":9223372036854775807,"max_safe_int":9007199254740991,"negative_large":-9223372036854775808};
}

export function createAppEdgeCasesLargeIntegerBoundaryValues(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/numbers/",
		handler_name: "edge_cases_large_integer_boundary_values",
		request_schema: {"additionalProperties":false,"properties":{"large_int":{"type":"integer"},"max_safe_int":{"type":"integer"},"negative_large":{"type":"integer"}},"required":["max_safe_int","large_int","negative_large"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_large_integer_boundary_values: edge_cases_large_integer_boundary_values,
		},
	};
}

/**
 * Handler for POST /nested/
 */
async function edgeCasesDeeplyNestedStructure10Levels(body: any): Promise<any> {
	return {"max_depth":10,"message":"Processed deeply nested structure","value_found":"deep"};
}

export function createAppEdgeCasesDeeplyNestedStructure10Levels(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/nested/",
		handler_name: "edge_cases_deeply_nested_structure_10_levels",
		request_schema: {"additionalProperties":false,"properties":{"level1":{"additionalProperties":false,"properties":{"level2":{"additionalProperties":false,"properties":{"level3":{"additionalProperties":false,"properties":{"level4":{"additionalProperties":false,"properties":{"level5":{"additionalProperties":false,"properties":{"level6":{"additionalProperties":false,"properties":{"level7":{"additionalProperties":false,"properties":{"level8":{"additionalProperties":false,"properties":{"level9":{"additionalProperties":false,"properties":{"level10":{"additionalProperties":false,"properties":{"depth":{"type":"integer"},"value":{"type":"string"}},"required":["value","depth"],"type":"object"}},"required":["level10"],"type":"object"}},"required":["level9"],"type":"object"}},"required":["level8"],"type":"object"}},"required":["level7"],"type":"object"}},"required":["level6"],"type":"object"}},"required":["level5"],"type":"object"}},"required":["level4"],"type":"object"}},"required":["level3"],"type":"object"}},"required":["level2"],"type":"object"}},"required":["level1"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_deeply_nested_structure_10_levels: edge_cases_deeply_nested_structure_10_levels,
		},
	};
}

/**
 * Handler for POST /nulls/
 */
async function edgeCasesEmptyAndNullValueHandling(body: any): Promise<any> {
	return {"empty_array_length":0,"empty_object_keys":0,"empty_string_length":0,"explicit_null_is_null":true,"false_is_false":true,"zero_is_falsy":true};
}

export function createAppEdgeCasesEmptyAndNullValueHandling(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/nulls/",
		handler_name: "edge_cases_empty_and_null_value_handling",
		request_schema: {"additionalProperties":false,"properties":{"empty_array":{"items":{},"type":"array"},"empty_object":{"additionalProperties":false,"properties":{},"type":"object"},"empty_string":{"type":"string"},"explicit_null":{"type":"null"},"false_boolean":{"type":"boolean"},"zero_number":{"type":"integer"}},"required":["explicit_null","empty_string","empty_array","empty_object","zero_number","false_boolean"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_empty_and_null_value_handling: edge_cases_empty_and_null_value_handling,
		},
	};
}

/**
 * Handler for POST /data
 */
async function edgeCases16NegativeZeroHandling(body: any): Promise<any> {
	return {"offset":0};
}

export function createAppEdgeCases16NegativeZeroHandling(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "edge_cases_16_negative_zero_handling",
		request_schema: {"properties":{"offset":{"type":"number"}},"required":["offset"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			edge_cases_16_negative_zero_handling: edge_cases_16_negative_zero_handling,
		},
	};
}

/**
 * Handler for GET /path/bool/{item_id}
 */
async function pathParamsBooleanPathParameterTrue(itemId: boolean): Promise<any> {
	return {"item_id":true};
}

export function createAppPathParamsBooleanPathParameterTrue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/bool/{item_id}",
		handler_name: "path_params_boolean_path_parameter_true",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"type":"boolean"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_boolean_path_parameter_true: path_params_boolean_path_parameter_true,
		},
	};
}

/**
 * Handler for GET /prices/{amount}
 */
async function pathParams29DecimalPathParamSuccess(amount: string): Promise<any> {
	return {"amount":"19.99"};
}

export function createAppPathParams29DecimalPathParamSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/prices/{amount}",
		handler_name: "path_params_29_decimal_path_param_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"amount":{"format":"decimal","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_29_decimal_path_param_success: path_params_29_decimal_path_param_success,
		},
	};
}

/**
 * Handler for GET /path/param-lt-gt/{item_id}
 */
async function pathParamsIntegerPathParameterWithCombinedLtAndGtConstraintsSuccess(itemId: number): Promise<any> {
	return {"item_id":2};
}

export function createAppPathParamsIntegerPathParameterWithCombinedLtAndGtConstraintsSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-lt-gt/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"exclusiveMaximum":3,"exclusiveMinimum":1,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success: path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success,
		},
	};
}

/**
 * Handler for GET /repos/{owner}/{repo}
 */
async function pathParams33StringPatternPathSuccess(owner: string, repo: string): Promise<any> {
	return {"owner":"spikard-labs","repo":"spikard-http"};
}

export function createAppPathParams33StringPatternPathSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/repos/{owner}/{repo}",
		handler_name: "path_params_33_string_pattern_path_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"owner":{"pattern":"^[a-zA-Z0-9-]+$","type":"string"},"repo":{"pattern":"^[a-zA-Z0-9-_]+$","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_33_string_pattern_path_success: path_params_33_string_pattern_path_success,
		},
	};
}

/**
 * Handler for GET /users/{username}
 */
async function pathParams31StringMinlengthPathFailure(username: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (username !== null && username !== undefined) {
		result["username"] = username;
	}
	return result;
}

export function createAppPathParams31StringMinlengthPathFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/{username}",
		handler_name: "path_params_31_string_minlength_path_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"username":{"minLength":3,"required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_31_string_minlength_path_failure: path_params_31_string_minlength_path_failure,
		},
	};
}

/**
 * Handler for GET /offset/{value}
 */
async function pathParams35NegativeIntegerPathParam(value: number): Promise<any> {
	return {"value":-100};
}

export function createAppPathParams35NegativeIntegerPathParam(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/offset/{value}",
		handler_name: "path_params_35_negative_integer_path_param",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"value":{"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_35_negative_integer_path_param: path_params_35_negative_integer_path_param,
		},
	};
}

/**
 * Handler for GET /models/{model_name}
 */
async function pathParamsEnumPathParameterInvalidValue(modelName: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (modelName !== null && modelName !== undefined) {
		result["modelName"] = modelName;
	}
	return result;
}

export function createAppPathParamsEnumPathParameterInvalidValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/models/{model_name}",
		handler_name: "path_params_enum_path_parameter_invalid_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"model_name":{"enum":["alexnet","resnet","lenet"],"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_enum_path_parameter_invalid_value: path_params_enum_path_parameter_invalid_value,
		},
	};
}

/**
 * Handler for GET /bookings/{timestamp}
 */
async function pathParams27DatetimeFormatPathParamSuccess(timestamp: Date): Promise<any> {
	return {"timestamp":"2025-10-30T14:30:00Z"};
}

export function createAppPathParams27DatetimeFormatPathParamSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/bookings/{timestamp}",
		handler_name: "path_params_27_datetime_format_path_param_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"timestamp":{"format":"date-time","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_27_datetime_format_path_param_success: path_params_27_datetime_format_path_param_success,
		},
	};
}

/**
 * Handler for GET /events/{date}
 */
async function pathParams25DateFormatInvalidFailure(date: Date): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (date !== null && date !== undefined) {
		result["date"] = date.toISOString();
	}
	return result;
}

export function createAppPathParams25DateFormatInvalidFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/events/{date}",
		handler_name: "path_params_25_date_format_invalid_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"date":{"format":"date","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_25_date_format_invalid_failure: path_params_25_date_format_invalid_failure,
		},
	};
}

/**
 * Handler for GET /path/param-lt/{item_id}
 */
async function pathParamsIntegerPathParameterWithLtConstraintSuccess(itemId: number): Promise<any> {
	return {"item_id":2};
}

export function createAppPathParamsIntegerPathParameterWithLtConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-lt/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_lt_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"exclusiveMaximum":3,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_lt_constraint_success: path_params_integer_path_parameter_with_lt_constraint_success,
		},
	};
}

/**
 * Handler for GET /path/param-gt/{item_id}
 */
async function pathParamsIntegerPathParameterWithGtConstraintSuccess(itemId: number): Promise<any> {
	return {"item_id":42};
}

export function createAppPathParamsIntegerPathParameterWithGtConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-gt/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_gt_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"exclusiveMinimum":3,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_gt_constraint_success: path_params_integer_path_parameter_with_gt_constraint_success,
		},
	};
}

/**
 * Handler for GET /delays/{duration}
 */
async function pathParams28DurationFormatPathParamSuccess(duration: string): Promise<any> {
	return {"duration":"P1DT2H30M"};
}

export function createAppPathParams28DurationFormatPathParamSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/delays/{duration}",
		handler_name: "path_params_28_duration_format_path_param_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"duration":{"format":"duration","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_28_duration_format_path_param_success: path_params_28_duration_format_path_param_success,
		},
	};
}

/**
 * Handler for GET /type-syntax/items-count/{count:int}
 */
async function pathParamsPathParameterTypeSyntaxWithOverride(count: number): Promise<any> {
	return {"count":"50"};
}

export function createAppPathParamsPathParameterTypeSyntaxWithOverride(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/type-syntax/items-count/{count:int}",
		handler_name: "path_params_path_parameter_type_syntax_with_override",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"count":{"maximum":100,"minimum":1,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_path_parameter_type_syntax_with_override: path_params_path_parameter_type_syntax_with_override,
		},
	};
}

/**
 * Handler for GET /items/{id}
 */
async function pathParams20UuidV3PathParamSuccess(id: string): Promise<any> {
	return {"id":"e8b5a51d-11c8-3310-a6ab-367563f20686"};
}

export function createAppPathParams20UuidV3PathParamSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/{id}",
		handler_name: "path_params_20_uuid_v3_path_param_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"format":"uuid","type":"string","uuidVersion":"3"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_20_uuid_v3_path_param_success: path_params_20_uuid_v3_path_param_success,
		},
	};
}

/**
 * Handler for GET /path/int/{item_id}
 */
async function pathParamsIntegerPathParameterInvalidString(itemId: number): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (itemId !== null && itemId !== undefined) {
		result["itemId"] = itemId;
	}
	return result;
}

export function createAppPathParamsIntegerPathParameterInvalidString(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/int/{item_id}",
		handler_name: "path_params_integer_path_parameter_invalid_string",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_invalid_string: path_params_integer_path_parameter_invalid_string,
		},
	};
}

/**
 * Handler for GET /users/{username}
 */
async function pathParams30StringMinlengthPathSuccess(username: string): Promise<any> {
	return {"username":"alice"};
}

export function createAppPathParams30StringMinlengthPathSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/{username}",
		handler_name: "path_params_30_string_minlength_path_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"username":{"minLength":3,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_30_string_minlength_path_success: path_params_30_string_minlength_path_success,
		},
	};
}

/**
 * Handler for GET /path/param-le/{item_id}
 */
async function pathParamsIntegerPathParameterWithLeConstraintSuccess(itemId: number): Promise<any> {
	return {"item_id":3};
}

export function createAppPathParamsIntegerPathParameterWithLeConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-le/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_le_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"maximum":3,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_le_constraint_success: path_params_integer_path_parameter_with_le_constraint_success,
		},
	};
}

/**
 * Handler for GET /type-syntax/items/{id:uuid}
 */
async function pathParamsPathParameterTypeSyntaxInvalidUuid(): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	return result;
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
			path_params_path_parameter_type_syntax_invalid_uuid: path_params_path_parameter_type_syntax_invalid_uuid,
		},
	};
}

/**
 * Handler for GET /files/{file_path:path}
 */
async function pathParamsPathTypeParameterFilePath(filePath: string): Promise<any> {
	return {"file_path":"home/johndoe/myfile.txt"};
}

export function createAppPathParamsPathTypeParameterFilePath(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/files/{file_path:path}",
		handler_name: "path_params_path_type_parameter_file_path",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"file_path":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_path_type_parameter_file_path: path_params_path_type_parameter_file_path,
		},
	};
}

/**
 * Handler for GET /type-syntax/items/{id:uuid}
 */
async function pathParamsPathParameterWithTypeSyntaxUuid(): Promise<any> {
	return {"id":"550e8400-e29b-41d4-a716-446655440000"};
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
			path_params_path_parameter_with_type_syntax_uuid: path_params_path_parameter_with_type_syntax_uuid,
		},
	};
}

/**
 * Handler for GET /users/{username}
 */
async function pathParams32StringMaxlengthPathFailure(username: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (username !== null && username !== undefined) {
		result["username"] = username;
	}
	return result;
}

export function createAppPathParams32StringMaxlengthPathFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/{username}",
		handler_name: "path_params_32_string_maxlength_path_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"username":{"maxLength":20,"required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_32_string_maxlength_path_failure: path_params_32_string_maxlength_path_failure,
		},
	};
}

/**
 * Handler for GET /path/int/{item_id}
 */
async function pathParamsIntegerPathParameterSuccess(itemId: number): Promise<any> {
	return {"item_id":42};
}

export function createAppPathParamsIntegerPathParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/int/{item_id}",
		handler_name: "path_params_integer_path_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_success: path_params_integer_path_parameter_success,
		},
	};
}

/**
 * Handler for GET /repos/{owner}
 */
async function pathParams34StringPatternPathFailure(owner: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (owner !== null && owner !== undefined) {
		result["owner"] = owner;
	}
	return result;
}

export function createAppPathParams34StringPatternPathFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/repos/{owner}",
		handler_name: "path_params_34_string_pattern_path_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"owner":{"pattern":"^[a-zA-Z0-9-]+$","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_34_string_pattern_path_failure: path_params_34_string_pattern_path_failure,
		},
	};
}

/**
 * Handler for GET /items/{id}
 */
async function pathParams21UuidV5PathParamSuccess(id: string): Promise<any> {
	return {"id":"630eb68f-e0fa-5ecc-887a-7c7a62614681"};
}

export function createAppPathParams21UuidV5PathParamSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/{id}",
		handler_name: "path_params_21_uuid_v5_path_param_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"format":"uuid","type":"string","uuidVersion":"5"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_21_uuid_v5_path_param_success: path_params_21_uuid_v5_path_param_success,
		},
	};
}

/**
 * Handler for GET /path/param-maxlength/{item_id}
 */
async function pathParamsStringPathParameterWithMaxLengthFailure(itemId: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (itemId !== null && itemId !== undefined) {
		result["itemId"] = itemId;
	}
	return result;
}

export function createAppPathParamsStringPathParameterWithMaxLengthFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-maxlength/{item_id}",
		handler_name: "path_params_string_path_parameter_with_max_length_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"maxLength":3,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_string_path_parameter_with_max_length_failure: path_params_string_path_parameter_with_max_length_failure,
		},
	};
}

/**
 * Handler for GET /path/param-minlength/{item_id}
 */
async function pathParamsStringPathParameterWithMinLengthFailure(itemId: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (itemId !== null && itemId !== undefined) {
		result["itemId"] = itemId;
	}
	return result;
}

export function createAppPathParamsStringPathParameterWithMinLengthFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-minlength/{item_id}",
		handler_name: "path_params_string_path_parameter_with_min_length_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"minLength":3,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_string_path_parameter_with_min_length_failure: path_params_string_path_parameter_with_min_length_failure,
		},
	};
}

/**
 * Handler for GET /{version}/{service_id}/{user_id}/{order_id}
 */
async function pathParamsMultiplePathParametersSuccess(orderId: string, serviceId: number, userId: string, version: number): Promise<any> {
	return {"order_id":"c892496f-b1fd-4b91-bdb8-b46f92df1716","service_id":1,"user_id":"abc","version":1.0};
}

export function createAppPathParamsMultiplePathParametersSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/{version}/{service_id}/{user_id}/{order_id}",
		handler_name: "path_params_multiple_path_parameters_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"order_id":{"format":"uuid","type":"string"},"service_id":{"type":"integer"},"user_id":{"type":"string"},"version":{"type":"number"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_multiple_path_parameters_success: path_params_multiple_path_parameters_success,
		},
	};
}

/**
 * Handler for GET /date/{date_param}
 */
async function pathParamsDatePathParameterSuccess(dateParam: Date): Promise<any> {
	return {"date_param":"2023-07-15"};
}

export function createAppPathParamsDatePathParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/date/{date_param}",
		handler_name: "path_params_date_path_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"date_param":{"format":"date","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_date_path_parameter_success: path_params_date_path_parameter_success,
		},
	};
}

/**
 * Handler for GET /path/param-gt/{item_id}
 */
async function pathParamsIntegerPathParameterWithGtConstraintFailure(itemId: number): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (itemId !== null && itemId !== undefined) {
		result["itemId"] = itemId;
	}
	return result;
}

export function createAppPathParamsIntegerPathParameterWithGtConstraintFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-gt/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_gt_constraint_failure",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"exclusiveMinimum":3,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_gt_constraint_failure: path_params_integer_path_parameter_with_gt_constraint_failure,
		},
	};
}

/**
 * Handler for GET /events/{date}
 */
async function pathParams24DateFormatPathParamSuccess(date: Date): Promise<any> {
	return {"date":"2025-10-30"};
}

export function createAppPathParams24DateFormatPathParamSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/events/{date}",
		handler_name: "path_params_24_date_format_path_param_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"date":{"format":"date","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_24_date_format_path_param_success: path_params_24_date_format_path_param_success,
		},
	};
}

/**
 * Handler for GET /path/float/{item_id}
 */
async function pathParamsFloatPathParameterSuccess(itemId: number): Promise<any> {
	return {"item_id":42.5};
}

export function createAppPathParamsFloatPathParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/float/{item_id}",
		handler_name: "path_params_float_path_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"type":"number"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_float_path_parameter_success: path_params_float_path_parameter_success,
		},
	};
}

/**
 * Handler for GET /type-syntax/users/{user_id:int}
 */
async function pathParamsPathParameterWithTypeSyntaxInteger(): Promise<any> {
	return {"user_id":"42"};
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
			path_params_path_parameter_with_type_syntax_integer: path_params_path_parameter_with_type_syntax_integer,
		},
	};
}

/**
 * Handler for GET /path/str/{item_id}
 */
async function pathParamsStringPathParameterSuccess(itemId: string): Promise<any> {
	return {"item_id":"foobar"};
}

export function createAppPathParamsStringPathParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/str/{item_id}",
		handler_name: "path_params_string_path_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_string_path_parameter_success: path_params_string_path_parameter_success,
		},
	};
}

/**
 * Handler for GET /items/{item_id}
 */
async function pathParamsUuidPathParameterSuccess(itemId: string): Promise<any> {
	return {"item_id":"ec38df32-ceda-4cfa-9b4a-1aeb94ad551a"};
}

export function createAppPathParamsUuidPathParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/{item_id}",
		handler_name: "path_params_uuid_path_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"format":"uuid","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_uuid_path_parameter_success: path_params_uuid_path_parameter_success,
		},
	};
}

/**
 * Handler for GET /path/param-ge/{item_id}
 */
async function pathParamsIntegerPathParameterWithGeConstraintSuccess(itemId: number): Promise<any> {
	return {"item_id":3};
}

export function createAppPathParamsIntegerPathParameterWithGeConstraintSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/param-ge/{item_id}",
		handler_name: "path_params_integer_path_parameter_with_ge_constraint_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"minimum":3,"type":"integer"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_integer_path_parameter_with_ge_constraint_success: path_params_integer_path_parameter_with_ge_constraint_success,
		},
	};
}

/**
 * Handler for GET /models/{model_name}
 */
async function pathParamsEnumPathParameterSuccess(modelName: string): Promise<any> {
	return {"model_name":"alexnet"};
}

export function createAppPathParamsEnumPathParameterSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/models/{model_name}",
		handler_name: "path_params_enum_path_parameter_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"model_name":{"enum":["alexnet","lenet","resnet"],"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_enum_path_parameter_success: path_params_enum_path_parameter_success,
		},
	};
}

/**
 * Handler for GET /path/bool/{item_id}
 */
async function pathParamsBooleanPathParameterNumeric1(itemId: boolean): Promise<any> {
	return {"item_id":true};
}

export function createAppPathParamsBooleanPathParameterNumeric1(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/path/bool/{item_id}",
		handler_name: "path_params_boolean_path_parameter_numeric_1",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"item_id":{"type":"boolean"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			path_params_boolean_path_parameter_numeric_1: path_params_boolean_path_parameter_numeric_1,
		},
	};
}

/**
 * Handler for POST /items/
 */
async function contentTypes415UnsupportedMediaType(body: any): Promise<any> {
	return {"detail":"Unsupported media type"};
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
			content_types_415_unsupported_media_type: content_types_415_unsupported_media_type,
		},
	};
}

/**
 * Handler for GET /xml
 */
async function contentTypesXmlResponseApplicationXml(): Promise<any> {
	return "<?xml version=\"1.0\"?><item><name>Item</name><price>42.0</price></item>";
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
			content_types_xml_response_application_xml: content_types_xml_response_application_xml,
		},
	};
}

/**
 * Handler for POST /data
 */
async function contentTypes14ContentTypeCaseInsensitive(body: any): Promise<any> {
	return {"name":"test"};
}

export function createAppContentTypes14ContentTypeCaseInsensitive(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_14_content_type_case_insensitive",
		request_schema: {"properties":{"name":{"type":"string"}},"required":["name"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_14_content_type_case_insensitive: content_types_14_content_type_case_insensitive,
		},
	};
}

/**
 * Handler for GET /items/unicode
 */
async function contentTypesJsonWithUtf8Charset(): Promise<any> {
	return {"emoji":"☕","name":"Café"};
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
			content_types_json_with_utf_8_charset: content_types_json_with_utf_8_charset,
		},
	};
}

/**
 * Handler for POST /data
 */
async function contentTypes16TextPlainNotAccepted(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppContentTypes16TextPlainNotAccepted(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_16_text_plain_not_accepted",
		request_schema: {"properties":{"data":{"type":"string"}},"required":["data"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_16_text_plain_not_accepted: content_types_16_text_plain_not_accepted,
		},
	};
}

/**
 * Handler for GET /download/document.pdf
 */
async function contentTypesPdfResponseApplicationPdf(): Promise<any> {
	return "pdf_binary_data";
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
			content_types_pdf_response_application_pdf: content_types_pdf_response_application_pdf,
		},
	};
}

/**
 * Handler for POST /data
 */
async function contentTypes20ContentLengthMismatch(body: any, contentLength: string): Promise<any> {
	return {"error":"Content-Length header does not match actual body size"};
}

export function createAppContentTypes20ContentLengthMismatch(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_20_content_length_mismatch",
		request_schema: {"properties":{"value":{"type":"string"}},"type":"object"},
		response_schema: undefined,
		parameter_schema: {"headers":{"Content-Length":{"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_20_content_length_mismatch: content_types_20_content_length_mismatch,
		},
	};
}

/**
 * Handler for POST /api/v1/resource
 */
async function contentTypes17VendorJsonAccepted(body: any): Promise<any> {
	return {"data":"value"};
}

export function createAppContentTypes17VendorJsonAccepted(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/api/v1/resource",
		handler_name: "content_types_17_vendor_json_accepted",
		request_schema: {"properties":{"data":{"type":"string"}},"required":["data"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_17_vendor_json_accepted: content_types_17_vendor_json_accepted,
		},
	};
}

/**
 * Handler for POST /data
 */
async function contentTypes13JsonWithCharsetUtf16(body: any): Promise<any> {
	return {"error":"Unsupported charset 'utf-16' for JSON. Only UTF-8 is supported."};
}

export function createAppContentTypes13JsonWithCharsetUtf16(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_13_json_with_charset_utf16",
		request_schema: {"properties":{"value":{"type":"string"}},"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_13_json_with_charset_utf16: content_types_13_json_with_charset_utf16,
		},
	};
}

/**
 * Handler for GET /items/json
 */
async function contentTypesJsonResponseApplicationJson(): Promise<any> {
	return {"name":"Item","price":42.0};
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
			content_types_json_response_application_json: content_types_json_response_application_json,
		},
	};
}

/**
 * Handler for POST /upload
 */
async function contentTypes15MultipartBoundaryRequired(): Promise<any> {
	return {"error":"multipart/form-data requires 'boundary' parameter"};
}

export function createAppContentTypes15MultipartBoundaryRequired(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "content_types_15_multipart_boundary_required",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"files":{"document":{"required":true}}},
		file_params: {"document":{"required":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_15_multipart_boundary_required: content_types_15_multipart_boundary_required,
		},
	};
}

/**
 * Handler for GET /accept-test/{id}
 */
async function contentTypesContentNegotiationAcceptHeader(id: string): Promise<any> {
	return {"id":1,"name":"Item"};
}

export function createAppContentTypesContentNegotiationAcceptHeader(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/accept-test/{id}",
		handler_name: "content_types_content_negotiation_accept_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"path":{"id":{"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_content_negotiation_accept_header: content_types_content_negotiation_accept_header,
		},
	};
}

/**
 * Handler for GET /html
 */
async function contentTypesHtmlResponseTextHtml(): Promise<any> {
	return "<html><body><h1>Hello</h1></body></html>";
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
			content_types_html_response_text_html: content_types_html_response_text_html,
		},
	};
}

/**
 * Handler for GET /images/photo.jpg
 */
async function contentTypesJpegImageResponseImageJpeg(): Promise<any> {
	return "jpeg_binary_data";
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
			content_types_jpeg_image_response_image_jpeg: content_types_jpeg_image_response_image_jpeg,
		},
	};
}

/**
 * Handler for POST /data
 */
async function contentTypes19MissingContentTypeDefaultJson(body: any): Promise<any> {
	return {"name":"test"};
}

export function createAppContentTypes19MissingContentTypeDefaultJson(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_19_missing_content_type_default_json",
		request_schema: {"properties":{"name":{"type":"string"}},"required":["name"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_19_missing_content_type_default_json: content_types_19_missing_content_type_default_json,
		},
	};
}

/**
 * Handler for GET /images/logo.png
 */
async function contentTypesPngImageResponseImagePng(): Promise<any> {
	return "png_binary_data";
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
			content_types_png_image_response_image_png: content_types_png_image_response_image_png,
		},
	};
}

/**
 * Handler for GET /text
 */
async function contentTypesPlainTextResponseTextPlain(): Promise<any> {
	return "Hello, World!";
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
			content_types_plain_text_response_text_plain: content_types_plain_text_response_text_plain,
		},
	};
}

/**
 * Handler for POST /data
 */
async function contentTypes18ContentTypeWithMultipleParams(body: any): Promise<any> {
	return {"value":"test"};
}

export function createAppContentTypes18ContentTypeWithMultipleParams(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "content_types_18_content_type_with_multiple_params",
		request_schema: {"properties":{"value":{"type":"string"}},"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			content_types_18_content_type_with_multiple_params: content_types_18_content_type_with_multiple_params,
		},
	};
}

/**
 * Handler for GET /export/data.csv
 */
async function contentTypesCsvResponseTextCsv(): Promise<any> {
	return "id,name,price\n1,Item A,10.0\n2,Item B,20.0";
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
			content_types_csv_response_text_csv: content_types_csv_response_text_csv,
		},
	};
}

/**
 * Handler for GET /download/file.bin
 */
async function contentTypesBinaryResponseApplicationOctetStream(): Promise<any> {
	return "binary_data_placeholder";
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
			content_types_binary_response_application_octet_stream: content_types_binary_response_application_octet_stream,
		},
	};
}

/**
 * Handler for POST /login/
 */
async function urlEncodedSimpleFormSubmissionSuccess(body: any): Promise<any> {
	return {"username":"johndoe"};
}

export function createAppUrlEncodedSimpleFormSubmissionSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/login/",
		handler_name: "url_encoded_simple_form_submission_success",
		request_schema: {"properties":{"password":{"type":"string"},"username":{"type":"string"}},"required":["username","password"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_simple_form_submission_success: url_encoded_simple_form_submission_success,
		},
	};
}

/**
 * Handler for POST /data
 */
async function urlEncoded15SpecialCharactersFieldNames(body: any): Promise<any> {
	return {"contact.email":"john@example.com","user-name":"JohnDoe"};
}

export function createAppUrlEncoded15SpecialCharactersFieldNames(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/data",
		handler_name: "url_encoded_15_special_characters_field_names",
		request_schema: {"properties":{"contact.email":{"format":"email","type":"string"},"user-name":{"type":"string"}},"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_15_special_characters_field_names: url_encoded_15_special_characters_field_names,
		},
	};
}

/**
 * Handler for POST /form/validated
 */
async function urlEncodedPatternValidationFail(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppUrlEncodedPatternValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/validated",
		handler_name: "url_encoded_pattern_validation_fail",
		request_schema: {"properties":{"username":{"pattern":"^[a-z0-9_]+$","type":"string"}},"required":["username"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_pattern_validation_fail: url_encoded_pattern_validation_fail,
		},
	};
}

/**
 * Handler for POST /settings
 */
async function urlEncoded22AdditionalPropertiesStrictFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppUrlEncoded22AdditionalPropertiesStrictFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/settings",
		handler_name: "url_encoded_22_additional_properties_strict_failure",
		request_schema: {"additionalProperties":false,"properties":{"theme":{"enum":["light","dark"],"type":"string"}},"required":["theme"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_22_additional_properties_strict_failure: url_encoded_22_additional_properties_strict_failure,
		},
	};
}

/**
 * Handler for POST /accounts
 */
async function urlEncoded17PatternValidationFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppUrlEncoded17PatternValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/accounts",
		handler_name: "url_encoded_17_pattern_validation_failure",
		request_schema: {"properties":{"account_id":{"pattern":"^ACC-[0-9]{6}$","type":"string"}},"required":["account_id"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_17_pattern_validation_failure: url_encoded_17_pattern_validation_failure,
		},
	};
}

/**
 * Handler for POST /subscribe
 */
async function urlEncoded20FormatEmailValidationFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppUrlEncoded20FormatEmailValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/subscribe",
		handler_name: "url_encoded_20_format_email_validation_failure",
		request_schema: {"properties":{"email":{"format":"email","type":"string"}},"required":["email"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_20_format_email_validation_failure: url_encoded_20_format_email_validation_failure,
		},
	};
}

/**
 * Handler for POST /form/tags
 */
async function urlEncodedMultipleValuesForSameField(body: any): Promise<any> {
	return {"tags":["python","fastapi","web"]};
}

export function createAppUrlEncodedMultipleValuesForSameField(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/tags",
		handler_name: "url_encoded_multiple_values_for_same_field",
		request_schema: {"properties":{"tags":{"items":{"type":"string"},"type":"array"}},"required":["tags"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_multiple_values_for_same_field: url_encoded_multiple_values_for_same_field,
		},
	};
}

/**
 * Handler for POST /login/
 */
async function urlEncodedRequiredFieldMissingValidationError(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppUrlEncodedRequiredFieldMissingValidationError(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/login/",
		handler_name: "url_encoded_required_field_missing_validation_error",
		request_schema: {"properties":{"password":{"type":"string"},"username":{"type":"string"}},"required":["username","password"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_required_field_missing_validation_error: url_encoded_required_field_missing_validation_error,
		},
	};
}

/**
 * Handler for POST /register
 */
async function urlEncoded13ArrayFieldSuccess(body: any): Promise<any> {
	return {"tags":["python","rust","typescript"]};
}

export function createAppUrlEncoded13ArrayFieldSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/register",
		handler_name: "url_encoded_13_array_field_success",
		request_schema: {"properties":{"tags":{"items":{"type":"string"},"minItems":1,"type":"array"}},"required":["tags"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_13_array_field_success: url_encoded_13_array_field_success,
		},
	};
}

/**
 * Handler for POST /form/
 */
async function urlEncodedNumericFieldTypeConversion(body: any): Promise<any> {
	return {"age":30,"username":"johndoe"};
}

export function createAppUrlEncodedNumericFieldTypeConversion(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_numeric_field_type_conversion",
		request_schema: {"properties":{"age":{"type":"integer"},"username":{"type":"string"}},"required":["username"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_numeric_field_type_conversion: url_encoded_numeric_field_type_conversion,
		},
	};
}

/**
 * Handler for POST /form/
 */
async function urlEncodedSpecialCharactersEncoding(body: any): Promise<any> {
	return {"description":"Test & Development","name":"John Doe"};
}

export function createAppUrlEncodedSpecialCharactersEncoding(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_special_characters_encoding",
		request_schema: {"properties":{"description":{"type":"string"},"name":{"type":"string"}},"required":["name"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_special_characters_encoding: url_encoded_special_characters_encoding,
		},
	};
}

/**
 * Handler for POST /form/
 */
async function urlEncodedBooleanFieldConversion(body: any): Promise<any> {
	return {"subscribe":true,"username":"johndoe"};
}

export function createAppUrlEncodedBooleanFieldConversion(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_boolean_field_conversion",
		request_schema: {"properties":{"subscribe":{"type":"boolean"},"username":{"type":"string"}},"required":["username"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_boolean_field_conversion: url_encoded_boolean_field_conversion,
		},
	};
}

/**
 * Handler for POST /form/
 */
async function urlEncodedEmptyStringValue(body: any): Promise<any> {
	return {"description":"","username":"johndoe"};
}

export function createAppUrlEncodedEmptyStringValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/",
		handler_name: "url_encoded_empty_string_value",
		request_schema: {"properties":{"description":{"type":"string"},"username":{"type":"string"}},"required":["username"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_empty_string_value: url_encoded_empty_string_value,
		},
	};
}

/**
 * Handler for POST /token
 */
async function urlEncodedOauth2PasswordGrantFlow(body: any): Promise<any> {
	return {"access_token":"johndoe","token_type":"bearer"};
}

export function createAppUrlEncodedOauth2PasswordGrantFlow(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/token",
		handler_name: "url_encoded_oauth2_password_grant_flow",
		request_schema: {"properties":{"grant_type":{"type":"string"},"password":{"type":"string"},"scope":{"type":"string"},"username":{"type":"string"}},"required":["username","password","grant_type"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_oauth2_password_grant_flow: url_encoded_oauth2_password_grant_flow,
		},
	};
}

/**
 * Handler for POST /tags
 */
async function urlEncoded19ArrayMinitemsValidationFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppUrlEncoded19ArrayMinitemsValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/tags",
		handler_name: "url_encoded_19_array_minitems_validation_failure",
		request_schema: {"properties":{"tags":{"items":{"type":"string"},"minItems":2,"type":"array"}},"required":["tags"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_19_array_minitems_validation_failure: url_encoded_19_array_minitems_validation_failure,
		},
	};
}

/**
 * Handler for POST /register/
 */
async function urlEncodedOptionalFieldMissingSuccess(body: any): Promise<any> {
	return {"email":null,"username":"johndoe"};
}

export function createAppUrlEncodedOptionalFieldMissingSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/register/",
		handler_name: "url_encoded_optional_field_missing_success",
		request_schema: {"properties":{"email":{"format":"email","type":["string","null"]},"password":{"type":"string"},"username":{"type":"string"}},"required":["username","password"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_optional_field_missing_success: url_encoded_optional_field_missing_success,
		},
	};
}

/**
 * Handler for POST /profile
 */
async function urlEncoded14NestedObjectBracketNotation(body: any): Promise<any> {
	return {"user":{"age":30,"email":"john@example.com","name":"John Doe"}};
}

export function createAppUrlEncoded14NestedObjectBracketNotation(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/profile",
		handler_name: "url_encoded_14_nested_object_bracket_notation",
		request_schema: {"properties":{"user":{"properties":{"age":{"minimum":0,"type":"integer"},"email":{"format":"email","type":"string"},"name":{"minLength":1,"type":"string"}},"required":["name","email"],"type":"object"}},"required":["user"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_14_nested_object_bracket_notation: url_encoded_14_nested_object_bracket_notation,
		},
	};
}

/**
 * Handler for POST /form/validated
 */
async function urlEncodedStringMaxLengthValidationFail(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppUrlEncodedStringMaxLengthValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/validated",
		handler_name: "url_encoded_string_max_length_validation_fail",
		request_schema: {"properties":{"username":{"maxLength":20,"type":"string"}},"required":["username"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_string_max_length_validation_fail: url_encoded_string_max_length_validation_fail,
		},
	};
}

/**
 * Handler for POST /products
 */
async function urlEncoded18IntegerMinimumValidationFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppUrlEncoded18IntegerMinimumValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/products",
		handler_name: "url_encoded_18_integer_minimum_validation_failure",
		request_schema: {"properties":{"quantity":{"minimum":1,"type":"integer"}},"required":["quantity"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_18_integer_minimum_validation_failure: url_encoded_18_integer_minimum_validation_failure,
		},
	};
}

/**
 * Handler for POST /products
 */
async function urlEncoded21IntegerTypeCoercionFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppUrlEncoded21IntegerTypeCoercionFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/products",
		handler_name: "url_encoded_21_integer_type_coercion_failure",
		request_schema: {"properties":{"price":{"type":"integer"}},"required":["price"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_21_integer_type_coercion_failure: url_encoded_21_integer_type_coercion_failure,
		},
	};
}

/**
 * Handler for POST /users
 */
async function urlEncoded16MinlengthValidationFailure(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppUrlEncoded16MinlengthValidationFailure(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/users",
		handler_name: "url_encoded_16_minlength_validation_failure",
		request_schema: {"properties":{"username":{"minLength":3,"type":"string"}},"required":["username"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_16_minlength_validation_failure: url_encoded_16_minlength_validation_failure,
		},
	};
}

/**
 * Handler for POST /form/validated
 */
async function urlEncodedStringMinLengthValidationFail(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppUrlEncodedStringMinLengthValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/form/validated",
		handler_name: "url_encoded_string_min_length_validation_fail",
		request_schema: {"properties":{"username":{"minLength":3,"type":"string"}},"required":["username"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			url_encoded_string_min_length_validation_fail: url_encoded_string_min_length_validation_fail,
		},
	};
}

/**
 * Handler for GET /headers/pattern
 */
async function headersHeaderRegexValidationSuccess(xRequestId: string): Promise<any> {
	return {"x_request_id":"12345"};
}

export function createAppHeadersHeaderRegexValidationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/pattern",
		handler_name: "headers_header_regex_validation_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"X-Request-Id":{"annotation":"str","pattern":"^[0-9]{3,}$","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_regex_validation_success: headers_header_regex_validation_success,
		},
	};
}

/**
 * Handler for GET /api/data
 */
async function headers33ApiKeyHeaderValid(xAPIKey: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (xAPIKey !== null && xAPIKey !== undefined) {
		result["xAPIKey"] = xAPIKey;
	}
	return result;
}

export function createAppHeaders33ApiKeyHeaderValid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "headers_33_api_key_header_valid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"X-API-Key":{"pattern":"^[a-f0-9]{32}$","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_33_api_key_header_valid: headers_33_api_key_header_valid,
		},
	};
}

/**
 * Handler for GET /headers/content-type
 */
async function headersContentTypeHeaderApplicationJson(contentType: string): Promise<any> {
	return {"content_type":"application/json"};
}

export function createAppHeadersContentTypeHeaderApplicationJson(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/content-type",
		handler_name: "headers_content_type_header_application_json",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Content-Type":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_content_type_header_application_json: headers_content_type_header_application_json,
		},
	};
}

/**
 * Handler for GET /headers/accept-language
 */
async function headersAcceptLanguageHeader(acceptLanguage: string): Promise<any> {
	return {"accept_language":"en-US,en;q=0.9"};
}

export function createAppHeadersAcceptLanguageHeader(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/accept-language",
		handler_name: "headers_accept_language_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Accept-Language":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_accept_language_header: headers_accept_language_header,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function headersXApiKeyRequiredHeaderSuccess(key: string): Promise<any> {
	return {"username":"secret"};
}

export function createAppHeadersXApiKeyRequiredHeaderSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_x_api_key_required_header_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"key":{"annotation":"str","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_x_api_key_required_header_success: headers_x_api_key_required_header_success,
		},
	};
}

/**
 * Handler for GET /headers/max-length
 */
async function headersHeaderValidationMaxLengthConstraintFail(xSessionId: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (xSessionId !== null && xSessionId !== undefined) {
		result["xSessionId"] = xSessionId;
	}
	return result;
}

export function createAppHeadersHeaderValidationMaxLengthConstraintFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/max-length",
		handler_name: "headers_header_validation_max_length_constraint_fail",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"X-Session-Id":{"annotation":"str","maxLength":20,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_validation_max_length_constraint_fail: headers_header_validation_max_length_constraint_fail,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function headersXApiKeyRequiredHeaderMissing(xAPIKey: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (xAPIKey !== null && xAPIKey !== undefined) {
		result["xAPIKey"] = xAPIKey;
	}
	return result;
}

export function createAppHeadersXApiKeyRequiredHeaderMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_x_api_key_required_header_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"X-API-Key":{"annotation":"str","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_x_api_key_required_header_missing: headers_x_api_key_required_header_missing,
		},
	};
}

/**
 * Handler for GET /headers/origin
 */
async function headersOriginHeader(origin: string): Promise<any> {
	return {"origin":"https://example.com"};
}

export function createAppHeadersOriginHeader(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/origin",
		handler_name: "headers_origin_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Origin":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_origin_header: headers_origin_header,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function headersUserAgentHeaderDefaultValue(userAgent: string): Promise<any> {
	return {"User-Agent":"testclient"};
}

export function createAppHeadersUserAgentHeaderDefaultValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "headers_user_agent_header_default_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"User-Agent":{"annotation":"str","default":"testclient","optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_user_agent_header_default_value: headers_user_agent_header_default_value,
		},
	};
}

/**
 * Handler for GET /protected
 */
async function headers32BearerTokenMissingPrefix(authorization: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (authorization !== null && authorization !== undefined) {
		result["authorization"] = authorization;
	}
	return result;
}

export function createAppHeaders32BearerTokenMissingPrefix(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/protected",
		handler_name: "headers_32_bearer_token_missing_prefix",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Authorization":{"pattern":"^Bearer [A-Za-z0-9-._~+/]+=*$","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_32_bearer_token_missing_prefix: headers_32_bearer_token_missing_prefix,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function headersOptionalHeaderWithNoneDefaultMissing(strangeHeader: string): Promise<any> {
	return {"strange_header":null};
}

export function createAppHeadersOptionalHeaderWithNoneDefaultMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "headers_optional_header_with_none_default_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"strange-header":{"annotation":"str","default":null,"optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_optional_header_with_none_default_missing: headers_optional_header_with_none_default_missing,
		},
	};
}

/**
 * Handler for GET /headers/pattern
 */
async function headersHeaderRegexValidationFail(xRequestId: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (xRequestId !== null && xRequestId !== undefined) {
		result["xRequestId"] = xRequestId;
	}
	return result;
}

export function createAppHeadersHeaderRegexValidationFail(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/pattern",
		handler_name: "headers_header_regex_validation_fail",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"X-Request-Id":{"annotation":"str","pattern":"^[0-9]{3,}$","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_regex_validation_fail: headers_header_regex_validation_fail,
		},
	};
}

/**
 * Handler for GET /protected
 */
async function headers31BearerTokenFormatInvalid(authorization: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (authorization !== null && authorization !== undefined) {
		result["authorization"] = authorization;
	}
	return result;
}

export function createAppHeaders31BearerTokenFormatInvalid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/protected",
		handler_name: "headers_31_bearer_token_format_invalid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Authorization":{"pattern":"^Bearer [A-Za-z0-9-._~+/]+=*$","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_31_bearer_token_format_invalid: headers_31_bearer_token_format_invalid,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function headersXApiKeyOptionalHeaderSuccess(key: string): Promise<any> {
	return {"msg":"Hello secret"};
}

export function createAppHeadersXApiKeyOptionalHeaderSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_x_api_key_optional_header_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"key":{"annotation":"str","optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_x_api_key_optional_header_success: headers_x_api_key_optional_header_success,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function headersAuthorizationHeaderSuccess(authorization: string): Promise<any> {
	return {"credentials":"foobar","scheme":"Digest"};
}

export function createAppHeadersAuthorizationHeaderSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_authorization_header_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Authorization":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_authorization_header_success: headers_authorization_header_success,
		},
	};
}

/**
 * Handler for GET /protected
 */
async function headers30BearerTokenFormatValid(authorization: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (authorization !== null && authorization !== undefined) {
		result["authorization"] = authorization;
	}
	return result;
}

export function createAppHeaders30BearerTokenFormatValid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/protected",
		handler_name: "headers_30_bearer_token_format_valid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Authorization":{"pattern":"^Bearer [A-Za-z0-9-._~+/]+=*$","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_30_bearer_token_format_valid: headers_30_bearer_token_format_valid,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function headersAuthorizationHeaderMissing(authorization: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (authorization !== null && authorization !== undefined) {
		result["authorization"] = authorization;
	}
	return result;
}

export function createAppHeadersAuthorizationHeaderMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_authorization_header_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Authorization":{"annotation":"str","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_authorization_header_missing: headers_authorization_header_missing,
		},
	};
}

/**
 * Handler for GET /headers/accept
 */
async function headersAcceptHeaderJson(accept: string): Promise<any> {
	return {"accept":"application/json"};
}

export function createAppHeadersAcceptHeaderJson(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/accept",
		handler_name: "headers_accept_header_json",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Accept":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_accept_header_json: headers_accept_header_json,
		},
	};
}

/**
 * Handler for GET /headers/accept-encoding
 */
async function headersAcceptEncodingHeader(acceptEncoding: string): Promise<any> {
	return {"accept_encoding":"gzip, deflate, br"};
}

export function createAppHeadersAcceptEncodingHeader(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/accept-encoding",
		handler_name: "headers_accept_encoding_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Accept-Encoding":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_accept_encoding_header: headers_accept_encoding_header,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function headersAuthorizationHeaderWrongScheme(authorization: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (authorization !== null && authorization !== undefined) {
		result["authorization"] = authorization;
	}
	return result;
}

export function createAppHeadersAuthorizationHeaderWrongScheme(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_authorization_header_wrong_scheme",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Authorization":{"annotation":"str","pattern":"^Digest .+","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_authorization_header_wrong_scheme: headers_authorization_header_wrong_scheme,
		},
	};
}

/**
 * Handler for GET /headers/validated
 */
async function headersHeaderValidationMinLengthConstraint(xToken: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (xToken !== null && xToken !== undefined) {
		result["xToken"] = xToken;
	}
	return result;
}

export function createAppHeadersHeaderValidationMinLengthConstraint(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/validated",
		handler_name: "headers_header_validation_min_length_constraint",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"X-Token":{"annotation":"str","minLength":3,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_validation_min_length_constraint: headers_header_validation_min_length_constraint,
		},
	};
}

/**
 * Handler for GET /headers/basic-auth
 */
async function headersBasicAuthenticationSuccess(authorization: string): Promise<any> {
	return {"password":"password","username":"username"};
}

export function createAppHeadersBasicAuthenticationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/basic-auth",
		handler_name: "headers_basic_authentication_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Authorization":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_basic_authentication_success: headers_basic_authentication_success,
		},
	};
}

/**
 * Handler for GET /headers/bearer-auth
 */
async function headersBearerTokenAuthenticationMissing(authorization: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (authorization !== null && authorization !== undefined) {
		result["authorization"] = authorization;
	}
	return result;
}

export function createAppHeadersBearerTokenAuthenticationMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/bearer-auth",
		handler_name: "headers_bearer_token_authentication_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Authorization":{"annotation":"str","pattern":"^Bearer .+","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_bearer_token_authentication_missing: headers_bearer_token_authentication_missing,
		},
	};
}

/**
 * Handler for GET /users/me
 */
async function headersXApiKeyOptionalHeaderMissing(key: string): Promise<any> {
	return {"msg":"Hello World"};
}

export function createAppHeadersXApiKeyOptionalHeaderMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/users/me",
		handler_name: "headers_x_api_key_optional_header_missing",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"key":{"annotation":"str","optional":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_x_api_key_optional_header_missing: headers_x_api_key_optional_header_missing,
		},
	};
}

/**
 * Handler for GET /headers/multiple
 */
async function headersMultipleCustomHeaders(xClientVersion: string, xRequestId: string, xTraceId: string): Promise<any> {
	return {"x_client_version":"1.2.3","x_request_id":"req-12345","x_trace_id":"trace-abc"};
}

export function createAppHeadersMultipleCustomHeaders(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/multiple",
		handler_name: "headers_multiple_custom_headers",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"X-Client-Version":{"annotation":"str","type":"string"},"X-Request-Id":{"annotation":"str","type":"string"},"X-Trace-Id":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_multiple_custom_headers: headers_multiple_custom_headers,
		},
	};
}

/**
 * Handler for GET /api/data
 */
async function headers34ApiKeyHeaderInvalid(xAPIKey: string): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (xAPIKey !== null && xAPIKey !== undefined) {
		result["xAPIKey"] = xAPIKey;
	}
	return result;
}

export function createAppHeaders34ApiKeyHeaderInvalid(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/api/data",
		handler_name: "headers_34_api_key_header_invalid",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"X-API-Key":{"pattern":"^[a-f0-9]{32}$","required":true,"type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_34_api_key_header_invalid: headers_34_api_key_header_invalid,
		},
	};
}

/**
 * Handler for GET /headers/bearer-auth
 */
async function headersBearerTokenAuthenticationSuccess(authorization: string): Promise<any> {
	return {"token":"valid_token_123"};
}

export function createAppHeadersBearerTokenAuthenticationSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/bearer-auth",
		handler_name: "headers_bearer_token_authentication_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Authorization":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_bearer_token_authentication_success: headers_bearer_token_authentication_success,
		},
	};
}

/**
 * Handler for GET /headers/host
 */
async function headersHostHeader(host: string): Promise<any> {
	return {"host":"example.com:8080"};
}

export function createAppHeadersHostHeader(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/host",
		handler_name: "headers_host_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Host":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_host_header: headers_host_header,
		},
	};
}

/**
 * Handler for GET /headers/referer
 */
async function headersRefererHeader(referer: string): Promise<any> {
	return {"referer":"https://example.com/page"};
}

export function createAppHeadersRefererHeader(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/referer",
		handler_name: "headers_referer_header",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"Referer":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_referer_header: headers_referer_header,
		},
	};
}

/**
 * Handler for GET /headers/underscore
 */
async function headersHeaderWithUnderscoreConversionExplicit(xToken: string): Promise<any> {
	return {"x_token":"secret123"};
}

export function createAppHeadersHeaderWithUnderscoreConversionExplicit(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/headers/underscore",
		handler_name: "headers_header_with_underscore_conversion_explicit",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"X-Token":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_with_underscore_conversion_explicit: headers_header_with_underscore_conversion_explicit,
		},
	};
}

/**
 * Handler for POST /echo
 */
async function headersHeaderCaseInsensitivityAccess(body: any): Promise<any> {
	return {"content_type_lower":"application/json","content_type_mixed":"application/json","content_type_upper":"application/json"};
}

export function createAppHeadersHeaderCaseInsensitivityAccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/echo",
		handler_name: "headers_header_case_insensitivity_access",
		request_schema: {"additionalProperties":false,"properties":{"test":{"type":"string"}},"required":["test"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_header_case_insensitivity_access: headers_header_case_insensitivity_access,
		},
	};
}

/**
 * Handler for GET /items/
 */
async function headersUserAgentHeaderCustomValue(userAgent: string): Promise<any> {
	return {"User-Agent":"Mozilla/5.0 Custom Browser"};
}

export function createAppHeadersUserAgentHeaderCustomValue(): SpikardApp {
	const route: RouteMetadata = {
		method: "GET",
		path: "/items/",
		handler_name: "headers_user_agent_header_custom_value",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"headers":{"User-Agent":{"annotation":"str","type":"string"}}},
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			headers_user_agent_header_custom_value: headers_user_agent_header_custom_value,
		},
	};
}

/**
 * Handler for POST /
 */
async function multipartMultipleValuesForSameFieldName(body: any): Promise<any> {
	return {"files":[{"content":"first file","content_type":"text/plain","filename":"file1.txt","size":10},{"content":"second file","content_type":"text/plain","filename":"file2.txt","size":11}],"tags":["python","rust","web"]};
}

export function createAppMultipartMultipleValuesForSameFieldName(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_multiple_values_for_same_field_name",
		request_schema: {"additionalProperties":false,"properties":{"files":{"items":{"format":"binary","type":"string"},"type":"array"},"tags":{"items":{"type":"string"},"type":"array"}},"required":["files"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_multiple_values_for_same_field_name: multipart_multiple_values_for_same_field_name,
		},
	};
}

/**
 * Handler for POST /upload
 */
async function multipart19FileMimeSpoofingPngAsJpeg(): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	return result;
}

export function createAppMultipart19FileMimeSpoofingPngAsJpeg(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_19_file_mime_spoofing_png_as_jpeg",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"files":{"image":{"content_type":["image/jpeg"],"required":true,"validate_magic_numbers":true}}},
		file_params: {"image":{"content_type":["image/jpeg"],"required":true,"validate_magic_numbers":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_19_file_mime_spoofing_png_as_jpeg: multipart_19_file_mime_spoofing_png_as_jpeg,
		},
	};
}

/**
 * Handler for POST /upload
 */
async function multipart20FileMimeSpoofingJpegAsPng(): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	return result;
}

export function createAppMultipart20FileMimeSpoofingJpegAsPng(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_20_file_mime_spoofing_jpeg_as_png",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"files":{"image":{"content_type":["image/png"],"required":true,"validate_magic_numbers":true}}},
		file_params: {"image":{"content_type":["image/png"],"required":true,"validate_magic_numbers":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_20_file_mime_spoofing_jpeg_as_png: multipart_20_file_mime_spoofing_jpeg_as_png,
		},
	};
}

/**
 * Handler for POST /upload
 */
async function multipart21FilePdfMagicNumberSuccess(): Promise<any> {
	return { status: 201 };
}

export function createAppMultipart21FilePdfMagicNumberSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_21_file_pdf_magic_number_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"files":{"document":{"content_type":["application/pdf"],"required":true,"validate_magic_numbers":true}}},
		file_params: {"document":{"content_type":["application/pdf"],"required":true,"validate_magic_numbers":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_21_file_pdf_magic_number_success: multipart_21_file_pdf_magic_number_success,
		},
	};
}

/**
 * Handler for POST /files/images-only
 */
async function multipartContentTypeValidationInvalidType(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppMultipartContentTypeValidationInvalidType(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/images-only",
		handler_name: "multipart_content_type_validation_invalid_type",
		request_schema: {"additionalProperties":false,"properties":{"file":{"format":"binary","type":"string"}},"type":"object"},
		response_schema: undefined,
		parameter_schema: {"files":{"file":{"content_type":["image/jpeg","image/png","image/gif"],"required":true}}},
		file_params: {"file":{"content_type":["image/jpeg","image/png","image/gif"],"required":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_content_type_validation_invalid_type: multipart_content_type_validation_invalid_type,
		},
	};
}

/**
 * Handler for POST /files/document
 */
async function multipartPdfFileUpload(body: any): Promise<any> {
	return {"content_type":"application/pdf","filename":"report.pdf","size":16};
}

export function createAppMultipartPdfFileUpload(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/document",
		handler_name: "multipart_pdf_file_upload",
		request_schema: {"additionalProperties":false,"properties":{"document":{"format":"binary","type":"string"}},"required":["document"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_pdf_file_upload: multipart_pdf_file_upload,
		},
	};
}

/**
 * Handler for POST /files/list
 */
async function multipartFileListUploadArrayOfFiles(body: any): Promise<any> {
	return {"filenames":["file1.txt","file2.txt"],"total_size":35};
}

export function createAppMultipartFileListUploadArrayOfFiles(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/list",
		handler_name: "multipart_file_list_upload_array_of_files",
		request_schema: {"additionalProperties":false,"properties":{"files":{"items":{"format":"binary","type":"string"},"type":"array"}},"required":["files"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_file_list_upload_array_of_files: multipart_file_list_upload_array_of_files,
		},
	};
}

/**
 * Handler for POST /files/optional
 */
async function multipartOptionalFileUploadProvided(body: any): Promise<any> {
	return {"content_type":"text/plain","filename":"optional.txt","size":21};
}

export function createAppMultipartOptionalFileUploadProvided(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/optional",
		handler_name: "multipart_optional_file_upload_provided",
		request_schema: {"additionalProperties":false,"properties":{"file":{"format":"binary","type":"string"}},"required":["file"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_optional_file_upload_provided: multipart_optional_file_upload_provided,
		},
	};
}

/**
 * Handler for POST /files/validated
 */
async function multipartFileSizeValidationTooLarge(body: any): Promise<any> {
	return {"detail":"File too large. Maximum size is 1MB"};
}

export function createAppMultipartFileSizeValidationTooLarge(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/validated",
		handler_name: "multipart_file_size_validation_too_large",
		request_schema: {"additionalProperties":false,"properties":{"file":{"format":"binary","type":"string"}},"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_file_size_validation_too_large: multipart_file_size_validation_too_large,
		},
	};
}

/**
 * Handler for POST /
 */
async function multipartMixedFilesAndFormData(body: any): Promise<any> {
	return {"active":"true","age":"25","file":{"content":"file data here","content_type":"text/plain","filename":"upload.txt","size":14},"username":"testuser"};
}

export function createAppMultipartMixedFilesAndFormData(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_mixed_files_and_form_data",
		request_schema: {"additionalProperties":false,"properties":{"active":{"type":"string"},"age":{"type":"string"},"file":{"format":"binary","type":"string"},"username":{"type":"string"}},"required":["file"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_mixed_files_and_form_data: multipart_mixed_files_and_form_data,
		},
	};
}

/**
 * Handler for POST /
 */
async function multipartSimpleFileUpload(body: any): Promise<any> {
	return {"test":{"content":"<file content>","content_type":"text/plain","filename":"test.txt","size":14}};
}

export function createAppMultipartSimpleFileUpload(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_simple_file_upload",
		request_schema: {"additionalProperties":false,"properties":{"test":{"format":"binary","type":"string"}},"required":["test"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_simple_file_upload: multipart_simple_file_upload,
		},
	};
}

/**
 * Handler for POST /files/upload
 */
async function multipartEmptyFileUpload(body: any): Promise<any> {
	return {"filename":"empty.txt","size":0};
}

export function createAppMultipartEmptyFileUpload(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/upload",
		handler_name: "multipart_empty_file_upload",
		request_schema: {"additionalProperties":false,"properties":{"file":{"format":"binary","type":"string"}},"required":["file"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_empty_file_upload: multipart_empty_file_upload,
		},
	};
}

/**
 * Handler for POST /files/optional
 */
async function multipartOptionalFileUploadMissing(body: any): Promise<any> {
	return {"file":null};
}

export function createAppMultipartOptionalFileUploadMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/optional",
		handler_name: "multipart_optional_file_upload_missing",
		request_schema: {"additionalProperties":false,"properties":{},"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_optional_file_upload_missing: multipart_optional_file_upload_missing,
		},
	};
}

/**
 * Handler for POST /
 */
async function multipartFileUploadWithoutFilename(body: any): Promise<any> {
	return {"test1":"<file1 content>"};
}

export function createAppMultipartFileUploadWithoutFilename(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_file_upload_without_filename",
		request_schema: {"additionalProperties":false,"properties":{"test1":{"format":"binary","type":"string"}},"required":["test1"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_file_upload_without_filename: multipart_file_upload_without_filename,
		},
	};
}

/**
 * Handler for POST /upload
 */
async function multipart18FileMagicNumberJpegSuccess(): Promise<any> {
	return { status: 201 };
}

export function createAppMultipart18FileMagicNumberJpegSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_18_file_magic_number_jpeg_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"files":{"image":{"content_type":["image/jpeg"],"required":true,"validate_magic_numbers":true}}},
		file_params: {"image":{"content_type":["image/jpeg"],"required":true,"validate_magic_numbers":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_18_file_magic_number_jpeg_success: multipart_18_file_magic_number_jpeg_success,
		},
	};
}

/**
 * Handler for POST /upload
 */
async function multipart22FileEmptyBuffer(): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	return result;
}

export function createAppMultipart22FileEmptyBuffer(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_22_file_empty_buffer",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"files":{"file":{"required":true,"validate_magic_numbers":true}}},
		file_params: {"file":{"required":true,"validate_magic_numbers":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_22_file_empty_buffer: multipart_22_file_empty_buffer,
		},
	};
}

/**
 * Handler for POST /upload
 */
async function multipart17FileMagicNumberPngSuccess(): Promise<any> {
	return { status: 201 };
}

export function createAppMultipart17FileMagicNumberPngSuccess(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/upload",
		handler_name: "multipart_17_file_magic_number_png_success",
		request_schema: undefined,
		response_schema: undefined,
		parameter_schema: {"files":{"image":{"content_type":["image/png"],"required":true,"validate_magic_numbers":true}}},
		file_params: {"image":{"content_type":["image/png"],"required":true,"validate_magic_numbers":true}},
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_17_file_magic_number_png_success: multipart_17_file_magic_number_png_success,
		},
	};
}

/**
 * Handler for POST /
 */
async function multipartFormDataWithoutFiles(body: any): Promise<any> {
	return {"some":"data"};
}

export function createAppMultipartFormDataWithoutFiles(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_form_data_without_files",
		request_schema: {"additionalProperties":false,"properties":{"some":{"type":"string"}},"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_form_data_without_files: multipart_form_data_without_files,
		},
	};
}

/**
 * Handler for POST /
 */
async function multipartMultipleFileUploads(body: any): Promise<any> {
	return {"test1":{"content":"<file1 content>","content_type":"text/plain","filename":"test1.txt","size":15},"test2":{"content":"<file2 content>","content_type":"text/plain","filename":"test2.txt","size":15}};
}

export function createAppMultipartMultipleFileUploads(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_multiple_file_uploads",
		request_schema: {"additionalProperties":false,"properties":{"test1":{"format":"binary","type":"string"},"test2":{"format":"binary","type":"string"}},"required":["test1","test2"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_multiple_file_uploads: multipart_multiple_file_uploads,
		},
	};
}

/**
 * Handler for POST /
 */
async function multipartFileUploadWithCustomHeaders(body: any): Promise<any> {
	return {"test2":{"content":"<file2 content>","content_type":"text/plain","filename":"test2.txt","headers":[["content-disposition","form-data; name=\"test2\"; filename=\"test2.txt\""],["content-type","text/plain"],["x-custom","f2"]],"size":15}};
}

export function createAppMultipartFileUploadWithCustomHeaders(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/",
		handler_name: "multipart_file_upload_with_custom_headers",
		request_schema: {"additionalProperties":false,"properties":{"test2":{"format":"binary","type":"string"}},"required":["test2"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_file_upload_with_custom_headers: multipart_file_upload_with_custom_headers,
		},
	};
}

/**
 * Handler for POST /files/required
 */
async function multipartRequiredFileUploadMissing(body: any): Promise<any> {
	// Echo back parameters for testing
	const result: Record<string, any> = {};
	if (body !== null && body !== undefined) {
		Object.assign(result, body);
	}
	return result;
}

export function createAppMultipartRequiredFileUploadMissing(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/required",
		handler_name: "multipart_required_file_upload_missing",
		request_schema: {"additionalProperties":false,"properties":{"file":{"format":"binary","type":"string"}},"required":["file"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_required_file_upload_missing: multipart_required_file_upload_missing,
		},
	};
}

/**
 * Handler for POST /files/image
 */
async function multipartImageFileUpload(body: any): Promise<any> {
	return {"content_type":"image/jpeg","filename":"photo.jpg","size":22};
}

export function createAppMultipartImageFileUpload(): SpikardApp {
	const route: RouteMetadata = {
		method: "POST",
		path: "/files/image",
		handler_name: "multipart_image_file_upload",
		request_schema: {"additionalProperties":false,"properties":{"image":{"format":"binary","type":"string"}},"required":["image"],"type":"object"},
		response_schema: undefined,
		parameter_schema: undefined,
		file_params: undefined,
		is_async: true,
	};

	return {
		routes: [route],
		handlers: {
			multipart_image_file_upload: multipart_image_file_upload,
		},
	};
}

// App factory functions:
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
// - createAppCors07CorsPreflightHeaderNotAllowed() for cors / 07_cors_preflight_header_not_allowed
// - createAppCorsCorsPreflightRequest() for cors / CORS preflight request
// - createAppCorsCorsWithCredentials() for cors / CORS with credentials
// - createAppCors08CorsMaxAge() for cors / 08_cors_max_age
// - createAppCors10CorsOriginNull() for cors / 10_cors_origin_null
// - createAppCorsCorsWildcardOrigin() for cors / CORS wildcard origin
// - createAppCorsCorsRequestBlocked() for cors / CORS request blocked
// - createAppCorsSimpleCorsRequest() for cors / Simple CORS request
// - createAppCors09CorsExposeHeaders() for cors / 09_cors_expose_headers
// - createAppCors06CorsPreflightMethodNotAllowed() for cors / 06_cors_preflight_method_not_allowed
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
