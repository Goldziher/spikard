# Kreuzberg napi-rs Implementation Patterns Analysis

## Overview
Kreuzberg's kreuzberg-node crate provides a comprehensive reference implementation for calling JavaScript handlers from Rust using napi-rs. This document captures their patterns for async JavaScript function invocation, ThreadsafeFunction usage, and test client implementations.

## Architecture

### Directory Structure
```
kreuzberg/crates/kreuzberg-node/
├── src/lib.rs              # Main NAPI bindings (1930+ lines)
├── Cargo.toml              # Dependencies: napi 3.4, napi-derive 3.3, async-trait
├── build.rs                # NAPI build configuration
├── package.json            # npm package metadata

kreuzberg/packages/typescript/src/
├── index.ts                # TypeScript wrapper layer (860 lines)
├── types.ts                # Type definitions
└── ocr/guten-ocr.ts        # OCR backend example

kreuzberg/examples/typescript/
├── custom-postprocessor.ts # Example with 6 different post-processor implementations
├── custom-validator.ts     # Example with 6 different validator implementations
└── basic.ts                # Basic extraction usage
```

---

## 1. ThreadsafeFunction Usage Pattern

### Overview
ThreadsafeFunction allows calling JavaScript async functions from Rust, with proper handling of the Node.js event loop.

### Building ThreadsafeFunction from JavaScript Object

**Location**: kreuzberg-node/src/lib.rs:1284-1294 (PostProcessor example)

```rust
// Get the JavaScript function from the passed object
let process_fn: Function<String, Promise<String>> = processor.get_named_property("process")?;

// Build ThreadsafeFunction with callback to wrap arguments
let tsfn = process_fn.build_threadsafe_function().build_callback(|ctx| {
    // build_callback transforms argument passing
    // Return vec![value] so JS receives it as separate arguments
    Ok(vec![ctx.value])
})?;

// Store in Arc for thread-safe sharing
let js_processor = JsPostProcessor {
    name: name.clone(),
    process_fn: Arc::new(tsfn),
    stage,
};
```

**Key Points**:
- Type signature: `Function<String, Promise<String>>` means:
  - Input: JSON string
  - Output: Promise that resolves to JSON string
- `build_callback()` wraps the value in a Vec
- Stored in `Arc<ThreadsafeFunction<...>>` for atomic reference counting
- Enable unsafe Send + Sync implementations (see below)

### Calling Async ThreadsafeFunction

**Location**: kreuzberg-node/src/lib.rs:1171-1183 (PostProcessor execution)

```rust
#[async_trait]
impl RustPostProcessor for JsPostProcessor {
    async fn process(
        &self,
        result: &mut kreuzberg::ExtractionResult,
        _config: &kreuzberg::ExtractionConfig,
    ) -> std::result::Result<(), kreuzberg::KreuzbergError> {
        // 1. Prepare JSON input
        let js_result = JsExtractionResult::try_from(result.clone())?;
        let json_input = serde_json::to_string(&js_result)?;

        // 2. Call JavaScript async function
        // Double await because:
        //   - call_async() returns Future<Result<Promise>>
        //   - The Promise itself needs awaiting
        let json_output = self
            .process_fn
            .call_async(json_input)          // First await
            .await
            .map_err(|e| kreuzberg::KreuzbergError::Plugin {
                message: format!("JS call failed: {}", e),
                plugin_name: self.name.clone(),
            })?
            .await                             // Second await
            .map_err(|e| kreuzberg::KreuzbergError::Plugin {
                message: format!("JS promise failed: {}", e),
                plugin_name: self.name.clone(),
            })?;

        // 3. Process result
        let updated: JsExtractionResult = serde_json::from_str(&json_output)?;
        let rust_result = kreuzberg::ExtractionResult::try_from(updated)?;

        *result = rust_result;
        Ok(())
    }
}
```

