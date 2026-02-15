/**
 * E2E tests for validation_errors
 * @generated
 */

import { TestClient } from "@spikard/node";
import { describe, expect, test } from "vitest";
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

describe("validation_errors", () => {
	test("Invalid UUID format", async () => {
		const app = createAppValidationErrorsInvalidUuidFormat();
		const client = new TestClient(app);

		const response = await client.get("/items/not-a-uuid");

		expect(response.statusCode).toBe(422);
	});

	test("Invalid boolean value", async () => {
		const app = createAppValidationErrorsInvalidBooleanValue();
		const client = new TestClient(app);

		const headers = {
			"x-token": "test-token",
		};
		const response = await client.get("/items/?q=test&is_active=maybe", headers);

		expect(response.statusCode).toBe(422);
	});

	test("Missing required query parameter", async () => {
		const app = createAppValidationErrorsMissingRequiredQueryParameter();
		const client = new TestClient(app);

		const headers = {
			"x-token": "test-token",
		};
		const response = await client.get("/items/", headers);

		expect(response.statusCode).toBe(422);
	});

	test("Array max_items constraint violation", async () => {
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

		expect(response.statusCode).toBe(422);
	});

	test("Numeric constraint violation - gt greater than", async () => {
		const app = createAppValidationErrorsNumericConstraintViolationGtGreaterThan();
		const client = new TestClient(app);

		const headers = {
			"x-token": "test-token",
		};
		const response = await client.get("/items/?q=test&price=0", headers);

		expect(response.statusCode).toBe(422);
	});

	test("String regex pattern mismatch", async () => {
		const app = createAppValidationErrorsStringRegexPatternMismatch();
		const client = new TestClient(app);

		const headers = {
			"x-token": "test-token",
		};
		const response = await client.get("/items/?q=invalid!", headers);

		expect(response.statusCode).toBe(422);
	});

	test("Invalid enum value", async () => {
		const app = createAppValidationErrorsInvalidEnumValue();
		const client = new TestClient(app);

		const response = await client.get("/models/invalid_model");

		expect(response.statusCode).toBe(422);
	});

	test("String min_length constraint violation", async () => {
		const app = createAppValidationErrorsStringMinLengthConstraintViolation();
		const client = new TestClient(app);

		const headers = {
			"x-token": "test-token",
		};
		const response = await client.get("/items/?q=ab", headers);

		expect(response.statusCode).toBe(422);
	});

	test("Multiple validation errors", async () => {
		const app = createAppValidationErrorsMultipleValidationErrors();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "X", price: -10, quantity: "not_a_number" };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("String max_length constraint violation", async () => {
		const app = createAppValidationErrorsStringMaxLengthConstraintViolation();
		const client = new TestClient(app);

		const headers = {
			"x-token": "test-token",
		};
		const response = await client.get(
			"/items/?q=this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter",
			headers,
		);

		expect(response.statusCode).toBe(422);
	});

	test("Nested object validation error", async () => {
		const app = createAppValidationErrorsNestedObjectValidationError();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Product", price: 10.0, seller: { name: "Jo", address: { city: "SF", zip_code: "123" } } };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("10_nested_error_path", async () => {
		const app = createAppValidationErrors10NestedErrorPath();
		const client = new TestClient(app);

		const json = { profile: { contact: { email: "invalid" } } };
		const response = await client.post("/profiles", { json });

		expect(response.statusCode).toBe(422);
	});

	test("Invalid datetime format", async () => {
		const app = createAppValidationErrorsInvalidDatetimeFormat();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 10.0, created_at: "not-a-datetime" };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("Array item validation error", async () => {
		const app = createAppValidationErrorsArrayItemValidationError();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 10.0, tags: ["tag1", "tag2", 123] };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("Missing required body field", async () => {
		const app = createAppValidationErrorsMissingRequiredBodyField();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item" };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("Body field type error - string for float", async () => {
		const app = createAppValidationErrorsBodyFieldTypeErrorStringForFloat();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: "not_a_float" };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(422);
	});

	test("Malformed JSON body", async () => {
		const app = createAppValidationErrorsMalformedJsonBody();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = '{"name": "Item", "price": }';
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(400);
	});

	test("Query param type error - string provided for int", async () => {
		const app = createAppValidationErrorsQueryParamTypeErrorStringProvidedForInt();
		const client = new TestClient(app);

		const headers = {
			"x-token": "test-token",
		};
		const response = await client.get("/items/?q=test&skip=not_a_number", headers);

		expect(response.statusCode).toBe(422);
	});

	test("Header validation error", async () => {
		const app = createAppValidationErrorsHeaderValidationError();
		const client = new TestClient(app);

		const response = await client.get("/items/?q=test");

		expect(response.statusCode).toBe(422);
	});

	test("09_multiple_validation_errors", async () => {
		const app = createAppValidationErrors09MultipleValidationErrors();
		const client = new TestClient(app);

		const json = { name: "ab", email: "invalid-email", age: 15 };
		const response = await client.post("/users", { json });

		expect(response.statusCode).toBe(422);
	});

	test("Numeric constraint violation - le less than or equal", async () => {
		const app = createAppValidationErrorsNumericConstraintViolationLeLessThanOrEqual();
		const client = new TestClient(app);

		const headers = {
			"x-token": "test-token",
		};
		const response = await client.get("/items/?q=test&limit=101", headers);

		expect(response.statusCode).toBe(422);
	});

	test("Array min_items constraint violation", async () => {
		const app = createAppValidationErrorsArrayMinItemsConstraintViolation();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { name: "Item", price: 10.0, tags: [] };
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(422);
	});
});
