"""E2E tests for path_params."""

import pytest
from typing import Any

async def test_boolean_path_parameter_true() -> None:
    """Tests boolean path parameter with 'True' string value."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_boolean_path_parameter_true

    app = create_app_path_params_boolean_path_parameter_true()
    client = TestClient(app)

    response = await client.get("/path/bool/True")

    assert response.status_code == 200
    response_data = response.json()


async def test_29_decimal_path_param_success() -> None:
    """Path parameter with decimal/money value should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_29_decimal_path_param_success

    app = create_app_path_params_29_decimal_path_param_success()
    client = TestClient(app)

    response = await client.get("/prices/19.99")

    assert response.status_code == 200
    response_data = response.json()


async def test_integer_path_parameter_with_combined_lt_and_gt_constraints_success() -> None:
    """Tests integer path parameter with both lt and gt validation (range check)."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success

    app = create_app_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success()
    client = TestClient(app)

    response = await client.get("/path/param-lt-gt/2")

    assert response.status_code == 200
    response_data = response.json()


async def test_33_string_pattern_path_success() -> None:
    """Path parameter matching regex pattern should succeed."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_33_string_pattern_path_success

    app = create_app_path_params_33_string_pattern_path_success()
    client = TestClient(app)

    response = await client.get("/repos/spikard-labs/spikard-http")

    assert response.status_code == 200
    response_data = response.json()


async def test_31_string_minlength_path_failure() -> None:
    """Path parameter with string below minLength constraint should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_31_string_minlength_path_failure

    app = create_app_path_params_31_string_minlength_path_failure()
    client = TestClient(app)

    response = await client.get("/users/ab")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_35_negative_integer_path_param() -> None:
    """Path parameter with negative integer should be parsed correctly."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_35_negative_integer_path_param

    app = create_app_path_params_35_negative_integer_path_param()
    client = TestClient(app)

    response = await client.get("/offset/-100")

    assert response.status_code == 200
    response_data = response.json()


async def test_enum_path_parameter_invalid_value() -> None:
    """Tests enum path parameter with invalid enum value returns 422."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_enum_path_parameter_invalid_value

    app = create_app_path_params_enum_path_parameter_invalid_value()
    client = TestClient(app)

    response = await client.get("/models/foo")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_27_datetime_format_path_param_success() -> None:
    """Path parameter with valid ISO 8601 datetime should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_27_datetime_format_path_param_success

    app = create_app_path_params_27_datetime_format_path_param_success()
    client = TestClient(app)

    response = await client.get("/bookings/2025-10-30T14:30:00Z")

    assert response.status_code == 200
    response_data = response.json()


async def test_25_date_format_invalid_failure() -> None:
    """Path parameter with invalid date format should fail validation."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_25_date_format_invalid_failure

    app = create_app_path_params_25_date_format_invalid_failure()
    client = TestClient(app)

    response = await client.get("/events/2025-13-45")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_integer_path_parameter_with_lt_constraint_success() -> None:
    """Tests integer path parameter with lt (less than) validation."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_integer_path_parameter_with_lt_constraint_success

    app = create_app_path_params_integer_path_parameter_with_lt_constraint_success()
    client = TestClient(app)

    response = await client.get("/path/param-lt/2")

    assert response.status_code == 200
    response_data = response.json()


async def test_integer_path_parameter_with_gt_constraint_success() -> None:
    """Tests integer path parameter with gt (greater than) validation succeeds."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_integer_path_parameter_with_gt_constraint_success

    app = create_app_path_params_integer_path_parameter_with_gt_constraint_success()
    client = TestClient(app)

    response = await client.get("/path/param-gt/42")

    assert response.status_code == 200
    response_data = response.json()


async def test_28_duration_format_path_param_success() -> None:
    """Path parameter with valid ISO 8601 duration should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_28_duration_format_path_param_success

    app = create_app_path_params_28_duration_format_path_param_success()
    client = TestClient(app)

    response = await client.get("/delays/P1DT2H30M")

    assert response.status_code == 200
    response_data = response.json()