**Critical Points**:
- **Double await**: `call_async().await.await`
  - First `.await` waits for the callback to be enqueued on Node.js event loop
  - Second `.await` waits for the Promise returned by JavaScript to resolve
- **No `spawn_blocking` needed**: Direct await works because we're already in async context
- **Error handling**: Each await step has distinct error context
- **Mutation semantics**: `&mut` result is updated in-place after JS processing

### Send + Sync Implementation

**Location**: kreuzberg-node/src/lib.rs:1110-1119

```rust
struct JsPostProcessor {
    name: String,
    process_fn: Arc<ThreadsafeFunction<String, Promise<String>, Vec<String>, napi::Status, false>>,
    stage: ProcessingStage,
}

// SAFETY: ThreadsafeFunction from napi-rs is designed to be Send + Sync.
// - ThreadsafeFunction uses internal synchronization primitives to safely call JavaScript from any thread
// - NAPI-RS guarantees thread-safe execution of callbacks by marshaling all JavaScript interactions
//   through the Node.js event loop via a callback queue
// - The JavaScript function reference is managed by the Node.js runtime and protected by NAPI-RS
// - No raw JavaScript values (napi_value) are stored directly; only the ThreadsafeFunction wrapper
// - The Arc<ThreadsafeFunction<...>> wrapper provides shared ownership with atomic reference counting
// - All cross-thread access goes through ThreadsafeFunction::call_async which handles synchronization
unsafe impl Send for JsPostProcessor {}
unsafe impl Sync for JsPostProcessor {}
```

---

## 2. JavaScript Handler Invocation Patterns

### Pattern 1: JSON Serialization Bridge

All JavaScript handlers use JSON serialization as the data transport layer.

**Why JSON?**
- NAPI limitations with complex nested objects across FFI boundaries
- JSON is reliable, language-independent serialization format
- Metadata can contain arbitrary fields from post-processors

**Flow**:
```
Rust ExtractionResult
  ↓ (try_from + serde_json::to_string)
JSON String
  ↓ (ThreadsafeFunction::call_async)
JavaScript handler receives string
  ↓ (JSON.parse)
JavaScript object
  ↓ (process/modify)
JavaScript object
  ↓ (JSON.stringify)
JSON String
  ↓ (ThreadsafeFunction returns)
Rust receives JSON string
  ↓ (serde_json::from_str + try_from)
Updated Rust ExtractionResult
```

### Pattern 2: TypeScript Wrapper Layer

**Location**: kreuzberg/packages/typescript/src/index.ts:527-590 (PostProcessor registration)

The TypeScript wrapper abstracts JSON serialization from users:

```typescript
export function registerPostProcessor(processor: PostProcessorProtocol): void {
    const binding = getBinding();

    // Wrap the processor to handle JSON serialization
    const wrappedProcessor = {
        name: processor.name.bind(processor),
        processingStage: processor.processingStage?.bind(processor),
        async process(...args: unknown[]): Promise<string> {
            // NAPI wraps vec![value] in ANOTHER array: [[json_string]]
            const wrappedValue = args[0] as unknown[];
            const jsonString = wrappedValue[0] as string;

            // Parse JSON string to object
            const wireResult = JSON.parse(jsonString) as {
                content: string;
                mime_type: string;
                metadata: string | Record<string, unknown>;
                // ... other fields
            };

            // Convert from snake_case (Rust) to camelCase (TypeScript)
            const result: ExtractionResult = {
                content: wireResult.content,
                mimeType: wireResult.mime_type,
                metadata: typeof wireResult.metadata === "string"
                    ? JSON.parse(wireResult.metadata)
                    : wireResult.metadata,
                tables: (wireResult.tables || []) as Table[],
                detectedLanguages: wireResult.detected_languages ?? null,
                chunks: wireResult.chunks as string[] | null,
            };

            // Call user's processor (may be sync or async)
            const updated = await processor.process(result);

            // Convert back from camelCase to snake_case
            const wireUpdated = {
                content: updated.content,
                mime_type: updated.mimeType,
                metadata: updated.metadata,
                tables: updated.tables,
                detected_languages: updated.detectedLanguages,
                chunks: updated.chunks,
            };

            // Return as JSON string
            return JSON.stringify(wireUpdated);
        },
    };

    binding.registerPostProcessor(wrappedProcessor);
}
```

