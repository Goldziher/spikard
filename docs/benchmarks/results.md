# Benchmark Results Dashboard

This dashboard displays the latest benchmark results from our continuous performance testing across all supported frameworks.

!!! info "Latest Benchmark Run"
    **Date:** <span id="benchmark-date">Loading...</span><br>
    **Commit:** <span id="benchmark-commit">Loading...</span><br>
    **Branch:** <span id="benchmark-branch">Loading...</span><br>
    **Frameworks:** <span id="benchmark-frameworks">Loading...</span><br>
    **Status:** <span id="benchmark-status">Loading...</span>

## Throughput Comparison

Requests per second across frameworks for different payload sizes.

<div class="benchmark-dashboard">
  <iframe
    src="../assets/benchmarks/throughput-by-framework.html"
    width="100%"
    height="600"
    frameborder="0"
    loading="lazy"
    title="Throughput by Framework">
  </iframe>
</div>

## Latency Percentiles

Response time distribution (P50, P90, P95, P99) across frameworks.

<div class="benchmark-dashboard">
  <iframe
    src="../assets/benchmarks/latency-percentiles.html"
    width="100%"
    height="800"
    frameborder="0"
    loading="lazy"
    title="Latency Percentiles">
  </iframe>
</div>

## Validation Overhead

Performance impact of request validation (validation vs raw comparison).

<div class="benchmark-dashboard">
  <iframe
    src="../assets/benchmarks/validation-overhead.html"
    width="100%"
    height="600"
    frameborder="0"
    loading="lazy"
    title="Validation Overhead">
  </iframe>
</div>

## Resource Utilization

Memory and CPU usage across frameworks.

<div class="benchmark-dashboard">
  <iframe
    src="../assets/benchmarks/resources.html"
    width="100%"
    height="600"
    frameborder="0"
    loading="lazy"
    title="Resource Utilization">
  </iframe>
</div>

## Methodology

See [Benchmark Methodology](methodology.md) for details on how these tests are conducted.

## Raw Data

Download the latest aggregated results: [aggregated.json](../assets/benchmarks/aggregated.json)

<script>
// Dynamically load metadata and inject into page
fetch('../assets/benchmarks/metadata.json')
  .then(response => response.json())
  .then(data => {
    // Format date
    const date = new Date(data.generated_at);
    document.getElementById('benchmark-date').textContent = date.toLocaleString();

    // Commit info
    const commit = data.git_commit?.substring(0, 7) || 'unknown';
    const commitUrl = data.git_commit
      ? `https://github.com/anthropics/spikard/commit/${data.git_commit}`
      : '#';
    document.getElementById('benchmark-commit').innerHTML =
      `<a href="${commitUrl}" target="_blank">${commit}</a>`;

    document.getElementById('benchmark-branch').textContent =
      data.git_branch || 'unknown';

    // Framework stats
    const frameworks = data.frameworks;
    document.getElementById('benchmark-frameworks').textContent =
      `${frameworks.completed}/${frameworks.total} completed`;

    // Status badge
    const status = frameworks.failed === 0 ? '✅ All passing' :
                   `⚠️ ${frameworks.failed} failed`;
    document.getElementById('benchmark-status').textContent = status;
  })
  .catch(err => {
    console.error('Failed to load benchmark metadata:', err);
    document.getElementById('benchmark-date').textContent = 'Error loading metadata';
  });
</script>
