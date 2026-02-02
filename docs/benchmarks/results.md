# Benchmark Results Dashboard

This dashboard displays the latest benchmark results from our continuous performance testing across all supported frameworks.

!!! info "Latest Benchmark Run"
    **Date:** <span id="benchmark-date">Loading...</span><br>
    **Commit:** <span id="benchmark-commit">Loading...</span><br>
    **Branch:** <span id="benchmark-branch">Loading...</span><br>
    **Frameworks:** <span id="benchmark-frameworks">Loading...</span><br>
    **Status:** <span id="benchmark-status">Loading...</span>

## Throughput Leaderboard

Requests per second across all frameworks, sorted by average throughput.

<div class="benchmark-dashboard">
  <iframe
    src="../assets/benchmarks/01-throughput-leaderboard.html"
    width="100%"
    height="600"
    frameborder="0"
    loading="lazy"
    title="Throughput Leaderboard">
  </iframe>
</div>

## Latency Distribution

Response time percentiles (P50, P90, P95, P99) across frameworks.

<div class="benchmark-dashboard">
  <iframe
    src="../assets/benchmarks/02-latency-distribution.html"
    width="100%"
    height="800"
    frameborder="0"
    loading="lazy"
    title="Latency Distribution">
  </iframe>
</div>

## Category Heatmap

Throughput breakdown by workload category (JSON, path params, query params, etc.).

<div class="benchmark-dashboard">
  <iframe
    src="../assets/benchmarks/03-category-heatmap.html"
    width="100%"
    height="600"
    frameborder="0"
    loading="lazy"
    title="Category Heatmap">
  </iframe>
</div>

## Raw vs Validated

Performance impact of request/response validation across frameworks.

<div class="benchmark-dashboard">
  <iframe
    src="../assets/benchmarks/04-raw-vs-validated.html"
    width="100%"
    height="600"
    frameborder="0"
    loading="lazy"
    title="Raw vs Validated">
  </iframe>
</div>

## Payload Scaling

Throughput vs JSON payload size (small to very-large).

<div class="benchmark-dashboard">
  <iframe
    src="../assets/benchmarks/05-payload-scaling.html"
    width="100%"
    height="600"
    frameborder="0"
    loading="lazy"
    title="Payload Scaling">
  </iframe>
</div>

## Resource Efficiency

Throughput vs memory usage with CPU as bubble size.

<div class="benchmark-dashboard">
  <iframe
    src="../assets/benchmarks/06-resource-efficiency.html"
    width="100%"
    height="600"
    frameborder="0"
    loading="lazy"
    title="Resource Efficiency">
  </iframe>
</div>

## Resource Usage

CPU and memory consumption across frameworks.

<div class="benchmark-dashboard">
  <iframe
    src="../assets/benchmarks/07-resource-usage.html"
    width="100%"
    height="600"
    frameborder="0"
    loading="lazy"
    title="Resource Usage">
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
      ? `https://github.com/Goldziher/spikard/commit/${data.git_commit}`
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
