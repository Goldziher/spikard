"""Benchmark different approaches for validation error input field handling.

Compares three approaches:
1. Pre-validation: Validate raw strings before type conversion
2. Dual-track: Store both raw and parsed values (current implementation)
3. Error reconstruction: Parse once, re-parse on error
"""

import time
from collections.abc import Callable, Sequence
from typing import Any, Literal, TypedDict
from urllib.parse import parse_qs


def parse_query_string_raw(query: str) -> dict[str, str]:
    """Parse query string keeping only first value as string."""
    parsed = parse_qs(query, keep_blank_values=True)
    return {k: v[0] for k, v in parsed.items()}


def parse_query_string_typed(query: str) -> dict[str, Any]:
    """Parse query string with type conversion."""
    parsed = parse_qs(query, keep_blank_values=True)
    result: dict[str, Any] = {}
    for k, v_list in parsed.items():
        v = v_list[0]
        if v.isdigit():
            result[k] = int(v)
        elif v.replace(".", "", 1).isdigit():
            result[k] = float(v)
        elif v.lower() in ("true", "false"):
            result[k] = v.lower() == "true"
        else:
            result[k] = v
    return result


class FieldRule(TypedDict, total=False):
    """Schema rule describing type and optional numeric constraints."""

    type: Literal["int", "float", "str"]
    gt: float
    lt: float
    required: bool


class ValidationErrorDetail(TypedDict, total=False):
    """Validation error entry captured during benchmarking."""

    field: str
    error: str
    input: Any
    gt: float
    lt: float


SchemaRules = dict[str, FieldRule]
ValidationErrors = list[ValidationErrorDetail]


def validate_typed(params: dict[str, Any], schema: SchemaRules) -> tuple[bool, ValidationErrors]:
    """Simple validation that checks type and constraints."""
    errors: ValidationErrors = []
    for field, rules in schema.items():
        value = params.get(field)

        if rules.get("required") and value is None:
            errors.append({"field": field, "error": "required", "input": None})
            continue

        if value is None:
            continue

        expected_type = rules.get("type")
        if (expected_type == "int" and not isinstance(value, int)) or (
            expected_type == "float" and not isinstance(value, (int, float))
        ):
            errors.append({"field": field, "error": "type", "input": value})

        if "gt" in rules and isinstance(value, (int, float)) and value <= rules["gt"]:
            errors.append({"field": field, "error": "gt", "input": value, "gt": rules["gt"]})
        if "lt" in rules and isinstance(value, (int, float)) and value >= rules["lt"]:
            errors.append({"field": field, "error": "lt", "input": value, "lt": rules["lt"]})

    return len(errors) == 0, errors


SCHEMA: SchemaRules = {
    "price": {"type": "float", "gt": 0, "required": False},
    "quantity": {"type": "int", "gt": 0, "required": True},
    "name": {"type": "str", "required": False},
}

VALID_QUERY: str = "price=10.5&quantity=5&name=test"

INVALID_QUERIES: list[str] = [
    "price=0&quantity=5&name=test",
    "price=10.5&name=test",
    "price=abc&quantity=5&name=test",
]


ApproachResult = tuple[bool, dict[str, Any] | list[ValidationErrorDetail]]
ApproachFunction = Callable[[str], ApproachResult]


def approach_1_pre_validation(query: str) -> ApproachResult:
    """Approach 1: Validate raw strings before type conversion."""
    raw_params = parse_query_string_raw(query)

    typed_params: dict[str, Any] = {}
    errors: ValidationErrors = []

    for field, rules in SCHEMA.items():
        raw_value = raw_params.get(field)

        if rules.get("required") and raw_value is None:
            errors.append({"field": field, "error": "required", "input": None})
            continue

        if raw_value is None:
            continue

        expected_type = rules.get("type")
        try:
            value: Any
            if expected_type == "int":
                value = int(raw_value)
            elif expected_type == "float":
                value = float(raw_value)
            else:
                value = raw_value

            if "gt" in rules and isinstance(value, (int, float)) and value <= rules["gt"]:
                errors.append({"field": field, "error": "gt", "input": raw_value, "gt": rules["gt"]})
                continue

            typed_params[field] = value

        except ValueError:
            errors.append({"field": field, "error": "type", "input": raw_value})

    if errors:
        return False, errors
    return True, typed_params


