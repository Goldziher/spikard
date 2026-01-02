/**
 * E2E tests for edge_cases
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
import {
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
} from "../app/main.ts";

	Deno.test("edge_cases: 19_emoji_in_strings", async () => {
		const app = createAppEdgeCases19EmojiInStrings();
		const client = new TestClient(app);

		const json = { text: "Hello 👋 World 🌍" };
		const response = await client.post("/messages", { json });

		assertEquals(response.statusCode, 201);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "text"));
		assertEquals(responseData.text, "Hello 👋 World 🌍");
	});

	Deno.test("edge_cases: 12_percent_encoded_special_chars", async () => {
		const app = createAppEdgeCases12PercentEncodedSpecialChars();
		const client = new TestClient(app);

		const response = await client.get("/search?term=hi%20there&term=hi%20there");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "term"));
		assertEquals(responseData.term, "hi there");
	});

	Deno.test("edge_cases: Special string values and escaping", async () => {
		const app = createAppEdgeCasesSpecialStringValuesAndEscaping();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { empty_string: "", whitespace: "   ", tabs_newlines: "line1\n\tline2\r\nline3", quotes: "He said \"hello\" and 'goodbye'", backslashes: "C:\\\\Users\\\\Path", unicode_escapes: "\\u0048\\u0065\\u006c\\u006c\\u006f", special_chars: "!@#$%^&*()_+-=[]{}|;':\",./<>?" };
		const response = await client.post("/strings/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "empty_string"));
		assertEquals(responseData.empty_string, "");
		assert(Object.hasOwn(responseData, "whitespace"));
		assertEquals(responseData.whitespace, "   ");
		assert(Object.hasOwn(responseData, "tabs_newlines"));
		assertEquals(responseData.tabs_newlines, "line1\n\tline2\r\nline3");
		assert(Object.hasOwn(responseData, "quotes"));
		assertEquals(responseData.quotes, "He said \"hello\" and 'goodbye'");
		assert(Object.hasOwn(responseData, "backslashes"));
		assertEquals(responseData.backslashes, "C:\\\\Users\\\\Path");
		assert(Object.hasOwn(responseData, "unicode_escapes"));
		assertEquals(responseData.unicode_escapes, "Hello");
		assert(Object.hasOwn(responseData, "special_chars"));
		assertEquals(responseData.special_chars, "!@#$%^&*()_+-=[]{}|;':\",./<>?");
	});

	Deno.test("edge_cases: 15_float_precision_preservation", async () => {
		const app = createAppEdgeCases15FloatPrecisionPreservation();
		const client = new TestClient(app);

		const json = { value: 3.141592653589793 };
		const response = await client.post("/calculate", { json });

		assertEquals(response.statusCode, 201);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "value"));
		assertEquals(responseData.value, 3.141592653589793);
	});

	Deno.test("edge_cases: 13_empty_string_query_param_preserved", async () => {
		const app = createAppEdgeCases13EmptyStringQueryParamPreserved();
		const client = new TestClient(app);

		const response = await client.get("/items?filter=&filter=");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "filter"));
		assertEquals(responseData.filter, "");
	});

	Deno.test("edge_cases: 24_array_with_holes", async () => {
		const app = createAppEdgeCases24ArrayWithHoles();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "items[0]=first&items[2]=third&items[5]=sixth";
		const response = await client.post("/items", { headers, form });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "items"));
		assertEquals(responseData.items.length, 3);
		assertEquals(responseData.items[0], "first");
		assertEquals(responseData.items[1], "third");
		assertEquals(responseData.items[2], "sixth");
	});

	Deno.test("edge_cases: 21_scientific_notation_number", async () => {
		const app = createAppEdgeCases21ScientificNotationNumber();
		const client = new TestClient(app);

		const json = { value: 123000.0 };
		const response = await client.post("/calculate", { json });

		assertEquals(response.statusCode, 201);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "value"));
		assertEquals(responseData.value, 123000);
	});

	Deno.test("edge_cases: Float precision and rounding", async () => {
		const app = createAppEdgeCasesFloatPrecisionAndRounding();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { value1: 0.1, value2: 0.2, expected_sum: 0.3, precise_value: 3.141592653589793, very_small: 1e-10, very_large: 1.7976931348623157e+308 };
		const response = await client.post("/calculations/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "sum"));
		assertEquals(responseData.sum, 0.30000000000000004);
		assert(Object.hasOwn(responseData, "precise_value"));
		assertEquals(responseData.precise_value, 3.141592653589793);
		assert(Object.hasOwn(responseData, "very_small"));
		assertEquals(responseData.very_small, 1e-10);
		assert(Object.hasOwn(responseData, "very_large"));
		assertEquals(responseData.very_large, 1.7976931348623157e+308);
	});

	Deno.test("edge_cases: Unicode and emoji handling", async () => {
		const app = createAppEdgeCasesUnicodeAndEmojiHandling();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json; charset=utf-8",
		};
		const json = { name: "Coffee Shop ☕", description: "Best café in München 🇩🇪", tags: ["食べ物", "音楽", "💰"], emoji_reactions: "👍❤️😂🎉" };
		const response = await client.post("/items/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "id"));
		assertEquals(responseData.id, 1);
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "Coffee Shop ☕");
		assert(Object.hasOwn(responseData, "description"));
		assertEquals(responseData.description, "Best café in München 🇩🇪");
		assert(Object.hasOwn(responseData, "tags"));
		assertEquals(responseData.tags.length, 3);
		assertEquals(responseData.tags[0], "食べ物");
		assertEquals(responseData.tags[1], "音楽");
		assertEquals(responseData.tags[2], "💰");
		assert(Object.hasOwn(responseData, "emoji_reactions"));
		assertEquals(responseData.emoji_reactions, "👍❤️😂🎉");
	});

	Deno.test("edge_cases: 17_extremely_long_string", async () => {
		const app = createAppEdgeCases17ExtremelyLongString();
		const client = new TestClient(app);

		const json = { content: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" };
		const response = await client.post("/text", { json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("edge_cases: 11_utf8_query_parameter", async () => {
		const app = createAppEdgeCases11Utf8QueryParameter();
		const client = new TestClient(app);

		const response = await client.get("/search?term=caf%C3%A9");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "term"));
		assertEquals(responseData.term, "café");
	});

	Deno.test("edge_cases: 18_unicode_normalization", async () => {
		const app = createAppEdgeCases18UnicodeNormalization();
		const client = new TestClient(app);

		const json = { name: "café" };
		const response = await client.post("/users", { json });

		assertEquals(response.statusCode, 201);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "café");
	});

	Deno.test("edge_cases: 20_null_byte_in_string", async () => {
		const app = createAppEdgeCases20NullByteInString();
		const client = new TestClient(app);

		const json = { filename: "file .txt" };
		const response = await client.post("/files", { json });

		assertEquals(response.statusCode, 422);
	});

	Deno.test("edge_cases: 23_deeply_nested_json_limit", async () => {
		const app = createAppEdgeCases23DeeplyNestedJsonLimit();
		const client = new TestClient(app);

		const json = { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { nested: { value: "deep" } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } };
		const response = await client.post("/data", { json });

		assertEquals(response.statusCode, 400);
	});

	Deno.test("edge_cases: 14_large_integer_boundary", async () => {
		const app = createAppEdgeCases14LargeIntegerBoundary();
		const client = new TestClient(app);

		const response = await client.get("/items?id=9007199254740991");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "id"));
		assertEquals(responseData.id, 9007199254740991);
	});

	Deno.test("edge_cases: 22_leading_zeros_integer", async () => {
		const app = createAppEdgeCases22LeadingZerosInteger();
		const client = new TestClient(app);

		const response = await client.get("/data?value=0123");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "value"));
		assertEquals(responseData.value, 123);
	});

	Deno.test("edge_cases: Large integer boundary values", async () => {
		const app = createAppEdgeCasesLargeIntegerBoundaryValues();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { max_safe_int: 9007199254740991, large_int: 9223372036854775807n, negative_large: -9223372036854775808n };
		const response = await client.post("/numbers/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "max_safe_int"));
		assertEquals(responseData.max_safe_int, 9007199254740991);
		assert(Object.hasOwn(responseData, "large_int"));
		assertEquals(responseData.large_int, "9223372036854775807");
		assert(Object.hasOwn(responseData, "negative_large"));
		assertEquals(responseData.negative_large, "-9223372036854775808");
	});

	Deno.test("edge_cases: Deeply nested structure 10 levels", async () => {
		const app = createAppEdgeCasesDeeplyNestedStructure10Levels();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { level1: { level2: { level3: { level4: { level5: { level6: { level7: { level8: { level9: { level10: { value: "deep", depth: 10 } } } } } } } } } } };
		const response = await client.post("/nested/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "message"));
		assertEquals(responseData.message, "Processed deeply nested structure");
		assert(Object.hasOwn(responseData, "max_depth"));
		assertEquals(responseData.max_depth, 10);
		assert(Object.hasOwn(responseData, "value_found"));
		assertEquals(responseData.value_found, "deep");
	});

	Deno.test("edge_cases: Empty and null value handling", async () => {
		const app = createAppEdgeCasesEmptyAndNullValueHandling();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = { explicit_null: null, empty_string: "", empty_array: [], empty_object: {  }, zero_number: 0, false_boolean: false };
		const response = await client.post("/nulls/", { headers, json });

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "explicit_null_is_null"));
		assertEquals(responseData.explicit_null_is_null, true);
		assert(Object.hasOwn(responseData, "empty_string_length"));
		assertEquals(responseData.empty_string_length, 0);
		assert(Object.hasOwn(responseData, "empty_array_length"));
		assertEquals(responseData.empty_array_length, 0);
		assert(Object.hasOwn(responseData, "empty_object_keys"));
		assertEquals(responseData.empty_object_keys, 0);
		assert(Object.hasOwn(responseData, "zero_is_falsy"));
		assertEquals(responseData.zero_is_falsy, true);
		assert(Object.hasOwn(responseData, "false_is_false"));
		assertEquals(responseData.false_is_false, true);
	});

	Deno.test("edge_cases: 16_negative_zero_handling", async () => {
		const app = createAppEdgeCases16NegativeZeroHandling();
		const client = new TestClient(app);

		const json = { offset: -0.0 };
		const response = await client.post("/data", { json });

		assertEquals(response.statusCode, 201);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "offset"));
		assertEquals(responseData.offset, 0);
	});