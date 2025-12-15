#!/usr/bin/env python3
"""Convert benchmark-harness `profile.json` outputs into Markdown tables.

Usage:
  python tools/benchmark-harness/scripts/bench_to_md.py results/benchmarks/<run_id> > summary.md
"""

from __future__ import annotations

import json
import sys
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True, slots=True)
class FrameworkSummary:
    """Aggregate summary extracted from a single `profile.json`."""

    framework: str
    avg_rps: float
    avg_latency_ms: float
    total_requests: int
    duration_secs: int
    by_category: dict[str, tuple[float, float]]  # category -> (avg_rps, avg_latency_ms)


def _fmt_float(value: float, decimals: int = 2) -> str:
    return f"{value:.{decimals}f}"


def _load_profile(path: Path) -> FrameworkSummary:
    payload = json.loads(path.read_text(encoding="utf-8"))
    framework = str(payload.get("framework", {}).get("name", path.parent.name))
    summary = payload["summary"]

    by_category: dict[str, tuple[float, float]] = {}
    for entry in summary.get("category_breakdown", []):
        category = str(entry["category"])
        by_category[category] = (float(entry["avg_requests_per_sec"]), float(entry["avg_latency_ms"]))

    return FrameworkSummary(
        framework=framework,
        avg_rps=float(summary["avg_requests_per_sec"]),
        avg_latency_ms=sum(lat for _, lat in by_category.values()) / max(len(by_category), 1),
        total_requests=int(summary["total_requests"]),
        duration_secs=int(summary["total_duration_secs"]),
        by_category=by_category,
    )


def _find_profiles(root: Path) -> list[Path]:
    return sorted(root.glob("benchmark-results-*/profile.json"))


def _render_overall_table(items: list[FrameworkSummary]) -> str:
    lines: list[str] = []
    lines.append("| Framework | Avg RPS | Avg Latency (ms) | Total Requests | Duration (s) |")
    lines.append("|---|---:|---:|---:|---:|")
    lines.extend(
        [
            f"| {item.framework} | {_fmt_float(item.avg_rps)} | {_fmt_float(item.avg_latency_ms)} |"
            f" {item.total_requests} | {item.duration_secs} |"
            for item in sorted(items, key=lambda x: x.avg_rps, reverse=True)
        ]
    )
    return "\n".join(lines)


def _render_category_table(items: list[FrameworkSummary]) -> str:
    categories: list[str] = sorted({c for item in items for c in item.by_category})

    lines: list[str] = []
    lines.append("| Framework | " + " | ".join(f"{c} RPS" for c in categories) + " |")
    lines.append("|---|" + "|".join("---:" for _ in categories) + "|")
    for item in sorted(items, key=lambda x: x.framework.lower()):
        row = [item.framework]
        for category in categories:
            rps = item.by_category.get(category, (0.0, 0.0))[0]
            row.append(_fmt_float(rps))
        lines.append("| " + " | ".join(row) + " |")

    lines.append("")
    lines.append("| Framework | " + " | ".join(f"{c} Lat (ms)" for c in categories) + " |")
    lines.append("|---|" + "|".join("---:" for _ in categories) + "|")
    for item in sorted(items, key=lambda x: x.framework.lower()):
        row = [item.framework]
        for category in categories:
            lat = item.by_category.get(category, (0.0, 0.0))[1]
            row.append(_fmt_float(lat))
        lines.append("| " + " | ".join(row) + " |")

    return "\n".join(lines)


def main() -> int:
    """CLI entrypoint."""
    if len(sys.argv) != 2:
        sys.stderr.write(f"{__doc__.strip()}\n")
        return 2

    root = Path(sys.argv[1]).resolve()
    profiles = _find_profiles(root)
    if not profiles:
        sys.stderr.write(f"No profiles found under: {root}\n")
        return 1

    items = [_load_profile(p) for p in profiles]

    out: list[str] = []
    out.append(f"# Benchmark Summary ({root.name})")
    out.append("")
    out.append("## Overall")
    out.append(_render_overall_table(items))
    out.append("")
    out.append("## By Category")
    out.append(_render_category_table(items))
    out.append("")

    sys.stdout.write("\n".join(out))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
