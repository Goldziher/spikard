#!/usr/bin/env python3
"""Generate focused benchmark charts with language-based color coding."""

from __future__ import annotations

import argparse
import json
import sys
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

import numpy as np
import pandas as pd
import plotly.graph_objects as go
from plotly.subplots import make_subplots

# ---------------------------------------------------------------------------
# Language / color mapping
# ---------------------------------------------------------------------------

LANGUAGE_GROUPS: dict[str, tuple[str, int]] = {
    # (language, shade_index)
    "spikard-rust": ("Rust", 0),
    "spikard-python": ("Python", 0),
    "fastapi": ("Python", 1),
    "litestar": ("Python", 2),
    "robyn": ("Python", 3),
    "spikard-node": ("Node.js", 0),
    "fastify": ("Node.js", 1),
    "hono": ("Node.js", 2),
    "morojs": ("Node.js", 3),
    "kito": ("Node.js", 4),
    "spikard-bun": ("Bun", 0),
    "elysia": ("Bun", 1),
    "spikard-ruby": ("Ruby", 0),
    "hanami-api": ("Ruby", 1),
    "roda": ("Ruby", 2),
    "spikard-php": ("PHP", 0),
    "trongate": ("PHP", 1),
    "phalcon": ("PHP", 2),
}

LANGUAGE_BASE_COLORS: dict[str, str] = {
    "Rust": "#1565C0",
    "Python": "#2E7D32",
    "Node.js": "#E65100",
    "Bun": "#F57C00",
    "Ruby": "#C62828",
    "PHP": "#6A1B9A",
}

# Pre-computed shade variants per language
_LANGUAGE_SHADES: dict[str, list[str]] = {
    "Rust": ["#1565C0"],
    "Python": ["#1B5E20", "#2E7D32", "#43A047", "#66BB6A"],
    "Node.js": ["#BF360C", "#D84315", "#E65100", "#EF6C00", "#F57C00"],
    "Bun": ["#F57C00", "#FFA726"],
    "Ruby": ["#B71C1C", "#C62828", "#D32F2F"],
    "PHP": ["#4A148C", "#6A1B9A", "#7B1FA2"],
}

CATEGORY_ORDER = ["json-bodies", "path-params", "query-params", "urlencoded", "multipart"]
CATEGORY_LABELS = {
    "json-bodies": "JSON Bodies",
    "path-params": "Path Params",
    "query-params": "Query Params",
    "urlencoded": "URL-Encoded",
    "multipart": "Multipart",
}

# Workload prefix -> category
_PREFIX_TO_CATEGORY = {
    "json-": "json-bodies",
    "path-": "path-params",
    "query-": "query-params",
    "urlencoded-": "urlencoded",
    "multipart-": "multipart",
}

# JSON body sizes for scaling chart
_JSON_SIZE_ORDER = ["json-small", "json-medium", "json-large", "json-very-large"]
_JSON_SIZE_LABELS = {"json-small": "Small", "json-medium": "Medium", "json-large": "Large", "json-very-large": "V-Large"}

# Chart layout defaults
_LAYOUT_DEFAULTS: dict[str, Any] = {
    "template": "plotly_white",
    "font": {"family": "Inter, Roboto, sans-serif", "size": 12},
    "margin": {"t": 60, "b": 60, "l": 160, "r": 40},
}

_IMG_KWARGS: dict[str, Any] = {"width": 1200, "height": 800, "scale": 2}


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------


def _workload_category(name: str) -> str:
    for prefix, cat in _PREFIX_TO_CATEGORY.items():
        if name.startswith(prefix):
            return cat
    return "other"


def _fw_color(framework: str) -> str:
    lang, idx = LANGUAGE_GROUPS.get(framework, ("Unknown", 0))
    shades = _LANGUAGE_SHADES.get(lang, ["#9E9E9E"])
    return shades[min(idx, len(shades) - 1)]


def _fw_language(framework: str) -> str:
    lang, _ = LANGUAGE_GROUPS.get(framework, ("Unknown", 0))
    return lang


def _lang_sort_key(framework: str) -> tuple[int, str, str]:
    lang = _fw_language(framework)
    lang_order = list(LANGUAGE_BASE_COLORS.keys())
    try:
        idx = lang_order.index(lang)
    except ValueError:
        idx = len(lang_order)
    return (idx, lang, framework)


