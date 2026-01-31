#!/usr/bin/env python3
"""Generate focused benchmark charts with dark theme and language-based color coding."""

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

# ---------------------------------------------------------------------------
# Language / color mapping
# ---------------------------------------------------------------------------

LANGUAGE_GROUPS: dict[str, tuple[str, int]] = {
    # language, shade_index
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

# Dark-theme-friendly colors (brighter, higher saturation)
LANGUAGE_BASE_COLORS: dict[str, str] = {
    "Rust": "#42A5F5",
    "Python": "#66BB6A",
    "Node.js": "#FF8A65",
    "Bun": "#FFB74D",
    "Ruby": "#EF5350",
    "PHP": "#AB47BC",
}

_LANGUAGE_SHADES: dict[str, list[str]] = {
    "Rust": ["#42A5F5"],
    "Python": ["#43A047", "#66BB6A", "#81C784", "#A5D6A7"],
    "Node.js": ["#E64A19", "#F4511E", "#FF5722", "#FF7043", "#FF8A65"],
    "Bun": ["#FFB74D", "#FFCC80"],
    "Ruby": ["#E53935", "#EF5350", "#EF9A9A"],
    "PHP": ["#8E24AA", "#AB47BC", "#CE93D8"],
}

# Language badge colors (for annotations)
_LANG_BADGE_COLORS: dict[str, str] = {
    "Rust": "#1E3A5F",
    "Python": "#1B3A1E",
    "Node.js": "#3E2013",
    "Bun": "#3E3013",
    "Ruby": "#3E1313",
    "PHP": "#2E1340",
}

CATEGORY_ORDER = ["json-bodies", "path-params", "query-params", "urlencoded", "multipart"]
CATEGORY_LABELS = {
    "json-bodies": "JSON Bodies",
    "path-params": "Path Params",
    "query-params": "Query Params",
    "urlencoded": "URL-Encoded",
    "multipart": "Multipart",
}

_PREFIX_TO_CATEGORY = {
    "json-": "json-bodies",
    "path-": "path-params",
    "query-": "query-params",
    "urlencoded-": "urlencoded",
    "multipart-": "multipart",
}

_JSON_SIZE_ORDER = ["json-small", "json-medium", "json-large", "json-very-large"]
_JSON_SIZE_LABELS = {
    "json-small": "Small",
    "json-medium": "Medium",
    "json-large": "Large",
    "json-very-large": "V-Large",
}

# Dark theme layout defaults
_LAYOUT_DEFAULTS: dict[str, Any] = {
    "template": "plotly_dark",
    "font": {"family": "Inter, SF Mono, monospace", "size": 13, "color": "#E0E0E0"},
    "paper_bgcolor": "#0D1117",
    "plot_bgcolor": "#161B22",
    "margin": {"t": 70, "b": 60, "l": 170, "r": 50},
}

_IMG_KWARGS: dict[str, Any] = {"width": 1400, "height": 900, "scale": 2}


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------


def _workload_category(name: str) -> str:
    """Return the benchmark category for a workload name."""
    for prefix, cat in _PREFIX_TO_CATEGORY.items():
        if name.startswith(prefix):
            return cat
    return "other"


def _fw_color(framework: str) -> str:
    """Return the color for a framework based on its language group."""
    lang, idx = LANGUAGE_GROUPS.get(framework, ("Unknown", 0))
    shades = _LANGUAGE_SHADES.get(lang, ["#9E9E9E"])
    return shades[min(idx, len(shades) - 1)]


def _fw_language(framework: str) -> str:
    """Return the language group name for a framework."""
    lang, _ = LANGUAGE_GROUPS.get(framework, ("Unknown", 0))
    return lang


def _is_spikard(framework: str) -> bool:
    """Return True if framework is a spikard variant."""
    return framework.startswith("spikard-")


def _fw_label(framework: str) -> str:
    """Return display label with language tag, e.g. 'fastapi (Python)'.

    For spikard variants, strip the language suffix since the tag already
    indicates it: 'spikard-rust' -> 'spikard (Rust)'.
    """
    lang = _fw_language(framework)
    if _is_spikard(framework):
        return f"spikard ({lang})"
    return f"{framework} ({lang})"


def _lang_sort_key(framework: str) -> tuple[int, str, str]:
    """Return a sort key tuple for ordering frameworks by language group."""
    lang = _fw_language(framework)
    lang_order = list(LANGUAGE_BASE_COLORS.keys())
    try:
        idx = lang_order.index(lang)
    except ValueError:
        idx = len(lang_order)
    return (idx, lang, framework)


def _sorted_frameworks(frameworks: list[str]) -> list[str]:
    """Sort frameworks by language group then name."""
    return sorted(frameworks, key=_lang_sort_key)


def load_aggregated_results(path: Path) -> dict[str, Any]:
    """Load aggregated benchmark results from a JSON file."""
    with path.open() as f:
        return json.load(f)


def extract_framework_data(data: dict[str, Any]) -> pd.DataFrame:
    """Extract framework benchmark data into a flat DataFrame."""
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

                rows.append(
                    {
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
                    }
                )

    return pd.DataFrame(rows)


def _write_figure(fig: go.Figure, output_dir: Path, name: str, fmt: str, **overrides: int) -> list[Path]:
    """Write a Plotly figure to disk in the requested format(s)."""
    paths: list[Path] = []
    formats = ["html", "svg", "png"] if fmt == "all" else [fmt]
    img_kwargs = {**_IMG_KWARGS, **overrides}
    for f in formats:
        p = output_dir / f"{name}.{f}"
        if f == "html":
            fig.write_html(str(p))
        else:
            fig.write_image(str(p), format=f, **img_kwargs)
        paths.append(p)
    return paths


def _add_language_dividers(fig: go.Figure, frameworks: list[str], horizontal: bool = True) -> None:
    """Add subtle divider lines between language groups on a chart."""
    prev_lang = None
    for i, fw in enumerate(frameworks):
        lang = _fw_language(fw)
        if prev_lang is not None and lang != prev_lang:
            pos = i - 0.5
            if horizontal:
                fig.add_hline(y=pos, line_dash="dot", line_color="#333", line_width=1)
            else:
                fig.add_vline(x=pos, line_dash="dot", line_color="#333", line_width=1)
        prev_lang = lang


# ---------------------------------------------------------------------------
# Chart 01: Throughput Leaderboard
# ---------------------------------------------------------------------------


def chart_throughput_leaderboard(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    """Horizontal bar chart sorted by RPS descending (highest first)."""
    if df.empty:
        return []

    summary = df.groupby("framework", as_index=False).agg(
        median_rps=("requests_per_sec", "median"),
        q25=("requests_per_sec", lambda x: x.quantile(0.25)),
        q75=("requests_per_sec", lambda x: x.quantile(0.75)),
    )

    # Sort ascending so plotly renders highest at top
    summary = summary.sort_values("median_rps", ascending=True)
    fws = summary["framework"].tolist()
    labels = [_fw_label(fw) for fw in fws]

    fig = go.Figure()
    fig.add_trace(
        go.Bar(
            y=labels,
            x=summary["median_rps"],
            orientation="h",
            marker_color=[_fw_color(fw) for fw in fws],
            marker_line_color=["white" if _is_spikard(fw) else "rgba(0,0,0,0)" for fw in fws],
            marker_line_width=[1.5 if _is_spikard(fw) else 0 for fw in fws],
            error_x={
                "type": "data",
                "symmetric": False,
                "array": (summary["q75"] - summary["median_rps"]).tolist(),
                "arrayminus": (summary["median_rps"] - summary["q25"]).tolist(),
                "color": "#555",
                "thickness": 1,
            },
            text=[f"{v:,.0f}" for v in summary["median_rps"]],
            textposition="outside",
            textfont={"color": "#E0E0E0"},
        )
    )

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title={"text": "Overall Throughput Leaderboard", "font": {"size": 18}},
        xaxis_title="Requests / second",
        height=max(500, len(fws) * 38 + 120),
        showlegend=False,
    )

    return _write_figure(fig, output_dir, "01-throughput-leaderboard", fmt)


# ---------------------------------------------------------------------------
# Chart 02: Latency Distribution (P50/P90/P99 grouped bars, log scale)
# ---------------------------------------------------------------------------


def chart_latency_distribution(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    """Grouped bar chart of P50/P90/P99 across all frameworks, log y-axis."""
    raw = df[~df["is_validated"]]
    if raw.empty:
        return []

    summary = raw.groupby("framework", as_index=False).agg(
        p50=("latency_p50_ms", "mean"),
        p90=("latency_p90_ms", "mean"),
        p99=("latency_p99_ms", "mean"),
    )

    # Sort by P50 ascending (fastest first)
    summary = summary.sort_values("p50", ascending=True)
    fws = summary["framework"].tolist()
    labels = [_fw_label(fw) for fw in fws]

    percentiles = [("p50", "P50", "#42A5F5"), ("p90", "P90", "#FFB74D"), ("p99", "P99", "#EF5350")]

    fig = go.Figure()
    for col, label, color in percentiles:
        fig.add_trace(
            go.Bar(
                x=labels,
                y=summary[col],
                name=label,
                marker_color=color,
                marker_line_color=[
                    "white" if _is_spikard(fw) else "rgba(0,0,0,0)" for fw in fws
                ],
                marker_line_width=[1 if _is_spikard(fw) else 0 for fw in fws],
                text=[f"{v:.1f}" for v in summary[col]],
                textposition="outside",
                textfont={"size": 9, "color": "#AAA"},
            )
        )

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title={"text": "Latency Distribution (P50 / P90 / P99)", "font": {"size": 18}},
        yaxis_title="Latency (ms)",
        yaxis_type="log",
        barmode="group",
        height=700,
        legend={"orientation": "h", "yanchor": "bottom", "y": 1.02, "xanchor": "center", "x": 0.5},
        xaxis={"tickangle": -45},
    )

    return _write_figure(fig, output_dir, "02-latency-distribution", fmt)


# ---------------------------------------------------------------------------
# Chart 03: Category Breakdown Heatmap
# ---------------------------------------------------------------------------


def chart_category_heatmap(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    """Heatmap of RPS by framework (y) and category (x), dark bg, viridis."""
    raw = df[~df["is_validated"]]
    if raw.empty:
        return []

    pivot = raw.pivot_table(
        index="framework", columns="category", values="requests_per_sec", aggfunc="mean",
    )
    cats = [c for c in CATEGORY_ORDER if c in pivot.columns]
    if not cats:
        return []

    pivot = pivot[cats]
    # Sort by overall mean RPS (ascending so highest ends up at top in plotly)
    pivot["_mean"] = pivot.mean(axis=1)
    pivot = pivot.sort_values("_mean", ascending=True)
    pivot = pivot.drop(columns=["_mean"])
    fws = pivot.index.tolist()

    # Replace NaN with None for distinct rendering
    z_values = pivot.values.tolist()
    text_values = []
    for row in z_values:
        text_row = []
        for v in row:
            if pd.isna(v):
                text_row.append("")
            else:
                text_row.append(f"{v:,.0f}")
        text_values.append(text_row)

    # Replace NaN with 0 for color but mark missing
    z_clean = []
    for row in z_values:
        z_clean.append([0 if pd.isna(v) else v for v in row])

    labels = [_fw_label(fw) for fw in fws]

    fig = go.Figure(
        go.Heatmap(
            z=z_clean,
            x=[CATEGORY_LABELS.get(c, c) for c in cats],
            y=labels,
            colorscale="Viridis",
            text=text_values,
            texttemplate="%{text}",
            textfont={"size": 11, "color": "#E0E0E0"},
            colorbar={"title": {"text": "RPS", "font": {"color": "#AAA"}}, "tickfont": {"color": "#AAA"}},
            hoverongaps=False,
        )
    )

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title={"text": "Throughput by Category (Mean RPS)", "font": {"size": 18}},
        height=max(500, len(fws) * 35 + 120),
    )

    return _write_figure(fig, output_dir, "03-category-heatmap", fmt)


# ---------------------------------------------------------------------------
# Chart 04: Raw vs Validated Throughput
# ---------------------------------------------------------------------------


def chart_raw_vs_validated(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    """Horizontal grouped bars: solid=raw, semi-transparent=validated, overhead annotation."""
    raw_df = df[~df["is_validated"]].groupby("framework", as_index=False)["requests_per_sec"].mean()
    val_df = df[df["is_validated"]].groupby("framework", as_index=False)["requests_per_sec"].mean()

    merged = raw_df.merge(val_df, on="framework", suffixes=("_raw", "_val"), how="inner")
    if merged.empty:
        return []

    merged["overhead_pct"] = (
        (merged["requests_per_sec_raw"] - merged["requests_per_sec_val"])
        / merged["requests_per_sec_raw"]
        * 100
    )

    # Sort by raw RPS ascending (plotly renders bottom-up, so highest ends at top)
    merged = merged.sort_values("requests_per_sec_raw", ascending=True)
    fws = merged["framework"].tolist()
    labels = [_fw_label(fw) for fw in fws]

    # Validated bars first (renders below in grouped mode)
    val_colors = []
    for fw in fws:
        base = _fw_color(fw)
        r, g, b = int(base[1:3], 16), int(base[3:5], 16), int(base[5:7], 16)
        val_colors.append(f"rgba({r},{g},{b},0.45)")

    fig = go.Figure()
    fig.add_trace(
        go.Bar(
            y=labels,
            x=merged["requests_per_sec_val"],
            orientation="h",
            name="Validated",
            marker_color=val_colors,
            marker_line_color=["white" if _is_spikard(fw) else "rgba(0,0,0,0)" for fw in fws],
            marker_line_width=[1 if _is_spikard(fw) else 0 for fw in fws],
            text=[f"Validated · {v:,.0f}" for v in merged["requests_per_sec_val"]],
            textposition="inside",
            insidetextanchor="start",
            textfont={"color": "white", "size": 10},
            showlegend=False,
        )
    )
    # Raw bars second (renders on top in grouped mode)
    fig.add_trace(
        go.Bar(
            y=labels,
            x=merged["requests_per_sec_raw"],
            orientation="h",
            name="Raw",
            marker_color=[_fw_color(fw) for fw in fws],
            marker_line_color=["white" if _is_spikard(fw) else "rgba(0,0,0,0)" for fw in fws],
            marker_line_width=[1.5 if _is_spikard(fw) else 0 for fw in fws],
            text=[f"Raw · {v:,.0f}" for v in merged["requests_per_sec_raw"]],
            textposition="inside",
            insidetextanchor="start",
            textfont={"color": "white", "size": 10},
            showlegend=False,
        )
    )

    # Add overhead % annotations on right side
    max_rps = merged["requests_per_sec_raw"].max()
    for idx, (_, row) in enumerate(merged.iterrows()):
        pct = row["overhead_pct"]
        if pct >= 0:
            ann_label = f"{pct:.0f}% overhead"
            ann_color = "#AAA"
        else:
            ann_label = f"+{-pct:.0f}% validated"
            ann_color = "#66BB6A"
        fig.add_annotation(
            x=max_rps * 1.12,
            y=labels[idx],
            text=ann_label,
            showarrow=False,
            font={"size": 10, "color": ann_color},
            xanchor="left",
        )

    layout_overrides = {**_LAYOUT_DEFAULTS, "margin": {"t": 60, "b": 60, "l": 210, "r": 140}}
    fig.update_layout(
        **layout_overrides,
        title={"text": "Raw vs Validated Throughput", "font": {"size": 18}},
        xaxis_title="Requests / second",
        barmode="group",
        bargap=0.25,
        height=max(600, len(fws) * 55 + 120),
    )

    return _write_figure(fig, output_dir, "04-raw-vs-validated", fmt)


# ---------------------------------------------------------------------------
# Chart 05: Payload Scaling (best per language + all spikard)
# ---------------------------------------------------------------------------


def chart_payload_scaling(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    """Faceted small-multiples: one panel per language, shared y-axis."""
    raw = df[(~df["is_validated"]) & (df["category"] == "json-bodies")]
    if raw.empty:
        return []

    sizes = [s for s in _JSON_SIZE_ORDER if s in raw["base_workload"].unique()]
    if len(sizes) < 2:
        return []

    x_labels = [_JSON_SIZE_LABELS.get(s, s) for s in sizes]

    # Find which languages have data
    langs = [lang for lang in LANGUAGE_BASE_COLORS if lang in raw["language"].unique()]
    n = len(langs)
    if n == 0:
        return []

    # Compute shared y-axis range across all data
    y_max = raw["requests_per_sec"].max() * 1.1

    cols = min(n, 3)
    rows = (n + cols - 1) // cols
    fig = make_subplots(
        rows=rows,
        cols=cols,
        subplot_titles=langs,
        shared_yaxes=True,
        horizontal_spacing=0.15,
        vertical_spacing=0.12,
    )

    # Style subplot titles with language colors
    for i, ann in enumerate(fig.layout.annotations):
        if i < len(langs):
            ann.font = {"size": 14, "color": LANGUAGE_BASE_COLORS.get(langs[i], "#AAA")}

    annotations = []

    for lang_idx, lang in enumerate(langs):
        r = lang_idx // cols + 1
        c = lang_idx % cols + 1
        lang_df = raw[raw["language"] == lang]
        fws = sorted(lang_df["framework"].unique())

        # Collect endpoint values for annotation positioning
        endpoint_data: list[tuple[float, str, str, bool]] = []  # (rps, label, color, is_sp)

        for fw in fws:
            fw_df = lang_df[lang_df["framework"] == fw]
            means = fw_df.groupby("base_workload", as_index=False)["requests_per_sec"].mean()
            means = means.set_index("base_workload").reindex(sizes).dropna()
            if means.empty:
                continue

            is_sp = _is_spikard(fw)
            color = _fw_color(fw)
            label = _fw_label(fw)
            last_rps = means["requests_per_sec"].iloc[-1]
            endpoint_data.append((last_rps, label, color, is_sp))

            fig.add_trace(
                go.Scatter(
                    x=x_labels,
                    y=means["requests_per_sec"],
                    mode="lines+markers",
                    name=label,
                    line={"color": color, "width": 3 if is_sp else 2},
                    marker={
                        "size": 9 if is_sp else 6,
                        "line": {"color": "white", "width": 2} if is_sp else {"width": 0},
                    },
                    showlegend=False,
                    legendgroup=lang,
                ),
                row=r,
                col=c,
            )

        # Add annotations at the right edge, staggered to avoid overlap
        # Determine axis refs for this subplot
        ax_suffix = "" if (lang_idx == 0) else str(lang_idx + 1)
        xref = f"x{ax_suffix}"
        yref = f"y{ax_suffix}"

        # Sort by RPS and push apart labels that are too close
        endpoint_data.sort(key=lambda t: t[0])
        min_gap = y_max * 0.06  # minimum vertical gap between labels
        adjusted_y = [t[0] for t in endpoint_data]
        for i in range(1, len(adjusted_y)):
            if adjusted_y[i] - adjusted_y[i - 1] < min_gap:
                adjusted_y[i] = adjusted_y[i - 1] + min_gap

        for i, (rps, label, color, _is_sp) in enumerate(endpoint_data):
            annotations.append(
                dict(
                    x=x_labels[-1],
                    y=adjusted_y[i],
                    xref=xref,
                    yref=yref,
                    text=f"  {label}  {rps:,.0f}",
                    showarrow=False,
                    font=dict(size=10, color=color),
                    xanchor="left",
                    yanchor="middle",
                )
            )

    # Unified y-axis range
    for r_idx in range(1, rows + 1):
        for c_idx in range(1, cols + 1):
            fig.update_yaxes(range=[0, y_max], row=r_idx, col=c_idx)
            if c_idx == 1:
                fig.update_yaxes(title_text="RPS" if r_idx == 1 else "", row=r_idx, col=c_idx)

    # Merge endpoint annotations with subplot title annotations
    existing = list(fig.layout.annotations)
    fig.update_layout(annotations=existing + annotations)

    layout_overrides = {**_LAYOUT_DEFAULTS, "margin": {"t": 70, "b": 60, "l": 80, "r": 180}}
    fig.update_layout(
        **layout_overrides,
        title={"text": "JSON Throughput vs Payload Size", "font": {"size": 18}},
        height=350 * rows + 80,
        showlegend=False,
    )

    return _write_figure(fig, output_dir, "05-payload-scaling", fmt, width=1800)


# ---------------------------------------------------------------------------
# Chart 06: Resource Efficiency Scatter
# ---------------------------------------------------------------------------


def chart_resource_efficiency(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    """Scatter: throughput (y) vs memory (x), bubble size = CPU%."""
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
    sizes = 12 + norm * 35

    # Compute efficiency score
    summary["efficiency"] = summary["mean_rps"] / summary["peak_mem"].clip(lower=1)

    fig = go.Figure()
    for i, row in summary.iterrows():
        fw = row["framework"]
        is_sp = _is_spikard(fw)
        label = _fw_label(fw)
        fig.add_trace(
            go.Scatter(
                x=[row["peak_mem"]],
                y=[row["mean_rps"]],
                mode="markers",
                marker={
                    "size": sizes.iloc[i],
                    "color": _fw_color(fw),
                    "opacity": 1.0 if is_sp else 0.7,
                    "line": {"color": "white", "width": 2} if is_sp else {"width": 0},
                },
                name=f"{label} · CPU {row['avg_cpu']:.0f}%",
                hovertext=f"{label}<br>{row['mean_rps']:,.0f} RPS<br>"
                          f"{row['peak_mem']:.0f} MB<br>CPU {row['avg_cpu']:.0f}%",
                hoverinfo="text",
                showlegend=True,
            )
        )

    # Quadrant guide lines
    mem_median = summary["peak_mem"].median()
    rps_median = summary["mean_rps"].median()
    fig.add_hline(y=rps_median, line_dash="dot", line_color="#333", line_width=1)
    fig.add_vline(x=mem_median, line_dash="dot", line_color="#333", line_width=1)

    # "Efficient zone" label
    fig.add_annotation(
        x=0.02, y=0.98, xref="paper", yref="paper",
        text="← High throughput / Low memory",
        showarrow=False,
        font={"size": 10, "color": "#555"},
    )

    # Annotate every point with its name, using smart positioning
    # Sort by RPS descending; stagger left/right to reduce overlap
    annotated = summary.sort_values("mean_rps", ascending=False).reset_index(drop=True)
    for idx, row in annotated.iterrows():
        fw = row["framework"]
        label = _fw_label(fw)
        # Alternate annotation side based on position relative to median
        if row["peak_mem"] > mem_median:
            ax, anchor = -30, "right"
        else:
            ax, anchor = 30, "left"
        fig.add_annotation(
            x=row["peak_mem"],
            y=row["mean_rps"],
            text=label,
            showarrow=True,
            arrowhead=0,
            arrowcolor="#444",
            ax=ax,
            ay=-10 - (idx % 3) * 8,
            font={"size": 9, "color": "#AAA"},
            xanchor=anchor,
        )

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title={"text": "Resource Efficiency (bubble size = CPU cores)", "font": {"size": 18}},
        xaxis_title="Peak Memory (MB)",
        yaxis_title="Mean RPS",
        height=800,
        legend={"font": {"size": 9}},
    )

    return _write_figure(fig, output_dir, "06-resource-efficiency", fmt)


# ---------------------------------------------------------------------------
# Chart 07: Resource Usage (efficiency score bar)
# ---------------------------------------------------------------------------


def chart_resource_usage(df: pd.DataFrame, output_dir: Path, fmt: str) -> list[Path]:
    """Two-panel: memory (log scale) and CPU cores used, horizontal bars."""
    if df.empty:
        return []

    summary = df.groupby("framework", as_index=False).agg(
        peak_mem=("memory_peak_mb", "mean"),
        avg_cpu=("cpu_avg_percent", "mean"),
        mean_rps=("requests_per_sec", "mean"),
    )

    # Convert CPU% to cores (100% = 1 core)
    summary["cpu_cores"] = summary["avg_cpu"] / 100.0

    # Sort by peak memory ascending (lowest memory at top)
    summary = summary.sort_values("peak_mem", ascending=False)
    fws = summary["framework"].tolist()
    labels = [_fw_label(fw) for fw in fws]

    fig = make_subplots(
        rows=1,
        cols=2,
        subplot_titles=["Peak Memory (MB)", "Avg CPU Cores Used"],
        horizontal_spacing=0.15,
    )

    # Style subplot titles
    for ann in fig.layout.annotations:
        ann.font = {"size": 13, "color": "#AAA"}

    fig.add_trace(
        go.Bar(
            y=labels,
            x=summary["peak_mem"],
            orientation="h",
            marker_color=[_fw_color(fw) for fw in fws],
            marker_line_color=["white" if _is_spikard(fw) else "rgba(0,0,0,0)" for fw in fws],
            marker_line_width=[1.5 if _is_spikard(fw) else 0 for fw in fws],
            text=[f"{v:.0f} MB" for v in summary["peak_mem"]],
            textposition="outside",
            textfont={"color": "#AAA"},
            showlegend=False,
        ),
        row=1,
        col=1,
    )

    # CPU as cores — format shows fractional cores
    fig.add_trace(
        go.Bar(
            y=labels,
            x=summary["cpu_cores"],
            orientation="h",
            marker_color=[_fw_color(fw) for fw in fws],
            marker_line_color=["white" if _is_spikard(fw) else "rgba(0,0,0,0)" for fw in fws],
            marker_line_width=[1.5 if _is_spikard(fw) else 0 for fw in fws],
            text=[f"{v:.1f}" for v in summary["cpu_cores"]],
            textposition="outside",
            textfont={"color": "#AAA"},
            showlegend=False,
        ),
        row=1,
        col=2,
    )

    fig.update_layout(
        **_LAYOUT_DEFAULTS,
        title={"text": "Resource Usage by Framework", "font": {"size": 18}},
        height=max(500, len(fws) * 38 + 120),
    )
    # Log scale on memory to make low-memory frameworks visible
    fig.update_xaxes(type="log", row=1, col=1)
    # Add annotation explaining cores
    fig.add_annotation(
        text="1 core = 100% CPU · values >1 indicate multi-core parallelism",
        xref="paper", yref="paper",
        x=0.98, y=-0.06,
        showarrow=False,
        font={"size": 10, "color": "#666"},
        xanchor="right",
    )

    return _write_figure(fig, output_dir, "07-resource-usage", fmt)


# ---------------------------------------------------------------------------
# Markdown table
# ---------------------------------------------------------------------------


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


# ---------------------------------------------------------------------------
# Metadata
# ---------------------------------------------------------------------------


def generate_metadata(data: dict[str, Any], output_dir: Path, charts: list[str]) -> None:
    """Write chart generation metadata to a JSON file."""
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
    "latency-distribution",
    "category-heatmap",
    "raw-vs-validated",
    "payload-scaling",
    "resource-efficiency",
    "resource-usage",
]

_CHART_FUNCS: dict[str, Any] = {
    "throughput-leaderboard": chart_throughput_leaderboard,
    "latency-distribution": chart_latency_distribution,
    "category-heatmap": chart_category_heatmap,
    "raw-vs-validated": chart_raw_vs_validated,
    "payload-scaling": chart_payload_scaling,
    "resource-efficiency": chart_resource_efficiency,
    "resource-usage": chart_resource_usage,
}


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------


def main() -> None:
    """CLI entry point for benchmark chart generation."""
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
            continue
        paths = func(df, args.output, args.format)
        if paths:
            generated.append(name)

    if args.markdown is not None:
        if args.markdown == "-":
            sys.stdout.write(generate_markdown_table(data) + "\n")
        else:
            generate_markdown_table(data, Path(args.markdown))

    if "summary" in data:
        generate_metadata(data, args.output, generated)


if __name__ == "__main__":
    main()
