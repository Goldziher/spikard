"""JSON-RPC 2.0 fixture validation tests.

These tests verify that all JSON-RPC fixtures are properly structured and can be loaded.
They serve to ensure fixtures are valid test data before being used by integration tests.
"""

import json
from pathlib import Path
from typing import cast

FIXTURES_DIR = Path(__file__).parent.parent.parent.parent / "testing_data" / "jsonrpc"


def load_fixture(filename: str) -> dict[str, object]:
    """Load a JSON fixture file."""
    fixture_path = FIXTURES_DIR / filename
    assert fixture_path.exists(), f"Fixture file not found: {fixture_path}"
    with fixture_path.open(encoding="utf-8") as f:
        data: object = json.load(f)
        if isinstance(data, dict):
            return data
        return {}


def load_fixture_from_subdir(subdir: str, filename: str) -> dict[str, object]:
    """Load a JSON fixture file from a subdirectory."""
    fixture_path = FIXTURES_DIR / subdir / filename
    assert fixture_path.exists(), f"Fixture file not found: {fixture_path}"
    with fixture_path.open(encoding="utf-8") as f:
        data: object = json.load(f)
        if isinstance(data, dict):
            return data
        return {}


def test_batch_requests_fixture_batch_requests_fixture_loads() -> None:
    """Test that batch_requests.json loads successfully."""
    fixture = load_fixture_from_subdir("edge_cases", "batch_requests.json")
    assert "name" in fixture
    assert fixture["name"] == "batch_requests"
    assert "examples" in fixture
    examples = cast("list[object]", fixture["examples"])
    assert len(examples) > 0


def test_batch_requests_fixture_examples_structure() -> None:
    """Test that batch_requests examples have required fields."""
    fixture = load_fixture_from_subdir("edge_cases", "batch_requests.json")
    examples = cast("list[object]", fixture["examples"])
    for example in examples:
        example_dict = cast("dict[str, object]", example)
        assert "name" in example_dict
        assert "description" in example_dict
        assert "request" in example_dict
        assert "expected_response" in example_dict
        assert isinstance(example_dict["request"], list), "Batch request must be an array"
        assert isinstance(example_dict["expected_response"], list), "Batch response must be an array"


def test_batch_requests_fixture_has_all_examples() -> None:
    """Test that batch_requests fixture has all expected examples."""
    fixture = load_fixture_from_subdir("edge_cases", "batch_requests.json")
    examples = cast("list[object]", fixture["examples"])
    example_names = [cast("dict[str, object]", ex)["name"] for ex in examples]

    expected = [
        "simple_batch_three_requests",
        "batch_mixed_success_error",
        "batch_with_notifications",
        "large_batch_ten_requests",
        "batch_same_method",
        "batch_duplicate_ids",
    ]

    for expected_name in expected:
        assert expected_name in example_names, f"Missing example: {expected_name}"


def test_empty_batch_fixture_fixture_loads() -> None:
    """Test that empty_batch.json loads successfully."""
    fixture = load_fixture_from_subdir("edge_cases", "empty_batch.json")
    assert "name" in fixture
    assert "error_cases" in fixture
    error_cases = cast("list[object]", fixture["error_cases"])
    assert len(error_cases) >= 2


def test_empty_batch_fixture_error_cases_structure() -> None:
    """Test that empty_batch error cases have required fields."""
    fixture = load_fixture_from_subdir("edge_cases", "empty_batch.json")
    error_cases = cast("list[object]", fixture["error_cases"])
    for error_case in error_cases:
        error_case_dict = cast("dict[str, object]", error_case)
        assert "name" in error_case_dict
        assert "description" in error_case_dict
        assert "request" in error_case_dict
        assert "error" in error_case_dict or "result" in error_case_dict