**Key Features**:
- Automatic case conversion: `mime_type` ↔ `mimeType`
- Metadata parsing: String JSON → Object (Rust sends stringified JSON)
- Supports both sync and async user processors via `await Promise.resolve()`
- Stores original processor as non-enumerable property for reference

### Pattern 3: Validator Implementation

**Location**: kreuzberg-node/src/lib.rs:1430-1487 (JsValidator)

```rust
#[async_trait]
impl RustValidator for JsValidator {
    async fn validate(
        &self,
        result: &kreuzberg::ExtractionResult,
        _config: &kreuzberg::ExtractionConfig,
    ) -> std::result::Result<(), kreuzberg::KreuzbergError> {
        // Convert and serialize
        let js_result = JsExtractionResult::try_from(result.clone())?;
        let json_input = serde_json::to_string(&js_result)?;

        // Call JavaScript validator
        // Validators return empty string on success, throw on failure
        self.validate_fn
            .call_async(json_input)
            .await
            .map_err(|e| {
                let err_msg = e.to_string();
                // Parse error type from message
                if err_msg.contains("ValidationError") || err_msg.contains("validation") {
                    kreuzberg::KreuzbergError::Validation {
                        message: err_msg,
                        source: None,
                    }
                } else {
                    kreuzberg::KreuzbergError::Plugin {
                        message: format!("JavaScript Validator '{}' call failed: {}", self.name, err_msg),
                        plugin_name: self.name.clone(),
                    }
                }
            })?
            .await
            .map_err(|e| {
                // Similar error mapping for Promise rejection
                kreuzberg::KreuzbergError::Validation { ... }
            })?;

        Ok(())
    }

    fn priority(&self) -> i32 {
        self.priority
    }
}
```

**Differences from PostProcessor**:
- Immutable reference: `&self` not `&mut self`
- No result return, only error
- Error messages with "ValidationError" prefix are mapped to validation errors
- Priority used for execution ordering

### Pattern 4: OCR Backend Implementation

**Location**: kreuzberg-node/src/lib.rs:1702-1814

```rust
#[async_trait]
impl RustOcrBackend for JsOcrBackend {
    async fn process_image(
        &self,
        image_bytes: &[u8],
        config: &kreuzberg::OcrConfig,
    ) -> std::result::Result<kreuzberg::ExtractionResult, kreuzberg::KreuzbergError> {
        // Convert bytes to Buffer
        let buffer = Buffer::from(image_bytes);
        let language = config.language.clone();

        // Call JavaScript with both arguments
        let json_output = self
            .process_image_fn
            .call_async((buffer, language))  // Tuple of arguments
            .await
            .map_err(|e| kreuzberg::KreuzbergError::Ocr {
                message: format!("JavaScript OCR backend failed: {}", e),
                source: Some(Box::new(e)),
            })?
            .await?;

        // Deserialize result - only expects JSON object, not full ExtractionResult
        let wire_result: serde_json::Value = serde_json::from_str(&json_output)?;

        // Extract specific fields from wire format
        let content = wire_result
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| kreuzberg::KreuzbergError::Ocr { ... })?
            .to_string();

        // Reconstruct ExtractionResult
        Ok(kreuzberg::ExtractionResult {
            content,
            mime_type,
            metadata,
            tables,
            detected_languages: None,
            chunks: None,
            images: None,
        })
    }
}
```

