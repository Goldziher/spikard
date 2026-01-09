#!/usr/bin/env python3
"""
Validate gRPC streaming fixture files against JSON schema.

This script validates all fixture files in testing_data/protobuf/streaming/
against the schema defined in testing_data/protobuf/streaming/schema.json.
"""

import json
import sys
from pathlib import Path
from typing import Dict, List, Tuple


def load_schema(schema_path: Path) -> Dict:
    """Load the JSON schema from file."""
    with open(schema_path, encoding='utf-8') as f:
        return json.load(f)


def validate_semantic(fixture_data: Dict, fixture_path: Path) -> List[str]:
    """
    Validate semantic correctness beyond JSON Schema capabilities.

    Checks:
    - handler.service matches a service in protobuf.services
    - handler.method exists in the specified service
    - input_type and output_type reference defined messages
    - Protobuf field numbers are unique within each message

    Returns list of semantic errors (empty if valid).
    """
    errors = []

    try:
        protobuf = fixture_data.get("protobuf", {})
        handler = fixture_data.get("handler", {})
        services = protobuf.get("services", [])
        messages = protobuf.get("messages", [])

        # Build message name set for validation
        message_names = {msg["name"] for msg in messages if isinstance(msg, dict) and "name" in msg}

        # Build service name -> service dict
        services_by_name = {
            svc["name"]: svc
            for svc in services
            if isinstance(svc, dict) and "name" in svc
        }

        # Validate handler.service exists in protobuf.services
        handler_service = handler.get("service", "")
        if handler_service:
            # Extract service name (handler.service is fully qualified like "example.v1.Service")
            service_short_name = handler_service.split(".")[-1] if "." in handler_service else handler_service

            # Check if service exists (match on short name or full name)
            matching_services = [
                name for name in services_by_name
                if name == service_short_name or name == handler_service
            ]

            if not matching_services:
                errors.append(
                    f"{fixture_path.name}: handler.service '{handler_service}' not found in protobuf.services"
                )
            else:
                # Validate handler.method exists in the service
                handler_method = handler.get("method", "")
                if handler_method:
                    service = services_by_name[matching_services[0]]
                    methods = service.get("methods", [])
                    method_names = {
                        method["name"]
                        for method in methods
                        if isinstance(method, dict) and "name" in method
                    }

                    if handler_method not in method_names:
                        errors.append(
                            f"{fixture_path.name}: handler.method '{handler_method}' not found in service '{service['name']}'"
                        )

                    # Validate input_type and output_type reference defined messages
                    for method in methods:
                        if not isinstance(method, dict):
                            continue

                        input_type = method.get("input_type", "")
                        output_type = method.get("output_type", "")

                        if input_type and input_type not in message_names:
                            errors.append(
                                f"{fixture_path.name}: method '{method.get('name')}' input_type '{input_type}' not found in protobuf.messages"
                            )

                        if output_type and output_type not in message_names:
                            errors.append(
                                f"{fixture_path.name}: method '{method.get('name')}' output_type '{output_type}' not found in protobuf.messages"
                            )

        # Validate protobuf field numbers are unique within each message
        for message in messages:
            if not isinstance(message, dict):
                continue

            message_name = message.get("name", "unknown")
            fields = message.get("fields", [])
            field_numbers = []

            for field in fields:
                if isinstance(field, dict) and "number" in field:
                    field_number = field["number"]
                    if field_number in field_numbers:
                        errors.append(
                            f"{fixture_path.name}: message '{message_name}' has duplicate field number {field_number}"
                        )
                    field_numbers.append(field_number)

    except Exception as e:
        errors.append(f"{fixture_path.name}: Semantic validation error: {e}")

    return errors


def validate_fixture(fixture_data: Dict, schema: Dict, fixture_path: Path) -> List[str]:
    """
    Validate a single fixture against the schema and semantic rules.

    Returns a list of validation error messages (empty if valid).
    """
    errors = []

    try:
        # Import jsonschema lazily to provide better error message if not installed
        import jsonschema
        from jsonschema import Draft7Validator

        # JSON Schema validation
        validator = Draft7Validator(schema)
        validation_errors = list(validator.iter_errors(fixture_data))

        for error in validation_errors:
            # Format path to the error location
            path = ".".join(str(p) for p in error.absolute_path) if error.absolute_path else "root"
            errors.append(f"{fixture_path.name}: [{path}] {error.message}")

        # Semantic validation (only if schema validation passes)
        if not errors:
            semantic_errors = validate_semantic(fixture_data, fixture_path)
            errors.extend(semantic_errors)

    except ImportError:
        print("Error: jsonschema library not found. Install with: pip install jsonschema")
        sys.exit(1)
    except Exception as e:
        errors.append(f"{fixture_path.name}: Unexpected validation error: {e}")

    return errors


def validate_all_fixtures() -> Tuple[int, int]:
    """
    Validate all fixtures in the streaming directory.

    Returns tuple of (valid_count, error_count).
    """
    # Locate schema file
    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    schema_path = project_root / "testing_data" / "protobuf" / "streaming" / "schema.json"
    fixtures_dir = project_root / "testing_data" / "protobuf" / "streaming"

    if not schema_path.exists():
        print(f"Error: Schema file not found at {schema_path}")
        return 0, 1

    if not fixtures_dir.exists():
        print(f"Error: Fixtures directory not found at {fixtures_dir}")
        return 0, 1

    # Load schema
    try:
        schema = load_schema(schema_path)
    except Exception as e:
        print(f"Error loading schema: {e}")
        return 0, 1

    # Find all fixture files
    fixture_files = []
    for category in ["server", "client", "bidirectional", "errors"]:
        category_dir = fixtures_dir / category
        if category_dir.exists():
            fixture_files.extend(sorted(category_dir.glob("*.json")))

    if not fixture_files:
        print("Warning: No fixture files found")
        return 0, 0

    # Validate each fixture
    all_errors = []
    valid_count = 0

    for fixture_path in fixture_files:
        try:
            with open(fixture_path, encoding='utf-8') as f:
                fixture_data = json.load(f)

            errors = validate_fixture(fixture_data, schema, fixture_path)

            if errors:
                all_errors.extend(errors)
            else:
                valid_count += 1

        except json.JSONDecodeError as e:
            all_errors.append(f"{fixture_path.name}: Invalid JSON - {e}")
        except Exception as e:
            all_errors.append(f"{fixture_path.name}: Failed to read - {e}")

    # Print results
    total_count = len(fixture_files)
    error_count = total_count - valid_count

    if all_errors:
        print(f"❌ Fixture validation errors ({error_count}/{total_count} failed):\n")
        for error in all_errors:
            print(f"  - {error}")
        print()

    if valid_count > 0:
        print(f"✓ {valid_count}/{total_count} fixtures valid")

    return valid_count, error_count


def main() -> int:
    """Main entry point."""
    valid_count, error_count = validate_all_fixtures()

    if error_count > 0:
        return 1

    if valid_count == 0:
        print("Warning: No fixtures validated")
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
