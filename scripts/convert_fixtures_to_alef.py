#!/usr/bin/env python3
"""Convert spikard testing_data/ fixtures to alef HTTP fixture format.

Usage:
    python scripts/convert_fixtures_to_alef.py
    python scripts/convert_fixtures_to_alef.py --dry-run
    python scripts/convert_fixtures_to_alef.py --validate
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path


def to_snake_case(name: str) -> str:
    """Convert a human-readable name to a unique snake_case identifier."""
    s = name.lower()
    s = re.sub(r"[^a-z0-9]+", "_", s)
    s = re.sub(r"_+", "_", s)
    return s.strip("_")


def convert_fixture(fixture: dict, category: str, seen_ids: set[str]) -> dict | None:
    """Convert a single spikard fixture to alef HTTP fixture format."""
    name = fixture.get("name", "")
    if not name:
        return None

    # Generate unique snake_case ID
    base_id = to_snake_case(name)
    fixture_id = base_id
    counter = 2
    while fixture_id in seen_ids:
        fixture_id = f"{base_id}_{counter}"
        counter += 1
    seen_ids.add(fixture_id)

    description = fixture.get("description", name)

    # Build the HTTP fixture
    handler = fixture.get("handler", {})
    request = fixture.get("request", {})
    expected = fixture.get("expected_response", {})

    # Skip fixtures without handler or request (e.g., protobuf-only, streaming)
    if not handler.get("route") and not handler.get("service"):
        return None
    if not request.get("method") and not request.get("message"):
        return None

    # Skip gRPC/protobuf fixtures (different test paradigm)
    if fixture.get("protobuf") or handler.get("service"):
        return None
    if expected.get("protobuf"):
        return None

    # Build handler
    alef_handler = {
        "route": handler.get("route", "/"),
        "method": handler.get("method", "GET"),
    }
    if handler.get("body_schema"):
        alef_handler["body_schema"] = handler["body_schema"]
    if handler.get("parameters"):
        alef_handler["parameters"] = handler["parameters"]
    if handler.get("middleware"):
        alef_handler["middleware"] = handler["middleware"]

    # Build request
    alef_request = {
        "method": request.get("method", handler.get("method", "GET")),
        "path": request.get("path", handler.get("route", "/")),
    }
    if request.get("headers"):
        alef_request["headers"] = request["headers"]
    if request.get("query_params"):
        alef_request["query_params"] = request["query_params"]
    if request.get("cookies"):
        alef_request["cookies"] = request["cookies"]
    if request.get("body") is not None:
        alef_request["body"] = request["body"]
    if request.get("content_type"):
        alef_request["content_type"] = request["content_type"]

    # Build expected response
    alef_expected: dict = {}
    status = expected.get("status_code")
    if isinstance(status, int):
        alef_expected["status_code"] = status
    elif isinstance(status, str):
        # gRPC status string — skip
        return None
    else:
        alef_expected["status_code"] = 200

    if expected.get("body") is not None:
        alef_expected["body"] = expected["body"]
    if expected.get("body_partial") is not None:
        alef_expected["body_partial"] = expected["body_partial"]
    if expected.get("headers"):
        alef_expected["headers"] = expected["headers"]
    if expected.get("validation_errors"):
        alef_expected["validation_errors"] = expected["validation_errors"]

    # Build alef fixture
    result: dict = {
        "id": fixture_id,
        "description": description,
        "category": category,
        "http": {
            "handler": alef_handler,
            "request": alef_request,
            "expected_response": alef_expected,
        },
    }

    tags = fixture.get("tags", [])
    if tags:
        result["tags"] = tags

    skip = fixture.get("skip", False)
    skip_reason = fixture.get("skip_reason", "")
    if skip:
        result["skip"] = {"reason": skip_reason or "Skipped"}

    return result


def process_category(category_dir: Path, category: str, seen_ids: set[str]) -> list[dict]:
    """Process all fixtures in a category directory."""
    fixtures = []
    for json_file in sorted(category_dir.glob("**/*.json")):
        if json_file.name.startswith("00-"):
            continue
        try:
            data = json.loads(json_file.read_text())
        except json.JSONDecodeError as e:
            print(f"  SKIP {json_file}: invalid JSON: {e}", file=sys.stderr)
            continue

        # Handle both single fixture and array of fixtures
        items = data if isinstance(data, list) else [data]
        for item in items:
            converted = convert_fixture(item, category, seen_ids)
            if converted:
                fixtures.append(converted)

    return fixtures


def main() -> None:
    parser = argparse.ArgumentParser(description="Convert spikard fixtures to alef format")
    parser.add_argument("--input", default="testing_data", help="Input directory")
    parser.add_argument("--output", default="fixtures", help="Output directory")
    parser.add_argument("--dry-run", action="store_true", help="Print what would be written")
    parser.add_argument("--validate", action="store_true", help="Validate output format")
    args = parser.parse_args()

    input_dir = Path(args.input)
    output_dir = Path(args.output)

    if not input_dir.exists():
        print(f"Error: {input_dir} does not exist", file=sys.stderr)
        sys.exit(1)

    seen_ids: set[str] = set()
    total_converted = 0
    total_skipped = 0
    category_stats: dict[str, int] = {}

    for category_path in sorted(input_dir.iterdir()):
        if not category_path.is_dir():
            continue
        category = category_path.name
        if category.startswith(("00-", ".")):
            continue

        fixtures = process_category(category_path, category, seen_ids)
        category_stats[category] = len(fixtures)
        total_converted += len(fixtures)

        if not fixtures:
            continue

        out_file = output_dir / f"{category}.json"

        if args.dry_run:
            print(f"  {out_file}: {len(fixtures)} fixtures")
            continue

        output_dir.mkdir(parents=True, exist_ok=True)
        out_file.write_text(json.dumps(fixtures, indent=2, ensure_ascii=False) + "\n")

    # Count skipped
    total_input = 0
    for json_file in input_dir.glob("**/*.json"):
        if json_file.name.startswith("00-"):
            continue
        try:
            data = json.loads(json_file.read_text())
            total_input += len(data) if isinstance(data, list) else 1
        except json.JSONDecodeError:
            pass
    total_skipped = total_input - total_converted

    print("\nConversion summary:")
    print(f"  Input:     {total_input} fixtures from {input_dir}/")
    print(f"  Converted: {total_converted} HTTP fixtures")
    print(f"  Skipped:   {total_skipped} (gRPC/protobuf/streaming/incomplete)")
    print(f"  Output:    {output_dir}/")
    print("\nCategories:")
    for cat, count in sorted(category_stats.items()):
        if count > 0:
            print(f"  {cat}: {count}")

    if args.validate:
        errors = 0
        for json_file in sorted(output_dir.glob("*.json")):
            data = json.loads(json_file.read_text())
            for f in data:
                if "id" not in f:
                    print(f"  ERROR: {json_file}: missing 'id'", file=sys.stderr)
                    errors += 1
                if "http" not in f:
                    print(f"  ERROR: {json_file}/{f.get('id', '?')}: missing 'http'", file=sys.stderr)
                    errors += 1
                http = f.get("http", {})
                if "handler" not in http:
                    print(f"  ERROR: {json_file}/{f['id']}: missing 'http.handler'", file=sys.stderr)
                    errors += 1
                if "request" not in http:
                    print(f"  ERROR: {json_file}/{f['id']}: missing 'http.request'", file=sys.stderr)
                    errors += 1
                if "expected_response" not in http:
                    print(f"  ERROR: {json_file}/{f['id']}: missing 'http.expected_response'", file=sys.stderr)
                    errors += 1
        if errors:
            print(f"\n{errors} validation errors found", file=sys.stderr)
            sys.exit(1)
        else:
            print("\nValidation passed: all fixtures have required fields")


if __name__ == "__main__":
    main()