**Key Differences**:
- Takes tuple input: `(Buffer, String)`
- Simpler output: partial result object, not full ExtractionResult
- Manual field extraction from JSON using `get()` and `and_then()`

---

## 3. Data Conversion Patterns

### ExtractionResult Conversion

**Rust → JavaScript** (lines 431-493):
```rust
impl TryFrom<RustExtractionResult> for JsExtractionResult {
    type Error = napi::Error;

    fn try_from(val: RustExtractionResult) -> Result<Self> {
        // Serialize metadata to JSON
        let metadata = serde_json::to_value(&val.metadata)
            .map_err(|e| Error::new(..., format!(...)))?;

        // Convert images with nested OCR results
        let images = if let Some(imgs) = val.images {
            let mut js_images = Vec::with_capacity(imgs.len());
            for img in imgs {
                let ocr_result = if let Some(ocr) = img.ocr_result {
                    Some(JsExtractionResult::try_from(*ocr).and_then(|js_res| {
                        serde_json::to_value(js_res).map_err(|e| {
                            Error::new(Status::GenericFailure, ...)
                        })
                    })?)
                } else {
                    None
                };

                js_images.push(JsExtractedImage {
                    data: img.data.into(),  // Converts Vec<u8> to napi::Buffer
                    format: img.format,
                    // ... other fields
                    ocr_result,
                });
            }
            Some(js_images)
        } else {
            None
        };

        Ok(JsExtractionResult {
            content: val.content,
            mime_type: val.mime_type,
            metadata,
            // ... other fields
            images,
        })
    }
}
```

**JavaScript → Rust** (lines 495-707):
```rust
impl TryFrom<JsExtractionResult> for RustExtractionResult {
    type Error = napi::Error;

    fn try_from(val: JsExtractionResult) -> Result<Self> {
        // Parse metadata from JSON value into object
        let metadata_map: std::collections::HashMap<String, serde_json::Value> =
            serde_json::from_value(val.metadata.clone())?;

        // Extract known fields and remaining fields go to 'additional'
        let language = metadata_map.remove("language")
            .and_then(|v| serde_json::from_value(v).ok());
        let date = metadata_map.remove("date")
            .and_then(|v| serde_json::from_value(v).ok());

        // ... extract known format fields ...

        // Everything remaining is additional metadata from post-processors
        let additional = metadata_map;

        let metadata = kreuzberg::Metadata {
            language,
            date,
            subject,
            format,
            image_preprocessing,
            json_schema,
            error,
            additional,  // Preserves post-processor additions
        };

        // Similar processing for images...

        Ok(RustExtractionResult {
            content: val.content,
            mime_type: val.mime_type,
            metadata,
            // ... other fields
            images,
        })
    }
}
```

**Critical Features**:
- Uses `serde_json::to_value()` / `from_value()` for flexible JSON handling
- Preserves arbitrary fields in `metadata.additional` HashMap
- Handles recursive structures (images with nested OCR results)
- Automatic Vec<u8> ↔ Buffer conversion

### Metadata Field Mapping

**Rust → NAPI** uses flattened structure for format-specific metadata:
```json
{
  "content": "...",
  "mime_type": "application/pdf",
  "metadata": {
    "format_type": "pdf",
    "title": "Document Title",
    "author": "John Doe",
    "page_count": 10,
    "custom_field_from_processor": "value"
  },
  "tables": [ ... ],
  "chunks": [ ... ]
}
```

Known fields extracted and mapped:
- PDF: title, author, keywords, creator, producer, creation_date, modification_date, page_count
- Excel: sheet_count, sheet_names
- Email: from_email, to_emails, cc_emails, bcc_emails, message_id, attachments
- Image: width, height, exif
- Custom post-processor fields: stored in `additional` HashMap

---

## 4. Error Handling Patterns

### Error Conversion

**Location**: kreuzberg-node/src/lib.rs:40-97

