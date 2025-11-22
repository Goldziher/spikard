# Spikard TODO

**Last Updated:** 2025-11-22
**Current Focus:** Multipart/Form-Data & Benchmark Harness

---

## 🎯 HIGHEST PRIORITY - Multipart/Form-Data Support

**Goal:** Ergonomic file upload support across ALL language bindings with proper typing and validation options.

**Status:**
- ✅ Rust HTTP layer complete (multipart parsing, JSON conversion)
- ✅ Python binding complete (UploadFile class, converter, tests, validation, ergonomic API)
- ✅ Python generator complete (generates typed handlers with UploadFile)
- ✅ Node.js: UploadFile class + converters complete
- ✅ Node.js: Test suite created (26 passing tests, TDD approach)
- ✅ Node.js: Ergonomic handler wrappers complete (wrapHandler, wrapBodyHandler, wrapHandlerWithContext)
- ✅ Test naming convention migrated to `.spec.ts` (51 files)
- ✅ Fresh benchmarks completed (Rust: 165K req/s, Python: 17.6K req/s)
- ✅ README files updated with latest benchmark results
- 🚧 Node.js: Test generator needs refactoring to use handler wrappers (low priority - e2e only)
- ❌ Ruby, Rust, WASM bindings missing
- ❌ Ruby, Rust generators need updates to match Python ergonomics

### Phase 1: TypeScript/Node.js Implementation

- [x] **Create UploadFile class** (`packages/node/src/upload.ts`)
  - [x] Properties: filename, size, contentType, headers
  - [x] Methods: read(), readAsync(), text(), seek(), tell(), close()
  - [x] Buffer-based backend
  - [x] TypeScript type definitions
  - [x] Export from main index

- [x] **Create converter utilities** (`packages/node/src/converters.ts`)
  - [x] convertFileMetadataToUploadFile() function
  - [x] processUploadFileFields() for recursive conversion
  - [x] convertHandlerBody() main entry point