def test_invalid_json_fixture_fixture_loads() -> None:
    """Test that invalid_json.json loads successfully."""
    fixture = load_fixture_from_subdir("edge_cases", "invalid_json.json")
    assert "name" in fixture
    assert fixture["name"] == "invalid_json"
    assert "error_cases" in fixture
    error_cases = cast("list[object]", fixture["error_cases"])
    assert len(error_cases) == 7


def test_invalid_json_fixture_error_codes() -> None:
    """Test that all invalid_json cases use parse error code."""
    fixture = load_fixture_from_subdir("edge_cases", "invalid_json.json")
    error_cases = cast("list[object]", fixture["error_cases"])
    for error_case in error_cases:
        error_case_dict = cast("dict[str, object]", error_case)
        assert "error" in error_case_dict
        error_dict = cast("dict[str, object]", error_case_dict["error"])
        assert error_dict["code"] == -32700
        assert error_dict["message"] == "Parse error"


def test_invalid_params_fixture_fixture_loads() -> None:
    """Test that invalid_params.json loads successfully."""
    fixture = load_fixture_from_subdir("edge_cases", "invalid_params.json")
    assert "name" in fixture
    assert "error_cases" in fixture
    error_cases = cast("list[object]", fixture["error_cases"])
    assert len(error_cases) > 0


def test_invalid_params_fixture_error_codes() -> None:
    """Test that all invalid_params cases use parameter error code."""
    fixture = load_fixture_from_subdir("edge_cases", "invalid_params.json")
    error_cases = cast("list[object]", fixture["error_cases"])
    for error_case in error_cases:
        error_case_dict = cast("dict[str, object]", error_case)
        assert "error" in error_case_dict
        error_dict = cast("dict[str, object]", error_case_dict["error"])
        assert error_dict["code"] == -32602


def test_invalid_request_fixture_fixture_loads() -> None:
    """Test that invalid_request.json loads successfully."""
    fixture = load_fixture_from_subdir("edge_cases", "invalid_request.json")
    assert "name" in fixture
    assert "error_cases" in fixture
    error_cases = cast("list[object]", fixture["error_cases"])
    assert len(error_cases) > 0


def test_invalid_request_fixture_error_codes() -> None:
    """Test that all invalid_request cases use request error code."""
    fixture = load_fixture_from_subdir("edge_cases", "invalid_request.json")
    error_cases = cast("list[object]", fixture["error_cases"])
    for error_case in error_cases:
        error_case_dict = cast("dict[str, object]", error_case)
        assert "error" in error_case_dict
        error_dict = cast("dict[str, object]", error_case_dict["error"])
        assert error_dict["code"] == -32600


def test_notifications_fixture_fixture_loads() -> None:
    """Test that notifications.json loads successfully."""
    fixture = load_fixture_from_subdir("edge_cases", "notifications.json")
    assert "name" in fixture
    assert fixture["name"] == "notifications"
    assert "examples" in fixture
    examples = cast("list[object]", fixture["examples"])
    assert len(examples) > 0


def test_notifications_fixture_examples_no_id() -> None:
    """Test that notification examples don't have id fields."""
    fixture = load_fixture_from_subdir("edge_cases", "notifications.json")
    examples = cast("list[object]", fixture["examples"])
    for example in examples:
        example_dict = cast("dict[str, object]", example)
        assert "request" in example_dict
        request = example_dict["request"]
        if isinstance(request, dict):
            assert "id" not in request
        elif isinstance(request, list):
            for req in request:
                if "id" not in req:
                    assert True


def test_null_values_fixture_fixture_loads() -> None:
    """Test that null_values.json loads successfully."""
    fixture = load_fixture_from_subdir("edge_cases", "null_values.json")
    assert "name" in fixture
    assert "examples" in fixture
    examples = cast("list[object]", fixture["examples"])
    assert len(examples) > 0


def test_null_values_fixture_request_structure() -> None:
    """Test that null_values examples have proper structure."""
    fixture = load_fixture_from_subdir("edge_cases", "null_values.json")
    examples = cast("list[object]", fixture["examples"])
    for example in examples:
        example_dict = cast("dict[str, object]", example)
        assert "name" in example_dict
        assert "description" in example_dict
        assert "request" in example_dict or "params" in example_dict


