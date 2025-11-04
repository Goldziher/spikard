/**
 * E2E tests for url_encoded
 * @generated
 */

import { TestClient } from "@spikard/node";
import { describe, expect, test } from "vitest";
import {
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
} from "../app/main.js";

describe("url_encoded", () => {
	test("Simple form submission - success", async () => {
		const app = createAppUrlEncodedSimpleFormSubmissionSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { password: "secret", username: "johndoe" };
		const response = await client.post("/login/", { headers, form });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("username");
		expect(responseData.username).toBe("johndoe");
	});

	test("15_special_characters_field_names", async () => {
		const app = createAppUrlEncoded15SpecialCharactersFieldNames();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "user-name=JohnDoe&contact.email=john%40example.com";
		const response = await client.post("/data", { headers, form });

		expect(response.statusCode).toBe(201);
		const responseData = response.json();
		expect(responseData).toHaveProperty("contact.email");
		expect(responseData["contact.email"]).toBe("john@example.com");
		expect(responseData).toHaveProperty("user-name");
		expect(responseData["user-name"]).toBe("JohnDoe");
	});

	test("Pattern validation - fail", async () => {
		const app = createAppUrlEncodedPatternValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { username: "john doe" };
		const response = await client.post("/form/validated", { headers, form });

		expect(response.statusCode).toBe(422);
	});

	test("22_additional_properties_strict_failure", async () => {
		const app = createAppUrlEncoded22AdditionalPropertiesStrictFailure();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "theme=dark&unknown_field=value";
		const response = await client.post("/settings", { headers, form });

		expect(response.statusCode).toBe(422);
	});

	test("17_pattern_validation_failure", async () => {
		const app = createAppUrlEncoded17PatternValidationFailure();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "account_id=INVALID123";
		const response = await client.post("/accounts", { headers, form });

		expect(response.statusCode).toBe(422);
	});

	test("20_format_email_validation_failure", async () => {
		const app = createAppUrlEncoded20FormatEmailValidationFailure();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "email=not-an-email";
		const response = await client.post("/subscribe", { headers, form });

		expect(response.statusCode).toBe(422);
	});

	test("Multiple values for same field", async () => {
		const app = createAppUrlEncodedMultipleValuesForSameField();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { tags: ["python", "fastapi", "web"] };
		const response = await client.post("/form/tags", { headers, form });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("tags");
		expect(responseData.tags.length).toBe(3);
		expect(responseData.tags[0]).toBe("python");
		expect(responseData.tags[1]).toBe("fastapi");
		expect(responseData.tags[2]).toBe("web");
	});

	test("Required field missing - validation error", async () => {
		const app = createAppUrlEncodedRequiredFieldMissingValidationError();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { password: "secret" };
		const response = await client.post("/login/", { headers, form });

		expect(response.statusCode).toBe(422);
	});

	test("13_array_field_success", async () => {
		const app = createAppUrlEncoded13ArrayFieldSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "tags[]=python&tags[]=rust&tags[]=typescript";
		const response = await client.post("/register", { headers, form });

		expect(response.statusCode).toBe(201);
		const responseData = response.json();
		expect(responseData).toHaveProperty("tags");
		expect(responseData.tags.length).toBe(3);
		expect(responseData.tags[0]).toBe("python");
		expect(responseData.tags[1]).toBe("rust");
		expect(responseData.tags[2]).toBe("typescript");
	});

	test("Numeric field type conversion", async () => {
		const app = createAppUrlEncodedNumericFieldTypeConversion();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { age: "30", username: "johndoe" };
		const response = await client.post("/form/", { headers, form });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("age");
		expect(responseData.age).toBe(30);
		expect(responseData).toHaveProperty("username");
		expect(responseData.username).toBe("johndoe");
	});

	test("Special characters encoding", async () => {
		const app = createAppUrlEncodedSpecialCharactersEncoding();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { description: "Test & Development", name: "John Doe" };
		const response = await client.post("/form/", { headers, form });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("description");
		expect(responseData.description).toBe("Test & Development");
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("John Doe");
	});

	test("Boolean field conversion", async () => {
		const app = createAppUrlEncodedBooleanFieldConversion();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { subscribe: "true", username: "johndoe" };
		const response = await client.post("/form/", { headers, form });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("subscribe");
		expect(responseData.subscribe).toBe(true);
		expect(responseData).toHaveProperty("username");
		expect(responseData.username).toBe("johndoe");
	});

	test("Empty string value", async () => {
		const app = createAppUrlEncodedEmptyStringValue();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { description: "", username: "johndoe" };
		const response = await client.post("/form/", { headers, form });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("description");
		expect(responseData.description).toBe("");
		expect(responseData).toHaveProperty("username");
		expect(responseData.username).toBe("johndoe");
	});

	test("OAuth2 password grant flow", async () => {
		const app = createAppUrlEncodedOauth2PasswordGrantFlow();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { grant_type: "password", password: "secret", scope: "", username: "johndoe" };
		const response = await client.post("/token", { headers, form });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("access_token");
		expect(responseData.access_token).toBe("johndoe");
		expect(responseData).toHaveProperty("token_type");
		expect(responseData.token_type).toBe("bearer");
	});

	test("19_array_minitems_validation_failure", async () => {
		const app = createAppUrlEncoded19ArrayMinitemsValidationFailure();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "tags[]=single";
		const response = await client.post("/tags", { headers, form });

		expect(response.statusCode).toBe(422);
	});

	test("Optional field missing - success", async () => {
		const app = createAppUrlEncodedOptionalFieldMissingSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { password: "secret", username: "johndoe" };
		const response = await client.post("/register/", { headers, form });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("email");
		expect(responseData.email).toBe(null);
		expect(responseData).toHaveProperty("username");
		expect(responseData.username).toBe("johndoe");
	});

	test("14_nested_object_bracket_notation", async () => {
		const app = createAppUrlEncoded14NestedObjectBracketNotation();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "user[name]=John%20Doe&user[email]=john@example.com&user[age]=30";
		const response = await client.post("/profile", { headers, form });

		expect(response.statusCode).toBe(201);
		const responseData = response.json();
		expect(responseData).toHaveProperty("user");
		expect(responseData.user).toHaveProperty("age");
		expect(responseData.user.age).toBe(30);
		expect(responseData.user).toHaveProperty("email");
		expect(responseData.user.email).toBe("john@example.com");
		expect(responseData.user).toHaveProperty("name");
		expect(responseData.user.name).toBe("John Doe");
	});

	test("String max_length validation - fail", async () => {
		const app = createAppUrlEncodedStringMaxLengthValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { username: "this_is_a_very_long_username_that_exceeds_limit" };
		const response = await client.post("/form/validated", { headers, form });

		expect(response.statusCode).toBe(422);
	});

	test("18_integer_minimum_validation_failure", async () => {
		const app = createAppUrlEncoded18IntegerMinimumValidationFailure();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "quantity=0";
		const response = await client.post("/products", { headers, form });

		expect(response.statusCode).toBe(422);
	});

	test("21_integer_type_coercion_failure", async () => {
		const app = createAppUrlEncoded21IntegerTypeCoercionFailure();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "price=not-a-number";
		const response = await client.post("/products", { headers, form });

		expect(response.statusCode).toBe(422);
	});

	test("16_minlength_validation_failure", async () => {
		const app = createAppUrlEncoded16MinlengthValidationFailure();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "username=ab";
		const response = await client.post("/users", { headers, form });

		expect(response.statusCode).toBe(422);
	});

	test("String min_length validation - fail", async () => {
		const app = createAppUrlEncodedStringMinLengthValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { username: "ab" };
		const response = await client.post("/form/validated", { headers, form });

		expect(response.statusCode).toBe(422);
	});
});