def _sorted_frameworks(frameworks: list[str]) -> list[str]:
    return sorted(frameworks, key=_lang_sort_key)


def load_aggregated_results(path: Path) -> dict[str, Any]:
    with path.open() as f:
        return json.load(f)


def extract_framework_data(data: dict[str, Any]) -> pd.DataFrame:
    rows = []
    for fw_result in data["frameworks"]:
        framework = fw_result["framework"]
        if fw_result["status"] != "completed":
            continue

        for suite in fw_result["profile"].get("suites", []):
            for workload in suite.get("workloads", []):
                metrics = workload.get("results", workload.get("metrics", {}))
                throughput = metrics.get("throughput", {})
                latency = metrics.get("latency", {})
                resources = metrics.get("resources", {})

                wname = workload["name"]
                is_validated = wname.startswith("validated/")
                base_workload = wname.removeprefix("validated/")

                rows.append({
                    "framework": framework,
                    "suite": suite["name"],
                    "workload": wname,
                    "base_workload": base_workload,
                    "is_validated": is_validated,
                    "category": _workload_category(base_workload),
                    "language": _fw_language(framework),
                    "color": _fw_color(framework),
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
                })

    return pd.DataFrame(rows)


def _write_figure(fig: go.Figure, output_dir: Path, name: str, fmt: str) -> list[Path]:
    paths: list[Path] = []
    formats = ["html", "svg", "png"] if fmt == "all" else [fmt]
    for f in formats:
        p = output_dir / f"{name}.{f}"
        if f == "html":
            fig.write_html(str(p))
        else:
            fig.write_image(str(p), format=f, **_IMG_KWARGS)
        paths.append(p)
    return paths


def _add_language_dividers(fig: go.Figure, frameworks: list[str], horizontal: bool = True) -> None:
    prev_lang = None
    for i, fw in enumerate(frameworks):
        lang = _fw_language(fw)
        if prev_lang is not None and lang != prev_lang:
            pos = i - 0.5
            if horizontal:
                fig.add_hline(y=pos, line_dash="dot", line_color="#BDBDBD", line_width=1)
            else:
                fig.add_vline(x=pos, line_dash="dot", line_color="#BDBDBD", line_width=1)
        prev_lang = lang


# ---------------------------------------------------------------------------
# Chart 01: Overall Throughput Leaderboard
# ---------------------------------------------------------------------------


def chart_throughput_leaderboard(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    if df.empty:
        return []

    summary = df.groupby("framework", as_index=False).agg(
        median_rps=("requests_per_sec", "median"),
        q25=("requests_per_sec", lambda x: x.quantile(0.25)),
        q75=("requests_per_sec", lambda x: x.quantile(0.75)),
    )

    fws = _sorted_frameworks(summary["framework"].tolist())
    summary = summary.set_index("framework").loc[fws].reset_index()

    fig = go.Figure()
    fig.add_trace(go.Bar(
        y=summary["framework"],
        x=summary["median_rps"],
        orientation="h",
        marker_color=[_fw_color(fw) for fw in summary["framework"]],
        error_x=dict(
            type="data",
            symmetric=False,
            array=(summary["q75"] - summary["median_rps"]).tolist(),
            arrayminus=(summary["median_rps"] - summary["q25"]).tolist(),
            color="#888",
            thickness=1.5,
        ),
        text=[f"{v:,.0f}" for v in summary["median_rps"]],
        textposition="outside",
    ))

    _add_language_dividers(fig, fws, horizontal=True)

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title="Overall Throughput (Median RPS, IQR error bars)",
        xaxis_title="Requests / second",
        height=max(500, len(fws) * 38 + 120),
        yaxis=dict(autorange="reversed"),
    )

    return _write_figure(fig, output_dir, "01-throughput-leaderboard", fmt)


# ---------------------------------------------------------------------------
# Chart 02: Throughput by Category
# ---------------------------------------------------------------------------