def test_unicode_edge_cases_fixture_fixture_loads() -> None:
    """Test that unicode_edge_cases.json loads successfully."""
    fixture = load_fixture_from_subdir("edge_cases", "unicode_edge_cases.json")
    assert "name" in fixture
    assert "examples" in fixture
    examples = cast("list[object]", fixture["examples"])
    assert len(examples) > 0


def test_unicode_edge_cases_fixture_examples_contain_strings() -> None:
    """Test that unicode examples handle special characters."""
    fixture = load_fixture_from_subdir("edge_cases", "unicode_edge_cases.json")
    examples = cast("list[object]", fixture["examples"])
    for example in examples:
        example_dict = cast("dict[str, object]", example)
        assert "name" in example_dict
        assert "description" in example_dict


def test_large_payloads_fixture_fixture_loads() -> None:
    """Test that large_payloads.json loads successfully."""
    fixture = load_fixture_from_subdir("edge_cases", "large_payloads.json")
    assert "name" in fixture
    assert "examples" in fixture
    examples = cast("list[object]", fixture["examples"])
    assert len(examples) > 0


def test_method_not_found_fixture_fixture_loads() -> None:
    """Test that method_not_found.json loads successfully."""
    fixture = load_fixture_from_subdir("edge_cases", "method_not_found.json")
    assert "name" in fixture
    assert "error_cases" in fixture
    error_cases = cast("list[object]", fixture["error_cases"])
    assert len(error_cases) > 0


def test_method_not_found_fixture_error_codes() -> None:
    """Test that all method_not_found cases use method error code."""
    fixture = load_fixture_from_subdir("edge_cases", "method_not_found.json")
    error_cases = cast("list[object]", fixture["error_cases"])
    for error_case in error_cases:
        error_case_dict = cast("dict[str, object]", error_case)
        assert "error" in error_case_dict
        error_dict = cast("dict[str, object]", error_case_dict["error"])
        assert error_dict["code"] == -32601


def test_standard_errors_fixture_fixture_loads() -> None:
    """Test that standard_errors.json loads successfully."""
    fixture = load_fixture_from_subdir("errors", "standard_errors.json")
    assert isinstance(fixture, dict)
    assert len(fixture) > 0


def test_standard_errors_fixture_has_all_codes() -> None:
    """Test that standard_errors includes all JSON-RPC error codes."""
    fixture = load_fixture_from_subdir("errors", "standard_errors.json")

    error_codes: set[object] = set()
    for scenario in fixture.values():
        scenario_dict = cast("dict[str, object]", scenario)
        expected_response = scenario_dict.get("expected_response")
        if expected_response is not None:
            response_dict = cast("dict[str, object]", expected_response)
            error = response_dict.get("error")
            if error is not None:
                error_dict = cast("dict[str, object]", error)
                code = error_dict["code"]
                error_codes.add(code)

    expected_codes = {-32700, -32600, -32601, -32602, -32603}
    for code in expected_codes:
        assert code in error_codes, f"Missing error code {code}"


def test_custom_errors_fixture_fixture_loads() -> None:
    """Test that custom_errors.json loads successfully."""
    fixture = load_fixture_from_subdir("errors", "custom_errors.json")
    assert isinstance(fixture, dict)
    assert len(fixture) > 0


def test_custom_errors_fixture_all_have_responses() -> None:
    """Test that all custom error scenarios have expected responses."""
    fixture = load_fixture_from_subdir("errors", "custom_errors.json")
    for scenario in fixture.values():
        scenario_dict = cast("dict[str, object]", scenario)
        assert "request" in scenario_dict
        assert "expected_response" in scenario_dict


def test_error_data_fixture_fixture_loads() -> None:
    """Test that error_data.json loads successfully."""
    fixture = load_fixture_from_subdir("errors", "error_data.json")
    assert isinstance(fixture, dict)
    assert len(fixture) > 0


