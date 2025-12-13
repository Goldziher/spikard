#!/usr/bin/env python3

"""Validate all test fixtures against their JSON schemas."""

import json
import sys
from pathlib import Path

try:
    import jsonschema
except ImportError:
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
        return (0, 0)

    with schema_path.open() as f:
        schema = json.load(f)

    fixtures = sorted(category_dir.glob("*.json"))
    fixtures = [f for f in fixtures if f.name != "schema.json"]

    if not fixtures:
        return (0, 0)

    valid_count = 0
    total_count = len(fixtures)

    for fixture_path in fixtures:
        with fixture_path.open() as f:
            try:
                fixture = json.load(f)
                jsonschema.validate(instance=fixture, schema=schema)
                valid_count += 1
            except json.JSONDecodeError:
                pass
            except jsonschema.ValidationError as e:
                if e.path:
                    pass

    return (valid_count, total_count)


def main(category: str | None = None) -> None:
    """Validate fixtures.

    Args:
        category: Specific category to validate, or None for all
    """
    if category:
        if category not in CATEGORIES:
            sys.exit(1)
        categories = [category]
    else:
        categories = CATEGORIES

    total_valid = 0
    total_fixtures = 0

    for cat in categories:
        valid, total = validate_category(cat)
        total_valid += valid
        total_fixtures += total

    if total_fixtures == 0:
        pass
    elif total_valid == total_fixtures:
        sys.exit(0)
    else:
        sys.exit(1)


if __name__ == "__main__":
    category = sys.argv[1] if len(sys.argv) > 1 else None
    main(category)
