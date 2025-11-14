"""E2E tests for query_params."""

from spikard.testing import TestClient
from app.main import (
    create_app_query_params_42_negative_integer_query_param,
    create_app_query_params_43_scientific_notation_float,
    create_app_query_params_44_string_minlength_validation_success,
    create_app_query_params_45_string_minlength_validation_failure,
    create_app_query_params_46_string_maxlength_validation_failure,
    create_app_query_params_47_pattern_validation_email_success,
    create_app_query_params_48_pattern_validation_email_failure,
    create_app_query_params_49_integer_gt_constraint_success,
    create_app_query_params_50_integer_gt_constraint_failure,
    create_app_query_params_51_integer_ge_constraint_boundary,
    create_app_query_params_52_integer_le_constraint_boundary,
    create_app_query_params_53_integer_le_constraint_failure,
    create_app_query_params_54_array_minitems_constraint_success,
    create_app_query_params_55_array_minitems_constraint_failure,
    create_app_query_params_56_array_maxitems_constraint_failure,
    create_app_query_params_57_boolean_empty_string_coercion,
    create_app_query_params_58_format_email_success,
    create_app_query_params_59_format_email_failure,
    create_app_query_params_60_format_ipv4_success,
    create_app_query_params_61_format_ipv4_failure,
    create_app_query_params_62_format_ipv6_success,
    create_app_query_params_63_format_uri_success,
    create_app_query_params_64_format_uri_failure,
    create_app_query_params_65_format_hostname_success,
    create_app_query_params_66_multipleof_constraint_success,
    create_app_query_params_67_multipleof_constraint_failure,
    create_app_query_params_68_array_uniqueitems_success,
    create_app_query_params_69_array_uniqueitems_failure,
    create_app_query_params_70_array_separator_pipe,
    create_app_query_params_71_array_separator_semicolon,
    create_app_query_params_72_array_separator_space,
    create_app_query_params_array_query_parameter_empty_array,
    create_app_query_params_array_query_parameter_single_value,
    create_app_query_params_boolean_query_parameter_numeric_1,
    create_app_query_params_boolean_query_parameter_true,
    create_app_query_params_date_query_parameter_success,
    create_app_query_params_datetime_query_parameter_success,
    create_app_query_params_enum_query_parameter_invalid_value,
    create_app_query_params_enum_query_parameter_success,
    create_app_query_params_float_query_param_with_ge_constraint_success,
    create_app_query_params_integer_query_param_with_ge_constraint_boundary,
    create_app_query_params_integer_query_param_with_gt_constraint_valid,
    create_app_query_params_integer_query_param_with_le_constraint_boundary,
    create_app_query_params_integer_query_param_with_lt_constraint_valid,
    create_app_query_params_integer_with_default_value_not_provided,
    create_app_query_params_integer_with_default_value_override,
    create_app_query_params_list_of_integers_multiple_values,
    create_app_query_params_list_of_strings_multiple_values,
    create_app_query_params_list_query_parameter_required_but_missing,
    create_app_query_params_list_with_default_empty_array_no_values_provided,
    create_app_query_params_multiple_query_parameters_with_different_types,
    create_app_query_params_optional_integer_query_parameter_missing,
    create_app_query_params_optional_query_parameter_with_default_value,
    create_app_query_params_optional_string_query_parameter_missing,
    create_app_query_params_optional_string_query_parameter_provided,
    create_app_query_params_query_parameter_with_special_characters_url_encoding,
    create_app_query_params_query_parameter_with_url_encoded_space,
    create_app_query_params_query_parameter_with_url_encoded_special_characters,
    create_app_query_params_required_integer_query_parameter_float_value,
    create_app_query_params_required_integer_query_parameter_invalid_type,
    create_app_query_params_required_integer_query_parameter_missing,
    create_app_query_params_required_integer_query_parameter_success,
    create_app_query_params_required_string_query_parameter_missing,
    create_app_query_params_required_string_query_parameter_success,
    create_app_query_params_string_query_param_with_max_length_constraint_fail,
    create_app_query_params_string_query_param_with_min_length_constraint_fail,
    create_app_query_params_string_query_param_with_regex_pattern_fail,
    create_app_query_params_string_validation_with_regex_failure,
    create_app_query_params_string_validation_with_regex_success,
    create_app_query_params_uuid_query_parameter_invalid_format,
    create_app_query_params_uuid_query_parameter_success,
)