async def test_path_parameter_type_syntax_with_override() -> None:
    """Tests that explicit parameter schema overrides auto-generated type syntax schema."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_path_parameter_type_syntax_with_override

    app = create_app_path_params_path_parameter_type_syntax_with_override()
    client = TestClient(app)

    response = await client.get("/type-syntax/items-count/50")

    assert response.status_code == 200
    response_data = response.json()


async def test_20_uuid_v3_path_param_success() -> None:
    """Path parameter with valid UUID v3 should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_20_uuid_v3_path_param_success

    app = create_app_path_params_20_uuid_v3_path_param_success()
    client = TestClient(app)

    response = await client.get("/items/e8b5a51d-11c8-3310-a6ab-367563f20686")

    assert response.status_code == 200
    response_data = response.json()


async def test_integer_path_parameter_invalid_string() -> None:
    """Tests integer path parameter with non-numeric string returns 422."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_integer_path_parameter_invalid_string

    app = create_app_path_params_integer_path_parameter_invalid_string()
    client = TestClient(app)

    response = await client.get("/path/int/foobar")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_30_string_minlength_path_success() -> None:
    """Path parameter with string meeting minLength constraint should succeed."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_30_string_minlength_path_success

    app = create_app_path_params_30_string_minlength_path_success()
    client = TestClient(app)

    response = await client.get("/users/alice")

    assert response.status_code == 200
    response_data = response.json()


async def test_integer_path_parameter_with_le_constraint_success() -> None:
    """Tests integer path parameter with le (less than or equal) validation at boundary."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_integer_path_parameter_with_le_constraint_success

    app = create_app_path_params_integer_path_parameter_with_le_constraint_success()
    client = TestClient(app)

    response = await client.get("/path/param-le/3")

    assert response.status_code == 200
    response_data = response.json()


async def test_path_parameter_type_syntax_invalid_uuid() -> None:
    """Tests that :uuid type syntax properly validates and rejects invalid UUIDs."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_path_parameter_type_syntax_invalid_uuid

    app = create_app_path_params_path_parameter_type_syntax_invalid_uuid()
    client = TestClient(app)

    response = await client.get("/type-syntax/items/not-a-uuid")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_path_type_parameter_file_path() -> None:
    """Tests path type parameter that captures remaining path segments."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_path_type_parameter_file_path

    app = create_app_path_params_path_type_parameter_file_path()
    client = TestClient(app)

    response = await client.get("/files/home/johndoe/myfile.txt")

    assert response.status_code == 200
    response_data = response.json()


async def test_path_parameter_with_type_syntax_uuid() -> None:
    """Tests path parameter with :uuid type syntax auto-generates correct schema."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_path_parameter_with_type_syntax_uuid

    app = create_app_path_params_path_parameter_with_type_syntax_uuid()
    client = TestClient(app)

    response = await client.get("/type-syntax/items/550e8400-e29b-41d4-a716-446655440000")

    assert response.status_code == 200
    response_data = response.json()


async def test_32_string_maxlength_path_failure() -> None:
    """Path parameter with string exceeding maxLength constraint should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_32_string_maxlength_path_failure

    app = create_app_path_params_32_string_maxlength_path_failure()
    client = TestClient(app)

    response = await client.get("/users/this_username_is_way_too_long_to_be_valid")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_integer_path_parameter_success() -> None:
    """Tests integer path parameter with valid value."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_integer_path_parameter_success

    app = create_app_path_params_integer_path_parameter_success()
    client = TestClient(app)

    response = await client.get("/path/int/42")

    assert response.status_code == 200
    response_data = response.json()


async def test_34_string_pattern_path_failure() -> None:
    """Path parameter not matching regex pattern should fail."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_34_string_pattern_path_failure

    app = create_app_path_params_34_string_pattern_path_failure()
    client = TestClient(app)

    response = await client.get("/repos/invalid@owner")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_21_uuid_v5_path_param_success() -> None:
    """Path parameter with valid UUID v5 should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_21_uuid_v5_path_param_success

    app = create_app_path_params_21_uuid_v5_path_param_success()
    client = TestClient(app)

    response = await client.get("/items/630eb68f-e0fa-5ecc-887a-7c7a62614681")

    assert response.status_code == 200
    response_data = response.json()