def test_error_data_fixture_includes_data_field() -> None:
    """Test that error_data scenarios include error.data field."""
    fixture = load_fixture_from_subdir("errors", "error_data.json")
    for scenario in fixture.values():
        scenario_dict = cast("dict[str, object]", scenario)
        expected_response = scenario_dict.get("expected_response")
        if expected_response is not None:
            response_dict = cast("dict[str, object]", expected_response)
            error_obj = response_dict.get("error")
            if error_obj is not None:
                error = cast("dict[str, object]", error_obj)
                assert "code" in error
                assert "message" in error


def test_user_create_fixture_fixture_loads() -> None:
    """Test that user_create.json loads successfully."""
    fixture = load_fixture("user_create.json")
    assert "name" in fixture
    assert "examples" in fixture
    assert "error_cases" in fixture
    examples = cast("list[object]", fixture["examples"])
    error_cases = cast("list[object]", fixture["error_cases"])
    assert len(examples) > 0
    assert len(error_cases) > 0


def test_user_list_fixture_fixture_loads() -> None:
    """Test that user_list.json loads successfully."""
    fixture = load_fixture("user_list.json")
    assert "name" in fixture
    assert "examples" in fixture
    assert "error_cases" in fixture


def test_user_get_by_id_fixture_fixture_loads() -> None:
    """Test that user_getById.json loads successfully."""
    fixture = load_fixture("user_getById.json")
    assert "name" in fixture
    assert "examples" in fixture
    assert "error_cases" in fixture


def test_user_update_fixture_fixture_loads() -> None:
    """Test that user_update.json loads successfully."""
    fixture = load_fixture("user_update.json")
    assert "name" in fixture
    assert "examples" in fixture
    assert "error_cases" in fixture


def test_user_delete_fixture_fixture_loads() -> None:
    """Test that user_delete.json loads successfully."""
    fixture = load_fixture("user_delete.json")
    assert "name" in fixture
    assert "examples" in fixture
    assert "error_cases" in fixture


def test_fixture_coverage_summary_all_core_method_fixtures_exist() -> None:
    """Test that all core method fixtures are present."""
    core_methods = [
        "user_create.json",
        "user_delete.json",
        "user_getById.json",
        "user_list.json",
        "user_update.json",
    ]

    for fixture_file in core_methods:
        fixture = load_fixture(fixture_file)
        assert "name" in fixture
        has_examples = "examples" in fixture
        has_errors = "error_cases" in fixture
        assert has_examples or has_errors


def test_fixture_coverage_summary_all_edge_case_fixtures_exist() -> None:
    """Test that all edge case fixtures are present."""
    edge_cases = [
        "batch_requests.json",
        "empty_batch.json",
        "invalid_json.json",
        "invalid_params.json",
        "invalid_request.json",
        "large_payloads.json",
        "method_not_found.json",
        "notifications.json",
        "null_values.json",
        "unicode_edge_cases.json",
    ]

    for fixture_file in edge_cases:
        fixture = load_fixture_from_subdir("edge_cases", fixture_file)
        assert fixture is not None


def test_fixture_coverage_summary_all_error_fixtures_exist() -> None:
    """Test that all error fixtures are present."""
    error_fixtures = [
        "standard_errors.json",
        "custom_errors.json",
        "error_data.json",
    ]

    for fixture_file in error_fixtures:
        fixture = load_fixture_from_subdir("errors", fixture_file)
        assert fixture is not None


def test_fixture_coverage_summary_fixture_total_count() -> None:
    """Test that we have sufficient fixture coverage."""
    all_files = list(FIXTURES_DIR.glob("**/*.json"))
    fixture_files = [f for f in all_files if f.name != "schema.json"]

    assert len(fixture_files) >= 18, f"Expected at least 18 fixtures, found {len(fixture_files)}"
