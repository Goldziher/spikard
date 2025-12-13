/**
 * E2E tests for validation_errors
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assertEquals } from "jsr:@std/assert@1";
import {
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
} from "../app/main.ts";

Deno.test("validation_errors: Invalid UUID format", async () => {
	const app = createAppValidationErrorsInvalidUuidFormat();
	const client = new TestClient(app);

	const response = await client.get("/items/not-a-uuid");

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Invalid boolean value", async () => {
	const app = createAppValidationErrorsInvalidBooleanValue();
	const client = new TestClient(app);

	const headers = {
		"x-token": "test-token",
	};
	const response = await client.get("/items/?q=test&is_active=maybe", headers);

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Missing required query parameter", async () => {
	const app = createAppValidationErrorsMissingRequiredQueryParameter();
	const client = new TestClient(app);

	const headers = {
		"x-token": "test-token",
	};
	const response = await client.get("/items/", headers);

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Array max_items constraint violation", async () => {
	const app = createAppValidationErrorsArrayMaxItemsConstraintViolation();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = {
		name: "Item",
		price: 10.0,
		tags: ["tag1", "tag2", "tag3", "tag4", "tag5", "tag6", "tag7", "tag8", "tag9", "tag10", "tag11"],
	};
	const response = await client.post("/items/", { headers, json });

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Numeric constraint violation - gt greater than", async () => {
	const app = createAppValidationErrorsNumericConstraintViolationGtGreaterThan();
	const client = new TestClient(app);

	const headers = {
		"x-token": "test-token",
	};
	const response = await client.get("/items/?q=test&price=0", headers);

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: String regex pattern mismatch", async () => {
	const app = createAppValidationErrorsStringRegexPatternMismatch();
	const client = new TestClient(app);

	const headers = {
		"x-token": "test-token",
	};
	const response = await client.get("/items/?q=invalid!", headers);

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Invalid enum value", async () => {
	const app = createAppValidationErrorsInvalidEnumValue();
	const client = new TestClient(app);

	const response = await client.get("/models/invalid_model");

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: String min_length constraint violation", async () => {
	const app = createAppValidationErrorsStringMinLengthConstraintViolation();
	const client = new TestClient(app);

	const headers = {
		"x-token": "test-token",
	};
	const response = await client.get("/items/?q=ab", headers);

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Multiple validation errors", async () => {
	const app = createAppValidationErrorsMultipleValidationErrors();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { name: "X", price: -10, quantity: "not_a_number" };
	const response = await client.post("/items/", { headers, json });

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: String max_length constraint violation", async () => {
	const app = createAppValidationErrorsStringMaxLengthConstraintViolation();
	const client = new TestClient(app);

	const headers = {
		"x-token": "test-token",
	};
	const response = await client.get(
		"/items/?q=this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter",
		headers,
	);

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Nested object validation error", async () => {
	const app = createAppValidationErrorsNestedObjectValidationError();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { name: "Product", price: 10.0, seller: { address: { city: "SF", zip_code: "123" }, name: "Jo" } };
	const response = await client.post("/items/", { headers, json });

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: 10_nested_error_path", async () => {
	const app = createAppValidationErrors10NestedErrorPath();
	const client = new TestClient(app);

	const json = { profile: { contact: { email: "invalid" } } };
	const response = await client.post("/profiles", { json });

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Invalid datetime format", async () => {
	const app = createAppValidationErrorsInvalidDatetimeFormat();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { created_at: "not-a-datetime", name: "Item", price: 10.0 };
	const response = await client.post("/items/", { headers, json });

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Array item validation error", async () => {
	const app = createAppValidationErrorsArrayItemValidationError();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { name: "Item", price: 10.0, tags: ["tag1", "tag2", 123] };
	const response = await client.post("/items/", { headers, json });

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Missing required body field", async () => {
	const app = createAppValidationErrorsMissingRequiredBodyField();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { name: "Item" };
	const response = await client.post("/items/", { headers, json });

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Body field type error - string for float", async () => {
	const app = createAppValidationErrorsBodyFieldTypeErrorStringForFloat();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { name: "Item", price: "not_a_float" };
	const response = await client.post("/items/", { headers, json });

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Malformed JSON body", async () => {
	const app = createAppValidationErrorsMalformedJsonBody();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = '{"name": "Item", "price": }';
	const response = await client.post("/items/", { headers, json });

	assertEquals(response.statusCode, 400);
});

Deno.test("validation_errors: Query param type error - string provided for int", async () => {
	const app = createAppValidationErrorsQueryParamTypeErrorStringProvidedForInt();
	const client = new TestClient(app);

	const headers = {
		"x-token": "test-token",
	};
	const response = await client.get("/items/?q=test&skip=not_a_number", headers);

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Header validation error", async () => {
	const app = createAppValidationErrorsHeaderValidationError();
	const client = new TestClient(app);

	const response = await client.get("/items/?q=test");

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: 09_multiple_validation_errors", async () => {
	const app = createAppValidationErrors09MultipleValidationErrors();
	const client = new TestClient(app);

	const json = { age: 15, email: "invalid-email", name: "ab" };
	const response = await client.post("/users", { json });

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Numeric constraint violation - le less than or equal", async () => {
	const app = createAppValidationErrorsNumericConstraintViolationLeLessThanOrEqual();
	const client = new TestClient(app);

	const headers = {
		"x-token": "test-token",
	};
	const response = await client.get("/items/?q=test&limit=101", headers);

	assertEquals(response.statusCode, 422);
});

Deno.test("validation_errors: Array min_items constraint violation", async () => {
	const app = createAppValidationErrorsArrayMinItemsConstraintViolation();
	const client = new TestClient(app);

	const headers = {
		"Content-Type": "application/json",
	};
	const json = { name: "Item", price: 10.0, tags: [] };
	const response = await client.post("/items/", { headers, json });

	assertEquals(response.statusCode, 422);
});
