#!/usr/bin/env python3
"""Generate interactive Plotly HTML charts from aggregated benchmark results."""

from __future__ import annotations

import argparse
import json
import sys
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

import pandas as pd
import plotly.graph_objects as go
from plotly.subplots import make_subplots


def load_aggregated_results(path: Path) -> dict[str, Any]:
    """Load and validate aggregated benchmark JSON."""
    with path.open() as f:
        return json.load(f)


def extract_framework_data(data: dict[str, Any]) -> pd.DataFrame:
    """Extract framework performance data into a pandas DataFrame."""
    rows = []

    for fw_result in data["frameworks"]:
        framework = fw_result["framework"]
        profile = fw_result["profile"]
        status = fw_result["status"]

        if status != "completed":
            continue

        for suite in profile.get("suites", []):
            for workload in suite.get("workloads", []):
                # Support both "results" (actual schema) and "metrics" (legacy)
                metrics = workload.get("results", workload.get("metrics", {}))
                throughput = metrics.get("throughput", {})
                latency = metrics.get("latency", {})
                resources = metrics.get("resources", {})

                row = {
                    "framework": framework,
                    "suite": suite["name"],
                    "workload": workload["name"],
                    "status": status,
                    "requests_per_sec": throughput.get("requests_per_sec", 0),
                    "total_requests": throughput.get("total_requests", 0),
                    "success_rate": throughput.get("success_rate", 0) * 100,
                    "latency_mean_ms": latency.get("mean_ms", 0),
                    "latency_p50_ms": latency.get("median_ms", latency.get("p50_ms", 0)),
                    "latency_p90_ms": latency.get("p90_ms", 0),
                    "latency_p95_ms": latency.get("p95_ms", 0),
                    "latency_p99_ms": latency.get("p99_ms", 0),
                    "latency_p999_ms": latency.get("p999_ms", 0),
                    "cpu_avg_percent": resources.get("cpu", {}).get("avg_percent", 0),
                    "cpu_peak_percent": resources.get("cpu", {}).get("peak_percent", 0),
                    "memory_avg_mb": resources.get("memory", {}).get("avg_mb", 0),
                    "memory_peak_mb": resources.get("memory", {}).get("peak_mb", 0),
                }
                rows.append(row)

    return pd.DataFrame(rows)


def get_color_scheme() -> list[str]:
    """Material Design color palette matching MkDocs theme."""
    return [
        "#1976d2",  # Blue
        "#388e3c",  # Green
        "#f57c00",  # Orange
        "#c62828",  # Red
        "#7b1fa2",  # Purple
        "#00796b",  # Teal
        "#c2185b",  # Pink
        "#5d4037",  # Brown
        "#455a64",  # Blue Grey
        "#fbc02d",  # Yellow
        "#0288d1",  # Light Blue
        "#689f38",  # Light Green
    ]


def _write_figure(fig: go.Figure, output_dir: Path, name: str, fmt: str) -> list[Path]:
    """Write a figure in the requested format(s). Returns list of output paths."""
    paths: list[Path] = []

    formats = ["html", "svg", "png"] if fmt == "all" else [fmt]

    for f in formats:
        if f == "html":
            p = output_dir / f"{name}.html"
            fig.write_html(str(p))
            paths.append(p)
        elif f in ("svg", "png"):
            p = output_dir / f"{name}.{f}"
            fig.write_image(str(p), format=f)
            paths.append(p)

    return paths


def generate_throughput_chart(
    data: dict[str, Any], output_dir: Path, title: str, fmt: str
) -> list[Path]:
    """Generate throughput comparison chart."""
    df = extract_framework_data(data)

    if df.empty:
        return []

    json_workloads = df[df["suite"] == "json-bodies"]

    if json_workloads.empty:
        json_workloads = df

    pivot = json_workloads.pivot_table(
        index="framework",
        columns="workload",
        values="requests_per_sec",
        aggfunc="mean",
    ).fillna(0)

    fig = go.Figure()

    colors = get_color_scheme()
    for idx, col in enumerate(pivot.columns):
        fig.add_trace(
            go.Bar(
                name=col,
                x=pivot.index,
                y=pivot[col],
                marker_color=colors[idx % len(colors)],
                text=[f"{v:.0f}" for v in pivot[col]],
                textposition="outside",
            )
        )

    fig.update_layout(
        title=f"{title} - Throughput Comparison",
        xaxis_title="Framework",
        yaxis_title="Requests / second",
        barmode="group",
        template="plotly_white",
        height=600,
        font={"family": "Roboto, sans-serif", "size": 12},
        showlegend=True,
        legend={
            "title": "Workload",
            "orientation": "v",
            "yanchor": "top",
            "y": 1,
            "xanchor": "left",
            "x": 1.02,
        },
    )

    return _write_figure(fig, output_dir, "throughput-by-framework", fmt)


