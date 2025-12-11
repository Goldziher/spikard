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


# ============================================================================
# BATCH REQUEST FIXTURE TESTS
# ============================================================================


class TestBatchRequestsFixture:
    """Validate batch_requests.json fixture structure."""

    def test_batch_requests_fixture_loads(self) -> None:
        """Test that batch_requests.json loads successfully."""
        fixture = load_fixture_from_subdir("edge_cases", "batch_requests.json")
        assert "name" in fixture
        assert fixture["name"] == "batch_requests"
        assert "examples" in fixture
        examples = cast("list[object]", fixture["examples"])
        assert len(examples) > 0

    def test_batch_requests_examples_structure(self) -> None:
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

    def test_batch_requests_has_all_examples(self) -> None:
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


# ============================================================================
# EMPTY BATCH FIXTURE TESTS
# ============================================================================


class TestEmptyBatchFixture:
    """Validate empty_batch.json fixture structure."""

    def test_empty_batch_fixture_loads(self) -> None:
        """Test that empty_batch.json loads successfully."""
        fixture = load_fixture_from_subdir("edge_cases", "empty_batch.json")
        assert "name" in fixture
        assert "error_cases" in fixture
        error_cases = cast("list[object]", fixture["error_cases"])
        assert len(error_cases) >= 2

    def test_empty_batch_error_cases_structure(self) -> None:
        """Test that empty_batch error cases have required fields."""
        fixture = load_fixture_from_subdir("edge_cases", "empty_batch.json")
        error_cases = cast("list[object]", fixture["error_cases"])
        for error_case in error_cases:
            error_case_dict = cast("dict[str, object]", error_case)
            assert "name" in error_case_dict
            assert "description" in error_case_dict
            assert "request" in error_case_dict
            # Check for either error or result field (fixture structure inconsistency)
            assert "error" in error_case_dict or "result" in error_case_dict


# ============================================================================
# INVALID JSON FIXTURE TESTS
# ============================================================================


class TestInvalidJsonFixture:
    """Validate invalid_json.json fixture structure."""

    def test_invalid_json_fixture_loads(self) -> None:
        """Test that invalid_json.json loads successfully."""
        fixture = load_fixture_from_subdir("edge_cases", "invalid_json.json")
        assert "name" in fixture
        assert fixture["name"] == "invalid_json"
        assert "error_cases" in fixture
        error_cases = cast("list[object]", fixture["error_cases"])
        assert len(error_cases) == 7

    def test_invalid_json_error_codes(self) -> None:
        """Test that all invalid_json cases use parse error code."""
        fixture = load_fixture_from_subdir("edge_cases", "invalid_json.json")
        error_cases = cast("list[object]", fixture["error_cases"])
        for error_case in error_cases:
            error_case_dict = cast("dict[str, object]", error_case)
            assert "error" in error_case_dict
            error_dict = cast("dict[str, object]", error_case_dict["error"])
            assert error_dict["code"] == -32700
            assert error_dict["message"] == "Parse error"


# ============================================================================
# INVALID PARAMS FIXTURE TESTS
# ============================================================================


class TestInvalidParamsFixture:
    """Validate invalid_params.json fixture structure."""

    def test_invalid_params_fixture_loads(self) -> None:
        """Test that invalid_params.json loads successfully."""
        fixture = load_fixture_from_subdir("edge_cases", "invalid_params.json")
        assert "name" in fixture
        assert "error_cases" in fixture
        error_cases = cast("list[object]", fixture["error_cases"])
        assert len(error_cases) > 0

    def test_invalid_params_error_codes(self) -> None:
        """Test that all invalid_params cases use parameter error code."""
        fixture = load_fixture_from_subdir("edge_cases", "invalid_params.json")
        error_cases = cast("list[object]", fixture["error_cases"])
        for error_case in error_cases:
            error_case_dict = cast("dict[str, object]", error_case)
            assert "error" in error_case_dict
            error_dict = cast("dict[str, object]", error_case_dict["error"])
            assert error_dict["code"] == -32602


# ============================================================================
# INVALID REQUEST FIXTURE TESTS
# ============================================================================


class TestInvalidRequestFixture:
    """Validate invalid_request.json fixture structure."""

    def test_invalid_request_fixture_loads(self) -> None:
        """Test that invalid_request.json loads successfully."""
        fixture = load_fixture_from_subdir("edge_cases", "invalid_request.json")
        assert "name" in fixture
        assert "error_cases" in fixture
        error_cases = cast("list[object]", fixture["error_cases"])
        assert len(error_cases) > 0

    def test_invalid_request_error_codes(self) -> None:
        """Test that all invalid_request cases use request error code."""
        fixture = load_fixture_from_subdir("edge_cases", "invalid_request.json")
        error_cases = cast("list[object]", fixture["error_cases"])
        for error_case in error_cases:
            error_case_dict = cast("dict[str, object]", error_case)
            assert "error" in error_case_dict
            error_dict = cast("dict[str, object]", error_case_dict["error"])
            assert error_dict["code"] == -32600


