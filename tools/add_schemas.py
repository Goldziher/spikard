#!/usr/bin/env python3

"""Add explicit schemas to all fixture files."""

import json
import re
import sys
from collections.abc import Iterable, Mapping, Sequence
from pathlib import Path
from typing import Any

Fixture = dict[str, Any]
ErrorList = Sequence[dict[str, Any]]
SectionSchema = dict[str, dict[str, Any]]
ParametersSchema = dict[str, SectionSchema]


def infer_type_from_value(value: Any) -> dict[str, Any]:
    """Infer JSON Schema type from a value."""
    schema: dict[str, Any]
    if value is None:
        schema = {"type": ["string", "null"]}
    elif isinstance(value, bool):
        schema = {"type": "boolean"}
    elif isinstance(value, int):
        schema = {"type": "integer"}
    elif isinstance(value, float):
        schema = {"type": "number"}
    elif isinstance(value, str):
        schema = {"type": "string"}
        if len(value) == 36 and value.count("-") == 4:
            schema["format"] = "uuid"
        elif "T" in value and value.endswith("Z"):
            schema["format"] = "date-time"
        elif len(value) == 10 and value.count("-") == 2:
            schema["format"] = "date"
    elif isinstance(value, list):
        schema = {"type": "array"}
        if value:
            schema["items"] = infer_type_from_value(value[0])
    elif isinstance(value, dict):
        properties: dict[str, Any] = {}
        for key, item in value.items():
            properties[key] = infer_type_from_value(item)
        schema = {"type": "object", "properties": properties}
    else:
        schema = {"type": "string"}
    return schema


def extract_required_fields_from_errors(errors: ErrorList) -> set[str]:
    """Extract required field names from validation errors."""
    required = set()
    for error in errors:
        if error.get("type") == "missing" and len(error.get("loc", [])) >= 2:
            field_name = error["loc"][1]
            required.add(field_name)
    return required


def extract_constraints_from_errors(errors: ErrorList) -> dict[str, dict[str, Any]]:
    """Extract validation constraints from error details."""
    constraints: dict[str, dict[str, Any]] = {}
    for error in errors:
        if len(error.get("loc", [])) >= 2:
            field_name = error["loc"][1]
            error_type = error.get("type", "")
            ctx = error.get("ctx", {})

            if field_name not in constraints:
                constraints[field_name] = {}

            if error_type == "string_too_short" and "min_length" in ctx:
                constraints[field_name]["minLength"] = ctx["min_length"]
            elif error_type == "string_too_long" and "max_length" in ctx:
                constraints[field_name]["maxLength"] = ctx["max_length"]
            elif error_type == "greater_than" and "gt" in ctx:
                constraints[field_name]["exclusiveMinimum"] = ctx["gt"]
            elif error_type == "less_than" and "lt" in ctx:
                constraints[field_name]["exclusiveMaximum"] = ctx["lt"]
            elif error_type == "string_pattern_mismatch" and "pattern" in ctx:
                constraints[field_name]["pattern"] = ctx["pattern"]

    return constraints


def add_body_schema(fixture: Fixture) -> bool:
    """Add body_schema to fixture if missing. Returns True if modified."""
    if not fixture.get("handler"):
        return False

    if "body_schema" in fixture["handler"]:
        return False

    request_body = fixture.get("request", {}).get("body")
    if not request_body:
        return False

    properties: dict[str, dict[str, Any]] = {}
    required_fields: set[str] = set()

    if isinstance(request_body, dict):
        for key, value in request_body.items():
            properties[key] = infer_type_from_value(value)

        if 200 <= fixture.get("expected_response", {}).get("status_code", 0) < 300:
            required_fields = set(request_body.keys())

    expected_response = fixture.get("expected_response", {})
    body = expected_response.get("body")
    errors: list[dict[str, Any]] = []
    if isinstance(body, dict):
        errors = body.get("detail", [])
    if errors and isinstance(errors, list):
        error_required = extract_required_fields_from_errors(errors)
        constraints = extract_constraints_from_errors(errors)

        for field in error_required:
            required_fields.add(field)
            if field not in properties:
                properties[field] = {"type": "string"}

        for field, field_constraints in constraints.items():
            if field in properties:
                properties[field].update(field_constraints)

    if not properties:
        return False

    schema: dict[str, Any] = {"type": "object", "properties": properties}

    if required_fields:
        schema["required"] = sorted(required_fields)

    fixture["handler"]["body_schema"] = schema
    return True


def _build_query_schema(query_params: Mapping[str, Any]) -> SectionSchema:
    schema: SectionSchema = {}
    for key, value in query_params.items():
        field_schema = infer_type_from_value(value)
        field_schema["optional"] = True
        schema[key] = field_schema
    return schema