def chart_throughput_by_category(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    raw = df[~df["is_validated"]].copy()
    if raw.empty:
        return []

    cats = [c for c in CATEGORY_ORDER if c in raw["category"].unique()]
    n = len(cats)
    if n == 0:
        return []

    rows, cols = (2, 3) if n > 3 else (1, n)
    fig = make_subplots(
        rows=rows, cols=cols,
        subplot_titles=[CATEGORY_LABELS.get(c, c) for c in cats],
        horizontal_spacing=0.12,
        vertical_spacing=0.15,
    )

    fws = _sorted_frameworks(raw["framework"].unique().tolist())

    for i, cat in enumerate(cats):
        r, c = divmod(i, cols)
        r += 1
        c += 1
        cat_df = raw[raw["category"] == cat]
        cat_mean = cat_df.groupby("framework", as_index=False)["requests_per_sec"].mean()
        cat_mean = cat_mean.set_index("framework").reindex(fws).dropna().reset_index()
        cat_mean = cat_mean.sort_values("requests_per_sec", ascending=True)

        fig.add_trace(go.Bar(
            y=cat_mean["framework"],
            x=cat_mean["requests_per_sec"],
            orientation="h",
            marker_color=[_fw_color(fw) for fw in cat_mean["framework"]],
            text=[f"{v:,.0f}" for v in cat_mean["requests_per_sec"]],
            textposition="outside",
            showlegend=False,
        ), row=r, col=c)

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title="Throughput by Category (Raw Workloads)",
        height=900,
    )

    return _write_figure(fig, output_dir, "02-throughput-by-category", fmt)


# ---------------------------------------------------------------------------
# Chart 03: Raw vs Validated Throughput (side-by-side, no derived %)
# ---------------------------------------------------------------------------


def chart_raw_vs_validated(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    raw_df = df[~df["is_validated"]].groupby("framework", as_index=False)["requests_per_sec"].mean()
    val_df = df[df["is_validated"]].groupby("framework", as_index=False)["requests_per_sec"].mean()

    merged = raw_df.merge(val_df, on="framework", suffixes=("_raw", "_val"), how="inner")
    if merged.empty:
        return []

    fws = _sorted_frameworks(merged["framework"].tolist())
    merged = merged.set_index("framework").loc[fws].reset_index()

    fig = go.Figure()
    fig.add_trace(go.Bar(
        y=merged["framework"],
        x=merged["requests_per_sec_raw"],
        orientation="h",
        name="Raw",
        marker_color=[_fw_color(fw) for fw in merged["framework"]],
        text=[f"{v:,.0f}" for v in merged["requests_per_sec_raw"]],
        textposition="outside",
        opacity=1.0,
    ))
    fig.add_trace(go.Bar(
        y=merged["framework"],
        x=merged["requests_per_sec_val"],
        orientation="h",
        name="Validated",
        marker_color=[_fw_color(fw) for fw in merged["framework"]],
        marker_pattern_shape="/",
        text=[f"{v:,.0f}" for v in merged["requests_per_sec_val"]],
        textposition="outside",
        opacity=0.6,
    ))

    _add_language_dividers(fig, fws, horizontal=True)

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title="Raw vs Validated Throughput (Mean RPS)",
        xaxis_title="Requests / second",
        barmode="group",
        height=max(500, len(fws) * 45 + 120),
        yaxis=dict(autorange="reversed"),
        legend=dict(orientation="h", yanchor="bottom", y=1.02, xanchor="left", x=0),
    )

    return _write_figure(fig, output_dir, "03-raw-vs-validated", fmt)


# ---------------------------------------------------------------------------
# Chart 04: Latency Faceted by Language
# ---------------------------------------------------------------------------


def chart_latency_by_language(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    raw = df[~df["is_validated"]]
    if raw.empty:
        return []

    langs = [l for l in LANGUAGE_BASE_COLORS if l in raw["language"].unique()]
    n = len(langs)
    if n == 0:
        return []

    fig = make_subplots(
        rows=1, cols=n,
        subplot_titles=langs,
        horizontal_spacing=0.06,
    )

    percentiles = [
        ("latency_p50_ms", "P50"),
        ("latency_p90_ms", "P90"),
        ("latency_p95_ms", "P95"),
        ("latency_p99_ms", "P99"),
    ]
    p_colors = ["#1976D2", "#F57C00", "#E53935", "#4A148C"]

    for col_idx, lang in enumerate(langs, 1):
        lang_df = raw[raw["language"] == lang]
        fws = _sorted_frameworks(lang_df["framework"].unique().tolist())
        means = lang_df.groupby("framework", as_index=False)[[p[0] for p in percentiles]].mean()
        means = means.set_index("framework").reindex(fws).dropna().reset_index()

        for pi, (pcol, pname) in enumerate(percentiles):
            fig.add_trace(go.Bar(
                x=means["framework"],
                y=means[pcol],
                name=pname,
                marker_color=p_colors[pi],
                showlegend=(col_idx == 1),
                text=[f"{v:.1f}" for v in means[pcol]],
                textposition="outside",
                textfont=dict(size=9),
            ), row=1, col=col_idx)

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title="Latency Percentiles by Language Group",
        barmode="group",
        height=600,
        legend=dict(orientation="h", yanchor="bottom", y=1.05, xanchor="center", x=0.5),
    )
    for i in range(1, n + 1):
        fig.update_yaxes(title_text="Latency (ms)" if i == 1 else "", row=1, col=i)

    return _write_figure(fig, output_dir, "04-latency-by-language", fmt)