def generate_latency_chart(
    data: dict[str, Any], output_dir: Path, title: str, fmt: str
) -> list[Path]:
    """Generate latency percentile comparison (2x2 subplot)."""
    df = extract_framework_data(data)

    if df.empty:
        return []

    small_json = df[(df["suite"] == "json-bodies") & (df["workload"].str.contains("small"))]

    if small_json.empty:
        small_json = df.groupby("framework", as_index=False).mean(numeric_only=True)

    fig = make_subplots(
        rows=2,
        cols=2,
        subplot_titles=("P50 (Median)", "P90", "P95", "P99"),
        vertical_spacing=0.12,
        horizontal_spacing=0.10,
    )

    colors = get_color_scheme()
    percentiles = [
        ("latency_p50_ms", 1, 1),
        ("latency_p90_ms", 1, 2),
        ("latency_p95_ms", 2, 1),
        ("latency_p99_ms", 2, 2),
    ]

    for col_name, row, col in percentiles:
        sorted_df = small_json.sort_values(col_name)
        fig.add_trace(
            go.Bar(
                x=sorted_df["framework"],
                y=sorted_df[col_name],
                marker_color=[colors[i % len(colors)] for i in range(len(sorted_df))],
                text=[f"{v:.2f}" for v in sorted_df[col_name]],
                textposition="outside",
                showlegend=False,
            ),
            row=row,
            col=col,
        )

    fig.update_xaxes(tickangle=-45)
    fig.update_yaxes(title_text="Latency (ms)", row=1, col=1)
    fig.update_yaxes(title_text="Latency (ms)", row=2, col=1)

    fig.update_layout(
        title_text=f"{title} - Latency Percentiles",
        template="plotly_white",
        height=800,
        font={"family": "Roboto, sans-serif", "size": 12},
        showlegend=False,
    )

    return _write_figure(fig, output_dir, "latency-percentiles", fmt)


def generate_validation_overhead_chart(
    data: dict[str, Any], output_dir: Path, title: str, fmt: str
) -> list[Path]:
    """Generate validation vs raw comparison chart."""
    df = extract_framework_data(data)

    if df.empty:
        return []

    df["is_validated"] = df["workload"].str.startswith("validated/")
    df["base_workload"] = df["workload"].str.replace("^validated/", "", regex=True)

    json_small = df[(df["suite"] == "json-bodies") & (df["workload"].str.contains("small"))]

    if json_small.empty:
        json_small = df

    # Pair raw and validated workloads for each framework
    pivot = json_small.pivot_table(
        index=["framework", "base_workload"],
        columns="is_validated",
        values="requests_per_sec",
        aggfunc="mean",
    )

    if not pivot.empty:
        pivot = pivot.reset_index()
        pivot = pivot.dropna(subset=[False, True])

    if pivot.empty or False not in pivot.columns or True not in pivot.columns:
        fig = go.Figure()
        fig.add_annotation(
            text="Insufficient validation/raw pairs for comparison",
            xref="paper",
            yref="paper",
            x=0.5,
            y=0.5,
            showarrow=False,
            font={"size": 16},
        )
    else:
        pivot["overhead_pct"] = ((pivot[False] - pivot[True]) / pivot[False]) * 100

        fig = go.Figure()

        colors = get_color_scheme()
        color_idx = 0

        for _, row in pivot.iterrows():
            fw = row["framework"]
            fig.add_trace(
                go.Bar(
                    name="Raw",
                    x=[f"{fw}\n{row['base_workload']}"],
                    y=[row[False]],
                    marker_color=colors[color_idx % len(colors)],
                    text=f"{row[False]:.0f}",
                    textposition="outside",
                    showlegend=(color_idx == 0),
                )
            )
            fig.add_trace(
                go.Bar(
                    name="Validated",
                    x=[f"{fw}\n{row['base_workload']}"],
                    y=[row[True]],
                    marker_color=colors[(color_idx + 1) % len(colors)],
                    text=f"{row[True]:.0f}",
                    textposition="outside",
                    showlegend=(color_idx == 0),
                )
            )

            overhead = row["overhead_pct"]
            fig.add_annotation(
                x=f"{fw}\n{row['base_workload']}",
                y=max(row[False], row[True]),
                text=f"{overhead:.1f}% overhead",
                showarrow=True,
                arrowhead=2,
                arrowsize=1,
                arrowwidth=1,
                arrowcolor="gray",
                ax=0,
                ay=-40,
                font={"size": 10, "color": "red" if overhead > 20 else "green" if overhead < 5 else "orange"},
            )
            color_idx += 2

    fig.update_layout(
        title=f"{title} - Validation Overhead",
        xaxis_title="Framework",
        yaxis_title="Requests / second",
        barmode="group",
        template="plotly_white",
        height=600,
        font={"family": "Roboto, sans-serif", "size": 12},
    )

    return _write_figure(fig, output_dir, "validation-overhead", fmt)


