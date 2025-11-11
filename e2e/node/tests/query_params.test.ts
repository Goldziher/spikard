/**
 * E2E tests for query_params
 * @generated
 */

import { TestClient } from "@spikard/node";
import { describe, expect, test } from "vitest";
import {
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
} from "../app/main.js";

describe("query_params", () => {
	test("String validation with regex - success", async () => {
		const app = createAppQueryParamsStringValidationWithRegexSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items/?item_query=fixedquery");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("item_query");
		expect(responseData.item_query).toBe("fixedquery");
	});

	test("49_integer_gt_constraint_success", async () => {
		const app = createAppQueryParams49IntegerGtConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items?limit=5");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("limit");
		expect(responseData.limit).toBe(5);
	});

	test("Enum query parameter - invalid value", async () => {
		const app = createAppQueryParamsEnumQueryParameterInvalidValue();
		const client = new TestClient(app);

		const response = await client.get("/query/enum?model=vgg16");

		expect(response.statusCode).toBe(422);
	});

	test("68_array_uniqueitems_success", async () => {
		const app = createAppQueryParams68ArrayUniqueitemsSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items?ids=1&ids=2&ids=3&ids=4");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("ids");
		expect(responseData.ids.length).toBe(4);
		expect(responseData.ids[0]).toBe(1);
		expect(responseData.ids[1]).toBe(2);
		expect(responseData.ids[2]).toBe(3);
		expect(responseData.ids[3]).toBe(4);
	});

	test("47_pattern_validation_email_success", async () => {
		const app = createAppQueryParams47PatternValidationEmailSuccess();
		const client = new TestClient(app);

		const response = await client.get("/subscribe?email=user%40example.com");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("email");
		expect(responseData.email).toBe("user@example.com");
	});

	test("Required integer query parameter - success", async () => {
		const app = createAppQueryParamsRequiredIntegerQueryParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/query/int?query=42");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe("foo bar 42");
	});

	test("Required string query parameter - missing", async () => {
		const app = createAppQueryParamsRequiredStringQueryParameterMissing();
		const client = new TestClient(app);

		const response = await client.get("/query");

		expect(response.statusCode).toBe(422);
	});

	test("57_boolean_empty_string_coercion", async () => {
		const app = createAppQueryParams57BooleanEmptyStringCoercion();
		const client = new TestClient(app);

		const response = await client.get("/items?active=");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("active");
		expect(responseData.active).toBe(false);
	});

	test("52_integer_le_constraint_boundary", async () => {
		const app = createAppQueryParams52IntegerLeConstraintBoundary();
		const client = new TestClient(app);

		const response = await client.get("/items?limit=100");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("limit");
		expect(responseData.limit).toBe(100);
	});

	test("List with default empty array - no values provided", async () => {
		const app = createAppQueryParamsListWithDefaultEmptyArrayNoValuesProvided();
		const client = new TestClient(app);

		const response = await client.get("/query/list-default");

		expect(response.statusCode).toBe(200);
	});

	test("Date query parameter - success", async () => {
		const app = createAppQueryParamsDateQueryParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/query/date?event_date=2024-01-15");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("event_date");
		expect(responseData.event_date).toBe("2024-01-15");
	});

	test("String query param with max_length constraint - fail", async () => {
		const app = createAppQueryParamsStringQueryParamWithMaxLengthConstraintFail();
		const client = new TestClient(app);

		const response = await client.get("/query/str-max-length?name=this_is_way_too_long");

		expect(response.statusCode).toBe(422);
	});

	test("45_string_minlength_validation_failure", async () => {
		const app = createAppQueryParams45StringMinlengthValidationFailure();
		const client = new TestClient(app);

		const response = await client.get("/search?term=ab");

		expect(response.statusCode).toBe(422);
	});

	test("Integer with default value - override", async () => {
		const app = createAppQueryParamsIntegerWithDefaultValueOverride();
		const client = new TestClient(app);

		const response = await client.get("/query/int/default?query=50");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe("foo bar 50");
	});

	test("67_multipleof_constraint_failure", async () => {
		const app = createAppQueryParams67MultipleofConstraintFailure();
		const client = new TestClient(app);

		const response = await client.get("/items?quantity=17");

		expect(response.statusCode).toBe(422);
	});

	test("58_format_email_success", async () => {
		const app = createAppQueryParams58FormatEmailSuccess();
		const client = new TestClient(app);

		const response = await client.get("/subscribe?email=user%40example.com");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("email");
		expect(responseData.email).toBe("user@example.com");
	});

	test("Integer query param with ge constraint - boundary", async () => {
		const app = createAppQueryParamsIntegerQueryParamWithGeConstraintBoundary();
		const client = new TestClient(app);

		const response = await client.get("/query/int-ge?value=10");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("value");
		expect(responseData.value).toBe(10);
	});

	test("Integer query param with gt constraint - valid", async () => {
		const app = createAppQueryParamsIntegerQueryParamWithGtConstraintValid();
		const client = new TestClient(app);

		const response = await client.get("/query/int-gt?value=1");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("value");
		expect(responseData.value).toBe(1);
	});

	test("Required integer query parameter - invalid type", async () => {
		const app = createAppQueryParamsRequiredIntegerQueryParameterInvalidType();
		const client = new TestClient(app);

		const response = await client.get("/query/int?query=baz");

		expect(response.statusCode).toBe(422);
	});

	test("Required integer query parameter - float value", async () => {
		const app = createAppQueryParamsRequiredIntegerQueryParameterFloatValue();
		const client = new TestClient(app);

		const response = await client.get("/query/int?query=42.5");

		expect(response.statusCode).toBe(422);
	});

	test("Query parameter with URL encoded special characters", async () => {
		const app = createAppQueryParamsQueryParameterWithUrlEncodedSpecialCharacters();
		const client = new TestClient(app);

		const response = await client.get("/query/basic?name=test%26value%3D123");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("test&value=123");
	});

	test("59_format_email_failure", async () => {
		const app = createAppQueryParams59FormatEmailFailure();
		const client = new TestClient(app);

		const response = await client.get("/subscribe?email=not-an-email");

		expect(response.statusCode).toBe(422);
	});

	test("43_scientific_notation_float", async () => {
		const app = createAppQueryParams43ScientificNotationFloat();
		const client = new TestClient(app);

		const response = await client.get("/stats?threshold=1.5e-3");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("threshold");
		expect(responseData.threshold).toBe(0.0015);
	});

	test("63_format_uri_success", async () => {
		const app = createAppQueryParams63FormatUriSuccess();
		const client = new TestClient(app);

		const response = await client.get("/redirect?url=https%3A%2F%2Fexample.com%2Fpath%3Fquery%3Dvalue");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("url");
		expect(responseData.url).toBe("https://example.com/path?query=value");
	});

	test("Boolean query parameter - numeric 1", async () => {
		const app = createAppQueryParamsBooleanQueryParameterNumeric1();
		const client = new TestClient(app);

		const response = await client.get("/query/bool?flag=1");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("flag");
		expect(responseData.flag).toBe(true);
	});

	test("String query param with min_length constraint - fail", async () => {
		const app = createAppQueryParamsStringQueryParamWithMinLengthConstraintFail();
		const client = new TestClient(app);

		const response = await client.get("/query/str-min-length?name=ab");

		expect(response.statusCode).toBe(422);
	});

	test("Optional string query parameter - provided", async () => {
		const app = createAppQueryParamsOptionalStringQueryParameterProvided();
		const client = new TestClient(app);

		const response = await client.get("/query/optional?query=baz");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe("foo bar baz");
	});

	test("List of integers - multiple values", async () => {
		const app = createAppQueryParamsListOfIntegersMultipleValues();
		const client = new TestClient(app);

		const response = await client.get("/query/list?device_ids=1&device_ids=2");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData.length).toBe(2);
		expect(responseData[0]).toBe(1);
		expect(responseData[1]).toBe(2);
	});

	test("Integer query param with lt constraint - valid", async () => {
		const app = createAppQueryParamsIntegerQueryParamWithLtConstraintValid();
		const client = new TestClient(app);

		const response = await client.get("/query/int-lt?value=49");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("value");
		expect(responseData.value).toBe(49);
	});

	test("42_negative_integer_query_param", async () => {
		const app = createAppQueryParams42NegativeIntegerQueryParam();
		const client = new TestClient(app);

		const response = await client.get("/items/negative?offset=-10");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("offset");
		expect(responseData.offset).toBe(-10);
	});

	test("46_string_maxlength_validation_failure", async () => {
		const app = createAppQueryParams46StringMaxlengthValidationFailure();
		const client = new TestClient(app);

		const response = await client.get("/search?term=this_is_way_too_long");

		expect(response.statusCode).toBe(422);
	});

	test("56_array_maxitems_constraint_failure", async () => {
		const app = createAppQueryParams56ArrayMaxitemsConstraintFailure();
		const client = new TestClient(app);

		const response = await client.get("/items?tags=a&tags=b&tags=c&tags=d&tags=e&tags=f");

		expect(response.statusCode).toBe(422);
	});

	test("String query param with regex pattern - fail", async () => {
		const app = createAppQueryParamsStringQueryParamWithRegexPatternFail();
		const client = new TestClient(app);

		const response = await client.get("/query/pattern?code=abc123");

		expect(response.statusCode).toBe(422);
	});

	test("44_string_minlength_validation_success", async () => {
		const app = createAppQueryParams44StringMinlengthValidationSuccess();
		const client = new TestClient(app);

		const response = await client.get("/search?term=foo");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("term");
		expect(responseData.term).toBe("foo");
	});

	test("61_format_ipv4_failure", async () => {
		const app = createAppQueryParams61FormatIpv4Failure();
		const client = new TestClient(app);

		const response = await client.get("/network?ip=999.999.999.999");

		expect(response.statusCode).toBe(422);
	});

	test("48_pattern_validation_email_failure", async () => {
		const app = createAppQueryParams48PatternValidationEmailFailure();
		const client = new TestClient(app);

		const response = await client.get("/subscribe?email=invalid-email");

		expect(response.statusCode).toBe(422);
	});

	test("Required integer query parameter - missing", async () => {
		const app = createAppQueryParamsRequiredIntegerQueryParameterMissing();
		const client = new TestClient(app);

		const response = await client.get("/query/int");

		expect(response.statusCode).toBe(422);
	});

	test("Query parameter with special characters - URL encoding", async () => {
		const app = createAppQueryParamsQueryParameterWithSpecialCharactersUrlEncoding();
		const client = new TestClient(app);

		const response = await client.get("/test?email=x%40test.com&special=%26%40A.ac");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("email");
		expect(responseData.email).toBe("x@test.com");
		expect(responseData).toHaveProperty("special");
		expect(responseData.special).toBe("&@A.ac");
	});

	test("List query parameter - required but missing", async () => {
		const app = createAppQueryParamsListQueryParameterRequiredButMissing();
		const client = new TestClient(app);

		const response = await client.get("/query/list");

		expect(response.statusCode).toBe(422);
	});

	test("Required string query parameter - success", async () => {
		const app = createAppQueryParamsRequiredStringQueryParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/query?query=baz");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe("foo bar baz");
	});

	test("66_multipleof_constraint_success", async () => {
		const app = createAppQueryParams66MultipleofConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items?quantity=15");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("quantity");
		expect(responseData.quantity).toBe(15);
	});

	test("53_integer_le_constraint_failure", async () => {
		const app = createAppQueryParams53IntegerLeConstraintFailure();
		const client = new TestClient(app);

		const response = await client.get("/items?limit=101");

		expect(response.statusCode).toBe(422);
	});

	test("Multiple query parameters with different types", async () => {
		const app = createAppQueryParamsMultipleQueryParametersWithDifferentTypes();
		const client = new TestClient(app);

		const response = await client.get("/query/multi-type?name=john&active=true&score=95.5&age=30");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("active");
		expect(responseData.active).toBe(true);
		expect(responseData).toHaveProperty("age");
		expect(responseData.age).toBe(30);
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("john");
		expect(responseData).toHaveProperty("score");
		expect(responseData.score).toBe(95.5);
	});

	test("71_array_separator_semicolon", async () => {
		const app = createAppQueryParams71ArraySeparatorSemicolon();
		const client = new TestClient(app);

		const response = await client.get("/items?colors=red;green;blue&colors=red%3Bgreen%3Bblue");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("colors");
		expect(responseData.colors.length).toBe(3);
		expect(responseData.colors[0]).toBe("red");
		expect(responseData.colors[1]).toBe("green");
		expect(responseData.colors[2]).toBe("blue");
	});

	test("70_array_separator_pipe", async () => {
		const app = createAppQueryParams70ArraySeparatorPipe();
		const client = new TestClient(app);

		const response = await client.get("/items?tags=python|rust|typescript&tags=python%7Crust%7Ctypescript");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("tags");
		expect(responseData.tags.length).toBe(3);
		expect(responseData.tags[0]).toBe("python");
		expect(responseData.tags[1]).toBe("rust");
		expect(responseData.tags[2]).toBe("typescript");
	});

	test("Integer with default value - not provided", async () => {
		const app = createAppQueryParamsIntegerWithDefaultValueNotProvided();
		const client = new TestClient(app);

		const response = await client.get("/query/int/default");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe("foo bar 10");
	});

	test("Boolean query parameter - true", async () => {
		const app = createAppQueryParamsBooleanQueryParameterTrue();
		const client = new TestClient(app);

		const response = await client.get("/query/bool?flag=true");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("flag");
		expect(responseData.flag).toBe(true);
	});

	test("Integer query param with le constraint - boundary", async () => {
		const app = createAppQueryParamsIntegerQueryParamWithLeConstraintBoundary();
		const client = new TestClient(app);

		const response = await client.get("/query/int-le?value=100");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("value");
		expect(responseData.value).toBe(100);
	});

	test("Float query param with ge constraint - success", async () => {
		const app = createAppQueryParamsFloatQueryParamWithGeConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/query/float-ge?price=0.01");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("price");
		expect(responseData.price).toBe(0.01);
	});

	test("51_integer_ge_constraint_boundary", async () => {
		const app = createAppQueryParams51IntegerGeConstraintBoundary();
		const client = new TestClient(app);

		const response = await client.get("/items?offset=0");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("offset");
		expect(responseData.offset).toBe(0);
	});

	test("Optional integer query parameter - missing", async () => {
		const app = createAppQueryParamsOptionalIntegerQueryParameterMissing();
		const client = new TestClient(app);

		const response = await client.get("/query/int/optional");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe("foo bar None");
	});

	test("69_array_uniqueitems_failure", async () => {
		const app = createAppQueryParams69ArrayUniqueitemsFailure();
		const client = new TestClient(app);

		const response = await client.get("/items?ids=1&ids=2&ids=2&ids=3");

		expect(response.statusCode).toBe(422);
	});

	test("72_array_separator_space", async () => {
		const app = createAppQueryParams72ArraySeparatorSpace();
		const client = new TestClient(app);

		const response = await client.get("/search?keywords=rust%20web%20framework&keywords=rust%20web%20framework");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("keywords");
		expect(responseData.keywords.length).toBe(3);
		expect(responseData.keywords[0]).toBe("rust");
		expect(responseData.keywords[1]).toBe("web");
		expect(responseData.keywords[2]).toBe("framework");
	});

	test("String validation with regex - failure", async () => {
		const app = createAppQueryParamsStringValidationWithRegexFailure();
		const client = new TestClient(app);

		const response = await client.get("/items/?item_query=nonregexquery");

		expect(response.statusCode).toBe(422);
	});

	test("65_format_hostname_success", async () => {
		const app = createAppQueryParams65FormatHostnameSuccess();
		const client = new TestClient(app);

		const response = await client.get("/dns?host=api.example.com");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("host");
		expect(responseData.host).toBe("api.example.com");
	});

	test("Query parameter with URL encoded space", async () => {
		const app = createAppQueryParamsQueryParameterWithUrlEncodedSpace();
		const client = new TestClient(app);

		const response = await client.get("/query/basic?name=hello%20world");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("name");
		expect(responseData.name).toBe("hello world");
	});

	test("List of strings - multiple values", async () => {
		const app = createAppQueryParamsListOfStringsMultipleValues();
		const client = new TestClient(app);

		const response = await client.get("/items/?q=foo&q=bar");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("q");
		expect(responseData.q.length).toBe(2);
		expect(responseData.q[0]).toBe("foo");
		expect(responseData.q[1]).toBe("bar");
	});

	test("Optional query parameter with default value", async () => {
		const app = createAppQueryParamsOptionalQueryParameterWithDefaultValue();
		const client = new TestClient(app);

		const response = await client.get("/query/optional-default");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("limit");
		expect(responseData.limit).toBe(10);
	});

	test("62_format_ipv6_success", async () => {
		const app = createAppQueryParams62FormatIpv6Success();
		const client = new TestClient(app);

		const response = await client.get("/network/ipv6?ip=2001%3A0db8%3A85a3%3A0000%3A0000%3A8a2e%3A0370%3A7334");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("ip");
		expect(responseData.ip).toBe("2001:0db8:85a3:0000:0000:8a2e:0370:7334");
	});

	test("Array query parameter - single value", async () => {
		const app = createAppQueryParamsArrayQueryParameterSingleValue();
		const client = new TestClient(app);

		const response = await client.get("/query/list-default?tags=apple");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData.length).toBe(1);
		expect(responseData[0]).toBe("apple");
	});

	test("Optional string query parameter - missing", async () => {
		const app = createAppQueryParamsOptionalStringQueryParameterMissing();
		const client = new TestClient(app);

		const response = await client.get("/query/optional");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toBe("foo bar None");
	});

	test("Datetime query parameter - success", async () => {
		const app = createAppQueryParamsDatetimeQueryParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/query/datetime?timestamp=2024-01-15T10%3A30%3A00Z");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("timestamp");
		expect(responseData.timestamp).toBe("2024-01-15T10:30:00Z");
	});

	test("UUID query parameter - invalid format", async () => {
		const app = createAppQueryParamsUuidQueryParameterInvalidFormat();
		const client = new TestClient(app);

		const response = await client.get("/query/uuid?item_id=not-a-uuid");

		expect(response.statusCode).toBe(422);
	});

	test("Array query parameter - empty array", async () => {
		const app = createAppQueryParamsArrayQueryParameterEmptyArray();
		const client = new TestClient(app);

		const response = await client.get("/query/list-default");

		expect(response.statusCode).toBe(200);
	});

	test("Enum query parameter - success", async () => {
		const app = createAppQueryParamsEnumQueryParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/query/enum?model=alexnet");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("model");
		expect(responseData.model).toBe("alexnet");
	});

	test("UUID query parameter - success", async () => {
		const app = createAppQueryParamsUuidQueryParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/query/uuid?item_id=c892496f-b1fd-4b91-bdb8-b46f92df1716");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("item_id");
		expect(responseData.item_id).toBe("c892496f-b1fd-4b91-bdb8-b46f92df1716");
	});

	test("50_integer_gt_constraint_failure", async () => {
		const app = createAppQueryParams50IntegerGtConstraintFailure();
		const client = new TestClient(app);

		const response = await client.get("/items?limit=0");

		expect(response.statusCode).toBe(422);
	});

	test("64_format_uri_failure", async () => {
		const app = createAppQueryParams64FormatUriFailure();
		const client = new TestClient(app);

		const response = await client.get("/redirect?url=not%20a%20uri");

		expect(response.statusCode).toBe(422);
	});

	test("54_array_minitems_constraint_success", async () => {
		const app = createAppQueryParams54ArrayMinitemsConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items?ids=1&ids=2&ids=3");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("ids");
		expect(responseData.ids.length).toBe(3);
		expect(responseData.ids[0]).toBe(1);
		expect(responseData.ids[1]).toBe(2);
		expect(responseData.ids[2]).toBe(3);
	});

	test("55_array_minitems_constraint_failure", async () => {
		const app = createAppQueryParams55ArrayMinitemsConstraintFailure();
		const client = new TestClient(app);

		const response = await client.get("/items?ids=1");

		expect(response.statusCode).toBe(422);
	});

	test("60_format_ipv4_success", async () => {
		const app = createAppQueryParams60FormatIpv4Success();
		const client = new TestClient(app);

		const response = await client.get("/network?ip=192.168.1.1");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("ip");
		expect(responseData.ip).toBe("192.168.1.1");
	});
});