def _build_path_schema(path_param_names: Iterable[str]) -> SectionSchema:
    schema: SectionSchema = {}
    for param in path_param_names:
        schema[param] = {"type": "string"}
    return schema


def _build_header_schema(headers: Mapping[str, Any]) -> SectionSchema:
    schema: SectionSchema = {}
    for key, value in headers.items():
        if key.lower() in {"content-type", "accept", "user-agent"}:
            continue
        field_schema = infer_type_from_value(value)
        field_schema["optional"] = True
        schema[key] = field_schema
    return schema


def _build_cookie_schema(cookies: Mapping[str, Any]) -> SectionSchema:
    schema: SectionSchema = {}
    for key, value in cookies.items():
        field_schema = infer_type_from_value(value)
        field_schema["optional"] = True
        schema[key] = field_schema
    return schema


def _apply_error_metadata(parameters: ParametersSchema, errors: ErrorList) -> None:
    if not errors:
        return

    source_map = {
        "query": "query",
        "path": "path",
        "header": "headers",
        "cookie": "cookies",
    }

    for error in errors:
        if error.get("type") != "missing":
            continue
        loc = error.get("loc", [])
        if len(loc) < 2:
            continue
        source = loc[0]
        field_name = loc[1]

        section_key = source_map.get(source)
        if section_key is None:
            continue
        section = parameters.get(section_key)
        if section is None:
            continue
        field_schema = section.get(field_name)
        if field_schema is None:
            continue
        if source != "path":
            field_schema["optional"] = False

    constraints = extract_constraints_from_errors(errors)
    for field, field_constraints in constraints.items():
        for section in parameters.values():
            if field in section:
                section[field].update(field_constraints)


def add_parameter_schema(fixture: Fixture) -> bool:
    """Add parameters schema to fixture if missing. Returns True if modified."""
    if not fixture.get("handler"):
        return False

    if "parameters" in fixture["handler"]:
        return False

    request = fixture.get("request", {})
    query_params = request.get("query_params", {})
    headers = request.get("headers", {})
    cookies = request.get("cookies", {})

    route = fixture["handler"].get("route", "")
    path_param_names = set(re.findall(r"\{(\w+)(?::\w+)?\}", route))

    parameters: ParametersSchema = {}

    if isinstance(query_params, Mapping):
        query_schema = _build_query_schema(query_params)
        if query_schema:
            parameters["query"] = query_schema

    if path_param_names:
        path_schema = _build_path_schema(path_param_names)
        if path_schema:
            parameters["path"] = path_schema

    if isinstance(headers, Mapping):
        header_schema = _build_header_schema(headers)
        if header_schema:
            parameters["headers"] = header_schema

    if isinstance(cookies, Mapping):
        cookie_schema = _build_cookie_schema(cookies)
        if cookie_schema:
            parameters["cookies"] = cookie_schema

    expected_response = fixture.get("expected_response", {})
    body = expected_response.get("body")
    errors: list[dict[str, Any]] = []
    if isinstance(body, dict):
        detail = body.get("detail")
        if isinstance(detail, list):
            errors = detail
    _apply_error_metadata(parameters, errors)

    if not parameters:
        return False

    fixture["handler"]["parameters"] = parameters
    return True


def process_fixture_file(file_path: Path) -> bool:
    """Process a single fixture file. Returns True if modified."""
    try:
        with file_path.open() as handle:
            fixture: Fixture = json.load(handle)

        modified = False
        modified |= add_body_schema(fixture)
        modified |= add_parameter_schema(fixture)

        if modified:
            with file_path.open("w") as handle:
                json.dump(fixture, handle, indent=2)
                handle.write("\n")
            return True

        return False
    except (OSError, json.JSONDecodeError) as exc:
        sys.stderr.write(f"Error processing {file_path}: {exc}\n")
        return False


def main() -> None:
    """Entry point for adding schemas across fixtures."""
    testing_data = Path("testing_data")
    if not testing_data.exists():
        sys.stderr.write("Error: testing_data directory not found\n")
        sys.exit(1)

    modified_count = 0
    total_count = 0

    for category_dir in testing_data.iterdir():
        if not category_dir.is_dir():
            continue

        for fixture_file in category_dir.glob("*.json"):
            if "SCHEMA" in fixture_file.name:
                continue

            total_count += 1
            if process_fixture_file(fixture_file):
                modified_count += 1
                sys.stdout.write(f"✓ {fixture_file.relative_to(testing_data)}\n")

    sys.stdout.write(f"\nModified {modified_count}/{total_count} fixtures\n")


if __name__ == "__main__":
    main()