```rust
fn convert_error(err: kreuzberg::KreuzbergError) -> napi::Error {
    match err {
        // IO errors - system-level file access issues
        KreuzbergError::Io(e) =>
            Error::new(Status::GenericFailure, format!("IO error: {}", e)),

        // Parsing errors - invalid document format, corrupt files
        KreuzbergError::Parsing { message, .. } =>
            Error::new(Status::InvalidArg, format!("Parsing error: {}", message)),

        // OCR errors - OCR processing failures
        KreuzbergError::Ocr { message, .. } =>
            Error::new(Status::GenericFailure, format!("OCR error: {}", message)),

        // Validation errors - invalid configuration or parameters
        KreuzbergError::Validation { message, .. } =>
            Error::new(Status::InvalidArg, format!("Validation error: {}", message)),

        // ... other variants ...

        // Unsupported format errors
        KreuzbergError::UnsupportedFormat(format) =>
            Error::new(Status::InvalidArg, format!("Unsupported format: {}", format)),

        // Other errors - catch-all
        KreuzbergError::Other(msg) =>
            Error::new(Status::GenericFailure, msg),
    }
}
```

**NAPI Status Codes Used**:
- `InvalidArg`: Validation failures, parsing errors, unsupported formats
- `GenericFailure`: I/O, OCR, cache, image processing, plugins

### Plugin Validation

```rust
fn validate_plugin_object(obj: &Object, plugin_type: &str, required_methods: &[&str]) -> Result<()> {
    let mut missing_methods = Vec::new();

    for method_name in required_methods {
        if !obj.has_named_property(method_name)? {
            missing_methods.push(*method_name);
        }
    }

    if !missing_methods.is_empty() {
        return Err(Error::new(
            Status::InvalidArg,
            format!(
                "{} is missing required methods: {}. Please ensure your plugin implements all required methods.",
                plugin_type,
                missing_methods.join(", ")
            ),
        ));
    }

    Ok(())
}
```

---

## 5. Test Client Implementation Pattern

### TypeScript Wrapper Testing

**Location**: kreuzberg/packages/typescript/src/index.ts:123-156

```typescript
// Allow tests to provide a mocked native binding
export function __setBindingForTests(mock: unknown): void {
    binding = mock;
    bindingInitialized = true;
}

export function __resetBindingForTests(): void {
    binding = null;
    bindingInitialized = false;
}

function getBinding(): any {
    if (bindingInitialized) {
        return binding;
    }

    try {
        if (typeof process !== "undefined" && process.versions && process.versions.node) {
            binding = require("kreuzberg-node");
            bindingInitialized = true;
            return binding;
        }
    } catch (error) {
        throw createNativeBindingError(error);
    }

    throw new Error("Failed to load Kreuzberg bindings...");
}
```

### Mock Binding Implementation

**Location**: kreuzberg/packages/typescript/tests/unit/helpers/mock-binding.ts

```typescript
export function createMockExtractionBinding() {
    return {
        extractFileSync: (path: string, mimeType: string | null, config: any) => {
            // Return mock ExtractionResult
            return {
                content: "Mock content",
                mimeType: "text/plain",
                metadata: { test: true },
                tables: [],
                chunks: null,
                detectedLanguages: null,
            };
        },

        extractFile: async (path: string, mimeType: string | null, config: any) => {
            // Same as sync version but wrapped in Promise
            return {
                content: "Mock content",
                mimeType: "text/plain",
                metadata: { test: true },
                tables: [],
                chunks: null,
                detectedLanguages: null,
            };
        },

        registerPostProcessor: (processor: any) => {
            // Store registered processor for inspection
            // Call with mock result for testing
        },

        registerValidator: (validator: any) => {
            // Store registered validator
        },

        // ... other methods ...
    };
}
```

### Test Pattern with Mock PostProcessor

**Location**: kreuzberg/packages/typescript/tests/unit/async-postprocessor.spec.ts:146-200+