async def test_string_validation_with_regex_success() -> None:
    """Tests string parameter with regex pattern validation - matching pattern."""

    async with TestClient(create_app_query_params_string_validation_with_regex_success()) as client:
        params = {
            "item_query": "fixedquery",
        }
        response = await client.get("/items/", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "item_query" in response_data
        assert response_data["item_query"] == "fixedquery"


async def test_49_integer_gt_constraint_success() -> None:
    """Integer query parameter greater than exclusive minimum should succeed."""

    async with TestClient(create_app_query_params_49_integer_gt_constraint_success()) as client:
        params = {
            "limit": "5",
        }
        response = await client.get("/items", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "limit" in response_data
        assert response_data["limit"] == 5


async def test_enum_query_parameter_invalid_value() -> None:
    """Tests enum query parameter with value not in enum."""

    async with TestClient(create_app_query_params_enum_query_parameter_invalid_value()) as client:
        params = {
            "model": "vgg16",
        }
        response = await client.get("/query/enum", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_68_array_uniqueitems_success() -> None:
    """Array query parameter with unique items should succeed."""

    async with TestClient(create_app_query_params_68_array_uniqueitems_success()) as client:
        params = {
            "ids": ["1", "2", "3", "4"],
        }
        response = await client.get("/items", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "ids" in response_data
        assert len(response_data["ids"]) == 4
        assert response_data["ids"][0] == 1
        assert response_data["ids"][1] == 2
        assert response_data["ids"][2] == 3
        assert response_data["ids"][3] == 4


async def test_47_pattern_validation_email_success() -> None:
    """String query parameter matching regex pattern should succeed."""

    async with TestClient(create_app_query_params_47_pattern_validation_email_success()) as client:
        params = {
            "email": "user@example.com",
        }
        response = await client.get("/subscribe", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "email" in response_data
        assert response_data["email"] == "user@example.com"


async def test_required_integer_query_parameter_success() -> None:
    """Tests a required integer query parameter with valid value."""

    async with TestClient(create_app_query_params_required_integer_query_parameter_success()) as client:
        params = {
            "query": 42,
        }
        response = await client.get("/query/int", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert response_data == "foo bar 42"


async def test_required_string_query_parameter_missing() -> None:
    """Tests a required string query parameter without providing value, should return 422."""

    async with TestClient(create_app_query_params_required_string_query_parameter_missing()) as client:
        response = await client.get("/query")

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_57_boolean_empty_string_coercion() -> None:
    """Boolean query parameter with empty string should coerce to false."""

    async with TestClient(create_app_query_params_57_boolean_empty_string_coercion()) as client:
        params = {
            "active": "",
        }
        response = await client.get("/items", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "active" in response_data
        assert response_data["active"] == False


async def test_52_integer_le_constraint_boundary() -> None:
    """Integer query parameter equal to maximum should succeed with le constraint."""

    async with TestClient(create_app_query_params_52_integer_le_constraint_boundary()) as client:
        params = {
            "limit": "100",
        }
        response = await client.get("/items", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "limit" in response_data
        assert response_data["limit"] == 100


async def test_list_with_default_empty_array_no_values_provided() -> None:
    """Tests list parameter with default=[] when no values provided, should return empty list."""

    async with TestClient(create_app_query_params_list_with_default_empty_array_no_values_provided()) as client:
        response = await client.get("/query/list-default")

        assert response.status_code == 200
        response_data = response.json()
        assert len(response_data) == 0


async def test_date_query_parameter_success() -> None:
    """Tests date query parameter with valid ISO date format."""

    async with TestClient(create_app_query_params_date_query_parameter_success()) as client:
        params = {
            "event_date": "2024-01-15",
        }
        response = await client.get("/query/date", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "event_date" in response_data
        assert response_data["event_date"] == "2024-01-15"


async def test_string_query_param_with_max_length_constraint_fail() -> None:
    """Tests string query parameter with max_length validation failure."""

    async with TestClient(create_app_query_params_string_query_param_with_max_length_constraint_fail()) as client:
        params = {
            "name": "this_is_way_too_long",
        }
        response = await client.get("/query/str-max-length", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_45_string_minlength_validation_failure() -> None:
    """String query parameter below minLength constraint should fail validation."""

    async with TestClient(create_app_query_params_45_string_minlength_validation_failure()) as client:
        params = {
            "term": "ab",
        }
        response = await client.get("/search", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_integer_with_default_value_override() -> None:
    """Tests integer parameter with default value when overridden with custom value."""

    async with TestClient(create_app_query_params_integer_with_default_value_override()) as client:
        params = {
            "query": 50,
        }
        response = await client.get("/query/int/default", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert response_data == "foo bar 50"


async def test_67_multipleof_constraint_failure() -> None:
    """Integer query parameter that is not multiple of constraint should fail."""

    async with TestClient(create_app_query_params_67_multipleof_constraint_failure()) as client:
        params = {
            "quantity": "17",
        }
        response = await client.get("/items", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_58_format_email_success() -> None:
    """Query parameter with valid email format should be accepted."""

    async with TestClient(create_app_query_params_58_format_email_success()) as client:
        params = {
            "email": "user@example.com",
        }
        response = await client.get("/subscribe", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "email" in response_data
        assert response_data["email"] == "user@example.com"


async def test_integer_query_param_with_ge_constraint_boundary() -> None:
    """Tests integer query parameter with ge validation at boundary value."""

    async with TestClient(create_app_query_params_integer_query_param_with_ge_constraint_boundary()) as client:
        params = {
            "value": "10",
        }
        response = await client.get("/query/int-ge", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "value" in response_data
        assert response_data["value"] == 10


async def test_integer_query_param_with_gt_constraint_valid() -> None:
    """Tests integer query parameter with gt validation, value above limit."""

    async with TestClient(create_app_query_params_integer_query_param_with_gt_constraint_valid()) as client:
        params = {
            "value": "1",
        }
        response = await client.get("/query/int-gt", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "value" in response_data
        assert response_data["value"] == 1


async def test_required_integer_query_parameter_invalid_type() -> None:
    """Tests integer query parameter with non-numeric string, should return 422."""

    async with TestClient(create_app_query_params_required_integer_query_parameter_invalid_type()) as client:
        params = {
            "query": "baz",
        }
        response = await client.get("/query/int", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_required_integer_query_parameter_float_value() -> None:
    """Tests integer query parameter with float string value, should return 422."""

    async with TestClient(create_app_query_params_required_integer_query_parameter_float_value()) as client:
        params = {
            "query": "42.5",
        }
        response = await client.get("/query/int", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_query_parameter_with_url_encoded_special_characters() -> None:
    """Tests query parameter with URL encoded special characters."""

    async with TestClient(create_app_query_params_query_parameter_with_url_encoded_special_characters()) as client:
        params = {
            "name": "test&value=123",
        }
        response = await client.get("/query/basic", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "test&value=123"


async def test_59_format_email_failure() -> None:
    """Query parameter with invalid email format should fail validation."""

    async with TestClient(create_app_query_params_59_format_email_failure()) as client:
        params = {
            "email": "not-an-email",
        }
        response = await client.get("/subscribe", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_43_scientific_notation_float() -> None:
    """Query parameter with scientific notation float should parse correctly."""

    async with TestClient(create_app_query_params_43_scientific_notation_float()) as client:
        params = {
            "threshold": "1.5e-3",
        }
        response = await client.get("/stats", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "threshold" in response_data
        assert response_data["threshold"] == 0.0015


async def test_63_format_uri_success() -> None:
    """Query parameter with valid URI should be accepted."""

    async with TestClient(create_app_query_params_63_format_uri_success()) as client:
        params = {
            "url": "https://example.com/path?query=value",
        }
        response = await client.get("/redirect", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "url" in response_data
        assert response_data["url"] == "https://example.com/path?query=value"


async def test_boolean_query_parameter_numeric_1() -> None:
    """Tests boolean query parameter with '1' converts to true."""

    async with TestClient(create_app_query_params_boolean_query_parameter_numeric_1()) as client:
        params = {
            "flag": "1",
        }
        response = await client.get("/query/bool", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "flag" in response_data
        assert response_data["flag"] == True


async def test_string_query_param_with_min_length_constraint_fail() -> None:
    """Tests string query parameter with min_length validation failure."""

    async with TestClient(create_app_query_params_string_query_param_with_min_length_constraint_fail()) as client:
        params = {
            "name": "ab",
        }
        response = await client.get("/query/str-min-length", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_optional_string_query_parameter_provided() -> None:
    """Tests optional string parameter with value provided."""

    async with TestClient(create_app_query_params_optional_string_query_parameter_provided()) as client:
        params = {
            "query": "baz",
        }
        response = await client.get("/query/optional", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert response_data == "foo bar baz"


async def test_list_of_integers_multiple_values() -> None:
    """Tests list query parameter with multiple integer values using same key."""

    async with TestClient(create_app_query_params_list_of_integers_multiple_values()) as client:
        params = {
            "device_ids": [1, 2],
        }
        response = await client.get("/query/list", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert len(response_data) == 2
        assert response_data[0] == 1
        assert response_data[1] == 2


async def test_integer_query_param_with_lt_constraint_valid() -> None:
    """Tests integer query parameter with lt validation, value below limit."""

    async with TestClient(create_app_query_params_integer_query_param_with_lt_constraint_valid()) as client:
        params = {
            "value": "49",
        }
        response = await client.get("/query/int-lt", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "value" in response_data
        assert response_data["value"] == 49


async def test_42_negative_integer_query_param() -> None:
    """Query parameter with negative integer value should be parsed correctly."""

    async with TestClient(create_app_query_params_42_negative_integer_query_param()) as client:
        params = {
            "offset": "-10",
        }
        response = await client.get("/items/negative", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "offset" in response_data
        assert response_data["offset"] == -10


async def test_46_string_maxlength_validation_failure() -> None:
    """String query parameter exceeding maxLength constraint should fail validation."""

    async with TestClient(create_app_query_params_46_string_maxlength_validation_failure()) as client:
        params = {
            "term": "this_is_way_too_long",
        }
        response = await client.get("/search", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_56_array_maxitems_constraint_failure() -> None:
    """Array query parameter exceeding maxItems constraint should fail validation."""

    async with TestClient(create_app_query_params_56_array_maxitems_constraint_failure()) as client:
        params = {
            "tags": ["a", "b", "c", "d", "e", "f"],
        }
        response = await client.get("/items", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_string_query_param_with_regex_pattern_fail() -> None:
    """Tests string query parameter with regex pattern validation failure."""

    async with TestClient(create_app_query_params_string_query_param_with_regex_pattern_fail()) as client:
        params = {
            "code": "abc123",
        }
        response = await client.get("/query/pattern", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_44_string_minlength_validation_success() -> None:
    """String query parameter meeting minLength constraint should succeed."""

    async with TestClient(create_app_query_params_44_string_minlength_validation_success()) as client:
        params = {
            "term": "foo",
        }
        response = await client.get("/search", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "term" in response_data
        assert response_data["term"] == "foo"


async def test_61_format_ipv4_failure() -> None:
    """Query parameter with invalid IPv4 address should fail validation."""

    async with TestClient(create_app_query_params_61_format_ipv4_failure()) as client:
        params = {
            "ip": "999.999.999.999",
        }
        response = await client.get("/network", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_48_pattern_validation_email_failure() -> None:
    """String query parameter not matching regex pattern should fail validation."""

    async with TestClient(create_app_query_params_48_pattern_validation_email_failure()) as client:
        params = {
            "email": "invalid-email",
        }
        response = await client.get("/subscribe", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_required_integer_query_parameter_missing() -> None:
    """Tests a required integer query parameter without providing value, should return 422."""

    async with TestClient(create_app_query_params_required_integer_query_parameter_missing()) as client:
        response = await client.get("/query/int")

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_query_parameter_with_special_characters_url_encoding() -> None:
    """Tests query parameters with special characters that need URL encoding."""

    async with TestClient(create_app_query_params_query_parameter_with_special_characters_url_encoding()) as client:
        params = {
            "email": "x@test.com",
            "special": "&@A.ac",
        }
        response = await client.get("/test", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "email" in response_data
        assert response_data["email"] == "x@test.com"
        assert "special" in response_data
        assert response_data["special"] == "&@A.ac"


async def test_list_query_parameter_required_but_missing() -> None:
    """Tests required list parameter without any values, should return 422."""

    async with TestClient(create_app_query_params_list_query_parameter_required_but_missing()) as client:
        response = await client.get("/query/list")

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_required_string_query_parameter_success() -> None:
    """Tests a required string query parameter with valid value."""

    async with TestClient(create_app_query_params_required_string_query_parameter_success()) as client:
        params = {
            "query": "baz",
        }
        response = await client.get("/query", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert response_data == "foo bar baz"


async def test_66_multipleof_constraint_success() -> None:
    """Integer query parameter that is multiple of constraint should succeed."""

    async with TestClient(create_app_query_params_66_multipleof_constraint_success()) as client:
        params = {
            "quantity": "15",
        }
        response = await client.get("/items", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "quantity" in response_data
        assert response_data["quantity"] == 15


async def test_53_integer_le_constraint_failure() -> None:
    """Integer query parameter exceeding maximum should fail validation."""

    async with TestClient(create_app_query_params_53_integer_le_constraint_failure()) as client:
        params = {
            "limit": "101",
        }
        response = await client.get("/items", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_multiple_query_parameters_with_different_types() -> None:
    """Tests multiple query parameters of different types in single request."""

    async with TestClient(create_app_query_params_multiple_query_parameters_with_different_types()) as client:
        params = {
            "active": "true",
            "score": "95.5",
            "name": "john",
            "age": "30",
        }
        response = await client.get("/query/multi-type", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "active" in response_data
        assert response_data["active"] == True
        assert "age" in response_data
        assert response_data["age"] == 30
        assert "name" in response_data
        assert response_data["name"] == "john"
        assert "score" in response_data
        assert response_data["score"] == 95.5


async def test_71_array_separator_semicolon() -> None:
    """Array query parameter with semicolon separator should parse correctly."""

    async with TestClient(create_app_query_params_71_array_separator_semicolon()) as client:
        params = {
            "colors": "red;green;blue",
        }
        response = await client.get("/items?colors=red;green;blue", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "colors" in response_data
        assert len(response_data["colors"]) == 3
        assert response_data["colors"][0] == "red"
        assert response_data["colors"][1] == "green"
        assert response_data["colors"][2] == "blue"


async def test_70_array_separator_pipe() -> None:
    """Array query parameter with pipe separator should parse correctly."""

    async with TestClient(create_app_query_params_70_array_separator_pipe()) as client:
        params = {
            "tags": "python|rust|typescript",
        }
        response = await client.get("/items?tags=python|rust|typescript", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "tags" in response_data
        assert len(response_data["tags"]) == 3
        assert response_data["tags"][0] == "python"
        assert response_data["tags"][1] == "rust"
        assert response_data["tags"][2] == "typescript"


async def test_integer_with_default_value_not_provided() -> None:
    """Tests integer parameter with default value when not provided."""

    async with TestClient(create_app_query_params_integer_with_default_value_not_provided()) as client:
        response = await client.get("/query/int/default")

        assert response.status_code == 200
        response_data = response.json()
        assert response_data == "foo bar 10"


async def test_boolean_query_parameter_true() -> None:
    """Tests boolean query parameter with 'true' string value."""

    async with TestClient(create_app_query_params_boolean_query_parameter_true()) as client:
        params = {
            "flag": "true",
        }
        response = await client.get("/query/bool", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "flag" in response_data
        assert response_data["flag"] == True


async def test_integer_query_param_with_le_constraint_boundary() -> None:
    """Tests integer query parameter with le validation at boundary value."""

    async with TestClient(create_app_query_params_integer_query_param_with_le_constraint_boundary()) as client:
        params = {
            "value": "100",
        }
        response = await client.get("/query/int-le", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "value" in response_data
        assert response_data["value"] == 100


async def test_float_query_param_with_ge_constraint_success() -> None:
    """Tests float query parameter with ge validation at boundary."""

    async with TestClient(create_app_query_params_float_query_param_with_ge_constraint_success()) as client:
        params = {
            "price": "0.01",
        }
        response = await client.get("/query/float-ge", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "price" in response_data
        assert response_data["price"] == 0.01


async def test_51_integer_ge_constraint_boundary() -> None:
    """Integer query parameter equal to minimum should succeed with ge constraint."""

    async with TestClient(create_app_query_params_51_integer_ge_constraint_boundary()) as client:
        params = {
            "offset": "0",
        }
        response = await client.get("/items", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "offset" in response_data
        assert response_data["offset"] == 0


async def test_optional_integer_query_parameter_missing() -> None:
    """Tests optional integer parameter without value, should not error."""

    async with TestClient(create_app_query_params_optional_integer_query_parameter_missing()) as client:
        response = await client.get("/query/int/optional")

        assert response.status_code == 200
        response_data = response.json()
        assert response_data == "foo bar None"


async def test_69_array_uniqueitems_failure() -> None:
    """Array query parameter with duplicate items should fail when uniqueItems is true."""

    async with TestClient(create_app_query_params_69_array_uniqueitems_failure()) as client:
        params = {
            "ids": ["1", "2", "2", "3"],
        }
        response = await client.get("/items", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_72_array_separator_space() -> None:
    """Array query parameter with space separator should parse correctly."""

    async with TestClient(create_app_query_params_72_array_separator_space()) as client:
        params = {
            "keywords": "rust web framework",
        }
        response = await client.get("/search?keywords=rust%20web%20framework", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "keywords" in response_data
        assert len(response_data["keywords"]) == 3
        assert response_data["keywords"][0] == "rust"
        assert response_data["keywords"][1] == "web"
        assert response_data["keywords"][2] == "framework"


async def test_string_validation_with_regex_failure() -> None:
    """Tests string parameter with regex pattern validation - non-matching pattern returns 422."""

    async with TestClient(create_app_query_params_string_validation_with_regex_failure()) as client:
        params = {
            "item_query": "nonregexquery",
        }
        response = await client.get("/items/", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_65_format_hostname_success() -> None:
    """Query parameter with valid hostname should be accepted."""

    async with TestClient(create_app_query_params_65_format_hostname_success()) as client:
        params = {
            "host": "api.example.com",
        }
        response = await client.get("/dns", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "host" in response_data
        assert response_data["host"] == "api.example.com"


async def test_query_parameter_with_url_encoded_space() -> None:
    """Tests query parameter with URL encoded space character."""

    async with TestClient(create_app_query_params_query_parameter_with_url_encoded_space()) as client:
        params = {
            "name": "hello world",
        }
        response = await client.get("/query/basic", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "name" in response_data
        assert response_data["name"] == "hello world"


async def test_list_of_strings_multiple_values() -> None:
    """Tests list of string query parameters with multiple values."""

    async with TestClient(create_app_query_params_list_of_strings_multiple_values()) as client:
        params = {
            "q": ["foo", "bar"],
        }
        response = await client.get("/items/", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "q" in response_data
        assert len(response_data["q"]) == 2
        assert response_data["q"][0] == "foo"
        assert response_data["q"][1] == "bar"


async def test_optional_query_parameter_with_default_value() -> None:
    """Tests optional query parameter that uses default when not provided."""

    async with TestClient(create_app_query_params_optional_query_parameter_with_default_value()) as client:
        response = await client.get("/query/optional-default")

        assert response.status_code == 200
        response_data = response.json()
        assert "limit" in response_data
        assert response_data["limit"] == 10


async def test_62_format_ipv6_success() -> None:
    """Query parameter with valid IPv6 address should be accepted."""

    async with TestClient(create_app_query_params_62_format_ipv6_success()) as client:
        params = {
            "ip": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
        }
        response = await client.get("/network/ipv6", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "ip" in response_data
        assert response_data["ip"] == "2001:0db8:85a3:0000:0000:8a2e:0370:7334"


async def test_array_query_parameter_single_value() -> None:
    """Tests array query parameter with single value."""

    async with TestClient(create_app_query_params_array_query_parameter_single_value()) as client:
        params = {
            "tags": "apple",
        }
        response = await client.get("/query/list-default", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert len(response_data) == 1
        assert response_data[0] == "apple"


async def test_optional_string_query_parameter_missing() -> None:
    """Tests optional string parameter without value, should return None/null."""

    async with TestClient(create_app_query_params_optional_string_query_parameter_missing()) as client:
        response = await client.get("/query/optional")

        assert response.status_code == 200
        response_data = response.json()
        assert response_data == "foo bar None"


async def test_datetime_query_parameter_success() -> None:
    """Tests datetime query parameter with valid ISO datetime format."""

    async with TestClient(create_app_query_params_datetime_query_parameter_success()) as client:
        params = {
            "timestamp": "2024-01-15T10:30:00Z",
        }
        response = await client.get("/query/datetime", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "timestamp" in response_data
        assert response_data["timestamp"] == "2024-01-15T10:30:00Z"


async def test_uuid_query_parameter_invalid_format() -> None:
    """Tests UUID query parameter with invalid UUID format."""

    async with TestClient(create_app_query_params_uuid_query_parameter_invalid_format()) as client:
        params = {
            "item_id": "not-a-uuid",
        }
        response = await client.get("/query/uuid", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_array_query_parameter_empty_array() -> None:
    """Tests array query parameter when no values are provided."""

    async with TestClient(create_app_query_params_array_query_parameter_empty_array()) as client:
        response = await client.get("/query/list-default")

        assert response.status_code == 200
        response_data = response.json()
        assert len(response_data) == 0


async def test_enum_query_parameter_success() -> None:
    """Tests enum query parameter with valid enum value."""

    async with TestClient(create_app_query_params_enum_query_parameter_success()) as client:
        params = {
            "model": "alexnet",
        }
        response = await client.get("/query/enum", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "model" in response_data
        assert response_data["model"] == "alexnet"


async def test_uuid_query_parameter_success() -> None:
    """Tests UUID query parameter with valid UUID format."""

    async with TestClient(create_app_query_params_uuid_query_parameter_success()) as client:
        params = {
            "item_id": "c892496f-b1fd-4b91-bdb8-b46f92df1716",
        }
        response = await client.get("/query/uuid", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "item_id" in response_data
        assert response_data["item_id"] == "c892496f-b1fd-4b91-bdb8-b46f92df1716"


async def test_50_integer_gt_constraint_failure() -> None:
    """Integer query parameter equal to exclusive minimum should fail validation."""

    async with TestClient(create_app_query_params_50_integer_gt_constraint_failure()) as client:
        params = {
            "limit": "0",
        }
        response = await client.get("/items", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_64_format_uri_failure() -> None:
    """Query parameter with invalid URI should fail validation."""

    async with TestClient(create_app_query_params_64_format_uri_failure()) as client:
        params = {
            "url": "not a uri",
        }
        response = await client.get("/redirect", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_54_array_minitems_constraint_success() -> None:
    """Array query parameter meeting minItems constraint should succeed."""

    async with TestClient(create_app_query_params_54_array_minitems_constraint_success()) as client:
        params = {
            "ids": ["1", "2", "3"],
        }
        response = await client.get("/items", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "ids" in response_data
        assert len(response_data["ids"]) == 3
        assert response_data["ids"][0] == 1
        assert response_data["ids"][1] == 2
        assert response_data["ids"][2] == 3


async def test_55_array_minitems_constraint_failure() -> None:
    """Array query parameter below minItems constraint should fail validation."""

    async with TestClient(create_app_query_params_55_array_minitems_constraint_failure()) as client:
        params = {
            "ids": ["1"],
        }
        response = await client.get("/items", params=params)

        assert response.status_code == 422
        response_data = response.json()
        # Validation should be done by framework, not handler
        assert "errors" in response_data or "detail" in response_data


async def test_60_format_ipv4_success() -> None:
    """Query parameter with valid IPv4 address should be accepted."""

    async with TestClient(create_app_query_params_60_format_ipv4_success()) as client:
        params = {
            "ip": "192.168.1.1",
        }
        response = await client.get("/network", params=params)

        assert response.status_code == 200
        response_data = response.json()
        assert "ip" in response_data
        assert response_data["ip"] == "192.168.1.1"
