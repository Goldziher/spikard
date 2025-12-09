/**
 * E2E tests for path_params
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { describe, expect, test } from "vitest";
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

describe("path_params", () => {
	test("Boolean path parameter - True", async () => {
		const app = createAppPathParamsBooleanPathParameterTrue();
		const client = new TestClient(app);

		const response = await client.get("/path/bool/True");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("item_id");
		expect(responseData.item_id).toBe(true);
	});

	test("29_decimal_path_param_success", async () => {
		const app = createAppPathParams29DecimalPathParamSuccess();
		const client = new TestClient(app);

		const response = await client.get("/prices/19.99");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("amount");
		expect(responseData.amount).toBe("19.99");
	});

	test("Integer path parameter with combined lt and gt constraints - success", async () => {
		const app = createAppPathParamsIntegerPathParameterWithCombinedLtAndGtConstraintsSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/param-lt-gt/2");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("item_id");
		expect(responseData.item_id).toBe(2);
	});

	test("33_string_pattern_path_success", async () => {
		const app = createAppPathParams33StringPatternPathSuccess();
		const client = new TestClient(app);

		const response = await client.get("/repos/spikard-labs/spikard-http");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("owner");
		expect(responseData.owner).toBe("spikard-labs");
		expect(responseData).toHaveProperty("repo");
		expect(responseData.repo).toBe("spikard-http");
	});

	test("31_string_minlength_path_failure", async () => {
		const app = createAppPathParams31StringMinlengthPathFailure();
		const client = new TestClient(app);

		const response = await client.get("/users/ab");

		expect(response.statusCode).toBe(422);
	});

	test("35_negative_integer_path_param", async () => {
		const app = createAppPathParams35NegativeIntegerPathParam();
		const client = new TestClient(app);

		const response = await client.get("/offset/-100");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("value");
		expect(responseData.value).toBe(-100);
	});

	test("Enum path parameter - invalid value", async () => {
		const app = createAppPathParamsEnumPathParameterInvalidValue();
		const client = new TestClient(app);

		const response = await client.get("/models/foo");

		expect(response.statusCode).toBe(422);
	});

	test("27_datetime_format_path_param_success", async () => {
		const app = createAppPathParams27DatetimeFormatPathParamSuccess();
		const client = new TestClient(app);

		const response = await client.get("/bookings/2025-10-30T14:30:00Z");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("timestamp");
		expect(responseData.timestamp).toBe("2025-10-30T14:30:00Z");
	});

	test("25_date_format_invalid_failure", async () => {
		const app = createAppPathParams25DateFormatInvalidFailure();
		const client = new TestClient(app);

		const response = await client.get("/events/2025-13-45");

		expect(response.statusCode).toBe(422);
	});

	test("Integer path parameter with lt constraint - success", async () => {
		const app = createAppPathParamsIntegerPathParameterWithLtConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/param-lt/2");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("item_id");
		expect(responseData.item_id).toBe(2);
	});

	test("Integer path parameter with gt constraint - success", async () => {
		const app = createAppPathParamsIntegerPathParameterWithGtConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/param-gt/42");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("item_id");
		expect(responseData.item_id).toBe(42);
	});

	test("28_duration_format_path_param_success", async () => {
		const app = createAppPathParams28DurationFormatPathParamSuccess();
		const client = new TestClient(app);

		const response = await client.get("/delays/P1DT2H30M");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("duration");
		expect(responseData.duration).toBe("P1DT2H30M");
	});

	test("Path parameter type syntax with override", async () => {
		const app = createAppPathParamsPathParameterTypeSyntaxWithOverride();
		const client = new TestClient(app);

		const response = await client.get("/type-syntax/items-count/50");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("count");
		expect(responseData.count).toBe("50");
	});

	test("20_uuid_v3_path_param_success", async () => {
		const app = createAppPathParams20UuidV3PathParamSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items/e8b5a51d-11c8-3310-a6ab-367563f20686");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("id");
		expect(responseData.id).toBe("e8b5a51d-11c8-3310-a6ab-367563f20686");
	});

	test("Integer path parameter - invalid string", async () => {
		const app = createAppPathParamsIntegerPathParameterInvalidString();
		const client = new TestClient(app);

		const response = await client.get("/path/int/foobar");

		expect(response.statusCode).toBe(422);
	});

	test("30_string_minlength_path_success", async () => {
		const app = createAppPathParams30StringMinlengthPathSuccess();
		const client = new TestClient(app);

		const response = await client.get("/users/alice");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("username");
		expect(responseData.username).toBe("alice");
	});

	test("Integer path parameter with le constraint - success", async () => {
		const app = createAppPathParamsIntegerPathParameterWithLeConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/param-le/3");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("item_id");
		expect(responseData.item_id).toBe(3);
	});

	test("Path parameter type syntax - invalid UUID", async () => {
		const app = createAppPathParamsPathParameterTypeSyntaxInvalidUuid();
		const client = new TestClient(app);

		const response = await client.get("/type-syntax/items/not-a-uuid");

		expect(response.statusCode).toBe(422);
	});

	test("Path type parameter - file path", async () => {
		const app = createAppPathParamsPathTypeParameterFilePath();
		const client = new TestClient(app);

		const response = await client.get("/files/home/johndoe/myfile.txt");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("file_path");
		expect(responseData.file_path).toBe("home/johndoe/myfile.txt");
	});

	test("Path parameter with type syntax - UUID", async () => {
		const app = createAppPathParamsPathParameterWithTypeSyntaxUuid();
		const client = new TestClient(app);

		const response = await client.get("/type-syntax/items/550e8400-e29b-41d4-a716-446655440000");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("id");
		expect(responseData.id).toBe("550e8400-e29b-41d4-a716-446655440000");
	});

	test("32_string_maxlength_path_failure", async () => {
		const app = createAppPathParams32StringMaxlengthPathFailure();
		const client = new TestClient(app);

		const response = await client.get("/users/this_username_is_way_too_long_to_be_valid");

		expect(response.statusCode).toBe(422);
	});

	test("Integer path parameter - success", async () => {
		const app = createAppPathParamsIntegerPathParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/int/42");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("item_id");
		expect(responseData.item_id).toBe(42);
	});

	test("34_string_pattern_path_failure", async () => {
		const app = createAppPathParams34StringPatternPathFailure();
		const client = new TestClient(app);

		const response = await client.get("/repos/invalid@owner");

		expect(response.statusCode).toBe(422);
	});

	test("21_uuid_v5_path_param_success", async () => {
		const app = createAppPathParams21UuidV5PathParamSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items/630eb68f-e0fa-5ecc-887a-7c7a62614681");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("id");
		expect(responseData.id).toBe("630eb68f-e0fa-5ecc-887a-7c7a62614681");
	});

	test("String path parameter with max_length - failure", async () => {
		const app = createAppPathParamsStringPathParameterWithMaxLengthFailure();
		const client = new TestClient(app);

		const response = await client.get("/path/param-maxlength/foobar");

		expect(response.statusCode).toBe(422);
	});

	test("String path parameter with min_length - failure", async () => {
		const app = createAppPathParamsStringPathParameterWithMinLengthFailure();
		const client = new TestClient(app);

		const response = await client.get("/path/param-minlength/fo");

		expect(response.statusCode).toBe(422);
	});

	test("Multiple path parameters - success", async () => {
		const app = createAppPathParamsMultiplePathParametersSuccess();
		const client = new TestClient(app);

		const response = await client.get("/1.0/1/abc/c892496f-b1fd-4b91-bdb8-b46f92df1716");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("order_id");
		expect(responseData.order_id).toBe("c892496f-b1fd-4b91-bdb8-b46f92df1716");
		expect(responseData).toHaveProperty("service_id");
		expect(responseData.service_id).toBe(1);
		expect(responseData).toHaveProperty("user_id");
		expect(responseData.user_id).toBe("abc");
		expect(responseData).toHaveProperty("version");
		expect(responseData.version).toBe(1.0);
	});

	test("Date path parameter - success", async () => {
		const app = createAppPathParamsDatePathParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/date/2023-07-15");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("date_param");
		expect(responseData.date_param).toBe("2023-07-15");
	});

	test("Integer path parameter with gt constraint - failure", async () => {
		const app = createAppPathParamsIntegerPathParameterWithGtConstraintFailure();
		const client = new TestClient(app);

		const response = await client.get("/path/param-gt/2");

		expect(response.statusCode).toBe(422);
	});

	test("24_date_format_path_param_success", async () => {
		const app = createAppPathParams24DateFormatPathParamSuccess();
		const client = new TestClient(app);

		const response = await client.get("/events/2025-10-30");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("date");
		expect(responseData.date).toBe("2025-10-30");
	});

	test("Float path parameter - success", async () => {
		const app = createAppPathParamsFloatPathParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/float/42.5");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("item_id");
		expect(responseData.item_id).toBe(42.5);
	});

	test("Path parameter with type syntax - integer", async () => {
		const app = createAppPathParamsPathParameterWithTypeSyntaxInteger();
		const client = new TestClient(app);

		const response = await client.get("/type-syntax/users/42");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("user_id");
		expect(responseData.user_id).toBe("42");
	});

	test("String path parameter - success", async () => {
		const app = createAppPathParamsStringPathParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/str/foobar");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("item_id");
		expect(responseData.item_id).toBe("foobar");
	});

	test("UUID path parameter - success", async () => {
		const app = createAppPathParamsUuidPathParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/items/ec38df32-ceda-4cfa-9b4a-1aeb94ad551a");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("item_id");
		expect(responseData.item_id).toBe("ec38df32-ceda-4cfa-9b4a-1aeb94ad551a");
	});

	test("Integer path parameter with ge constraint - success", async () => {
		const app = createAppPathParamsIntegerPathParameterWithGeConstraintSuccess();
		const client = new TestClient(app);

		const response = await client.get("/path/param-ge/3");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("item_id");
		expect(responseData.item_id).toBe(3);
	});

	test("Enum path parameter - success", async () => {
		const app = createAppPathParamsEnumPathParameterSuccess();
		const client = new TestClient(app);

		const response = await client.get("/models/alexnet");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("model_name");
		expect(responseData.model_name).toBe("alexnet");
	});

	test("Boolean path parameter - numeric 1", async () => {
		const app = createAppPathParamsBooleanPathParameterNumeric1();
		const client = new TestClient(app);

		const response = await client.get("/path/bool/1");

		expect(response.statusCode).toBe(200);
		const responseData = response.json();
		expect(responseData).toHaveProperty("item_id");
		expect(responseData.item_id).toBe(true);
	});
});