def generate_resource_chart(
    data: dict[str, Any], output_dir: Path, title: str, fmt: str
) -> list[Path]:
    """Generate resource utilization chart (dual-axis: memory bars + CPU line)."""
    df = extract_framework_data(data)

    if df.empty:
        return []

    resources = (
        df.groupby("framework", as_index=False)[["memory_peak_mb", "cpu_avg_percent"]]
        .mean()
        .sort_values("memory_peak_mb", ascending=False)
    )

    fig = make_subplots(specs=[[{"secondary_y": True}]])

    colors = get_color_scheme()
    fig.add_trace(
        go.Bar(
            name="Peak Memory",
            x=resources["framework"],
            y=resources["memory_peak_mb"],
            marker_color=colors[0],
            text=[f"{v:.1f}" for v in resources["memory_peak_mb"]],
            textposition="outside",
        ),
        secondary_y=False,
    )

    fig.add_trace(
        go.Scatter(
            name="Avg CPU",
            x=resources["framework"],
            y=resources["cpu_avg_percent"],
            mode="lines+markers",
            marker={"color": colors[2], "size": 8},
            line={"width": 2, "color": colors[2]},
        ),
        secondary_y=True,
    )

    fig.update_xaxes(title_text="Framework", tickangle=-45)
    fig.update_yaxes(title_text="Memory (MB)", secondary_y=False)
    fig.update_yaxes(title_text="CPU (%)", secondary_y=True)

    fig.update_layout(
        title=f"{title} - Resource Utilization",
        template="plotly_white",
        height=600,
        font={"family": "Roboto, sans-serif", "size": 12},
        hovermode="x unified",
    )

    return _write_figure(fig, output_dir, "resources", fmt)


def generate_throughput_summary(
    data: dict[str, Any], output_dir: Path, fmt: str
) -> list[Path]:
    """Generate a simple horizontal bar chart of avg RPS per framework (for README)."""
    df = extract_framework_data(data)
    if df.empty:
        return []

    # All frameworks now run all workloads (raw + validated), so no need to filter for common workloads
    summary = (
        df.groupby("framework", as_index=False)["requests_per_sec"]
        .mean()
        .sort_values("requests_per_sec", ascending=True)
    )

    n_workloads = df["workload"].nunique()
    colors = get_color_scheme()
    fig = go.Figure(
        go.Bar(
            y=summary["framework"],
            x=summary["requests_per_sec"],
            orientation="h",
            marker_color=[colors[i % len(colors)] for i in range(len(summary))],
            text=[f"{v:,.0f}" for v in summary["requests_per_sec"]],
            textposition="outside",
        )
    )
    fig.update_layout(
        title=f"Average Requests/sec by Framework (across {n_workloads} workloads)",
        xaxis_title="Requests / second",
        template="plotly_white",
        height=max(400, len(summary) * 35 + 100),
        margin={"l": 200},
        font={"family": "Roboto, sans-serif", "size": 12},
    )

    return _write_figure(fig, output_dir, "throughput-summary", fmt)


def generate_latency_summary(
    data: dict[str, Any], output_dir: Path, fmt: str
) -> list[Path]:
    """Generate a simple horizontal bar chart of P99 latency per framework (for README)."""
    df = extract_framework_data(data)
    if df.empty:
        return []

    # All frameworks now run all workloads (raw + validated), so no need to filter for common workloads
    n_workloads = df["workload"].nunique()
    summary = (
        df.groupby("framework", as_index=False)["latency_p99_ms"]
        .mean()
        .sort_values("latency_p99_ms", ascending=True)
    )

    colors = get_color_scheme()
    fig = go.Figure(
        go.Bar(
            y=summary["framework"],
            x=summary["latency_p99_ms"],
            orientation="h",
            marker_color=[colors[i % len(colors)] for i in range(len(summary))],
            text=[f"{v:.2f} ms" for v in summary["latency_p99_ms"]],
            textposition="outside",
        )
    )
    fig.update_layout(
        title=f"P99 Latency by Framework (across {n_workloads} workloads)",
        xaxis_title="Latency (ms)",
        template="plotly_white",
        height=max(400, len(summary) * 35 + 100),
        margin={"l": 200},
        font={"family": "Roboto, sans-serif", "size": 12},
    )

    return _write_figure(fig, output_dir, "latency-summary", fmt)


