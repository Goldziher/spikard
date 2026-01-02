/**
 * E2E tests for path_params
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assert, assertEquals } from "jsr:@std/assert@1";
import {
	createAppPathParams20UuidV3PathParamSuccess,
	createAppPathParams21UuidV5PathParamSuccess,
	createAppPathParams24DateFormatPathParamSuccess,
	createAppPathParams25DateFormatInvalidFailure,
	createAppPathParams27DatetimeFormatPathParamSuccess,
	createAppPathParams28DurationFormatPathParamSuccess,
	createAppPathParams29DecimalPathParamSuccess,
	createAppPathParams30StringMinlengthPathSuccess,
	createAppPathParams31StringMinlengthPathFailure,
	createAppPathParams32StringMaxlengthPathFailure,
	createAppPathParams33StringPatternPathSuccess,
	createAppPathParams34StringPatternPathFailure,
	createAppPathParams35NegativeIntegerPathParam,
	createAppPathParamsBooleanPathParameterNumeric1,
	createAppPathParamsBooleanPathParameterTrue,
	createAppPathParamsDatePathParameterSuccess,
	createAppPathParamsEnumPathParameterInvalidValue,
	createAppPathParamsEnumPathParameterSuccess,
	createAppPathParamsFloatPathParameterSuccess,
	createAppPathParamsIntegerPathParameterInvalidString,
	createAppPathParamsIntegerPathParameterSuccess,
	createAppPathParamsIntegerPathParameterWithCombinedLtAndGtConstraintsSuccess,
	createAppPathParamsIntegerPathParameterWithGeConstraintSuccess,
	createAppPathParamsIntegerPathParameterWithGtConstraintFailure,
	createAppPathParamsIntegerPathParameterWithGtConstraintSuccess,
	createAppPathParamsIntegerPathParameterWithLeConstraintSuccess,
	createAppPathParamsIntegerPathParameterWithLtConstraintSuccess,
	createAppPathParamsMultiplePathParametersSuccess,
	createAppPathParamsPathParameterTypeSyntaxInvalidUuid,
	createAppPathParamsPathParameterTypeSyntaxWithOverride,
	createAppPathParamsPathParameterWithTypeSyntaxInteger,
	createAppPathParamsPathParameterWithTypeSyntaxUuid,
	createAppPathParamsPathTypeParameterFilePath,
	createAppPathParamsStringPathParameterSuccess,
	createAppPathParamsStringPathParameterWithMaxLengthFailure,
	createAppPathParamsStringPathParameterWithMinLengthFailure,
	createAppPathParamsUuidPathParameterSuccess,
} from "../app/main.ts";

	Deno.test("path_params: Boolean path parameter - True", async () => {
		const app = createAppPathParamsBooleanPathParameterTrue();
		const client = new TestClient(app);

		const response = await client.get("/path/bool/True");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "item_id"));
		assertEquals(responseData.item_id, true);
	});

	Deno.test("path_params: 29_decimal_path_param_success", async () => {
		const app = createAppPathParams29DecimalPathParamSuccess();
		const client = new TestClient(app);

		const response = await client.get("/prices/19.99");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "amount"));
		assertEquals(responseData.amount, "19.99");
	});

	Deno.test("path_params: Integer path parameter with combined lt and gt constraints - success", async () => {
		const app = createAppPathParamsIntegerPathParameterWithCombinedLtAndGtConstraintsSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/param-lt-gt/2");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "item_id"));
		assertEquals(responseData.item_id, 2);
	});

	Deno.test("path_params: 33_string_pattern_path_success", async () => {
		const app = createAppPathParams33StringPatternPathSuccess();
		const client = new TestClient(app);

		const response = await client.get("/repos/spikard-labs/spikard-http");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "owner"));
		assertEquals(responseData.owner, "spikard-labs");
		assert(Object.hasOwn(responseData, "repo"));
		assertEquals(responseData.repo, "spikard-http");
	});

	Deno.test("path_params: 31_string_minlength_path_failure", async () => {
		const app = createAppPathParams31StringMinlengthPathFailure();
		const client = new TestClient(app);

		const response = await client.get("/users/ab");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("path_params: 35_negative_integer_path_param", async () => {
		const app = createAppPathParams35NegativeIntegerPathParam();
		const client = new TestClient(app);

		const response = await client.get("/offset/-100");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "value"));
		assertEquals(responseData.value, -100);
	});

	Deno.test("path_params: Enum path parameter - invalid value", async () => {
		const app = createAppPathParamsEnumPathParameterInvalidValue();
		const client = new TestClient(app);

		const response = await client.get("/models/foo");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("path_params: 27_datetime_format_path_param_success", async () => {
		const app = createAppPathParams27DatetimeFormatPathParamSuccess();
		const client = new TestClient(app);

		const response = await client.get("/bookings/2025-10-30T14:30:00Z");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "timestamp"));
		assertEquals(responseData.timestamp, "2025-10-30T14:30:00Z");
	});

	Deno.test("path_params: 25_date_format_invalid_failure", async () => {
		const app = createAppPathParams25DateFormatInvalidFailure();
		const client = new TestClient(app);

		const response = await client.get("/events/2025-13-45");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("path_params: Integer path parameter with lt constraint - success", async () => {
		const app = createAppPathParamsIntegerPathParameterWithLtConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/param-lt/2");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "item_id"));
		assertEquals(responseData.item_id, 2);
	});

	Deno.test("path_params: Integer path parameter with gt constraint - success", async () => {
		const app = createAppPathParamsIntegerPathParameterWithGtConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/param-gt/42");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "item_id"));
		assertEquals(responseData.item_id, 42);
	});

	Deno.test("path_params: 28_duration_format_path_param_success", async () => {
		const app = createAppPathParams28DurationFormatPathParamSuccess();
		const client = new TestClient(app);

		const response = await client.get("/delays/P1DT2H30M");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "duration"));
		assertEquals(responseData.duration, "P1DT2H30M");
	});

	Deno.test("path_params: Path parameter type syntax with override", async () => {
		const app = createAppPathParamsPathParameterTypeSyntaxWithOverride();
		const client = new TestClient(app);

		const response = await client.get("/type-syntax/items-count/50");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "count"));
		assertEquals(responseData.count, "50");
	});

	Deno.test("path_params: 20_uuid_v3_path_param_success", async () => {
		const app = createAppPathParams20UuidV3PathParamSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items/e8b5a51d-11c8-3310-a6ab-367563f20686");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "id"));
		assertEquals(responseData.id, "e8b5a51d-11c8-3310-a6ab-367563f20686");
	});

	Deno.test("path_params: Integer path parameter - invalid string", async () => {
		const app = createAppPathParamsIntegerPathParameterInvalidString();
		const client = new TestClient(app);

		const response = await client.get("/path/int/foobar");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("path_params: 30_string_minlength_path_success", async () => {
		const app = createAppPathParams30StringMinlengthPathSuccess();
		const client = new TestClient(app);

		const response = await client.get("/users/alice");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "username"));
		assertEquals(responseData.username, "alice");
	});

	Deno.test("path_params: Integer path parameter with le constraint - success", async () => {
		const app = createAppPathParamsIntegerPathParameterWithLeConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/param-le/3");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "item_id"));
		assertEquals(responseData.item_id, 3);
	});

	Deno.test("path_params: Path parameter type syntax - invalid UUID", async () => {
		const app = createAppPathParamsPathParameterTypeSyntaxInvalidUuid();
		const client = new TestClient(app);

		const response = await client.get("/type-syntax/items/not-a-uuid");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("path_params: Path type parameter - file path", async () => {
		const app = createAppPathParamsPathTypeParameterFilePath();
		const client = new TestClient(app);

		const response = await client.get("/files/home/johndoe/myfile.txt");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "file_path"));
		assertEquals(responseData.file_path, "home/johndoe/myfile.txt");
	});

	Deno.test("path_params: Path parameter with type syntax - UUID", async () => {
		const app = createAppPathParamsPathParameterWithTypeSyntaxUuid();
		const client = new TestClient(app);

		const response = await client.get("/type-syntax/items/550e8400-e29b-41d4-a716-446655440000");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "id"));
		assertEquals(responseData.id, "550e8400-e29b-41d4-a716-446655440000");
	});

	Deno.test("path_params: 32_string_maxlength_path_failure", async () => {
		const app = createAppPathParams32StringMaxlengthPathFailure();
		const client = new TestClient(app);

		const response = await client.get("/users/this_username_is_way_too_long_to_be_valid");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("path_params: Integer path parameter - success", async () => {
		const app = createAppPathParamsIntegerPathParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/int/42");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "item_id"));
		assertEquals(responseData.item_id, 42);
	});

	Deno.test("path_params: 34_string_pattern_path_failure", async () => {
		const app = createAppPathParams34StringPatternPathFailure();
		const client = new TestClient(app);

		const response = await client.get("/repos/invalid@owner");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("path_params: 21_uuid_v5_path_param_success", async () => {
		const app = createAppPathParams21UuidV5PathParamSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items/630eb68f-e0fa-5ecc-887a-7c7a62614681");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "id"));
		assertEquals(responseData.id, "630eb68f-e0fa-5ecc-887a-7c7a62614681");
	});

	Deno.test("path_params: String path parameter with max_length - failure", async () => {
		const app = createAppPathParamsStringPathParameterWithMaxLengthFailure();
		const client = new TestClient(app);

		const response = await client.get("/path/param-maxlength/foobar");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("path_params: String path parameter with min_length - failure", async () => {
		const app = createAppPathParamsStringPathParameterWithMinLengthFailure();
		const client = new TestClient(app);

		const response = await client.get("/path/param-minlength/fo");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("path_params: Multiple path parameters - success", async () => {
		const app = createAppPathParamsMultiplePathParametersSuccess();
		const client = new TestClient(app);

		const response = await client.get("/1.0/1/abc/c892496f-b1fd-4b91-bdb8-b46f92df1716");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "version"));
		assertEquals(responseData.version, 1.0);
		assert(Object.hasOwn(responseData, "service_id"));
		assertEquals(responseData.service_id, 1);
		assert(Object.hasOwn(responseData, "user_id"));
		assertEquals(responseData.user_id, "abc");
		assert(Object.hasOwn(responseData, "order_id"));
		assertEquals(responseData.order_id, "c892496f-b1fd-4b91-bdb8-b46f92df1716");
	});

	Deno.test("path_params: Date path parameter - success", async () => {
		const app = createAppPathParamsDatePathParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/date/2023-07-15");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "date_param"));
		assertEquals(responseData.date_param, "2023-07-15");
	});

	Deno.test("path_params: Integer path parameter with gt constraint - failure", async () => {
		const app = createAppPathParamsIntegerPathParameterWithGtConstraintFailure();
		const client = new TestClient(app);

		const response = await client.get("/path/param-gt/2");

		assertEquals(response.statusCode, 422);
	});

	Deno.test("path_params: 24_date_format_path_param_success", async () => {
		const app = createAppPathParams24DateFormatPathParamSuccess();
		const client = new TestClient(app);

		const response = await client.get("/events/2025-10-30");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "date"));
		assertEquals(responseData.date, "2025-10-30");
	});

	Deno.test("path_params: Float path parameter - success", async () => {
		const app = createAppPathParamsFloatPathParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/float/42.5");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "item_id"));
		assertEquals(responseData.item_id, 42.5);
	});

	Deno.test("path_params: Path parameter with type syntax - integer", async () => {
		const app = createAppPathParamsPathParameterWithTypeSyntaxInteger();
		const client = new TestClient(app);

		const response = await client.get("/type-syntax/users/42");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "user_id"));
		assertEquals(responseData.user_id, "42");
	});

	Deno.test("path_params: String path parameter - success", async () => {
		const app = createAppPathParamsStringPathParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/str/foobar");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "item_id"));
		assertEquals(responseData.item_id, "foobar");
	});

	Deno.test("path_params: UUID path parameter - success", async () => {
		const app = createAppPathParamsUuidPathParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items/ec38df32-ceda-4cfa-9b4a-1aeb94ad551a");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "item_id"));
		assertEquals(responseData.item_id, "ec38df32-ceda-4cfa-9b4a-1aeb94ad551a");
	});

	Deno.test("path_params: Integer path parameter with ge constraint - success", async () => {
		const app = createAppPathParamsIntegerPathParameterWithGeConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/param-ge/3");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "item_id"));
		assertEquals(responseData.item_id, 3);
	});

	Deno.test("path_params: Enum path parameter - success", async () => {
		const app = createAppPathParamsEnumPathParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/models/alexnet");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "model_name"));
		assertEquals(responseData.model_name, "alexnet");
	});

	Deno.test("path_params: Boolean path parameter - numeric 1", async () => {
		const app = createAppPathParamsBooleanPathParameterNumeric1();
		const client = new TestClient(app);

		const response = await client.get("/path/bool/1");

		assertEquals(response.statusCode, 200);
		const responseData = response.json();
		assert(Object.hasOwn(responseData, "item_id"));
		assertEquals(responseData.item_id, true);
	});