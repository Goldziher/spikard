#!/usr/bin/env python3
"""Validate all test fixtures against their JSON schemas."""

import json
import sys
from pathlib import Path
from typing import Optional

try:
    import jsonschema
except ImportError:
    print("Error: jsonschema package not installed")
    print("Install with: pip install jsonschema")
    sys.exit(1)


FIXTURES_DIR = Path(__file__).parent.parent
CATEGORIES = [
    "multipart",
    "query_params",
    "headers",
    "cookies",
    "json_bodies",
    "url_encoded",
    "path_params",
    "status_codes",
    "content_types",
    "validation_errors",
    "http_methods",
    "cors",
    "edge_cases",
]


def validate_category(category: str) -> tuple[int, int]:
    """Validate all fixtures in a category.

    Returns:
        (valid_count, total_count)
    """
    category_dir = FIXTURES_DIR / category
    schema_path = category_dir / "schema.json"

    if not schema_path.exists():
        print(f"⚠️  {category}: No schema.json found, skipping")
        return (0, 0)

    with open(schema_path) as f:
        schema = json.load(f)

    fixtures = sorted(category_dir.glob("*.json"))
    fixtures = [f for f in fixtures if f.name != "schema.json"]

    if not fixtures:
        print(f"ℹ️  {category}: No fixtures found")
        return (0, 0)

    valid_count = 0
    total_count = len(fixtures)

    print(f"\n📂 {category}:")
    for fixture_path in fixtures:
        with open(fixture_path) as f:
            try:
                fixture = json.load(f)
                jsonschema.validate(instance=fixture, schema=schema)
                print(f"  ✅ {fixture_path.name}")
                valid_count += 1
            except json.JSONDecodeError as e:
                print(f"  ❌ {fixture_path.name}: Invalid JSON - {e}")
            except jsonschema.ValidationError as e:
                print(f"  ❌ {fixture_path.name}: Schema validation failed")
                print(f"     {e.message}")
                if e.path:
                    print(f"     Path: {'.'.join(str(p) for p in e.path)}")

    return (valid_count, total_count)


def main(category: Optional[str] = None):
    """Validate fixtures.

    Args:
        category: Specific category to validate, or None for all
    """
    if category:
        if category not in CATEGORIES:
            print(f"Error: Unknown category '{category}'")
            print(f"Available: {', '.join(CATEGORIES)}")
            sys.exit(1)
        categories = [category]
    else:
        categories = CATEGORIES

    print("🔍 Validating test fixtures...\n")

    total_valid = 0
    total_fixtures = 0

    for cat in categories:
        valid, total = validate_category(cat)
        total_valid += valid
        total_fixtures += total

    print("\n" + "=" * 50)
    if total_fixtures == 0:
        print("ℹ️  No fixtures found to validate")
    elif total_valid == total_fixtures:
        print(f"✅ All {total_fixtures} fixtures are valid!")
        sys.exit(0)
    else:
        print(f"❌ {total_valid}/{total_fixtures} fixtures valid")
        print(f"   {total_fixtures - total_valid} fixtures failed validation")
        sys.exit(1)


if __name__ == "__main__":
    category = sys.argv[1] if len(sys.argv) > 1 else None
    main(category)