def approach_2_dual_track(query: str) -> ApproachResult:
    """Approach 2: Store both raw and parsed values (current implementation)."""
    raw_params = parse_query_string_raw(query)
    typed_params = parse_query_string_typed(query)

    valid, errors = validate_typed(typed_params, SCHEMA)

    if not valid:
        for error in errors:
            field = error["field"]
            if field in raw_params:
                error["input"] = raw_params[field]
        return False, errors

    return True, typed_params


def approach_3_error_reconstruction(query: str) -> ApproachResult:
    """Approach 3: Parse once, re-parse on error."""
    typed_params = parse_query_string_typed(query)

    valid, errors = validate_typed(typed_params, SCHEMA)

    if not valid:
        raw_params = parse_query_string_raw(query)
        for error in errors:
            field = error["field"]
            if field in raw_params:
                error["input"] = raw_params[field]
        return False, errors

    return True, typed_params


class BenchmarkResult(TypedDict):
    """Summary metrics collected for a benchmarked approach."""

    name: str
    total_time: float
    avg_time_ms: float
    requests_per_sec: float


def benchmark_approach(
    name: str,
    approach_func: ApproachFunction,
    queries: Sequence[str],
    iterations: int = 10000,
) -> BenchmarkResult:
    """Benchmark an approach with given queries."""
    start = time.perf_counter()

    for _ in range(iterations):
        for query in queries:
            approach_func(query)

    end = time.perf_counter()
    total_time = end - start
    avg_time_ms = (total_time / (iterations * len(queries))) * 1000
    requests_per_sec = (iterations * len(queries)) / total_time

    return {
        "name": name,
        "total_time": total_time,
        "avg_time_ms": avg_time_ms,
        "requests_per_sec": requests_per_sec,
    }


def main() -> None:
    """Run benchmarks for all approaches."""
    iterations = 100000


    results_happy: list[BenchmarkResult] = []
    for approach_name, approach_func in [
        ("Approach 1: Pre-validation", approach_1_pre_validation),
        ("Approach 2: Dual-track (current)", approach_2_dual_track),
        ("Approach 3: Error reconstruction", approach_3_error_reconstruction),
    ]:
        result = benchmark_approach(approach_name, approach_func, [VALID_QUERY], iterations)
        results_happy.append(result)


    results_error: list[BenchmarkResult] = []
    for approach_name, approach_func in [
        ("Approach 1: Pre-validation", approach_1_pre_validation),
        ("Approach 2: Dual-track (current)", approach_2_dual_track),
        ("Approach 3: Error reconstruction", approach_3_error_reconstruction),
    ]:
        result = benchmark_approach(approach_name, approach_func, INVALID_QUERIES, iterations)
        results_error.append(result)

    fastest_happy = min(results_happy, key=lambda x: x["avg_time_ms"])
    fastest_error = min(results_error, key=lambda x: x["avg_time_ms"])

    _ = [
        ((result["avg_time_ms"] / fastest_happy["avg_time_ms"]) - 1) * 100
        for result in sorted(results_happy, key=lambda x: x["avg_time_ms"])
    ]
    _ = [
        ((result["avg_time_ms"] / fastest_error["avg_time_ms"]) - 1) * 100
        for result in sorted(results_error, key=lambda x: x["avg_time_ms"])
    ]

    weighted_scores: list[tuple[str, float]] = []
    for i, result_happy in enumerate(results_happy):
        result_error = results_error[i]
        weighted_avg = (0.95 * result_happy["avg_time_ms"]) + (0.05 * result_error["avg_time_ms"])
        weighted_scores.append((result_happy["name"], weighted_avg))

    weighted_scores.sort(key=lambda x: x[1])


if __name__ == "__main__":
    main()