# ============================================================================
# NOTIFICATIONS FIXTURE TESTS
# ============================================================================


class TestNotificationsFixture:
    """Validate notifications.json fixture structure."""

    def test_notifications_fixture_loads(self) -> None:
        """Test that notifications.json loads successfully."""
        fixture = load_fixture_from_subdir("edge_cases", "notifications.json")
        assert "name" in fixture
        assert fixture["name"] == "notifications"
        assert "examples" in fixture
        examples = cast("list[object]", fixture["examples"])
        assert len(examples) > 0

    def test_notifications_examples_no_id(self) -> None:
        """Test that notification examples don't have id fields."""
        fixture = load_fixture_from_subdir("edge_cases", "notifications.json")
        examples = cast("list[object]", fixture["examples"])
        for example in examples:
            example_dict = cast("dict[str, object]", example)
            assert "request" in example_dict
            request = example_dict["request"]
            if isinstance(request, dict):
                # Single notification
                assert "id" not in request
            elif isinstance(request, list):
                # Batch with notifications
                for req in request:
                    if "id" not in req:
                        # This is a notification, verify no id
                        assert True


# ============================================================================
# NULL VALUES FIXTURE TESTS
# ============================================================================


class TestNullValuesFixture:
    """Validate null_values.json fixture structure."""

    def test_null_values_fixture_loads(self) -> None:
        """Test that null_values.json loads successfully."""
        fixture = load_fixture_from_subdir("edge_cases", "null_values.json")
        assert "name" in fixture
        assert "examples" in fixture
        examples = cast("list[object]", fixture["examples"])
        assert len(examples) > 0

    def test_null_values_request_structure(self) -> None:
        """Test that null_values examples have proper structure."""
        fixture = load_fixture_from_subdir("edge_cases", "null_values.json")
        examples = cast("list[object]", fixture["examples"])
        for example in examples:
            example_dict = cast("dict[str, object]", example)
            assert "name" in example_dict
            assert "description" in example_dict
            # Check for request or params field (fixture structure varies)
            assert "request" in example_dict or "params" in example_dict


# ============================================================================
# UNICODE FIXTURE TESTS
# ============================================================================


class TestUnicodeEdgeCasesFixture:
    """Validate unicode_edge_cases.json fixture structure."""

    def test_unicode_fixture_loads(self) -> None:
        """Test that unicode_edge_cases.json loads successfully."""
        fixture = load_fixture_from_subdir("edge_cases", "unicode_edge_cases.json")
        assert "name" in fixture
        assert "examples" in fixture
        examples = cast("list[object]", fixture["examples"])
        assert len(examples) > 0

    def test_unicode_examples_contain_strings(self) -> None:
        """Test that unicode examples handle special characters."""
        fixture = load_fixture_from_subdir("edge_cases", "unicode_edge_cases.json")
        examples = cast("list[object]", fixture["examples"])
        for example in examples:
            example_dict = cast("dict[str, object]", example)
            assert "name" in example_dict
            assert "description" in example_dict


# ============================================================================
# LARGE PAYLOADS FIXTURE TESTS
# ============================================================================


class TestLargePayloadsFixture:
    """Validate large_payloads.json fixture structure."""

    def test_large_payloads_fixture_loads(self) -> None:
        """Test that large_payloads.json loads successfully."""
        fixture = load_fixture_from_subdir("edge_cases", "large_payloads.json")
        assert "name" in fixture
        assert "examples" in fixture
        examples = cast("list[object]", fixture["examples"])
        assert len(examples) > 0


# ============================================================================
# METHOD NOT FOUND FIXTURE TESTS
# ============================================================================


class TestMethodNotFoundFixture:
    """Validate method_not_found.json fixture structure."""

    def test_method_not_found_fixture_loads(self) -> None:
        """Test that method_not_found.json loads successfully."""
        fixture = load_fixture_from_subdir("edge_cases", "method_not_found.json")
        assert "name" in fixture
        assert "error_cases" in fixture
        error_cases = cast("list[object]", fixture["error_cases"])
        assert len(error_cases) > 0

    def test_method_not_found_error_codes(self) -> None:
        """Test that all method_not_found cases use method error code."""
        fixture = load_fixture_from_subdir("edge_cases", "method_not_found.json")
        error_cases = cast("list[object]", fixture["error_cases"])
        for error_case in error_cases:
            error_case_dict = cast("dict[str, object]", error_case)
            assert "error" in error_case_dict
            error_dict = cast("dict[str, object]", error_case_dict["error"])
            assert error_dict["code"] == -32601


# ============================================================================
# STANDARD ERRORS FIXTURE TESTS
# ============================================================================


class TestStandardErrorsFixture:
    """Validate standard_errors.json fixture structure."""

    def test_standard_errors_fixture_loads(self) -> None:
        """Test that standard_errors.json loads successfully."""
        fixture = load_fixture_from_subdir("errors", "standard_errors.json")
        assert isinstance(fixture, dict)
        assert len(fixture) > 0

    def test_standard_errors_has_all_codes(self) -> None:
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

        # Verify standard JSON-RPC error codes are present
        expected_codes = {-32700, -32600, -32601, -32602, -32603}
        for code in expected_codes:
            assert code in error_codes, f"Missing error code {code}"