# ---------------------------------------------------------------------------
# Chart 05: P99 Latency Leaderboard
# ---------------------------------------------------------------------------


def chart_latency_p99_leaderboard(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    if df.empty:
        return []

    summary = df.groupby("framework", as_index=False)["latency_p99_ms"].mean()
    summary = summary.sort_values("latency_p99_ms", ascending=True)

    fig = go.Figure(go.Bar(
        y=summary["framework"],
        x=summary["latency_p99_ms"],
        orientation="h",
        marker_color=[_fw_color(fw) for fw in summary["framework"]],
        text=[f"{v:.2f} ms" for v in summary["latency_p99_ms"]],
        textposition="outside",
    ))

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title="P99 Latency Ranking",
        xaxis_title="Latency (ms)",
        xaxis_type="log",
        height=max(400, len(summary) * 35 + 120),
    )

    return _write_figure(fig, output_dir, "05-latency-p99-leaderboard", fmt)


# ---------------------------------------------------------------------------
# Chart 06: JSON Payload Scaling (all frameworks)
# ---------------------------------------------------------------------------


def chart_payload_scaling_all(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    raw = df[(~df["is_validated"]) & (df["category"] == "json-bodies")]
    if raw.empty:
        return []

    sizes = [s for s in _JSON_SIZE_ORDER if s in raw["base_workload"].unique()]
    if len(sizes) < 2:
        return []

    fws = _sorted_frameworks(raw["framework"].unique().tolist())
    fig = go.Figure()

    for fw in fws:
        fw_df = raw[raw["framework"] == fw]
        means = fw_df.groupby("base_workload", as_index=False)["requests_per_sec"].mean()
        means = means.set_index("base_workload").reindex(sizes).dropna()
        if means.empty:
            continue
        fig.add_trace(go.Scatter(
            x=[_JSON_SIZE_LABELS.get(s, s) for s in means.index],
            y=means["requests_per_sec"],
            mode="lines+markers",
            name=fw,
            line=dict(color=_fw_color(fw), width=2),
            marker=dict(size=7),
        ))

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title="JSON Throughput vs Body Size (All Frameworks)",
        xaxis_title="Body Size",
        yaxis_title="Requests / second",
        height=600,
        legend=dict(font=dict(size=10)),
    )

    return _write_figure(fig, output_dir, "06-payload-scaling-all", fmt)


# ---------------------------------------------------------------------------
# Chart 07: Payload Scaling (best per language)
# ---------------------------------------------------------------------------


def chart_payload_scaling_best(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    raw = df[(~df["is_validated"]) & (df["category"] == "json-bodies")]
    if raw.empty:
        return []

    lang_best: dict[str, str] = {}
    for lang in LANGUAGE_BASE_COLORS:
        lang_df = raw[raw["language"] == lang]
        if lang_df.empty:
            continue
        best = lang_df.groupby("framework")["requests_per_sec"].median().idxmax()
        lang_best[lang] = best

    sizes = [s for s in _JSON_SIZE_ORDER if s in raw["base_workload"].unique()]
    if len(sizes) < 2:
        return []

    fig = go.Figure()
    for lang, fw in lang_best.items():
        fw_df = raw[raw["framework"] == fw]
        means = fw_df.groupby("base_workload", as_index=False)["requests_per_sec"].mean()
        means = means.set_index("base_workload").reindex(sizes).dropna()
        if means.empty:
            continue
        fig.add_trace(go.Scatter(
            x=[_JSON_SIZE_LABELS.get(s, s) for s in means.index],
            y=means["requests_per_sec"],
            mode="lines+markers",
            name=f"{fw} ({lang})",
            line=dict(color=LANGUAGE_BASE_COLORS[lang], width=3),
            marker=dict(size=9),
        ))

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title="JSON Throughput vs Body Size (Best per Language)",
        xaxis_title="Body Size",
        yaxis_title="Requests / second",
        height=600,
    )

    return _write_figure(fig, output_dir, "07-payload-scaling-best", fmt)


