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

- [x] **Python profiler integration** (`src/profile/python.rs`)
  - [x] Basic py-spy integration structure
  - [x] py-spy flamegraph generation (speedscope format)
  - [x] Application instrumentation module (`profiling/python_metrics.py`)
  - [x] Collect GC metrics (gc_collections with gen0/gen1/gen2 breakdown)
  - [x] Measure handler_time_ms, serialization_time_ms (when instrumented)
  - [x] Automatic metrics file writing on shutdown
  - [x] Graceful fallback when py-spy not installed
  - [ ] GIL metrics extraction from py-spy data (future enhancement)
  - [ ] FFI overhead measurement (requires deeper instrumentation)

- [x] **Node profiler integration** (`src/profile/node.rs`)
  - [x] Application instrumentation module structure
  - [x] V8 heap metrics collection (v8_heap_used_mb, v8_heap_total_mb)
  - [x] Event loop lag measurement support
  - [x] GC metrics (gc_time_ms)
  - [x] Graceful fallback without instrumentation
  - [ ] Node --prof integration (future enhancement)

- [x] **Ruby profiler integration** (`src/profile/ruby.rs`)
  - [x] Application instrumentation module structure
  - [x] GC metrics collection (gc_count, gc_time_ms)
  - [x] Heap metrics collection (heap_allocated_pages, heap_live_slots)
  - [x] Graceful fallback without instrumentation
  - [ ] stackprof integration (future enhancement)

- [x] **Rust profiler integration** (`src/profile/rust.rs`)
  - [x] Application instrumentation module structure
  - [x] Heap allocation tracking support
  - [x] Platform-specific profiler detection (perf on Linux, Instruments on macOS)
  - [x] Graceful fallback without instrumentation
  - [ ] Automatic profiler attachment (future enhancement)

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

**CURRENT PRIORITY - End-to-End Testing:**
1. ✅ Profile runner module - COMPLETE
2. ✅ All language profilers (Python/Node/Ruby/Rust) - COMPLETE
3. ✅ Profile CLI subcommand - COMPLETE
4. ⏳ Test Python binding end-to-end - IN PROGRESS
5. ⏳ Test Node/Ruby/Rust bindings
6. ⏳ Fix integration issues
7. ⏳ Verify all 4 bindings work perfectly

**Next (After Testing Complete):**
8. Phase 4: Compare runner module
9. Phase 4: Statistical analysis
10. Phase 4: Report generation
11. Phase 4: Compare CLI subcommand
12. Test compare mode end-to-end

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
- Phase 3: Profile runner module
  - Complete ProfileRunner implementation with all metrics
  - Profile CLI subcommand
  - End-to-end testing with json-bodies suite
  - Python profiler integration with GC metrics collection
  - Application instrumentation module for Python (`profiling/python_metrics.py`)
  - Fixture loading from `testing_data/` directory
  - Baseline comparison functionality
  - Runtime and framework version detection
  - Dynamic port allocation
- Documentation: `docs/benchmarks/harness-design.md`
- Metadata collection (git, host info)
- Documentation reorganization (kebab-case, TODO.md tracking)

**🚧 In Progress:**
- Phase 3: End-to-end testing and validation (CURRENT PRIORITY)

**📋 Next Up (Phase 3 Completion):**
1. Test Python binding with profile mode (`spikard-python-workloads`)
2. Verify Node binding works with benchmark harness
3. Verify Ruby binding works with benchmark harness
4. Verify Rust binding works with benchmark harness
5. Run full json-bodies suite on all 4 bindings
6. Fix any integration issues discovered
7. Document profiler usage and setup

**📋 After Testing (Phase 4):**
- Create compare runner module (`src/compare/mod.rs`)
- Implement statistical analysis (t-test, p-values)
- Generate markdown comparison reports
- Add Compare CLI subcommand

**✅ Recently Completed:**
- Phase 3: All language-specific profilers (with code review and cleanup)
  - ✅ Python profiler (py-spy + GC/timing metrics)
  - ✅ Node profiler (V8 heap + event loop metrics)
  - ✅ Ruby profiler (GC + heap metrics)
  - ✅ Rust profiler (heap tracking + platform profilers)
  - ✅ Code review: Removed dead code, fixed unsafe blocks, zero warnings
