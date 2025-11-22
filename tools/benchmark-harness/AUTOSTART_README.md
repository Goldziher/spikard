# Benchmark Harness Auto-Start Implementation Documentation

This directory contains comprehensive documentation for implementing auto-start functionality in the benchmark-harness codebase.

## Documentation Files

### 1. AUTOSTART_EXPLORATION.md
**Comprehensive codebase analysis covering:**
- Current architecture and module structure
- Directory structure overview with file counts
- Key modules and their responsibilities
- Current server management flow
- Framework detection logic
- Application structure and configurations
- Test file locations
- Configuration structures (ServerConfig, RunnerConfig, ProfileRunnerConfig)
- Summary of implementation needs

**Start here for:** Understanding the existing codebase architecture

### 2. AUTOSTART_IMPLEMENTATION.md
**Implementation strategy and detailed guide covering:**
- Quick reference with current architecture diagram
- Code flow diagram showing start_server() as bottleneck
- Framework detection logic comparison (current vs. needed)
- Module responsibilities summary
- Health check implementation details
- Configuration structures explanation
- Application entry point patterns
- 4-phase implementation strategy
- Files to modify with specific line numbers
- Testing strategy with code examples
- Impact analysis and backward compatibility
- Estimated effort (8-10 hours total)
- Implementation questions and recommendations

**Start here for:** Planning and executing the implementation

## Key Findings

### Single Entry Point
The `start_server()` function in `src/server.rs` (lines 92-213) is the single bottleneck for all server startup. All server creation flows through this function:

```
BenchmarkRunner::run() → start_server()
ProfileRunner::run()   → start_server()
```

### 10 Supported Frameworks
1. spikard-rust
2. spikard-python  
3. spikard-node
4. spikard-ruby
5. spikard-wasm
6. axum-baseline
7. fastapi
8. fastapi-granian
9. robyn
10. fastify

### Critical Files (Absolute Paths)
- Server startup: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/src/server.rs`
- Benchmark runner: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/src/runner.rs`
- Profile runner: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/src/profile/runner.rs`
- CLI entry: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/src/main.rs`

## Implementation Summary

### What Needs to be Built

1. **Framework Registry Module** (`src/framework.rs`)
   - Framework metadata structures
   - Registry of supported frameworks
   - Auto-detection function

2. **Enhanced server.rs**
   - Import framework registry
   - Add auto-detection logic
   - Refactor command building

3. **Updated CLI** (main.rs)
   - Make --framework optional
   - Add auto-detection when flag omitted

4. **Tests**
   - Framework detection tests
   - Auto-start integration tests

### Effort Estimate
- **Total Time**: 8-10 hours
- **Files to Create**: 1 new (framework.rs)
- **Files to Modify**: 4-5 existing

## Quick Navigation

### For Understanding Architecture
1. Read AUTOSTART_EXPLORATION.md - "Directory Structure Overview"
2. Read AUTOSTART_EXPLORATION.md - "Current Server Management Flow"
3. Read AUTOSTART_IMPLEMENTATION.md - "Code Flow Diagram"

### For Implementation Planning
1. Read AUTOSTART_IMPLEMENTATION.md - "Implementation Strategy"
2. Read AUTOSTART_IMPLEMENTATION.md - "Files to Modify"
3. Read AUTOSTART_IMPLEMENTATION.md - "Testing Strategy"

### For Adding New Frameworks
1. Read AUTOSTART_IMPLEMENTATION.md - "Phase 1: Framework Registry"
2. Create entry in registry() function
3. Add detection logic for your framework
4. Add tests

## Framework Detection Algorithm

The proposed auto-detection scans app_dir for language-specific files:

```
Cargo.toml (with spikard) → spikard-rust
server.py + pyproject.toml → spikard-python
server.ts → spikard-node
server.rb → spikard-ruby
server.js → spikard-wasm
```

## Health Check Pattern

All servers implement the same pattern:
1. Accept port as first CLI argument
2. Listen on http://0.0.0.0:{port}
3. Provide /health endpoint (or respond to GET /)
4. Return 2xx status when ready

This enables 30-attempt health check loop with 1-second waits (30 second max).

## Current Architecture Limitation

The current implementation requires users to specify `--framework` explicitly. The auto-start feature will:
- Auto-detect framework from app_dir contents
- Make `--framework` optional
- Fall back to explicit flag if detection fails
- Maintain backward compatibility

## Next Steps

1. Review AUTOSTART_EXPLORATION.md for current state
2. Review AUTOSTART_IMPLEMENTATION.md for detailed implementation plan
3. Start with Phase 1 (create framework.rs)
4. Follow implementation strategy in AUTOSTART_IMPLEMENTATION.md
5. Use testing strategy for validation

---

**Generated**: 2025-11-22
**Codebase Location**: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/`
**Exploration Depth**: Medium (complete module structure, key functions, entry points)
