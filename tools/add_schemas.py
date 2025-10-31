#!/usr/bin/env python3
"""Add explicit schemas to all fixture files."""

import json
import sys
from pathlib import Path
from typing import Any


def infer_type_from_value(value: Any) -> dict[str, Any]:
    """Infer JSON Schema type from a value."""
    if value is None:
        return {"type": ["string", "null"]}
    elif isinstance(value, bool):
        return {"type": "boolean"}
    elif isinstance(value, int):
        return {"type": "integer"}
    elif isinstance(value, float):
        return {"type": "number"}
    elif isinstance(value, str):
        # Check for special formats
        if len(value) == 36 and value.count('-') == 4:
            return {"type": "string", "format": "uuid"}
        elif 'T' in value and 'Z' in value:
            return {"type": "string", "format": "date-time"}
        elif len(value) == 10 and value.count('-') == 2:
            return {"type": "string", "format": "date"}
        return {"type": "string"}
    elif isinstance(value, list):
        if value:
            item_schema = infer_type_from_value(value[0])
            return {"type": "array", "items": item_schema}
        return {"type": "array"}
    elif isinstance(value, dict):
        properties = {}
        for k, v in value.items():
            properties[k] = infer_type_from_value(v)
        return {"type": "object", "properties": properties}
    return {"type": "string"}


def extract_required_fields_from_errors(errors: list[dict]) -> set[str]:
    """Extract required field names from validation errors."""
    required = set()
    for error in errors:
        if error.get("type") == "missing" and len(error.get("loc", [])) >= 2:
            field_name = error["loc"][1]
            required.add(field_name)
    return required


def extract_constraints_from_errors(errors: list[dict]) -> dict[str, dict]:
    """Extract validation constraints from error details."""
    constraints = {}
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


def add_body_schema(fixture: dict) -> bool:
    """Add body_schema to fixture if missing. Returns True if modified."""
    if not fixture.get("handler"):
        return False

    # Skip if already has body_schema
    if "body_schema" in fixture["handler"]:
        return False

    request_body = fixture.get("request", {}).get("body")
    if not request_body:
        return False

    # Infer schema from request body
    properties = {}
    required_fields = set()

    if isinstance(request_body, dict):
        for key, value in request_body.items():
            properties[key] = infer_type_from_value(value)

        # All fields in successful requests are assumed required
        if 200 <= fixture.get("expected_response", {}).get("status_code", 0) < 300:
            required_fields = set(request_body.keys())

    # Check validation errors for required fields and constraints
    expected_response = fixture.get("expected_response", {})
    body = expected_response.get("body")
    errors = []
    if isinstance(body, dict):
        errors = body.get("detail", [])
    if errors and isinstance(errors, list):
        error_required = extract_required_fields_from_errors(errors)
        constraints = extract_constraints_from_errors(errors)

        # For validation errors, infer missing required fields
        for field in error_required:
            required_fields.add(field)
            if field not in properties:
                properties[field] = {"type": "string"}  # Default type

        # Apply constraints
        for field, field_constraints in constraints.items():
            if field in properties:
                properties[field].update(field_constraints)

    if not properties:
        return False

    schema = {
        "type": "object",
        "properties": properties
    }

    if required_fields:
        schema["required"] = sorted(required_fields)

    fixture["handler"]["body_schema"] = schema
    return True


def add_parameter_schema(fixture: dict) -> bool:
    """Add parameters schema to fixture if missing. Returns True if modified."""
    if not fixture.get("handler"):
        return False

    # Skip if already has parameters
    if "parameters" in fixture["handler"]:
        return False

    request = fixture.get("request", {})
    query_params = request.get("query_params", {})
    path = request.get("path", "")
    headers = request.get("headers", {})
    cookies = request.get("cookies", {})

    # Extract path parameters from route
    route = fixture["handler"].get("route", "")
    import re
    path_param_names = set(re.findall(r'\{(\w+)(?::\w+)?\}', route))

    parameters = {}

    # Add query parameters
    if query_params:
        query_schema = {}
        for key, value in query_params.items():
            query_schema[key] = infer_type_from_value(value)
            query_schema[key]["optional"] = True  # Default to optional

        if query_schema:
            parameters["query"] = query_schema

    # Add path parameters
    if path_param_names:
        path_schema = {}
        # Try to extract values from path
        for param in path_param_names:
            # Path params are always required
            path_schema[param] = {"type": "string"}  # Default to string

        if path_schema:
            parameters["path"] = path_schema

    # Add headers (exclude standard ones)
    if headers:
        header_schema = {}
        for key, value in headers.items():
            if key.lower() not in ["content-type", "accept", "user-agent"]:
                header_schema[key] = infer_type_from_value(value)
                header_schema[key]["optional"] = True

        if header_schema:
            parameters["headers"] = header_schema

    # Add cookies
    if cookies:
        cookie_schema = {}
        for key, value in cookies.items():
            cookie_schema[key] = infer_type_from_value(value)
            cookie_schema[key]["optional"] = True

        if cookie_schema:
            parameters["cookies"] = cookie_schema

    # Check validation errors for required parameters
    expected_response = fixture.get("expected_response", {})
    body = expected_response.get("body")
    errors = []
    if isinstance(body, dict):
        errors = body.get("detail", [])
    if errors and isinstance(errors, list):
        for error in errors:
            if error.get("type") == "missing" and len(error.get("loc", [])) >= 2:
                source = error["loc"][0]  # query, path, header, cookie
                field_name = error["loc"][1]

                if source == "query" and "query" in parameters:
                    if field_name in parameters["query"]:
                        parameters["query"][field_name]["optional"] = False
                elif source == "path" and "path" in parameters:
                    if field_name in parameters["path"]:
                        pass  # Path params are always required
                elif source == "header" and "headers" in parameters:
                    if field_name in parameters["headers"]:
                        parameters["headers"][field_name]["optional"] = False
                elif source == "cookie" and "cookies" in parameters:
                    if field_name in parameters["cookies"]:
                        parameters["cookies"][field_name]["optional"] = False

        # Apply constraints
        constraints = extract_constraints_from_errors(errors)
        for field, field_constraints in constraints.items():
            for section in parameters.values():
                if field in section:
                    section[field].update(field_constraints)

    if not parameters:
        return False

    fixture["handler"]["parameters"] = parameters
    return True


def process_fixture_file(file_path: Path) -> bool:
    """Process a single fixture file. Returns True if modified."""
    try:
        with open(file_path) as f:
            fixture = json.load(f)

        modified = False
        modified |= add_body_schema(fixture)
        modified |= add_parameter_schema(fixture)

        if modified:
            with open(file_path, 'w') as f:
                json.dump(fixture, f, indent=2)
                f.write('\n')
            return True

        return False
    except Exception as e:
        print(f"Error processing {file_path}: {e}", file=sys.stderr)
        return False


def main():
    testing_data = Path("testing_data")
    if not testing_data.exists():
        print("Error: testing_data directory not found", file=sys.stderr)
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
                print(f"✓ {fixture_file.relative_to(testing_data)}")

    print(f"\nModified {modified_count}/{total_count} fixtures")


if __name__ == "__main__":
    main()