# ---------------------------------------------------------------------------
# Chart 08: Resource Efficiency Scatter
# ---------------------------------------------------------------------------


def chart_resource_efficiency(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    if df.empty:
        return []

    summary = df.groupby("framework", as_index=False).agg(
        mean_rps=("requests_per_sec", "mean"),
        peak_mem=("memory_peak_mb", "mean"),
        avg_cpu=("cpu_avg_percent", "mean"),
    )

    cpu_vals = summary["avg_cpu"]
    if cpu_vals.max() > cpu_vals.min():
        norm = (cpu_vals - cpu_vals.min()) / (cpu_vals.max() - cpu_vals.min())
    else:
        norm = pd.Series([0.5] * len(cpu_vals))
    sizes = 10 + norm * 30

    fig = go.Figure()
    for i, row in summary.iterrows():
        fw = row["framework"]
        fig.add_trace(go.Scatter(
            x=[row["peak_mem"]],
            y=[row["mean_rps"]],
            mode="markers+text",
            marker=dict(size=sizes.iloc[i], color=_fw_color(fw), opacity=0.8),  # type: ignore[arg-type]
            text=[fw],
            textposition="top center",
            textfont=dict(size=10),
            name=f"{fw} (CPU {row['avg_cpu']:.0f}%)",
            showlegend=True,
        ))

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title="Throughput vs Memory (point size = CPU%)",
        xaxis_title="Peak Memory (MB)",
        yaxis_title="Mean RPS",
        height=700,
        legend=dict(font=dict(size=9)),
    )

    return _write_figure(fig, output_dir, "08-resource-efficiency", fmt)


# ---------------------------------------------------------------------------
# Chart 09: Resource Usage Bars
# ---------------------------------------------------------------------------


def chart_resource_usage(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    if df.empty:
        return []

    summary = df.groupby("framework", as_index=False).agg(
        peak_mem=("memory_peak_mb", "mean"),
        avg_cpu=("cpu_avg_percent", "mean"),
    )
    fws = _sorted_frameworks(summary["framework"].tolist())
    summary = summary.set_index("framework").loc[fws].reset_index()

    fig = make_subplots(
        rows=1, cols=2,
        subplot_titles=["Peak Memory (MB)", "Avg CPU (%)"],
        horizontal_spacing=0.15,
    )

    fig.add_trace(go.Bar(
        y=summary["framework"],
        x=summary["peak_mem"],
        orientation="h",
        marker_color=[_fw_color(fw) for fw in summary["framework"]],
        text=[f"{v:.0f}" for v in summary["peak_mem"]],
        textposition="outside",
        showlegend=False,
    ), row=1, col=1)

    fig.add_trace(go.Bar(
        y=summary["framework"],
        x=summary["avg_cpu"],
        orientation="h",
        marker_color=[_fw_color(fw) for fw in summary["framework"]],
        text=[f"{v:.0f}%" for v in summary["avg_cpu"]],
        textposition="outside",
        showlegend=False,
    ), row=1, col=2)

    _add_language_dividers(fig, fws, horizontal=True)

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title="Resource Usage by Framework",
        height=max(500, len(fws) * 38 + 120),
    )
    fig.update_yaxes(autorange="reversed", row=1, col=1)
    fig.update_yaxes(autorange="reversed", row=1, col=2)

    return _write_figure(fig, output_dir, "09-resource-usage", fmt)


# ---------------------------------------------------------------------------
# Chart 10: Throughput Heatmap
# ---------------------------------------------------------------------------


