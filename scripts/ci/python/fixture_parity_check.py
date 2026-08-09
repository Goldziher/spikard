#!/usr/bin/env python3
"""Detect drift between fixtures/ (alef's canonical input) and testing_data/ (per-binding twins).

Background: fixtures/<category>.json holds arrays of {id, category, description, tags, http}
fixture objects; alef reads only this tree. testing_data/<category>/*.json holds one file per
fixture, historically hand-authored, in a flatter shape ({name, description, source, handler,
request, expected_response, tags, notes, ...}). The two trees are meant to describe the same
test cases for different consumers (alef codegen vs. per-language binding test suites) but are
hand-edited independently and have no enforced link, so they drift silently: an edit to one
tree has zero effect on the other.

There is no `id` field on the testing_data side. The only empirically-reliable correspondence
key, found by inspecting the corpus, is:

    fixtures_entry.id == slugify(testing_data_entry.name)

(lowercase, non-alphanumeric runs collapsed to a single underscore). This holds for every
testing_data entry that was actually derived from a fixtures/ entry. testing_data also contains
a large body of legacy, hand-ported content (from framework test suites, e.g. `source.framework:
fastapi`) that predates fixtures/ and has no fixtures/ counterpart at all -- that content is
reported as informational, not as drift, because there is nothing on the fixtures/ side for it
to disagree with.

Not every testing_data/<name> directory is a twin of a fixtures/<name>.json file. Several
directories hold an entirely different kind of artifact (raw OpenAPI/AsyncAPI/OpenRPC schema
documents, an RPC method catalogue, protobuf fixtures, SQL-to-HTTP handlers) that fixtures/
does not model at all. Those are excluded from comparison via NON_TWIN_TESTING_DATA_DIRS below.

Usage:
    python3 scripts/ci/python/fixture_parity_check.py [--json]

Exit status: 0 if fixtures/ and testing_data/ fully agree wherever they overlap, 1 otherwise.
"""

from __future__ import annotations

import argparse
import json
import re
from collections import OrderedDict
from dataclasses import dataclass, field
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[3]
FIXTURES_DIR = REPO_ROOT / "fixtures"
TESTING_DATA_DIR = REPO_ROOT / "testing_data"

# ~keep: fixtures/<key>.json -> testing_data/<dir>/ believed (and verified, see module docstring)
# to hold the twin of that fixture file. A category maps to [] when testing_data/ carries no
# counterpart directory at all -- fixtures with zero cross-tree coverage.
CATEGORY_MAP: dict[str, list[str]] = {
    "asyncapi": [],  # testing_data/asyncapi_schemas holds raw AsyncAPI docs, not http fixtures
    "auth": ["auth"],
    "background_tasks": [],  # no testing_data counterpart at all
    "background": ["background"],
    "body_limits": ["body_limits"],
    "compression": ["compression"],
    "content_types": ["content_types"],
    "cookies": ["cookies"],
    "cors": ["cors"],
    "di": ["di"],
    "edge_cases": ["edge_cases"],
    "graphql_operations": [],  # testing_data/graphql is an independent, much larger corpus
    "graphql_schema": [],
    "grpc": [],  # no testing_data counterpart at all
    "headers": ["headers"],
    "http_methods": ["http_methods"],
    "json_bodies": ["json_bodies"],
    "jsonrpc": [],  # testing_data/jsonrpc holds an RPC method schema catalogue, not http fixtures
    "lifecycle_hooks": ["lifecycle_hooks"],
    "multipart": ["multipart"],
    "openapi": [],  # testing_data/openapi_schemas holds raw OpenAPI documents, not http fixtures
    "openrpc": [],  # testing_data/openrpc_schemas holds raw OpenRPC documents, not http fixtures
    "path_params": ["path_params"],
    "problem_details": [],  # no testing_data counterpart at all
    "query_params": ["query_params"],
    "rate_limit": ["rate_limit"],
    "request_id": ["request_id"],
    "request_timeout": ["request_timeout"],
    "response": [],  # no testing_data counterpart at all
    "server_config": [],  # no testing_data counterpart at all
    "sse": [],  # testing_data/sse holds AsyncAPI channel/message defs, not http fixtures
    "static_files": ["static_files"],
    "status_codes": ["status_codes"],
    "streaming": ["streaming"],
    "upload": [],  # no testing_data counterpart at all
    "url_encoded": ["url_encoded"],
    "validation_errors": ["validation_errors"],
    "websocket": [],  # testing_data/websockets holds AsyncAPI channel/message defs, not http fixtures
}

# testing_data/ directories that are not twins of any fixtures/*.json file at all -- independent
# corpora with their own purpose (spec documents, RPC catalogues, protobuf/SQL codegen fixtures).
NON_TWIN_TESTING_DATA_DIRS = {
    "asyncapi_schemas",
    "graphql",
    "jsonrpc",
    "openapi_schemas",
    "openrpc_schemas",
    "protobuf",
    "schemas",
    "scripts",
    "sql_handlers",
    "sse",
    "websockets",
}

