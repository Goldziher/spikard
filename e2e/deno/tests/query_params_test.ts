/**
 * E2E tests for query_params
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
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
} from "../app/main.ts";

	Deno.test("query_params: String validation with regex - success", async () => {
		const app = createAppQueryParamsStringValidationWithRegexSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items/?item_query=fixedquery");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "item_query"));
		assertEquals(responseData.item_query, "fixedquery");
	});

	Deno.test("query_params: 49_integer_gt_constraint_success", async () => {
		const app = createAppQueryParams49IntegerGtConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items?limit=5");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "limit"));
		assertEquals(responseData.limit, 5);
	});

	Deno.test("query_params: Enum query parameter - invalid value", async () => {
		const app = createAppQueryParamsEnumQueryParameterInvalidValue();
		const client = new TestClient(app);

		const response = await client.get("/query/enum?model=vgg16");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: 68_array_uniqueitems_success", async () => {
		const app = createAppQueryParams68ArrayUniqueitemsSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items?ids=1&ids=2&ids=3&ids=4");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "ids"));
		assertEquals(responseData.ids.length, 4);
		assertEquals(responseData.ids[0], 1);
		assertEquals(responseData.ids[1], 2);
		assertEquals(responseData.ids[2], 3);
		assertEquals(responseData.ids[3], 4);
	});

	Deno.test("query_params: 47_pattern_validation_email_success", async () => {
		const app = createAppQueryParams47PatternValidationEmailSuccess();
		const client = new TestClient(app);

		const response = await client.get("/subscribe?email=user%40example.com");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "email"));
		assertEquals(responseData.email, "user@example.com");
	});

	Deno.test("query_params: Required integer query parameter - success", async () => {
		const app = createAppQueryParamsRequiredIntegerQueryParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/query/int?query=42");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "foo bar 42");
	});

	Deno.test("query_params: Required string query parameter - missing", async () => {
		const app = createAppQueryParamsRequiredStringQueryParameterMissing();
		const client = new TestClient(app);

		const response = await client.get("/query");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: 57_boolean_empty_string_coercion", async () => {
		const app = createAppQueryParams57BooleanEmptyStringCoercion();
		const client = new TestClient(app);

		const response = await client.get("/items?active=");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "active"));
		assertEquals(responseData.active, false);
	});

	Deno.test("query_params: 52_integer_le_constraint_boundary", async () => {
		const app = createAppQueryParams52IntegerLeConstraintBoundary();
		const client = new TestClient(app);

		const response = await client.get("/items?limit=100");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "limit"));
		assertEquals(responseData.limit, 100);
	});

	Deno.test("query_params: List with default empty array - no values provided", async () => {
		const app = createAppQueryParamsListWithDefaultEmptyArrayNoValuesProvided();
		const client = new TestClient(app);

		const response = await client.get("/query/list-default");

		assertEquals(response.statusCode, 200);
	});

	Deno.test("query_params: Date query parameter - success", async () => {
		const app = createAppQueryParamsDateQueryParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/query/date?event_date=2024-01-15");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "event_date"));
		assertEquals(responseData.event_date, "2024-01-15");
	});

	Deno.test("query_params: String query param with max_length constraint - fail", async () => {
		const app = createAppQueryParamsStringQueryParamWithMaxLengthConstraintFail();
		const client = new TestClient(app);

		const response = await client.get("/query/str-max-length?name=this_is_way_too_long");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: 45_string_minlength_validation_failure", async () => {
		const app = createAppQueryParams45StringMinlengthValidationFailure();
		const client = new TestClient(app);

		const response = await client.get("/search?term=ab");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: Integer with default value - override", async () => {
		const app = createAppQueryParamsIntegerWithDefaultValueOverride();
		const client = new TestClient(app);

		const response = await client.get("/query/int/default?query=50");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "foo bar 50");
	});

	Deno.test("query_params: 67_multipleof_constraint_failure", async () => {
		const app = createAppQueryParams67MultipleofConstraintFailure();
		const client = new TestClient(app);

		const response = await client.get("/items?quantity=17");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: 58_format_email_success", async () => {
		const app = createAppQueryParams58FormatEmailSuccess();
		const client = new TestClient(app);

		const response = await client.get("/subscribe?email=user%40example.com");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "email"));
		assertEquals(responseData.email, "user@example.com");
	});

	Deno.test("query_params: Integer query param with ge constraint - boundary", async () => {
		const app = createAppQueryParamsIntegerQueryParamWithGeConstraintBoundary();
		const client = new TestClient(app);

		const response = await client.get("/query/int-ge?value=10");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "value"));
		assertEquals(responseData.value, 10);
	});

	Deno.test("query_params: Integer query param with gt constraint - valid", async () => {
		const app = createAppQueryParamsIntegerQueryParamWithGtConstraintValid();
		const client = new TestClient(app);

		const response = await client.get("/query/int-gt?value=1");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "value"));
		assertEquals(responseData.value, 1);
	});

	Deno.test("query_params: Required integer query parameter - invalid type", async () => {
		const app = createAppQueryParamsRequiredIntegerQueryParameterInvalidType();
		const client = new TestClient(app);

		const response = await client.get("/query/int?query=baz");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: Required integer query parameter - float value", async () => {
		const app = createAppQueryParamsRequiredIntegerQueryParameterFloatValue();
		const client = new TestClient(app);

		const response = await client.get("/query/int?query=42.5");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: Query parameter with URL encoded special characters", async () => {
		const app = createAppQueryParamsQueryParameterWithUrlEncodedSpecialCharacters();
		const client = new TestClient(app);

		const response = await client.get("/query/basic?name=test%26value%3D123");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "test&value=123");
	});

	Deno.test("query_params: 59_format_email_failure", async () => {
		const app = createAppQueryParams59FormatEmailFailure();
		const client = new TestClient(app);

		const response = await client.get("/subscribe?email=not-an-email");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: 43_scientific_notation_float", async () => {
		const app = createAppQueryParams43ScientificNotationFloat();
		const client = new TestClient(app);

		const response = await client.get("/stats?threshold=1.5e-3");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "threshold"));
		assertEquals(responseData.threshold, 0.0015);
	});

	Deno.test("query_params: 63_format_uri_success", async () => {
		const app = createAppQueryParams63FormatUriSuccess();
		const client = new TestClient(app);

		const response = await client.get("/redirect?url=https%3A%2F%2Fexample.com%2Fpath%3Fquery%3Dvalue");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "url"));
		assertEquals(responseData.url, "https://example.com/path?query=value");
	});

	Deno.test("query_params: Boolean query parameter - numeric 1", async () => {
		const app = createAppQueryParamsBooleanQueryParameterNumeric1();
		const client = new TestClient(app);

		const response = await client.get("/query/bool?flag=1");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "flag"));
		assertEquals(responseData.flag, true);
	});

	Deno.test("query_params: String query param with min_length constraint - fail", async () => {
		const app = createAppQueryParamsStringQueryParamWithMinLengthConstraintFail();
		const client = new TestClient(app);

		const response = await client.get("/query/str-min-length?name=ab");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: Optional string query parameter - provided", async () => {
		const app = createAppQueryParamsOptionalStringQueryParameterProvided();
		const client = new TestClient(app);

		const response = await client.get("/query/optional?query=baz");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "foo bar baz");
	});

	Deno.test("query_params: List of integers - multiple values", async () => {
		const app = createAppQueryParamsListOfIntegersMultipleValues();
		const client = new TestClient(app);

		const response = await client.get("/query/list?device_ids=1&device_ids=2");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData.length, 2);
		assertEquals(responseData[0], 1);
		assertEquals(responseData[1], 2);
	});

	Deno.test("query_params: Integer query param with lt constraint - valid", async () => {
		const app = createAppQueryParamsIntegerQueryParamWithLtConstraintValid();
		const client = new TestClient(app);

		const response = await client.get("/query/int-lt?value=49");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "value"));
		assertEquals(responseData.value, 49);
	});

	Deno.test("query_params: 42_negative_integer_query_param", async () => {
		const app = createAppQueryParams42NegativeIntegerQueryParam();
		const client = new TestClient(app);

		const response = await client.get("/items/negative?offset=-10");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "offset"));
		assertEquals(responseData.offset, -10);
	});

	Deno.test("query_params: 46_string_maxlength_validation_failure", async () => {
		const app = createAppQueryParams46StringMaxlengthValidationFailure();
		const client = new TestClient(app);

		const response = await client.get("/search?term=this_is_way_too_long");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: 56_array_maxitems_constraint_failure", async () => {
		const app = createAppQueryParams56ArrayMaxitemsConstraintFailure();
		const client = new TestClient(app);

		const response = await client.get("/items?tags=a&tags=b&tags=c&tags=d&tags=e&tags=f");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: String query param with regex pattern - fail", async () => {
		const app = createAppQueryParamsStringQueryParamWithRegexPatternFail();
		const client = new TestClient(app);

		const response = await client.get("/query/pattern?code=abc123");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: 44_string_minlength_validation_success", async () => {
		const app = createAppQueryParams44StringMinlengthValidationSuccess();
		const client = new TestClient(app);

		const response = await client.get("/search?term=foo");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "term"));
		assertEquals(responseData.term, "foo");
	});

	Deno.test("query_params: 61_format_ipv4_failure", async () => {
		const app = createAppQueryParams61FormatIpv4Failure();
		const client = new TestClient(app);

		const response = await client.get("/network?ip=999.999.999.999");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: 48_pattern_validation_email_failure", async () => {
		const app = createAppQueryParams48PatternValidationEmailFailure();
		const client = new TestClient(app);

		const response = await client.get("/subscribe?email=invalid-email");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: Required integer query parameter - missing", async () => {
		const app = createAppQueryParamsRequiredIntegerQueryParameterMissing();
		const client = new TestClient(app);

		const response = await client.get("/query/int");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: Query parameter with special characters - URL encoding", async () => {
		const app = createAppQueryParamsQueryParameterWithSpecialCharactersUrlEncoding();
		const client = new TestClient(app);

		const response = await client.get("/test?email=x%40test.com&special=%26%40A.ac");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "email"));
		assertEquals(responseData.email, "x@test.com");
		assert(Object.hasOwn(responseData, "special"));
		assertEquals(responseData.special, "&@A.ac");
	});

	Deno.test("query_params: List query parameter - required but missing", async () => {
		const app = createAppQueryParamsListQueryParameterRequiredButMissing();
		const client = new TestClient(app);

		const response = await client.get("/query/list");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: Required string query parameter - success", async () => {
		const app = createAppQueryParamsRequiredStringQueryParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/query?query=baz");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "foo bar baz");
	});

	Deno.test("query_params: 66_multipleof_constraint_success", async () => {
		const app = createAppQueryParams66MultipleofConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items?quantity=15");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "quantity"));
		assertEquals(responseData.quantity, 15);
	});

	Deno.test("query_params: 53_integer_le_constraint_failure", async () => {
		const app = createAppQueryParams53IntegerLeConstraintFailure();
		const client = new TestClient(app);

		const response = await client.get("/items?limit=101");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: Multiple query parameters with different types", async () => {
		const app = createAppQueryParamsMultipleQueryParametersWithDifferentTypes();
		const client = new TestClient(app);

		const response = await client.get("/query/multi-type?name=john&active=true&age=30&score=95.5");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "john");
		assert(Object.hasOwn(responseData, "age"));
		assertEquals(responseData.age, 30);
		assert(Object.hasOwn(responseData, "active"));
		assertEquals(responseData.active, true);
		assert(Object.hasOwn(responseData, "score"));
		assertEquals(responseData.score, 95.5);
	});

	Deno.test("query_params: 71_array_separator_semicolon", async () => {
		const app = createAppQueryParams71ArraySeparatorSemicolon();
		const client = new TestClient(app);

		const response = await client.get("/items?colors=red;green;blue&colors=red%3Bgreen%3Bblue");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "colors"));
		assertEquals(responseData.colors.length, 3);
		assertEquals(responseData.colors[0], "red");
		assertEquals(responseData.colors[1], "green");
		assertEquals(responseData.colors[2], "blue");
	});

	Deno.test("query_params: 70_array_separator_pipe", async () => {
		const app = createAppQueryParams70ArraySeparatorPipe();
		const client = new TestClient(app);

		const response = await client.get("/items?tags=python|rust|typescript&tags=python%7Crust%7Ctypescript");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "tags"));
		assertEquals(responseData.tags.length, 3);
		assertEquals(responseData.tags[0], "python");
		assertEquals(responseData.tags[1], "rust");
		assertEquals(responseData.tags[2], "typescript");
	});

	Deno.test("query_params: Integer with default value - not provided", async () => {
		const app = createAppQueryParamsIntegerWithDefaultValueNotProvided();
		const client = new TestClient(app);

		const response = await client.get("/query/int/default");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "foo bar 10");
	});

	Deno.test("query_params: Boolean query parameter - true", async () => {
		const app = createAppQueryParamsBooleanQueryParameterTrue();
		const client = new TestClient(app);

		const response = await client.get("/query/bool?flag=true");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "flag"));
		assertEquals(responseData.flag, true);
	});

	Deno.test("query_params: Integer query param with le constraint - boundary", async () => {
		const app = createAppQueryParamsIntegerQueryParamWithLeConstraintBoundary();
		const client = new TestClient(app);

		const response = await client.get("/query/int-le?value=100");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "value"));
		assertEquals(responseData.value, 100);
	});

	Deno.test("query_params: Float query param with ge constraint - success", async () => {
		const app = createAppQueryParamsFloatQueryParamWithGeConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/query/float-ge?price=0.01");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "price"));
		assertEquals(responseData.price, 0.01);
	});

	Deno.test("query_params: 51_integer_ge_constraint_boundary", async () => {
		const app = createAppQueryParams51IntegerGeConstraintBoundary();
		const client = new TestClient(app);

		const response = await client.get("/items?offset=0");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "offset"));
		assertEquals(responseData.offset, 0);
	});

	Deno.test("query_params: Optional integer query parameter - missing", async () => {
		const app = createAppQueryParamsOptionalIntegerQueryParameterMissing();
		const client = new TestClient(app);

		const response = await client.get("/query/int/optional");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "foo bar None");
	});

	Deno.test("query_params: 69_array_uniqueitems_failure", async () => {
		const app = createAppQueryParams69ArrayUniqueitemsFailure();
		const client = new TestClient(app);

		const response = await client.get("/items?ids=1&ids=2&ids=2&ids=3");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: 72_array_separator_space", async () => {
		const app = createAppQueryParams72ArraySeparatorSpace();
		const client = new TestClient(app);

		const response = await client.get("/search?keywords=rust%20web%20framework&keywords=rust%20web%20framework");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "keywords"));
		assertEquals(responseData.keywords.length, 3);
		assertEquals(responseData.keywords[0], "rust");
		assertEquals(responseData.keywords[1], "web");
		assertEquals(responseData.keywords[2], "framework");
	});

	Deno.test("query_params: String validation with regex - failure", async () => {
		const app = createAppQueryParamsStringValidationWithRegexFailure();
		const client = new TestClient(app);

		const response = await client.get("/items/?item_query=nonregexquery");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: 65_format_hostname_success", async () => {
		const app = createAppQueryParams65FormatHostnameSuccess();
		const client = new TestClient(app);

		const response = await client.get("/dns?host=api.example.com");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "host"));
		assertEquals(responseData.host, "api.example.com");
	});

	Deno.test("query_params: Query parameter with URL encoded space", async () => {
		const app = createAppQueryParamsQueryParameterWithUrlEncodedSpace();
		const client = new TestClient(app);

		const response = await client.get("/query/basic?name=hello%20world");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "name"));
		assertEquals(responseData.name, "hello world");
	});

	Deno.test("query_params: List of strings - multiple values", async () => {
		const app = createAppQueryParamsListOfStringsMultipleValues();
		const client = new TestClient(app);

		const response = await client.get("/items/?q=foo&q=bar");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "q"));
		assertEquals(responseData.q.length, 2);
		assertEquals(responseData.q[0], "foo");
		assertEquals(responseData.q[1], "bar");
	});

	Deno.test("query_params: Optional query parameter with default value", async () => {
		const app = createAppQueryParamsOptionalQueryParameterWithDefaultValue();
		const client = new TestClient(app);

		const response = await client.get("/query/optional-default");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "limit"));
		assertEquals(responseData.limit, 10);
	});

	Deno.test("query_params: 62_format_ipv6_success", async () => {
		const app = createAppQueryParams62FormatIpv6Success();
		const client = new TestClient(app);

		const response = await client.get("/network/ipv6?ip=2001%3A0db8%3A85a3%3A0000%3A0000%3A8a2e%3A0370%3A7334");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "ip"));
		assertEquals(responseData.ip, "2001:0db8:85a3:0000:0000:8a2e:0370:7334");
	});

	Deno.test("query_params: Array query parameter - single value", async () => {
		const app = createAppQueryParamsArrayQueryParameterSingleValue();
		const client = new TestClient(app);

		const response = await client.get("/query/list-default?tags=apple");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData.length, 1);
		assertEquals(responseData[0], "apple");
	});

	Deno.test("query_params: Optional string query parameter - missing", async () => {
		const app = createAppQueryParamsOptionalStringQueryParameterMissing();
		const client = new TestClient(app);

		const response = await client.get("/query/optional");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assertEquals(responseData, "foo bar None");
	});

	Deno.test("query_params: Datetime query parameter - success", async () => {
		const app = createAppQueryParamsDatetimeQueryParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/query/datetime?timestamp=2024-01-15T10%3A30%3A00Z");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "timestamp"));
		assertEquals(responseData.timestamp, "2024-01-15T10:30:00Z");
	});

	Deno.test("query_params: UUID query parameter - invalid format", async () => {
		const app = createAppQueryParamsUuidQueryParameterInvalidFormat();
		const client = new TestClient(app);

		const response = await client.get("/query/uuid?item_id=not-a-uuid");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: Array query parameter - empty array", async () => {
		const app = createAppQueryParamsArrayQueryParameterEmptyArray();
		const client = new TestClient(app);

		const response = await client.get("/query/list-default");

		assertEquals(response.statusCode, 200);
	});

	Deno.test("query_params: Enum query parameter - success", async () => {
		const app = createAppQueryParamsEnumQueryParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/query/enum?model=alexnet");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "model"));
		assertEquals(responseData.model, "alexnet");
	});

	Deno.test("query_params: UUID query parameter - success", async () => {
		const app = createAppQueryParamsUuidQueryParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/query/uuid?item_id=c892496f-b1fd-4b91-bdb8-b46f92df1716");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "item_id"));
		assertEquals(responseData.item_id, "c892496f-b1fd-4b91-bdb8-b46f92df1716");
	});

	Deno.test("query_params: 50_integer_gt_constraint_failure", async () => {
		const app = createAppQueryParams50IntegerGtConstraintFailure();
		const client = new TestClient(app);

		const response = await client.get("/items?limit=0");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: 64_format_uri_failure", async () => {
		const app = createAppQueryParams64FormatUriFailure();
		const client = new TestClient(app);

		const response = await client.get("/redirect?url=not%20a%20uri");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: 54_array_minitems_constraint_success", async () => {
		const app = createAppQueryParams54ArrayMinitemsConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items?ids=1&ids=2&ids=3");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "ids"));
		assertEquals(responseData.ids.length, 3);
		assertEquals(responseData.ids[0], 1);
		assertEquals(responseData.ids[1], 2);
		assertEquals(responseData.ids[2], 3);
	});

	Deno.test("query_params: 55_array_minitems_constraint_failure", async () => {
		const app = createAppQueryParams55ArrayMinitemsConstraintFailure();
		const client = new TestClient(app);

		const response = await client.get("/items?ids=1");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("query_params: 60_format_ipv4_success", async () => {
		const app = createAppQueryParams60FormatIpv4Success();
		const client = new TestClient(app);

		const response = await client.get("/network?ip=192.168.1.1");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "ip"));
		assertEquals(responseData.ip, "192.168.1.1");
	});