def chart_throughput_heatmap(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    raw = df[~df["is_validated"]]
    if raw.empty:
        return []

    pivot = raw.groupby(["framework", "category"])["requests_per_sec"].mean().unstack(fill_value=0)
    cats = [c for c in CATEGORY_ORDER if c in pivot.columns]
    if not cats:
        return []

    pivot = pivot[cats]
    fws = _sorted_frameworks(pivot.index.tolist())
    pivot = pivot.reindex(fws)

    fig = go.Figure(go.Heatmap(
        z=pivot.values,
        x=[CATEGORY_LABELS.get(c, c) for c in cats],
        y=pivot.index.tolist(),
        colorscale="Blues",
        text=[[f"{v:,.0f}" for v in row] for row in pivot.values],
        texttemplate="%{text}",
        textfont=dict(size=10),
        colorbar=dict(title="RPS"),
    ))

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title="Throughput Heatmap (Mean RPS by Category)",
        height=max(500, len(fws) * 35 + 120),
        yaxis=dict(autorange="reversed"),
    )

    return _write_figure(fig, output_dir, "10-throughput-heatmap", fmt)


# ---------------------------------------------------------------------------
# Markdown table
# ---------------------------------------------------------------------------


def generate_markdown_table(data: dict[str, Any], output_file: Path | None = None) -> str:
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


# ---------------------------------------------------------------------------
# Metadata
# ---------------------------------------------------------------------------


def generate_metadata(data: dict[str, Any], output_dir: Path, charts: list[str]) -> None:
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
        "charts": charts,
    }
    with (output_dir / "metadata.json").open("w") as f:
        json.dump(metadata, f, indent=2)


# ---------------------------------------------------------------------------
# Chart registry
# ---------------------------------------------------------------------------

ALL_CHARTS = [
    "throughput-leaderboard",
    "throughput-by-category",
    "raw-vs-validated",
    "latency-by-language",
    "latency-p99-leaderboard",
    "payload-scaling-all",
    "payload-scaling-best",
    "resource-efficiency",
    "resource-usage",
    "throughput-heatmap",
]

_CHART_FUNCS: dict[str, Any] = {
    "throughput-leaderboard": chart_throughput_leaderboard,
    "throughput-by-category": chart_throughput_by_category,
    "raw-vs-validated": chart_raw_vs_validated,
    "latency-by-language": chart_latency_by_language,
    "latency-p99-leaderboard": chart_latency_p99_leaderboard,
    "payload-scaling-all": chart_payload_scaling_all,
    "payload-scaling-best": chart_payload_scaling_best,
    "resource-efficiency": chart_resource_efficiency,
    "resource-usage": chart_resource_usage,
    "throughput-heatmap": chart_throughput_heatmap,
}


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------


def main() -> None:
    parser = argparse.ArgumentParser(description="Generate benchmark charts with language-based color coding")
    parser.add_argument("--input", type=Path, required=True, help="Path to aggregated.json")
    parser.add_argument("--output", type=Path, required=True, help="Output directory for charts")
    parser.add_argument(
        "--charts",
        default="all",
        help=f"Comma-separated chart names: {','.join(ALL_CHARTS)},all",
    )
    parser.add_argument("--title", default="Benchmark Results", help="Title prefix (unused, kept for compat)")
    parser.add_argument(
        "--format",
        default="html",
        choices=["html", "svg", "png", "all"],
        help="Output format",
    )
    parser.add_argument(
        "--markdown",
        nargs="?",
        const="-",
        default=None,
        help="Output markdown table to file (or stdout if '-')",
    )
    args = parser.parse_args()

    data = load_aggregated_results(args.input)
    args.output.mkdir(parents=True, exist_ok=True)

    chart_names = [c.strip() for c in args.charts.split(",")]
    if "all" in chart_names:
        chart_names = list(ALL_CHARTS)

    df = extract_framework_data(data)
    generated: list[str] = []

    for name in chart_names:
        func = _CHART_FUNCS.get(name)
        if func is None:
            print(f"Warning: unknown chart '{name}', skipping", file=sys.stderr)
            continue
        paths = func(df, args.output, args.format)
        if paths:
            generated.append(name)
            print(f"  Generated: {paths[0].name}")

    if args.markdown is not None:
        if args.markdown == "-":
            sys.stdout.write(generate_markdown_table(data) + "\n")
        else:
            generate_markdown_table(data, Path(args.markdown))

    if "summary" in data:
        generate_metadata(data, args.output, generated)

    print(f"\nDone: {len(generated)} charts generated in {args.output}")


if __name__ == "__main__":
    main()