EXCLUDE_FILENAMES = {"schema.json"}
# testing_data-only metadata fields that legitimately exist on only one side and are not drift.
TESTING_DATA_ONLY_META = {"name", "source", "notes"}


def slugify(name: object) -> str | None:
    """Turn a testing_data `name` field into the slug form fixtures/ ids use."""
    if not isinstance(name, str):
        return None
    slug = re.sub(r"[^a-z0-9]+", "_", name.lower())
    return slug.strip("_") or None


@dataclass
class TwinFile:
    """A single testing_data/ JSON file, parsed."""

    path: Path
    content: dict


@dataclass
class CategoryResult:
    """Parity outcome for one fixtures/<category>.json file."""

    fixture_category: str
    testing_data_dirs: list[str]
    fixture_count: int = 0
    unmatched_fixture_ids: list[str] = field(default_factory=list)
    matched_pairs: list[tuple[str, TwinFile]] = field(default_factory=list)
    disagreements: list[tuple[str, TwinFile, list[str]]] = field(default_factory=list)


def load_testing_data_twins(dirnames: list[str]) -> dict[str, list[TwinFile]]:
    """Index every testing_data file under dirnames by slugify(name)."""
    by_slug: dict[str, list[TwinFile]] = {}
    for dirname in dirnames:
        base = TESTING_DATA_DIR / dirname
        if not base.is_dir():
            continue
        for full in base.rglob("*.json"):
            if full.name in EXCLUDE_FILENAMES or full.name.startswith("00-"):
                continue
            try:
                content = json.loads(full.read_text(encoding="utf-8"))
            except (json.JSONDecodeError, OSError):
                continue
            if not isinstance(content, dict):
                continue
            slug = slugify(content.get("name"))
            if slug:
                by_slug.setdefault(slug, []).append(TwinFile(full, content))
    return by_slug


def fixture_comparable_payload(item: dict, twin_content: dict) -> dict:
    """Project a fixtures/ entry onto the same shape a testing_data twin carries.

    `category`, `tags` and `skip` are fixtures/-side metadata that most testing_data files simply
    do not carry -- the two trees were never structurally identical. Including them
    unconditionally reported one disagreement per entry (210 of 228 `.category` hits) and buried
    the real mismatches, so they are compared only against a twin that actually declares them. ~keep
    """
    payload = dict(item["http"]) if "http" in item else dict(item.get("asyncapi", {}))
    if "websocket" in item:
        payload["websocket"] = item["websocket"]
    for key in ("category", "tags", "skip"):
        if key in item and key in twin_content:
            payload[key] = item[key]
    return payload


def testing_data_comparable_payload(content: dict) -> dict:
    """Strip testing_data-only metadata so the remainder is comparable to a fixture payload."""
    return {k: v for k, v in content.items() if k not in TESTING_DATA_ONLY_META and k != "description"}


def diff_values(path: str, fixture_value: object, testing_data_value: object, out: list[str]) -> None:
    """Recursively record every point where fixture_value and testing_data_value disagree."""
    if isinstance(fixture_value, dict) and isinstance(testing_data_value, dict):
        for key in sorted(set(fixture_value) | set(testing_data_value)):
            if key not in fixture_value:
                out.append(f"{path}.{key}: missing in fixtures/, testing_data/ has {testing_data_value[key]!r}")
            elif key not in testing_data_value:
                out.append(f"{path}.{key}: fixtures/ has {fixture_value[key]!r}, missing in testing_data/")
            else:
                diff_values(f"{path}.{key}", fixture_value[key], testing_data_value[key], out)
    elif isinstance(fixture_value, list) and isinstance(testing_data_value, list):
        if len(fixture_value) != len(testing_data_value):
            out.append(
                f"{path}: length differs -- fixtures/={len(fixture_value)} testing_data/={len(testing_data_value)}"
            )
        else:
            for i, (fv, tv) in enumerate(zip(fixture_value, testing_data_value, strict=True)):
                diff_values(f"{path}[{i}]", fv, tv, out)
    elif fixture_value != testing_data_value:
        out.append(f"{path}: fixtures/={fixture_value!r} testing_data/={testing_data_value!r}")


