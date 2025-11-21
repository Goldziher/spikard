# Spikard TODO

## Benchmark Harness - Profile & Compare Modes

### Phase 3: Profile Mode Implementation

- [x] **Create profile runner module** (`tools/benchmark-harness/src/profile/mod.rs`)
  - [x] Implement `ProfileRunner` struct
  - [x] Orchestrate workload suite execution
  - [x] Collect structured results per `ProfileResult` schema
  - [x] Integrate with existing server/load_generator modules

- [x] **Profile CLI subcommand** (`src/main.rs`)
  - [x] Add `profile` subcommand to CLI
  - [x] Parse options (framework, app-dir, suite, profiler, baseline, output)
  - [x] Invoke ProfileRunner
  - [x] Write structured JSON output
  - [x] Print summary to console

- [x] **End-to-end testing**
  - [x] Test with json-bodies suite (4 workloads)
  - [x] Verify JSON output structure
  - [x] Validate performance metrics collection

- [x] **Complete profile runner implementation**
  - [x] Extract latency metrics from OhaOutput (From<OhaOutput> trait)
  - [x] Calculate CPU p95 properly (ResourceMonitor::cpu_percentile)
  - [x] Load body data from testing_data fixtures (load_body_from_fixtures)
  - [x] Runtime version detection (python3/node/ruby/rustc --version)
  - [x] Framework version detection (parse Cargo.toml)
  - [x] Dynamic port allocation (scan 8100-8199)
  - [x] Baseline comparison (load_baseline_comparison)
  - [ ] Extract query parameters from URL path (future enhancement)

- [ ] **Python profiler integration** (`src/profile/python.rs`)
  - [x] Basic py-spy integration structure
  - [ ] Capture gil_wait_time_ms, gil_contention_percent
  - [ ] Measure ffi_overhead_ms, handler_time_ms
  - [ ] Generate flamegraph (optional)
  - [ ] Collect GC metrics (gc_collections, gc_time_ms)

- [ ] **Node profiler integration** (`src/profile/node.rs`)
  - [ ] Node --prof integration
  - [ ] V8 heap metrics (v8_heap_used_mb, v8_heap_total_mb)
  - [ ] Event loop lag measurement
  - [ ] Flamegraph generation

- [ ] **Ruby profiler integration** (`src/profile/ruby.rs`)
  - [ ] stackprof integration
  - [ ] GC metrics (gc_count, gc_time_ms)
  - [ ] Heap metrics (heap_allocated_pages, heap_live_slots)
  - [ ] Flamegraph generation

- [ ] **Rust profiler integration** (`src/profile/rust.rs`)
  - [ ] perf integration (Linux)
  - [ ] Instruments integration (macOS)
  - [ ] Heap allocation tracking
  - [ ] Flamegraph generation

### Phase 4: Compare Mode Implementation

- [ ] **Create compare runner module** (`tools/benchmark-harness/src/compare/mod.rs`)
  - [ ] Implement `CompareRunner` struct
  - [ ] Multi-framework orchestration
  - [ ] Parallel execution with proper port management
  - [ ] Collect results per `CompareResult` schema

- [ ] **Statistical analysis** (`src/compare/analysis.rs`)
  - [ ] Implement t-test for statistical significance
  - [ ] Calculate p-values and confidence intervals
  - [ ] Determine winner per workload
  - [ ] Compute performance ratios (framework_a_vs_framework_b)
  - [ ] Generate overall summary (workloads_won, category_winners)

- [ ] **Report generation** (`src/compare/report.rs`)
  - [ ] Generate markdown comparison report
  - [ ] Per-workload comparison tables
  - [ ] Performance ratio visualizations
  - [ ] Statistical significance indicators
  - [ ] Summary section with overall winner

- [ ] **Compare CLI subcommand** (`src/main.rs`)
  - [ ] Add `compare` subcommand to CLI
  - [ ] Parse options (frameworks, apps, suite, output, report)
  - [ ] Invoke CompareRunner
  - [ ] Write structured JSON output
  - [ ] Generate markdown report (if --report specified)
  - [ ] Print summary to console

### Phase 5: CI Integration

- [ ] **GitHub Actions workflow** (`.github/workflows/benchmark.yml`)
  - [ ] Profile mode job (run on push to main)
  - [ ] Compare mode job (run on PRs)
  - [ ] Artifact upload (JSON results)
  - [ ] PR comment with comparison report

- [ ] **Historical tracking**
  - [ ] Store baseline results in `results/baseline/`
  - [ ] Commit baseline updates on main branch
  - [ ] Version by git commit hash

- [ ] **Regression detection**
  - [ ] Define regression thresholds (e.g., >10% slowdown)
  - [ ] Compare PR results against main baseline
  - [ ] Flag regressions in PR comments
  - [ ] Block merge on critical regressions (configurable)

- [ ] **Auto-generated analytics** (future)
  - [ ] Time-series database (InfluxDB/TimescaleDB)
  - [ ] Grafana dashboards
  - [ ] Trend analysis
  - [ ] Email alerts on regressions

### Testing & Validation

- [ ] **Unit tests**
  - [ ] Schema serialization/deserialization
  - [ ] Metadata collection (git, host info)
  - [ ] Workload suite loading
  - [ ] Statistical analysis functions

- [ ] **Integration tests**
  - [ ] Profile mode end-to-end
  - [ ] Compare mode end-to-end
  - [ ] CLI argument parsing
  - [ ] JSON output validation

- [ ] **Documentation**
  - [ ] Update README with CLI examples
  - [ ] Document profiler requirements (py-spy, perf, etc.)
  - [ ] Add CI setup guide
  - [ ] Create user guide for interpreting results

### Cleanup & Migration

- [ ] **Remove deprecated bash scripts**
  - [ ] Delete `tools/benchmark-harness/scripts/comprehensive_benchmark.sh`
  - [ ] Delete `tools/benchmark-harness/scripts/test_workload.sh`
  - [ ] Update documentation references

- [ ] **Update BENCHMARK_RESULTS.md**
  - [ ] Re-run benchmarks with new CLI
  - [ ] Update results with new schema format
  - [ ] Add profiling insights (GIL overhead, etc.)

---

## Priority Order

**Immediate (Week 1):**
1. Phase 3: Profile runner module
2. Phase 3: Python profiler (py-spy)
3. Phase 3: Profile CLI subcommand
4. Test profile mode end-to-end

**Next (Week 2):**
5. Phase 4: Compare runner module
6. Phase 4: Statistical analysis
7. Phase 4: Report generation
8. Phase 4: Compare CLI subcommand
9. Test compare mode end-to-end

**Future (Week 3+):**
10. Phase 3: Node/Ruby/Rust profilers
11. Phase 5: CI integration
12. Phase 5: Historical tracking
13. Cleanup & migration

---

## Current Progress

**✅ Completed:**
- Phase 1: Schema & data model (`src/schema/`)
- Phase 2: Workload suite system (15 workloads, 5 suites)
- Documentation: `docs/benchmarks/harness-design.md`
- Metadata collection (git, host info)
- Documentation reorganization (kebab-case, TODO.md tracking)

**🚧 In Progress:**
- Phase 3: Profile runner module (started, needs API alignment)
  - Created `src/profile/mod.rs`, `runner.rs`, `python.rs`
  - Need to align with existing `load_generator` and `monitor` APIs
  - Compilation errors to fix before continuing

**📋 Next Up:**
- Fix API mismatches in profile runner
- Complete profile runner implementation
- Add profile CLI subcommand