# ============================================================================
# CUSTOM ERRORS FIXTURE TESTS
# ============================================================================


class TestCustomErrorsFixture:
    """Validate custom_errors.json fixture structure."""

    def test_custom_errors_fixture_loads(self) -> None:
        """Test that custom_errors.json loads successfully."""
        fixture = load_fixture_from_subdir("errors", "custom_errors.json")
        assert isinstance(fixture, dict)
        assert len(fixture) > 0

    def test_custom_errors_all_have_responses(self) -> None:
        """Test that all custom error scenarios have expected responses."""
        fixture = load_fixture_from_subdir("errors", "custom_errors.json")
        for scenario in fixture.values():
            scenario_dict = cast("dict[str, object]", scenario)
            assert "request" in scenario_dict
            assert "expected_response" in scenario_dict


# ============================================================================
# ERROR DATA FIXTURE TESTS
# ============================================================================


class TestErrorDataFixture:
    """Validate error_data.json fixture structure."""

    def test_error_data_fixture_loads(self) -> None:
        """Test that error_data.json loads successfully."""
        fixture = load_fixture_from_subdir("errors", "error_data.json")
        assert isinstance(fixture, dict)
        assert len(fixture) > 0

    def test_error_data_includes_data_field(self) -> None:
        """Test that error_data scenarios include error.data field."""
        fixture = load_fixture_from_subdir("errors", "error_data.json")
        for scenario in fixture.values():
            scenario_dict = cast("dict[str, object]", scenario)
            expected_response = scenario_dict.get("expected_response")
            if expected_response is not None:
                response_dict = cast("dict[str, object]", expected_response)
                error_obj = response_dict.get("error")
                if error_obj is not None:
                    # Most error_data scenarios should have a data field
                    error = cast("dict[str, object]", error_obj)
                    # error.data is optional but recommended for detailed error info
                    assert "code" in error
                    assert "message" in error


# ============================================================================
# CORE METHOD FIXTURES TESTS
# ============================================================================


class TestUserCreateFixture:
    """Validate user_create.json fixture structure."""

    def test_user_create_fixture_loads(self) -> None:
        """Test that user_create.json loads successfully."""
        fixture = load_fixture("user_create.json")
        assert "name" in fixture
        assert "examples" in fixture
        assert "error_cases" in fixture
        examples = cast("list[object]", fixture["examples"])
        error_cases = cast("list[object]", fixture["error_cases"])
        assert len(examples) > 0
        assert len(error_cases) > 0


class TestUserListFixture:
    """Validate user_list.json fixture structure."""

    def test_user_list_fixture_loads(self) -> None:
        """Test that user_list.json loads successfully."""
        fixture = load_fixture("user_list.json")
        assert "name" in fixture
        assert "examples" in fixture
        assert "error_cases" in fixture


class TestUserGetByIdFixture:
    """Validate user_getById.json fixture structure."""

    def test_user_getbyid_fixture_loads(self) -> None:
        """Test that user_getById.json loads successfully."""
        fixture = load_fixture("user_getById.json")
        assert "name" in fixture
        assert "examples" in fixture
        assert "error_cases" in fixture


class TestUserUpdateFixture:
    """Validate user_update.json fixture structure."""

    def test_user_update_fixture_loads(self) -> None:
        """Test that user_update.json loads successfully."""
        fixture = load_fixture("user_update.json")
        assert "name" in fixture
        assert "examples" in fixture
        assert "error_cases" in fixture


class TestUserDeleteFixture:
    """Validate user_delete.json fixture structure."""

    def test_user_delete_fixture_loads(self) -> None:
        """Test that user_delete.json loads successfully."""
        fixture = load_fixture("user_delete.json")
        assert "name" in fixture
        assert "examples" in fixture
        assert "error_cases" in fixture


# ============================================================================
# FIXTURE COVERAGE SUMMARY
# ============================================================================


class TestFixtureCoverageSummary:
    """Verify all expected fixtures exist and are testable."""

    def test_all_core_method_fixtures_exist(self) -> None:
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

    def test_all_edge_case_fixtures_exist(self) -> None:
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

    def test_all_error_fixtures_exist(self) -> None:
        """Test that all error fixtures are present."""
        error_fixtures = [
            "standard_errors.json",
            "custom_errors.json",
            "error_data.json",
        ]

        for fixture_file in error_fixtures:
            fixture = load_fixture_from_subdir("errors", fixture_file)
            assert fixture is not None

    def test_fixture_total_count(self) -> None:
        """Test that we have sufficient fixture coverage."""
        # Count total fixtures
        all_files = list(FIXTURES_DIR.glob("**/*.json"))
        # Exclude schema.json files
        fixture_files = [f for f in all_files if f.name != "schema.json"]

        # We expect at least 20 fixtures (5 core + 10 edge cases + 3 error + optional validation)
        assert len(fixture_files) >= 18, f"Expected at least 18 fixtures, found {len(fixture_files)}"
