#!/usr/bin/env python3
"""Generate interactive Plotly HTML charts from aggregated benchmark results."""

from __future__ import annotations

import argparse
import json
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

import pandas as pd
import plotly.express as px
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
                metrics = workload.get("metrics", {})
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
                    "latency_p50_ms": latency.get("p50_ms", 0),
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


def generate_throughput_chart(
    data: dict[str, Any], output_dir: Path, title: str
) -> Path:
    """Generate throughput comparison chart."""
    df = extract_framework_data(data)

    if df.empty:
        print("No data available for throughput chart")
        return output_dir / "throughput-by-framework.html"

    json_workloads = df[df["suite"] == "json-bodies"]

    if json_workloads.empty:
        print("Warning: No json-bodies workload data found")
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

    output_path = output_dir / "throughput-by-framework.html"
    fig.write_html(str(output_path))
    print(f"✅ Generated: {output_path.name}")
    return output_path


def generate_latency_chart(
    data: dict[str, Any], output_dir: Path, title: str
) -> Path:
    """Generate latency percentile comparison (2x2 subplot)."""
    df = extract_framework_data(data)

    if df.empty:
        print("No data available for latency chart")
        return output_dir / "latency-percentiles.html"

    small_json = df[
        (df["suite"] == "json-bodies") & (df["workload"].str.contains("small"))
    ]

    if small_json.empty:
        print("Warning: No small JSON workload found, using all data")
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

    output_path = output_dir / "latency-percentiles.html"
    fig.write_html(str(output_path))
    print(f"✅ Generated: {output_path.name}")
    return output_path


def generate_validation_overhead_chart(
    data: dict[str, Any], output_dir: Path, title: str
) -> Path:
    """Generate validation vs raw comparison chart."""
    df = extract_framework_data(data)

    if df.empty:
        print("No data available for validation overhead chart")
        return output_dir / "validation-overhead.html"

    df["base_framework"] = df["framework"].str.replace(
        "-validation|-raw", "", regex=True
    )
    df["variant"] = df["framework"].str.extract(r"(validation|raw)$")[0]

    json_small = df[
        (df["suite"] == "json-bodies") & (df["workload"].str.contains("small"))
    ]

    if json_small.empty:
        json_small = df

    paired = json_small[json_small["variant"].notna()]

    pivot = paired.pivot_table(
        index="base_framework",
        columns="variant",
        values="requests_per_sec",
        aggfunc="mean",
    ).dropna()

    if "validation" not in pivot.columns or "raw" not in pivot.columns:
        print("Warning: Not enough validation/raw pairs found")
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
        pivot["overhead_pct"] = (
            (pivot["raw"] - pivot["validation"]) / pivot["raw"]
        ) * 100

        fig = go.Figure()

        colors = get_color_scheme()

        for idx, fw in enumerate(pivot.index):
            fig.add_trace(
                go.Bar(
                    name=f"{fw}-validation",
                    x=[fw],
                    y=[pivot.loc[fw, "validation"]],
                    marker_color=colors[idx * 2 % len(colors)],
                    text=f"{pivot.loc[fw, 'validation']:.0f}",
                    textposition="outside",
                    showlegend=False,
                )
            )
            fig.add_trace(
                go.Bar(
                    name=f"{fw}-raw",
                    x=[fw],
                    y=[pivot.loc[fw, "raw"]],
                    marker_color=colors[(idx * 2 + 1) % len(colors)],
                    text=f"{pivot.loc[fw, 'raw']:.0f}",
                    textposition="outside",
                    showlegend=False,
                )
            )

            overhead = pivot.loc[fw, "overhead_pct"]
            fig.add_annotation(
                x=fw,
                y=max(pivot.loc[fw, "validation"], pivot.loc[fw, "raw"]),
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

    fig.update_layout(
        title=f"{title} - Validation Overhead",
        xaxis_title="Framework",
        yaxis_title="Requests / second",
        barmode="group",
        template="plotly_white",
        height=600,
        font={"family": "Roboto, sans-serif", "size": 12},
    )

    output_path = output_dir / "validation-overhead.html"
    fig.write_html(str(output_path))
    print(f"✅ Generated: {output_path.name}")
    return output_path


def generate_resource_chart(
    data: dict[str, Any], output_dir: Path, title: str
) -> Path:
    """Generate resource utilization chart (dual-axis: memory bars + CPU line)."""
    df = extract_framework_data(data)

    if df.empty:
        print("No data available for resource chart")
        return output_dir / "resources.html"

    resources = (
        df.groupby("framework", as_index=False)[
            ["memory_peak_mb", "cpu_avg_percent"]
        ]
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

    output_path = output_dir / "resources.html"
    fig.write_html(str(output_path))
    print(f"✅ Generated: {output_path.name}")
    return output_path


def generate_metadata(data: dict[str, Any], output_dir: Path) -> None:
    """Generate metadata.json for dynamic injection into docs."""
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
        "charts": [
            "throughput-by-framework.html",
            "latency-percentiles.html",
            "validation-overhead.html",
            "resources.html",
        ],
    }

    output_path = output_dir / "metadata.json"
    with output_path.open("w") as f:
        json.dump(metadata, f, indent=2)

    print(f"✅ Generated: {output_path.name}")


def main() -> None:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Generate Plotly HTML charts from aggregated benchmark results"
    )
    parser.add_argument(
        "--input", type=Path, required=True, help="Path to aggregated.json"
    )
    parser.add_argument(
        "--output", type=Path, required=True, help="Output directory for charts"
    )
    parser.add_argument(
        "--charts",
        default="all",
        help="Comma-separated chart types: throughput,latency,validation,resources,all",
    )
    parser.add_argument(
        "--title", default="Benchmark Results", help="Title prefix for charts"
    )
    args = parser.parse_args()

    print(f"📖 Loading aggregated results from {args.input}...")
    data = load_aggregated_results(args.input)
    print(
        f"✅ Loaded {data['summary']['total_frameworks']} frameworks "
        f"({data['summary']['completed']} completed)"
    )

    args.output.mkdir(parents=True, exist_ok=True)

    chart_types = [c.strip() for c in args.charts.split(",")]
    if "all" in chart_types:
        chart_types = ["throughput", "latency", "validation", "resources"]

    print(f"\n📊 Generating {len(chart_types)} chart(s)...")

    for chart_type in chart_types:
        if chart_type == "throughput":
            generate_throughput_chart(data, args.output, args.title)
        elif chart_type == "latency":
            generate_latency_chart(data, args.output, args.title)
        elif chart_type == "validation":
            generate_validation_overhead_chart(data, args.output, args.title)
        elif chart_type == "resources":
            generate_resource_chart(data, args.output, args.title)
        else:
            print(f"⚠️  Unknown chart type: {chart_type}")

    print("\n📝 Generating metadata...")
    generate_metadata(data, args.output)

    print(f"\n✅ All charts generated in {args.output}")


if __name__ == "__main__":
    main()