- [x] **Ergonomic handler wrapper** (`packages/node/src/handler-wrapper.ts`)
  - [x] Create typed handler wrapper function
  - [x] Auto-convert file metadata → UploadFile instances
  - [x] Support typed body parameters (like Python's dataclass)
  - [x] **Same ergonomics as Python**: `function handler(body: UploadRequest)`
  - [x] Zero boilerplate - no manual JSON parsing needed
  - [x] Three wrapper variants: wrapHandler, wrapBodyHandler, wrapHandlerWithContext

- [ ] **Update code generator** (`tools/test-generator/src/node_app.rs`)
  - [ ] Generate handlers using UploadFile type for file fields
  - [ ] Import UploadFile from @spikard/node
  - [ ] Generate interface types for request bodies with files
  - [ ] Use handler wrapper for automatic conversion
  - [ ] Match Python generator ergonomics
  - [ ] **NOTE**: Low priority - generator only used for e2e test apps, not user code

- [ ] **Zod validation support**
  - [ ] Test WITHOUT Zod (raw objects, fastest)
  - [ ] Test WITH Zod (typed, convenience)
  - [ ] Benchmark validation overhead
  - [ ] Document performance tradeoffs

- [ ] **Comprehensive tests** (`e2e/node/tests/upload.test.ts`)
  - [ ] Single file upload
  - [ ] Multiple file upload
  - [ ] Optional file upload
  - [ ] Mixed form data + files
  - [ ] Large file handling
  - [ ] With/without Zod validation
  - [ ] Verify ergonomic API (no manual JSON.parse)

### Phase 2: Ruby Implementation

- [x] **Create UploadFile class** (`packages/ruby/lib/spikard/upload_file.rb`)
  - [x] Properties: filename, size, content_type, headers
  - [x] Methods: read, rewind, close, seek, tell, text
  - [x] StringIO-based backend
  - [x] Ruby idiomatic API (snake_case, attr_reader)

- [x] **Create converter utilities** (`packages/ruby/lib/spikard/converters.rb`)
  - [x] convert_file_metadata_to_upload_file method
  - [x] process_upload_file_fields for recursive conversion
  - [x] convert_handler_body main entry point

- [ ] **Ergonomic handler wrapper** (`packages/ruby/lib/spikard/handler_wrapper.rb`)
  - [ ] Create typed handler wrapper
  - [ ] Auto-convert file metadata → UploadFile instances
  - [ ] Support Dry::Struct for typed bodies
  - [ ] **Same ergonomics as Python**: `def handler(body)`
  - [ ] Zero boilerplate - automatic conversion

- [ ] **Update code generator** (`tools/app-generator/src/generators/ruby.rs`)
  - [ ] Generate handlers using UploadFile type for file fields
  - [ ] Require 'spikard/upload_file'
  - [ ] Generate Dry::Struct classes for request bodies with files
  - [ ] Use handler wrapper for automatic conversion
  - [ ] Match Python generator ergonomics

- [ ] **Dry::Schema / Dry::Struct support**
  - [ ] Test WITHOUT validation (fastest)
  - [ ] Test WITH Dry::Schema (typed DTOs)
  - [ ] Test WITH Dry::Struct (typed classes)
  - [ ] Benchmark overhead
  - [ ] Document patterns

- [ ] **Comprehensive tests**
  - [ ] Full upload test suite
  - [ ] Validation scenarios
  - [ ] Verify ergonomic API (automatic conversion)

### Phase 3: Rust Implementation

- [x] **Create UploadFile struct** (`crates/spikard/src/upload.rs`)
  - [x] Fields: filename, size, content_type, content (Bytes)
  - [x] Methods: read(), read_to_string(), as_bytes(), content_type_or_default()
  - [x] Implement `serde::Deserialize` and `serde::Serialize`
  - [x] Implement `Read` and `Seek` traits
  - [x] Clone, Debug, Send, Sync traits
  - [x] Automatic base64 decoding support
  - [x] Comprehensive test suite (7 tests passing)

- [ ] **Create converter utilities** (`crates/spikard/src/converters.rs`)
  - [ ] convert_file_metadata_to_upload_file function
  - [ ] process_upload_file_fields for recursive conversion
  - [ ] Integrate with serde deserialization

- [ ] **Handler integration** (`crates/spikard-http/src/handler.rs`)
  - [ ] Auto-deserialize from JSON metadata to UploadFile
  - [ ] Zero-copy where possible (Bytes::from)
  - [ ] Handle single/multiple/optional files in handler signatures
  - [ ] **Same ergonomics as Python**: `async fn handler(body: UploadRequest)`

- [ ] **Update code generator** (`tools/app-generator/src/generators/rust.rs`)
  - [ ] Generate handlers using UploadFile type for file fields
  - [ ] Use spikard::UploadFile in generated structs
  - [ ] Generate typed request structs with UploadFile fields
  - [ ] Match Python generator ergonomics

- [ ] **Serde validation**
  - [ ] Test WITHOUT serde (fastest)
  - [ ] Test WITH serde validation
  - [ ] Benchmark overhead

- [ ] **Comprehensive tests**
  - [ ] Upload test suite
  - [ ] Async file handling
  - [ ] Verify ergonomic API (automatic deserialization)

### Phase 4: WASM Implementation

- [ ] **Create UploadFile class** (`packages/wasm/src/upload.ts`)
  - [ ] Integrate with browser File API
  - [ ] Properties: filename, size, type
  - [ ] Methods: arrayBuffer(), text(), stream()
  - [ ] TypeScript definitions

- [ ] **Rust binding integration** (`crates/spikard-wasm/src/handler.rs`)
  - [ ] Convert JSON → UploadFile
  - [ ] Handle browser file objects
  - [ ] Flexible body parameter naming

- [ ] **Browser tests**
  - [ ] File input integration
  - [ ] Drag-and-drop support
  - [ ] Multiple file selection

---

## 🛠️ HIGHEST PRIORITY - Code Generator Updates

**Goal:** Ensure all language generators produce ergonomic, typed handler code that matches Python's quality.

**Current State:**
- ✅ Python generator: Generates typed handlers with dataclasses, UploadFile support, automatic conversion
- ❌ Node generator: Currently generates manual JSON.parse boilerplate
- ❌ Ruby generator: Currently generates manual JSON parsing
- ❌ Rust generator: Needs UploadFile support + typed request structs

### Generator Consistency Requirements

**All generators must:**
1. Generate typed request/response interfaces/structs/classes
2. Import UploadFile type for file upload fields
3. Use handler wrappers for automatic conversion (no manual parsing)
4. Support validation libraries (Zod/Dry::Schema/Serde) as opt-in
5. Generate the SAME ergonomic API across all languages

**Example: Python (REFERENCE IMPLEMENTATION)**
```python
@dataclass
class UploadRequest:
    file: UploadFile
    description: str

@app.post("/upload")
def upload_handler(body: UploadRequest):
    return {"filename": body.file.filename}
```

**Example: Node.js (TARGET)**
```typescript
interface UploadRequest {
    file: UploadFile;
    description: string;
}

app.post("/upload", async ({ body }: { body: UploadRequest }) => {
    return { filename: body.file.filename };
});
```

**Example: Ruby (TARGET)**
```ruby
class UploadRequest < Dry::Struct
  attribute :file, UploadFile
  attribute :description, String
end

app.post("/upload") do |body|
  { filename: body.file.filename }
end
```

**Example: Rust (TARGET)**
```rust
#[derive(Deserialize)]
struct UploadRequest {
    file: UploadFile,
    description: String,
}

async fn upload_handler(body: UploadRequest) -> impl IntoResponse {
    json!({ "filename": body.file.filename })
}
```

### Implementation Tasks

- [ ] **Update Node.js generator** (`tools/app-generator/src/generators/node.rs`)
  - [ ] Generate TypeScript interfaces for request bodies
  - [ ] Import UploadFile for file fields
  - [ ] Remove manual JSON.parse boilerplate
  - [ ] Use handler wrapper pattern

- [ ] **Update Ruby generator** (`tools/app-generator/src/generators/ruby.rs`)
  - [ ] Generate Dry::Struct classes for request bodies
  - [ ] Require UploadFile for file fields
  - [ ] Remove manual JSON parsing
  - [ ] Use handler wrapper pattern

- [ ] **Update Rust generator** (`tools/app-generator/src/generators/rust.rs`)
  - [ ] Generate request structs with derives
  - [ ] Use UploadFile for file fields
  - [ ] Auto-deserialize via serde

- [ ] **Regenerate all test fixtures**
  - [ ] Run generator for all testing_data fixtures
  - [ ] Verify Python handlers still work (no regression)
  - [ ] Verify Node handlers use new pattern
  - [ ] Verify Ruby handlers use new pattern
  - [ ] Verify Rust handlers use new pattern

---

## 🔥 HIGHEST PRIORITY - Benchmark Harness Auto-Start

**Problem:** Harness currently requires manually starting servers. Need full automation.

**Goal:** `benchmark-harness profile spikard-python` should:
1. Build the test app
2. Start the server automatically
3. Run the workloads
4. Collect results
5. Stop the server
6. Return structured output

### Implementation Tasks

- [ ] **Auto-start infrastructure** (`tools/benchmark-harness/src/server/auto_start.rs`)
  - [ ] Detect framework from app directory
  - [ ] Build command generation (pip install, npm install, bundle install, cargo build)
  - [ ] Start command generation (python, node, ruby, cargo run)
  - [ ] Port allocation and binding check
  - [ ] Health check / readiness probe
  - [ ] Graceful shutdown on completion

- [ ] **Per-framework server managers**
  - [ ] Python: `uv run python app.py` or `uvicorn app:app`
  - [ ] Node: `node server.js` or `npm start`
  - [ ] Ruby: `ruby server.rb` or `bundle exec ruby server.rb`
  - [ ] Rust: `cargo run --release`

- [ ] **Process lifecycle management**
  - [ ] Capture stdout/stderr for debugging
  - [ ] Timeout handling (server fails to start)
  - [ ] Resource cleanup (kill process group)
  - [ ] Error reporting (server crashes)

- [ ] **Integration with ProfileRunner**
  - [ ] Start server before workloads
  - [ ] Wait for readiness
  - [ ] Run all workloads
  - [ ] Collect profiler output
  - [ ] Stop server after completion

- [ ] **Testing**
  - [ ] Test auto-start for all 4 languages
  - [ ] Test with missing dependencies (should fail gracefully)
  - [ ] Test with port conflicts
  - [ ] Test with server crashes
  - [ ] Verify cleanup on interrupt (Ctrl+C)

---

## 📊 Phase 4: Compare Mode (After Above Complete)

- [ ] **Create compare runner module** (`tools/benchmark-harness/src/compare/mod.rs`)
  - [ ] Multi-framework orchestration
  - [ ] Parallel execution with port management
  - [ ] Auto-start all framework servers
  - [ ] Collect results per `CompareResult` schema

- [ ] **Statistical analysis** (`src/compare/analysis.rs`)
  - [ ] Implement t-test for statistical significance
  - [ ] Calculate p-values and confidence intervals
  - [ ] Determine winner per workload
  - [ ] Performance ratio computation

- [ ] **Report generation** (`src/compare/report.rs`)
  - [ ] Markdown comparison tables
  - [ ] Performance visualizations
  - [ ] Statistical significance indicators
  - [ ] Overall winner summary

- [ ] **Compare CLI subcommand**
  - [ ] `benchmark-harness compare --frameworks spikard-python,fastapi`
  - [ ] JSON output
  - [ ] Markdown report generation

---

## 🧪 Validation Overhead Benchmarks

**Goal:** Measure performance impact of validation layers across all languages.

### Test Matrix

Each language tests:
1. **No validation** (raw objects) - baseline performance
2. **With validation** (Pydantic/Zod/Dry::Schema/Serde) - convenience vs performance

### Workloads

- [ ] **Simple JSON body** (user registration)
  - Measure: serialization overhead

- [ ] **Complex nested object** (order with items)
  - Measure: deep validation cost

- [ ] **File upload** (multipart/form-data)
  - Measure: file conversion overhead

- [ ] **Mixed form data** (files + JSON fields)
  - Measure: combined overhead

### Expected Findings

Document for each language:
- Validation overhead % (e.g., "Pydantic adds 15% latency")
- When validation is worth it (safety vs speed)
- Optimization techniques (msgspec, Zod pre-compilation)

---

## 📋 Remaining Tasks (Lower Priority)

### Benchmark Harness Enhancements

- [ ] Extract query parameters from URL path
- [ ] GIL metrics extraction from py-spy data
- [ ] FFI overhead measurement
- [ ] Node --prof integration
- [ ] Ruby stackprof integration
- [ ] Rust profiler auto-attachment (perf/Instruments)

### CI Integration (Phase 5)

- [ ] GitHub Actions workflow
- [ ] Historical baseline tracking
- [ ] Regression detection
- [ ] PR comments with comparison

### Cleanup

- [ ] Remove deprecated bash scripts
- [ ] Update BENCHMARK_RESULTS.md with new format
- [ ] Documentation updates

---

## 📈 Success Criteria

### Multipart/Form-Data
- ✅ All 5 languages (Python, Node, Ruby, Rust, WASM) have UploadFile support
- ✅ Tests pass for single/multiple/optional file uploads
- ✅ Validation overhead documented for each language
- ✅ Zero breaking changes to existing APIs

### Benchmark Harness
- ✅ `benchmark-harness profile <framework>` works without manual setup
- ✅ Auto-builds and auto-starts all framework servers
- ✅ Compare mode runs multi-framework benchmarks
- ✅ Statistical analysis determines winners
- ✅ Markdown reports generated automatically

---

## 🎯 Current Focus (This Week)

**Priority 1:** Code generator ergonomics - ALL languages must match Python quality
- Node.js: Handler wrapper + generator updates (remove JSON.parse boilerplate)
- Ruby: Handler wrapper + generator updates
- Rust: UploadFile struct + auto-deserialization

**Priority 2:** Complete UploadFile implementation across all bindings
- Node.js: Handler integration + tests
- Ruby: Full implementation
- Rust: Native UploadFile support
- WASM: Browser integration

**Priority 3:** Benchmark harness auto-start infrastructure
- Auto-build and auto-start servers
- Full automation for profiling

**Blocked until above complete:**
- Compare mode (needs auto-start)
- Validation benchmarks (needs all UploadFile implementations + generators)
- CI integration (needs stable benchmark harness)

**Key Principle:**
Every language binding must provide the SAME ergonomic, zero-boilerplate experience as Python.
Users should write `function handler(body: TypedRequest)` NOT `JSON.parse(requestJson)`.
