# Framework Comparison Report

**Baseline:** spikard-python
**Date:** 2025-11-23T10:25:06.666063+00:00
**Suite:** all

## Summary

| Framework | Runtime | Verdict | Overall |
|-----------|---------|---------|----------|
| spikard-python | Python 3.12.8 | baseline | 📊 |
| fastapi | unknown | significantly worse | 📉 |
| litestar | unknown | significantly worse | 📉 |

**Overall Winner:** spikard-python

## Performance Metrics

| Framework | Avg RPS | Avg Latency (ms) | Success Rate | Workloads |
|-----------|---------|------------------|--------------|----------|
| spikard-python | 35779.29 | 7.44 | 100.00% | 18 |
| fastapi | 12775.84 | 7.90 | 100.00% | 18 |
| litestar | 26357.56 | 4.59 | 100.00% | 18 |

## Statistical Analysis

### fastapi vs spikard-python

| Metric | t-statistic | p-value | Significant | Effect Size |
|--------|-------------|---------|-------------|-------------|
| requests_per_sec | 2.07 | 0.0545 | ✗ | 0.69 (large) |
| latency_p50_ms | -0.11 | 0.9115 | ✗ | -0.04 (small) |
| latency_p95_ms | 1.87 | 0.0782 | ✗ | 0.62 (large) |
| latency_p99_ms | 2.39 | 0.0288 | ✓ | 0.80 (large) |

### litestar vs spikard-python

| Metric | t-statistic | p-value | Significant | Effect Size |
|--------|-------------|---------|-------------|-------------|
| requests_per_sec | 0.84 | 0.4142 | ✗ | 0.28 (medium) |
| latency_p50_ms | 2.55 | 0.0190 | ✓ | 0.85 (very_large) |
| latency_p95_ms | 3.54 | 0.0023 | ✓ | 1.18 (very_large) |
| latency_p99_ms | 3.47 | 0.0024 | ✓ | 1.16 (very_large) |

---
**Legend:** ✓ = statistically significant (p < 0.05), ✗ = not significant