async def test_string_path_parameter_with_max_length_failure() -> None:
    """Tests string path parameter with max_length validation fails when too long."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_string_path_parameter_with_max_length_failure

    app = create_app_path_params_string_path_parameter_with_max_length_failure()
    client = TestClient(app)

    response = await client.get("/path/param-maxlength/foobar")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_string_path_parameter_with_min_length_failure() -> None:
    """Tests string path parameter with min_length validation fails."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_string_path_parameter_with_min_length_failure

    app = create_app_path_params_string_path_parameter_with_min_length_failure()
    client = TestClient(app)

    response = await client.get("/path/param-minlength/fo")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_multiple_path_parameters_success() -> None:
    """Tests multiple path parameters in single route."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_multiple_path_parameters_success

    app = create_app_path_params_multiple_path_parameters_success()
    client = TestClient(app)

    response = await client.get("/1.0/1/abc/c892496f-b1fd-4b91-bdb8-b46f92df1716")

    assert response.status_code == 200
    response_data = response.json()


async def test_date_path_parameter_success() -> None:
    """Tests date path parameter with ISO format date."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_date_path_parameter_success

    app = create_app_path_params_date_path_parameter_success()
    client = TestClient(app)

    response = await client.get("/date/2023-07-15")

    assert response.status_code == 200
    response_data = response.json()


async def test_integer_path_parameter_with_gt_constraint_failure() -> None:
    """Tests integer path parameter with gt validation fails when value too small."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_integer_path_parameter_with_gt_constraint_failure

    app = create_app_path_params_integer_path_parameter_with_gt_constraint_failure()
    client = TestClient(app)

    response = await client.get("/path/param-gt/2")

    assert response.status_code == 422
    response_data = response.json()
    # Validation should be done by framework, not handler
    assert "errors" in response_data or "detail" in response_data


async def test_24_date_format_path_param_success() -> None:
    """Path parameter with valid ISO date format should be accepted."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_24_date_format_path_param_success

    app = create_app_path_params_24_date_format_path_param_success()
    client = TestClient(app)

    response = await client.get("/events/2025-10-30")

    assert response.status_code == 200
    response_data = response.json()


async def test_float_path_parameter_success() -> None:
    """Tests float path parameter with valid value."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_float_path_parameter_success

    app = create_app_path_params_float_path_parameter_success()
    client = TestClient(app)

    response = await client.get("/path/float/42.5")

    assert response.status_code == 200
    response_data = response.json()


async def test_path_parameter_with_type_syntax_integer() -> None:
    """Tests path parameter with :int type syntax auto-generates correct schema."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_path_parameter_with_type_syntax_integer

    app = create_app_path_params_path_parameter_with_type_syntax_integer()
    client = TestClient(app)

    response = await client.get("/type-syntax/users/42")

    assert response.status_code == 200
    response_data = response.json()


async def test_string_path_parameter_success() -> None:
    """Tests string path parameter with valid value."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_string_path_parameter_success

    app = create_app_path_params_string_path_parameter_success()
    client = TestClient(app)

    response = await client.get("/path/str/foobar")

    assert response.status_code == 200
    response_data = response.json()


async def test_uuid_path_parameter_success() -> None:
    """Tests UUID path parameter with valid UUID format."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_uuid_path_parameter_success

    app = create_app_path_params_uuid_path_parameter_success()
    client = TestClient(app)

    response = await client.get("/items/ec38df32-ceda-4cfa-9b4a-1aeb94ad551a")

    assert response.status_code == 200
    response_data = response.json()


async def test_integer_path_parameter_with_ge_constraint_success() -> None:
    """Tests integer path parameter with ge (greater than or equal) validation at boundary."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_integer_path_parameter_with_ge_constraint_success

    app = create_app_path_params_integer_path_parameter_with_ge_constraint_success()
    client = TestClient(app)

    response = await client.get("/path/param-ge/3")

    assert response.status_code == 200
    response_data = response.json()


async def test_enum_path_parameter_success() -> None:
    """Tests enum path parameter with valid enum value."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_enum_path_parameter_success

    app = create_app_path_params_enum_path_parameter_success()
    client = TestClient(app)

    response = await client.get("/models/alexnet")

    assert response.status_code == 200
    response_data = response.json()


async def test_boolean_path_parameter_numeric_1() -> None:
    """Tests boolean path parameter with '1' converts to true."""
    from spikard.testing import TestClient
    from app.main import create_app_path_params_boolean_path_parameter_numeric_1

    app = create_app_path_params_boolean_path_parameter_numeric_1()
    client = TestClient(app)

    response = await client.get("/path/bool/1")

    assert response.status_code == 200
    response_data = response.json()