def check_category(fixture_stem: str, td_dirs: list[str]) -> CategoryResult:
    """Compare one fixtures/<fixture_stem>.json against its testing_data/ twin(s), if any."""
    items = json.loads((FIXTURES_DIR / f"{fixture_stem}.json").read_text(encoding="utf-8"))
    result = CategoryResult(fixture_category=fixture_stem, testing_data_dirs=td_dirs, fixture_count=len(items))
    if not td_dirs:
        result.unmatched_fixture_ids = [item["id"] for item in items]
        return result

    by_slug = load_testing_data_twins(td_dirs)
    for item in items:
        fid = item["id"]
        twins = by_slug.get(fid)
        if not twins:
            result.unmatched_fixture_ids.append(fid)
            continue
        for twin in twins:
            result.matched_pairs.append((fid, twin))
            fx_payload = fixture_comparable_payload(item, twin.content)
            td_payload = testing_data_comparable_payload(twin.content)
            diffs: list[str] = []
            diff_values("", fx_payload, td_payload, diffs)
            if diffs:
                result.disagreements.append((fid, twin, diffs))
    return result


def check_unmapped_testing_data_dirs() -> list[str]:
    """testing_data/ directories neither mapped to a fixture category nor allowlisted as
    independent.

    New directories land here until someone classifies them.
    """
    known = {d for dirs in CATEGORY_MAP.values() for d in dirs} | NON_TWIN_TESTING_DATA_DIRS
    actual = {p.name for p in TESTING_DATA_DIR.iterdir() if p.is_dir()}
    return sorted(actual - known)


def print_text_report(
    results: OrderedDict[str, CategoryResult],
    unmapped_dirs: list[str],
    totals: dict[str, int],
) -> None:
    """Print the human-readable parity report to stdout."""
    print("fixtures/ <-> testing_data/ parity check")
    print("=" * 72)
    print(f"fixture entries examined:        {totals['fixtures']}")
    print(f"matched to a testing_data twin:  {totals['matched']}")
    print(f"fixtures with NO twin found:     {totals['unmatched']}")
    print(f"matched pairs that DISAGREE:     {totals['disagreeing']}")
    print()

    if unmapped_dirs:
        print(
            f"UNCLASSIFIED testing_data/ directories (add to CATEGORY_MAP or "
            f"NON_TWIN_TESTING_DATA_DIRS): {unmapped_dirs}"
        )
        print()

    for stem, result in results.items():
        if not result.unmatched_fixture_ids and not result.disagreements:
            continue
        print(f"--- {stem} ---")
        if result.unmatched_fixture_ids:
            if not result.testing_data_dirs:
                print(
                    f"  category has NO testing_data/ counterpart at all "
                    f"({len(result.unmatched_fixture_ids)} fixtures uncovered)"
                )
            else:
                print(
                    f"  {len(result.unmatched_fixture_ids)} fixture id(s) with no "
                    f"testing_data/{result.testing_data_dirs[0]} twin:"
                )
                for fid in result.unmatched_fixture_ids:
                    print(f"    - {fid}")
        for fid, twin, diffs in result.disagreements:
            print(f"  DISAGREEMENT id={fid}")
            print(f"    testing_data: {twin.path.relative_to(REPO_ROOT)}")
            for d in diffs:
                print(f"      {d}")
        print()


def main() -> int:
    """Run the parity check and return the process exit code."""
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--json", action="store_true", help="emit machine-readable JSON instead of text")
    args = parser.parse_args()

    results: OrderedDict[str, CategoryResult] = OrderedDict()
    for stem in sorted(CATEGORY_MAP):
        results[stem] = check_category(stem, CATEGORY_MAP[stem])

    unmapped_dirs = check_unmapped_testing_data_dirs()

    totals = {
        "fixtures": sum(r.fixture_count for r in results.values()),
        "unmatched": sum(len(r.unmatched_fixture_ids) for r in results.values()),
        "matched": sum(len(r.matched_pairs) for r in results.values()),
        "disagreeing": sum(len(r.disagreements) for r in results.values()),
    }

    if args.json:
        payload = {
            "total_fixtures": totals["fixtures"],
            "total_matched_pairs": totals["matched"],
            "total_unmatched_fixtures": totals["unmatched"],
            "total_disagreeing_pairs": totals["disagreeing"],
            "unmapped_testing_data_dirs": unmapped_dirs,
            "categories": {
                stem: {
                    "testing_data_dirs": r.testing_data_dirs,
                    "fixture_count": r.fixture_count,
                    "matched": len(r.matched_pairs),
                    "unmatched_fixture_ids": r.unmatched_fixture_ids,
                    "disagreements": [
                        {"id": fid, "testing_data_path": str(twin.path.relative_to(REPO_ROOT)), "diffs": diffs}
                        for fid, twin, diffs in r.disagreements
                    ],
                }
                for stem, r in results.items()
            },
        }
        print(json.dumps(payload, indent=2))
    else:
        print_text_report(results, unmapped_dirs, totals)

    failed = totals["unmatched"] > 0 or totals["disagreeing"] > 0 or bool(unmapped_dirs)
    if not args.json:
        print("RESULT: " + ("FAIL" if failed else "PASS"))
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
