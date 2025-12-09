/**
 * E2E tests for edge_cases
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { describe, expect, test } from "vitest";
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

describe("edge_cases", () => {
	test("19_emoji_in_strings", async () => {
		const app = createAppEdgeCases19EmojiInStrings();
		const client = new TestClient(app);

		const json = { text: "Hello 👋 World 🌍" };
		const response = await client.post("/messages", { json });

		expect(response.statusCode).toBe(201);
		const responseData = response.json();
		expect(responseData).toHaveProperty("text");
		expect(responseData.text).toBe("Hello 👋 World 🌍");
	});

	test("12_percent_encoded_special_chars", async () => {
		const app = createAppEdgeCases12PercentEncodedSpecialChars();
		const client = new TestClient(app);

		const response = await client.get("/search?term=hi%20there&term=hi%20there");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("term");
		expect(responseData.term).toBe("hi there");
	});

	test("Special string values and escaping", async () => {
		const app = createAppEdgeCasesSpecialStringValuesAndEscaping();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {
			backslashes: "C:\\\\Users\\\\Path",
			empty_string: "",
			quotes: "He said \"hello\" and 'goodbye'",
			special_chars: "!@#$%^&*()_+-=[]{}|;':\",./<>?",
			tabs_newlines: "line1\n\tline2\r\nline3",
			unicode_escapes: "\\u0048\\u0065\\u006c\\u006c\\u006f",
			whitespace: "   ",
		};
		const response = await client.post("/strings/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("backslashes");
		expect(responseData.backslashes).toBe("C:\\\\Users\\\\Path");
		expect(responseData).toHaveProperty("empty_string");
		expect(responseData.empty_string).toBe("");
		expect(responseData).toHaveProperty("quotes");
		expect(responseData.quotes).toBe("He said \"hello\" and 'goodbye'");
		expect(responseData).toHaveProperty("special_chars");
		expect(responseData.special_chars).toBe("!@#$%^&*()_+-=[]{}|;':\",./<>?");
		expect(responseData).toHaveProperty("tabs_newlines");
		expect(responseData.tabs_newlines).toBe("line1\n\tline2\r\nline3");
		expect(responseData).toHaveProperty("unicode_escapes");
		expect(responseData.unicode_escapes).toBe("Hello");
		expect(responseData).toHaveProperty("whitespace");
		expect(responseData.whitespace).toBe("   ");
	});

	test("15_float_precision_preservation", async () => {
		const app = createAppEdgeCases15FloatPrecisionPreservation();
		const client = new TestClient(app);

		const json = { value: 3.141592653589793 };
		const response = await client.post("/calculate", { json });

		expect(response.statusCode).toBe(201);
		const responseData = response.json();
		expect(responseData).toHaveProperty("value");
		expect(responseData.value).toBe(3.141592653589793);
	});

	test("13_empty_string_query_param_preserved", async () => {
		const app = createAppEdgeCases13EmptyStringQueryParamPreserved();
		const client = new TestClient(app);

		const response = await client.get("/items?filter=&filter=");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("filter");
		expect(responseData.filter).toBe("");
	});

	test("24_array_with_holes", async () => {
		const app = createAppEdgeCases24ArrayWithHoles();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/x-www-form-urlencoded",
		};
		const form = "items[0]=first&items[2]=third&items[5]=sixth";
		const response = await client.post("/items", { headers, form });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("items");
		expect(responseData.items.length).toBe(3);
		expect(responseData.items[0]).toBe("first");
		expect(responseData.items[1]).toBe("third");
		expect(responseData.items[2]).toBe("sixth");
	});

	test("21_scientific_notation_number", async () => {
		const app = createAppEdgeCases21ScientificNotationNumber();
		const client = new TestClient(app);

		const json = { value: 123000.0 };
		const response = await client.post("/calculate", { json });

		expect(response.statusCode).toBe(201);
		const responseData = response.json();
		expect(responseData).toHaveProperty("value");
		expect(responseData.value).toBe(123000);
	});

	test("Float precision and rounding", async () => {
		const app = createAppEdgeCasesFloatPrecisionAndRounding();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {
			expected_sum: 0.3,
			precise_value: 3.141592653589793,
			value1: 0.1,
			value2: 0.2,
			very_large: 1.7976931348623157e308,
			very_small: 1e-10,
		};
		const response = await client.post("/calculations/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("precise_value");
		expect(responseData.precise_value).toBe(3.141592653589793);
		expect(responseData).toHaveProperty("sum");
		expect(responseData.sum).toBe(0.30000000000000004);
		expect(responseData).toHaveProperty("very_large");
		expect(responseData.very_large).toBe(1.7976931348623157e308);
		expect(responseData).toHaveProperty("very_small");
		expect(responseData.very_small).toBe(1e-10);
	});

	test("Unicode and emoji handling", async () => {
		const app = createAppEdgeCasesUnicodeAndEmojiHandling();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json; charset=utf-8",
		};
		const json = {
			description: "Best café in München 🇩🇪",
			emoji_reactions: "👍❤️😂🎉",
			name: "Coffee Shop ☕",
			tags: ["食べ物", "音楽", "💰"],
		};
		const response = await client.post("/items/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("description");
		expect(responseData.description).toBe("Best café in München 🇩🇪");
		expect(responseData).toHaveProperty("emoji_reactions");
		expect(responseData.emoji_reactions).toBe("👍❤️😂🎉");
		expect(responseData).toHaveProperty("id");
		expect(responseData.id).toBe(1);
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("Coffee Shop ☕");
		expect(responseData).toHaveProperty("tags");
		expect(responseData.tags.length).toBe(3);
		expect(responseData.tags[0]).toBe("食べ物");
		expect(responseData.tags[1]).toBe("音楽");
		expect(responseData.tags[2]).toBe("💰");
	});

	test("17_extremely_long_string", async () => {
		const app = createAppEdgeCases17ExtremelyLongString();
		const client = new TestClient(app);

		const json = {
			content:
				"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
		};
		const response = await client.post("/text", { json });

		expect(response.statusCode).toBe(422);
	});

	test("11_utf8_query_parameter", async () => {
		const app = createAppEdgeCases11Utf8QueryParameter();
		const client = new TestClient(app);

		const response = await client.get("/search?term=caf%C3%A9");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("term");
		expect(responseData.term).toBe("café");
	});

	test("18_unicode_normalization", async () => {
		const app = createAppEdgeCases18UnicodeNormalization();
		const client = new TestClient(app);

		const json = { name: "café" };
		const response = await client.post("/users", { json });

		expect(response.statusCode).toBe(201);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("café");
	});

	test("20_null_byte_in_string", async () => {
		const app = createAppEdgeCases20NullByteInString();
		const client = new TestClient(app);

		const json = { filename: "file .txt" };
		const response = await client.post("/files", { json });

		expect(response.statusCode).toBe(422);
	});

	test("23_deeply_nested_json_limit", async () => {
		const app = createAppEdgeCases23DeeplyNestedJsonLimit();
		const client = new TestClient(app);

		const json = {
			nested: {
				nested: {
					nested: {
						nested: {
							nested: {
								nested: {
									nested: {
										nested: {
											nested: {
												nested: {
													nested: {
														nested: {
															nested: {
																nested: {
																	nested: {
																		nested: {
																			nested: {
																				nested: {
																					nested: {
																						nested: {
																							nested: {
																								nested: {
																									nested: {
																										nested: {
																											nested: {
																												nested: {
																													nested: {
																														nested: {
																															nested: {
																																nested: {
																																	nested: {
																																		nested: {
																																			nested: {
																																				nested: {
																																					nested: {
																																						nested: {
																																							nested: {
																																								nested: {
																																									nested: {
																																										nested: {
																																											nested: {
																																												nested: {
																																													nested: {
																																														nested: {
																																															nested: {
																																																nested: {
																																																	nested: {
																																																		nested: {
																																																			nested: {
																																																				value: "deep",
																																																			},
																																																		},
																																																	},
																																																},
																																															},
																																														},
																																													},
																																												},
																																											},
																																										},
																																									},
																																								},
																																							},
																																						},
																																					},
																																				},
																																			},
																																		},
																																	},
																																},
																															},
																														},
																													},
																												},
																											},
																										},
																									},
																								},
																							},
																						},
																					},
																				},
																			},
																		},
																	},
																},
															},
														},
													},
												},
											},
										},
									},
								},
							},
						},
					},
				},
			},
		};
		const response = await client.post("/data", { json });

		expect(response.statusCode).toBe(400);
	});

	test("14_large_integer_boundary", async () => {
		const app = createAppEdgeCases14LargeIntegerBoundary();
		const client = new TestClient(app);

		const response = await client.get("/items?id=9007199254740991");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("id");
		expect(responseData.id).toBe(9007199254740991);
	});

	test("22_leading_zeros_integer", async () => {
		const app = createAppEdgeCases22LeadingZerosInteger();
		const client = new TestClient(app);

		const response = await client.get("/data?value=0123");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("value");
		expect(responseData.value).toBe(123);
	});

	test("Large integer boundary values", async () => {
		const app = createAppEdgeCasesLargeIntegerBoundaryValues();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {
			large_int: 9223372036854775807n,
			max_safe_int: 9007199254740991,
			negative_large: -9223372036854775808n,
		};
		const response = await client.post("/numbers/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("large_int");
		expect(responseData.large_int).toBe("9223372036854775807");
		expect(responseData).toHaveProperty("max_safe_int");
		expect(responseData.max_safe_int).toBe(9007199254740991);
		expect(responseData).toHaveProperty("negative_large");
		expect(responseData.negative_large).toBe("-9223372036854775808");
	});

	test("Deeply nested structure 10 levels", async () => {
		const app = createAppEdgeCasesDeeplyNestedStructure10Levels();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {
			level1: {
				level2: {
					level3: {
						level4: {
							level5: { level6: { level7: { level8: { level9: { level10: { depth: 10, value: "deep" } } } } } },
						},
					},
				},
			},
		};
		const response = await client.post("/nested/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("max_depth");
		expect(responseData.max_depth).toBe(10);
		expect(responseData).toHaveProperty("message");
		expect(responseData.message).toBe("Processed deeply nested structure");
		expect(responseData).toHaveProperty("value_found");
		expect(responseData.value_found).toBe("deep");
	});

	test("Empty and null value handling", async () => {
		const app = createAppEdgeCasesEmptyAndNullValueHandling();
		const client = new TestClient(app);

		const headers = {
			"Content-Type": "application/json",
		};
		const json = {
			empty_array: [],
			empty_object: {},
			empty_string: "",
			explicit_null: null,
			false_boolean: false,
			zero_number: 0,
		};
		const response = await client.post("/nulls/", { headers, json });

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("empty_array_length");
		expect(responseData.empty_array_length).toBe(0);
		expect(responseData).toHaveProperty("empty_object_keys");
		expect(responseData.empty_object_keys).toBe(0);
		expect(responseData).toHaveProperty("empty_string_length");
		expect(responseData.empty_string_length).toBe(0);
		expect(responseData).toHaveProperty("explicit_null_is_null");
		expect(responseData.explicit_null_is_null).toBe(true);
		expect(responseData).toHaveProperty("false_is_false");
		expect(responseData.false_is_false).toBe(true);
		expect(responseData).toHaveProperty("zero_is_falsy");
		expect(responseData.zero_is_falsy).toBe(true);
	});

	test("16_negative_zero_handling", async () => {
		const app = createAppEdgeCases16NegativeZeroHandling();
		const client = new TestClient(app);

		const json = { offset: -0.0 };
		const response = await client.post("/data", { json });

		expect(response.statusCode).toBe(201);
		const responseData = response.json();
		expect(responseData).toHaveProperty("offset");
		expect(responseData.offset).toBe(0);
	});
});