def generate_markdown_table(data: dict[str, Any], output_file: Path | None = None) -> str:
    """Generate a markdown summary table of benchmark results."""
    df = extract_framework_data(data)
    if df.empty:
        return "No benchmark data available."

    summary = (
        df.groupby("framework", as_index=False)
        .agg(
            avg_rps=("requests_per_sec", "mean"),
            p50_ms=("latency_p50_ms", "mean"),
            p99_ms=("latency_p99_ms", "mean"),
            cpu_pct=("cpu_avg_percent", "mean"),
            mem_mb=("memory_peak_mb", "mean"),
        )
        .sort_values("avg_rps", ascending=False)
    )

    lines = [
        "| Framework | Avg RPS | P50 (ms) | P99 (ms) | CPU (%) | Mem (MB) |",
        "|-----------|---------|----------|----------|---------|----------|",
    ]
    for _, row in summary.iterrows():
        lines.append(
            f"| {row['framework']} "
            f"| {row['avg_rps']:,.0f} "
            f"| {row['p50_ms']:.2f} "
            f"| {row['p99_ms']:.2f} "
            f"| {row['cpu_pct']:.1f} "
            f"| {row['mem_mb']:.1f} |"
        )

    table = "\n".join(lines)

    if output_file is not None:
        output_file.write_text(table + "\n")

    return table


def generate_metadata(data: dict[str, Any], output_dir: Path, charts: list[str]) -> None:
    """Generate metadata.json for dynamic injection into docs."""
    chart_files = []
    for name in charts:
        chart_files.append(f"{name}.html")

    metadata = {
        "generated_at": datetime.now(UTC).isoformat(),
        "workflow_run_id": data["metadata"]["run_id"],
        "workflow_run_url": data["metadata"]["run_url"],
        "git_commit": data["metadata"].get("commit"),
        "git_branch": data["metadata"].get("branch"),
        "frameworks": {
            "total": data["summary"]["total_frameworks"],
            "completed": data["summary"]["completed"],
            "failed": data["summary"]["failed"],
        },
        "charts": chart_files,
    }

    output_path = output_dir / "metadata.json"
    with output_path.open("w") as f:
        json.dump(metadata, f, indent=2)


def main() -> None:
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Generate Plotly HTML charts from aggregated benchmark results")
    parser.add_argument("--input", type=Path, required=True, help="Path to aggregated.json")
    parser.add_argument("--output", type=Path, required=True, help="Output directory for charts")
    parser.add_argument(
        "--charts",
        default="all",
        help="Comma-separated chart types: throughput,latency,validation,resources,throughput-summary,latency-summary,all",
    )
    parser.add_argument("--title", default="Benchmark Results", help="Title prefix for charts")
    parser.add_argument(
        "--format",
        default="html",
        choices=["html", "svg", "png", "all"],
        help="Output format: html (interactive), svg/png (static images), all",
    )
    parser.add_argument(
        "--markdown",
        nargs="?",
        const="-",
        default=None,
        help="Output markdown table to file (or stdout if no path / '-' given)",
    )
    args = parser.parse_args()

    data = load_aggregated_results(args.input)

    args.output.mkdir(parents=True, exist_ok=True)

    chart_types = [c.strip() for c in args.charts.split(",")]
    if "all" in chart_types:
        chart_types = [
            "throughput",
            "latency",
            "validation",
            "resources",
            "throughput-summary",
            "latency-summary",
        ]

    generated_charts: list[str] = []

    for chart_type in chart_types:
        if chart_type == "throughput":
            generate_throughput_chart(data, args.output, args.title, args.format)
            generated_charts.append("throughput-by-framework")
        elif chart_type == "latency":
            generate_latency_chart(data, args.output, args.title, args.format)
            generated_charts.append("latency-percentiles")
        elif chart_type == "validation":
            generate_validation_overhead_chart(data, args.output, args.title, args.format)
            generated_charts.append("validation-overhead")
        elif chart_type == "resources":
            generate_resource_chart(data, args.output, args.title, args.format)
            generated_charts.append("resources")
        elif chart_type == "throughput-summary":
            generate_throughput_summary(data, args.output, args.format)
            generated_charts.append("throughput-summary")
        elif chart_type == "latency-summary":
            generate_latency_summary(data, args.output, args.format)
            generated_charts.append("latency-summary")

    if args.markdown is not None:
        if args.markdown == "-":
            table = generate_markdown_table(data)
            sys.stdout.write(table + "\n")
        else:
            md_path = Path(args.markdown)
            table = generate_markdown_table(data, md_path)
            print(f"Markdown table written to {md_path}")

    if "summary" in data:
        generate_metadata(data, args.output, generated_charts)


if __name__ == "__main__":
    main()