```typescript
describe("Async PostProcessor Support", () => {
    beforeEach(() => {
        const mockBinding = createMockExtractionBinding();
        __setBindingForTests(mockBinding);
        clearPostProcessors();
    });

    afterEach(() => {
        clearPostProcessors();
        __resetBindingForTests();
    });

    it("should register and call async processor", async () => {
        const processor = new AsyncWordCountProcessor();
        registerPostProcessor(processor);

        // With mock binding, the processor's wrapped version
        // is stored and can be inspected
        const result = await extractBytes(
            new Uint8Array([/* ... */]),
            "text/plain"
        );

        // The mock binding will call the wrapped processor
        expect(result.metadata.async_word_count).toBeDefined();
    });
});
```

---

## 6. Execution Flow and Pipeline Integration

### Full Extraction Pipeline

```
TypeScript: await extractFile('document.pdf', null, config)
  ↓
Rust NAPI: pub async fn extract_file(
             file_path: String,
             mime_type: Option<String>,
             config: Option<JsExtractionConfig>
           ) -> Result<JsExtractionResult>
  ↓
Rust Core: kreuzberg::extract_file(&file_path, mime_type, &config).await
  ↓
Extraction Pipeline:
  1. Load and parse document → ExtractionResult

  2. Run Post-Processors (Early, Middle, Late):
     ├─ Acquire read lock on post_processor_registry
     ├─ For each processor in order:
     │   ├─ Convert result to JSON
     │   ├─ Call processor.process() via ThreadsafeFunction
     │   ├─ Wait for Promise to resolve
     │   ├─ Deserialize updated result
     │   └─ Update in-place
     └─ Release lock

  3. Calculate quality score

  4. Run chunking (if enabled)

  5. Run Validators (sorted by priority):
     ├─ Acquire read lock on validator_registry
     ├─ For each validator (highest priority first):
     │   ├─ Convert result to JSON
     │   ├─ Call validator.validate() via ThreadsafeFunction
     │   ├─ Wait for Promise to resolve
     │   └─ Handle any validation errors (fail-fast)
     └─ Release lock

  6. Return final result

  ↓
NAPI: Convert Rust result to JavaScript object
  ├─ Serialize metadata to JSON value
  ├─ Convert images with nested OCR results
  ├─ Build JsExtractionResult
  └─ Return to TypeScript

  ↓
TypeScript: Process result
  ├─ Convert result with convertResult()
  ├─ Parse metadata strings to objects
  ├─ Convert snake_case to camelCase
  └─ Return to user
```

### Registry Pattern

Post-processors and validators stored in global thread-safe registries:

```rust
// From kreuzberg core
pub static POST_PROCESSOR_REGISTRY: RwLock<ProcessorRegistry> = ...;
pub static VALIDATOR_REGISTRY: RwLock<ValidatorRegistry> = ...;

// Register creates Arc<dyn Trait>
pub fn register_post_processor(processor: Object) -> Result<()> {
    let registry = get_post_processor_registry();
    let mut registry = registry.write()?;
    registry.register(Arc::new(js_processor), priority)?;
    Ok(())
}

// Pipeline reads during execution
let registry = get_post_processor_registry();
let registry = registry.read()?;
for processor in registry.list_by_stage(ProcessingStage::Middle) {
    processor.process(&mut result, config).await?;
}
```

---

## 7. Best Practices from Kreuzberg

### 1. JSON Serialization for Complex Types
- Always use JSON for FFI boundaries with complex nested objects
- Provides language-independent data interchange
- Simpler than trying to represent complex Rust types in NAPI

### 2. TypeScript Wrapper Layer
- Wrap NAPI bindings in TypeScript for better DX
- Automatic case conversion (snake_case ↔ camelCase)
- Automatic JSON parsing/stringifying
- Preserves type information for users

