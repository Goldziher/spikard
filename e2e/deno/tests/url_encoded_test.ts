/**
 * E2E tests for url_encoded
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
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
} from "../app/main.ts";

	Deno.test("url_encoded: Simple form submission - success", async () => {
		const app = createAppUrlEncodedSimpleFormSubmissionSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { password: "secret", username: "johndoe" };
		const response = await client.post("/login/", { headers, form });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "username"));
		assertEquals(responseData.username, "johndoe");
	});

	Deno.test("url_encoded: 15_special_characters_field_names", async () => {
		const app = createAppUrlEncoded15SpecialCharactersFieldNames();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "user-name=JohnDoe&contact.email=john%40example.com";
		const response = await client.post("/data", { headers, form });

		assertEquals(response.statusCode, 201);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "user-name"));
		assertEquals(responseData["user-name"], "JohnDoe");
		assert(Object.hasOwn(responseData, "contact.email"));
		assertEquals(responseData["contact.email"], "john@example.com");
	});

	Deno.test("url_encoded: Pattern validation - fail", async () => {
		const app = createAppUrlEncodedPatternValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { username: "john doe" };
		const response = await client.post("/form/validated", { headers, form });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("url_encoded: 22_additional_properties_strict_failure", async () => {
		const app = createAppUrlEncoded22AdditionalPropertiesStrictFailure();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "theme=dark&unknown_field=value";
		const response = await client.post("/settings", { headers, form });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("url_encoded: 17_pattern_validation_failure", async () => {
		const app = createAppUrlEncoded17PatternValidationFailure();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "account_id=INVALID123";
		const response = await client.post("/accounts", { headers, form });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("url_encoded: 20_format_email_validation_failure", async () => {
		const app = createAppUrlEncoded20FormatEmailValidationFailure();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "email=not-an-email";
		const response = await client.post("/subscribe", { headers, form });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("url_encoded: Multiple values for same field", async () => {
		const app = createAppUrlEncodedMultipleValuesForSameField();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { tags: ["python", "fastapi", "web"] };
		const response = await client.post("/form/tags", { headers, form });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "tags"));
		assertEquals(responseData.tags.length, 3);
		assertEquals(responseData.tags[0], "python");
		assertEquals(responseData.tags[1], "fastapi");
		assertEquals(responseData.tags[2], "web");
	});

	Deno.test("url_encoded: Required field missing - validation error", async () => {
		const app = createAppUrlEncodedRequiredFieldMissingValidationError();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { password: "secret" };
		const response = await client.post("/login/", { headers, form });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("url_encoded: 13_array_field_success", async () => {
		const app = createAppUrlEncoded13ArrayFieldSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "tags[]=python&tags[]=rust&tags[]=typescript";
		const response = await client.post("/register", { headers, form });

		assertEquals(response.statusCode, 201);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "tags"));
		assertEquals(responseData.tags.length, 3);
		assertEquals(responseData.tags[0], "python");
		assertEquals(responseData.tags[1], "rust");
		assertEquals(responseData.tags[2], "typescript");
	});

	Deno.test("url_encoded: Numeric field type conversion", async () => {
		const app = createAppUrlEncodedNumericFieldTypeConversion();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { username: "johndoe", age: "30" };
		const response = await client.post("/form/", { headers, form });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "username"));
		assertEquals(responseData.username, "johndoe");
		assert(Object.hasOwn(responseData, "age"));
		assertEquals(responseData.age, 30);
	});

	Deno.test("url_encoded: Special characters encoding", async () => {
		const app = createAppUrlEncodedSpecialCharactersEncoding();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { description: "Test & Development", name: "John Doe" };
		const response = await client.post("/form/", { headers, form });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "John Doe");
		assert(Object.hasOwn(responseData, "description"));
		assertEquals(responseData.description, "Test & Development");
	});

	Deno.test("url_encoded: Boolean field conversion", async () => {
		const app = createAppUrlEncodedBooleanFieldConversion();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { subscribe: "true", username: "johndoe" };
		const response = await client.post("/form/", { headers, form });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "username"));
		assertEquals(responseData.username, "johndoe");
		assert(Object.hasOwn(responseData, "subscribe"));
		assertEquals(responseData.subscribe, true);
	});

	Deno.test("url_encoded: Empty string value", async () => {
		const app = createAppUrlEncodedEmptyStringValue();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { description: "", username: "johndoe" };
		const response = await client.post("/form/", { headers, form });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "username"));
		assertEquals(responseData.username, "johndoe");
		assert(Object.hasOwn(responseData, "description"));
		assertEquals(responseData.description, "");
	});

	Deno.test("url_encoded: OAuth2 password grant flow", async () => {
		const app = createAppUrlEncodedOauth2PasswordGrantFlow();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { username: "johndoe", scope: "", grant_type: "password", password: "secret" };
		const response = await client.post("/token", { headers, form });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "access_token"));
		assertEquals(responseData.access_token, "johndoe");
		assert(Object.hasOwn(responseData, "token_type"));
		assertEquals(responseData.token_type, "bearer");
	});

	Deno.test("url_encoded: 19_array_minitems_validation_failure", async () => {
		const app = createAppUrlEncoded19ArrayMinitemsValidationFailure();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "tags[]=single";
		const response = await client.post("/tags", { headers, form });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("url_encoded: Optional field missing - success", async () => {
		const app = createAppUrlEncodedOptionalFieldMissingSuccess();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { username: "johndoe", password: "secret" };
		const response = await client.post("/register/", { headers, form });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "username"));
		assertEquals(responseData.username, "johndoe");
		assert(Object.hasOwn(responseData, "email"));
		assertEquals(responseData.email, null);
	});

	Deno.test("url_encoded: 14_nested_object_bracket_notation", async () => {
		const app = createAppUrlEncoded14NestedObjectBracketNotation();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "user[name]=John%20Doe&user[email]=john@example.com&user[age]=30";
		const response = await client.post("/profile", { headers, form });

		assertEquals(response.statusCode, 201);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "user"));
		assert(Object.hasOwn(responseData.user, "name"));
		assertEquals(responseData.user.name, "John Doe");
		assert(Object.hasOwn(responseData.user, "email"));
		assertEquals(responseData.user.email, "john@example.com");
		assert(Object.hasOwn(responseData.user, "age"));
		assertEquals(responseData.user.age, 30);
	});

	Deno.test("url_encoded: String max_length validation - fail", async () => {
		const app = createAppUrlEncodedStringMaxLengthValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { username: "this_is_a_very_long_username_that_exceeds_limit" };
		const response = await client.post("/form/validated", { headers, form });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("url_encoded: 18_integer_minimum_validation_failure", async () => {
		const app = createAppUrlEncoded18IntegerMinimumValidationFailure();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "quantity=0";
		const response = await client.post("/products", { headers, form });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("url_encoded: 21_integer_type_coercion_failure", async () => {
		const app = createAppUrlEncoded21IntegerTypeCoercionFailure();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "price=not-a-number";
		const response = await client.post("/products", { headers, form });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("url_encoded: 16_minlength_validation_failure", async () => {
		const app = createAppUrlEncoded16MinlengthValidationFailure();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "username=ab";
		const response = await client.post("/users", { headers, form });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("url_encoded: String min_length validation - fail", async () => {
		const app = createAppUrlEncodedStringMinLengthValidationFail();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = { username: "ab" };
		const response = await client.post("/form/validated", { headers, form });

		assertEquals(response.statusCode, 422);
	});