### 3. Async Without `spawn_blocking`
- ThreadsafeFunction already handles thread pool management
- Direct `await` works from async Rust context
- No need for explicit blocking operations
- Cleaner error handling with single error chain

### 4. Error Typing and Mapping
- Map domain errors to appropriate NAPI status codes
- Preserve detailed error context in messages
- Test error messages to distinguish error types
- Provide actionable error messages to users

### 5. Plugin Validation
- Validate required methods exist before registration
- Clear error messages listing missing methods
- Check for empty/duplicate names
- Validate configuration parameters

### 6. Metadata Preservation
- Use HashMap for custom fields from post-processors
- Preserve arbitrary metadata additions
- Distinguish known fields from custom fields
- Round-trip conversion: Rust → JSON → TypeScript → JSON → Rust

### 7. Arc<ThreadsafeFunction<...>> Pattern
- Use Arc for shared ownership across thread boundaries
- Implement Send + Sync with documentation of safety
- ThreadsafeFunction handles internal synchronization
- No raw napi_value stored directly

### 8. Testing Strategy
- Mock binding support via `__setBindingForTests()`
- Separate mock binding creation function
- Reset bindings between tests
- Test both sync and async code paths

---

## 8. Comparison: PostProcessor vs Validator vs OCR Backend

| Aspect | PostProcessor | Validator | OCR Backend |
|--------|---------------|-----------|-------------|
| **Input** | String (JSON) | String (JSON) | Tuple (Buffer, String) |
| **Output** | String (JSON result) | String (empty on success) | String (JSON result) |
| **Error Behavior** | Updates result, continues | Throws error, fails | Throws error, fails |
| **Reference** | `&mut result` | `&result` | N/A |
| **Execution** | All run, ordered by stage | Stop on first failure | Single backend per config |
| **Registry** | Global per process | Global per process | Global per process |
| **Use Case** | Enrich/modify results | Validate quality | Custom image processing |

---

## 9. Key Insights from FFI_PATTERNS.md

### ThreadsafeFunction Types
```rust
// PostProcessor: String input, Promise<String> output
Function<String, Promise<String>>
↓
Arc<ThreadsafeFunction<String, Promise<String>, Vec<String>, napi::Status, false>>

// Validator: Same signature
Function<String, Promise<String>>
↓
Arc<ThreadsafeFunction<String, Promise<String>, Vec<String>, napi::Status, false>>

// OCR Backend: Tuple input, Promise<String> output
Function<(Buffer, String), Promise<String>>
↓
Arc<ThreadsafeFunction<(Buffer, String), Promise<String>, Vec<(Buffer, String)>, napi::Status, false>>
```

### Double Await Pattern

```rust
// This pattern repeats for all async handlers:
self.handler_fn
    .call_async(json_input)    // Enqueue callback, get Future<Result<Promise<String>>>
    .await                      // Wait for callback to execute
    .map_err(...)?              // Handle callback invocation errors
    .await                      // Wait for Promise returned by JS to resolve
    .map_err(...)?              // Handle Promise rejection
```

### Critical Build Requirement
- NAPI-RS requires proper build process: `pnpm run build` not just `cargo build`
- JavaScript bindings must be regenerated from `#[napi]` macros
- Stale bindings can cause mysterious runtime failures

---

## Summary

Kreuzberg's napi-rs implementation provides a production-quality reference for:

1. **Calling JavaScript async functions from Rust** via ThreadsafeFunction with double-await pattern
2. **Handling complex data types** through JSON serialization with automatic case conversion
3. **Plugin architecture** with global registries and priority-ordered execution
4. **Type-safe FFI** through TypeScript wrapper layer above raw NAPI bindings
5. **Async-friendly patterns** that work with Tokio without blocking issues
6. **Error handling** that maps between Rust and JavaScript error semantics
7. **Testing strategies** with mock bindings and dependency injection

The patterns are well-documented with safety comments and follow Rust best practices for unsafe code, error handling, and API design.
