# Observability and OpenAPI Generation

**Date:** January 2025
**Research-driven design document based on 2024-2025 ecosystem analysis**

## Executive Summary

This document outlines Spikard's approach to:

1. **OpenTelemetry instrumentation** - Full-stack observability from Rust through Python/TypeScript handlers
2. **OpenAPI generation** - Automatic OpenAPI 3.1 spec generation from JSON Schema
3. **Zero-overhead tracing** - Compile-time feature flags for production deployments
4. **Cross-language context propagation** - Seamless trace context across Rust ↔ Python/TypeScript boundaries

## 1. OpenTelemetry Architecture

### Core Principle
**Instrumentation lives in Rust** - Optional feature flag, zero cost when disabled.

### Rationale (2024-2025 Research)
- Axum ecosystem has mature OpenTelemetry integration via `tracing` + `opentelemetry-rust`
- `pyo3-opentelemetry` provides context propagation across Rust ↔ Python boundary
- Semantic conventions standardized in `opentelemetry-semantic-conventions` crate
- Feature flags allow zero-overhead in production if not needed

### Dependency Stack

```toml
# Cargo.toml - OpenTelemetry feature
[features]
default = []
otel = [
    "opentelemetry",
    "opentelemetry_sdk",
    "opentelemetry-otlp",
    "opentelemetry-semantic-conventions",
    "tracing-opentelemetry",
    "axum-tracing-opentelemetry",
]

[dependencies]
# Core tracing (always enabled - low overhead)
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# OpenTelemetry (optional feature)
opentelemetry = { version = "0.27", optional = true }
opentelemetry_sdk = { version = "0.27", optional = true, features = ["rt-tokio"] }
opentelemetry-otlp = { version = "0.27", optional = true, features = ["grpc-tonic", "tokio"] }
opentelemetry-semantic-conventions = { version = "0.27", optional = true }
tracing-opentelemetry = { version = "0.27", optional = true }
axum-tracing-opentelemetry = { version = "0.22", optional = true }

# PyO3 bindings (for Python context propagation)
pyo3-opentelemetry = { version = "0.2", optional = true }
```

### Architecture Diagram

```
┌─────────────────────────────────────────────────────┐
│  External Observability Backend (Jaeger, Tempo, etc)│
└─────────────────────┬───────────────────────────────┘
                      │ OTLP/gRPC
                      │
┌─────────────────────▼───────────────────────────────┐
│             Rust OpenTelemetry Layer                │
│  • TraceLayer (tower-http)                          │
│  • opentelemetry_sdk::TracerProvider                │
│  • OTLP Exporter                                     │
└─────────────┬───────────────────────┬───────────────┘
              │                       │
      ┌───────▼────────┐     ┌────────▼────────┐
      │  HTTP Request  │     │  Handler Span   │
      │  Span          │     │  (per route)    │
      │  • http.method │     │  • handler.name │
      │  • http.route  │     │  • custom attrs │
      │  • status_code │     └────────┬────────┘
      └────────────────┘              │
                                      │
                         ┌────────────▼─────────────┐
                         │  Language Binding Span   │
                         │  (Python/TypeScript)     │
                         │  • Propagated context    │
                         │  • User handler code     │
                         └──────────────────────────┘
```

## 2. Instrumentation Strategy

### 2.1 Automatic HTTP Instrumentation

All HTTP requests automatically traced with semantic conventions:

```rust
use axum_tracing_opentelemetry::{opentelemetry_tracing_layer, response_with_trace_layer};
use tower_http::trace::TraceLayer;

#[cfg(feature = "otel")]
pub fn create_router_with_otel(app: SpikardApp) -> Router {
    let router = create_base_router(app);

    router
        .layer(opentelemetry_tracing_layer())  // Automatic span creation
        .layer(response_with_trace_layer())     // Add trace_id to response headers
        .layer(TraceLayer::new_for_http())      // HTTP semantic conventions
}

#[cfg(not(feature = "otel"))]
pub fn create_router_with_otel(app: SpikardApp) -> Router {
    // Zero overhead - just return base router
    create_base_router(app)
}
```

### 2.2 Handler-Level Spans

Each route handler automatically gets a span with rich context:

```rust
use tracing::{info_span, Instrument};
use opentelemetry_semantic_conventions as semconv;

#[cfg(feature = "otel")]
async fn invoke_handler(
    handler_name: &str,
    method: &str,
    route_path: &str,
    request_data: RequestData,
) -> Result<Response, Error> {
    // Create a span for this specific handler
    let span = info_span!(
        "spikard.handler",
        // Standard semantic conventions
        { semconv::trace::HTTP_REQUEST_METHOD } = method,
        { semconv::trace::HTTP_ROUTE } = route_path,
        { semconv::trace::CODE_FUNCTION } = handler_name,
        { semconv::trace::CODE_NAMESPACE } = "spikard.handlers",

        // Spikard-specific attributes
        "spikard.handler.name" = handler_name,
        "spikard.handler.language" = "python",  // or "typescript", "rust"
        "spikard.handler.path_params" = ?request_data.path_params,
        "spikard.handler.query_params" = ?request_data.query_params,

        // Response attributes (added after handler completes)
        { semconv::trace::HTTP_RESPONSE_STATUS_CODE } = tracing::field::Empty,
        "spikard.response.body_size" = tracing::field::Empty,
    );

    // Execute handler within span
    let response = handler.call(request_data)
        .instrument(span.clone())
        .await?;

    // Record response attributes
    span.record(semconv::trace::HTTP_RESPONSE_STATUS_CODE, response.status_code);
    span.record("spikard.response.body_size", response.body.len());

    Ok(response)
}

#[cfg(not(feature = "otel"))]
async fn invoke_handler(
    handler_name: &str,
    method: &str,
    route_path: &str,
    request_data: RequestData,
) -> Result<Response, Error> {
    // No instrumentation overhead
    handler.call(request_data).await
}
```

### 2.3 Python Handler Context Propagation

Use `pyo3-opentelemetry` to propagate trace context into Python handlers:

```rust
// crates/spikard-py/src/lib.rs
use pyo3_opentelemetry::pypropagate;

#[cfg(feature = "otel")]
#[pyfunction]
#[pypropagate]  // Automatically propagates OpenTelemetry context
fn invoke_python_handler(
    py: Python,
    handler: PyObject,
    request_data: PyObject,
) -> PyResult<PyObject> {
    // Context is automatically propagated from Rust to Python
    // Python code can now use opentelemetry-api to create child spans
    handler.call1(py, (request_data,))
}

#[cfg(not(feature = "otel"))]
#[pyfunction]
fn invoke_python_handler(
    py: Python,
    handler: PyObject,
    request_data: PyObject,
) -> PyResult<PyObject> {
    // Direct call, no overhead
    handler.call1(py, (request_data,))
}
```

### 2.4 Python Handler Instrumentation (User Code)

Python handlers can create child spans using standard OpenTelemetry API:

```python
from spikard import get
from opentelemetry import trace

tracer = trace.get_tracer(__name__)

@get("/users/{user_id:int}")
async def get_user(user_id: int) -> dict:
    # Automatically part of the parent span from Rust
    with tracer.start_as_current_span("fetch_user_from_db") as span:
        span.set_attribute("user_id", user_id)

        # Your database call here
        user = await db.get_user(user_id)

        span.set_attribute("user.found", user is not None)
        return user
```

### 2.5 TypeScript Handler Context Propagation

Similar approach for Node.js bindings using OpenTelemetry JS API:

```typescript
// TypeScript handler with automatic context propagation
import { Spikard } from 'spikard';
import { trace } from '@opentelemetry/api';

const app = new Spikard();
const tracer = trace.getTracer('my-app');

app.get('/users/:userId', {
  params: z.object({ userId: z.number() }),
  handler: async ({ params }) => {
    // Context automatically propagated from Rust
    return await tracer.startActiveSpan('fetch_user', async (span) => {
      span.setAttribute('user_id', params.userId);

      const user = await db.getUser(params.userId);

      span.end();
      return user;
    });
  }
});
```

## 3. Configuration and Initialization

### 3.1 Rust OTEL Setup

```rust
#[cfg(feature = "otel")]
pub fn init_otel_tracing(
    service_name: &str,
    otlp_endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_sdk::{trace as sdktrace, Resource};
    use opentelemetry_semantic_conventions as semconv;

    // Create OTLP exporter
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_endpoint)
        )
        .with_trace_config(
            sdktrace::Config::default()
                .with_resource(Resource::new(vec![
                    KeyValue::new(semconv::resource::SERVICE_NAME, service_name.to_string()),
                    KeyValue::new(semconv::resource::SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
                ]))
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    // Set up tracing subscriber with OpenTelemetry layer
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}
```

### 3.2 Python API

```python
from spikard import Spikard
from spikard.observability import OpenTelemetry

app = Spikard(
    observability=OpenTelemetry(
        service_name="my-python-app",
        otlp_endpoint="http://localhost:4317",  # Jaeger, Tempo, etc.
        export_interval_millis=5000,
        # Optional: customize attributes
        resource_attributes={
            "deployment.environment": "production",
            "service.version": "1.0.0",
        }
    )
)
```

### 3.3 TypeScript API

```typescript
import { Spikard, OpenTelemetry } from 'spikard';

const app = new Spikard({
  observability: new OpenTelemetry({
    serviceName: 'my-typescript-app',
    otlpEndpoint: 'http://localhost:4317',
    exportIntervalMillis: 5000,
    resourceAttributes: {
      'deployment.environment': 'production',
      'service.version': '1.0.0',
    }
  })
});
```

### 3.4 Environment Variables

Standard OpenTelemetry environment variables are respected:

```bash
# Service identification
export OTEL_SERVICE_NAME="spikard-app"
export OTEL_RESOURCE_ATTRIBUTES="service.version=1.0.0,deployment.environment=prod"

# OTLP exporter configuration
export OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317"
export OTEL_EXPORTER_OTLP_PROTOCOL="grpc"  # or "http/protobuf"

# Trace sampling
export OTEL_TRACES_SAMPLER="parentbased_traceidratio"
export OTEL_TRACES_SAMPLER_ARG="0.1"  # Sample 10% of traces

# Semantic conventions
export OTEL_SEMCONV_STABILITY_OPT_IN="http"  # Use stable HTTP conventions
```

## 4. OpenAPI Generation

### 4.1 Strategy

Since we already have JSON Schema for all parameters and bodies, OpenAPI generation is straightforward:

```
JSON Schema (runtime) → OpenAPI 3.1 Spec (compile-time or startup)
```

### 4.2 Implementation Approach

**Option A: Runtime Generation (Simpler, Flexible)**
- Generate OpenAPI spec at application startup
- Convert JSON Schemas to OpenAPI schemas (1:1 mapping in OpenAPI 3.1)
- Serve spec at `/openapi.json` endpoint

**Option B: Compile-Time Generation with utoipa (Type-safe, Faster)**
- Use `utoipa` derive macros on Rust structs
- Generate OpenAPI at compile time
- Trade-off: Less flexible for dynamic schemas from Python/TypeScript

**Recommended: Hybrid Approach**
- Use JSON Schema → OpenAPI conversion (runtime)
- Cache the generated spec after first request
- Allow users to export spec to file for static hosting

### 4.3 JSON Schema to OpenAPI 3.1 Conversion

OpenAPI 3.1 fully supports JSON Schema Draft 2020-12, so conversion is mostly 1:1:

```rust
use schemars::schema::RootSchema;
use serde_json::{json, Value};

pub fn json_schema_to_openapi_schema(schema: &RootSchema) -> Value {
    // OpenAPI 3.1 uses JSON Schema directly
    // We just need to wrap it in OpenAPI structure
    serde_json::to_value(schema).expect("Valid JSON Schema")
}

pub fn generate_openapi_spec(app: &SpikardApp) -> Value {
    let mut paths = json!({});

    for route in &app.routes {
        let path_item = json!({
            route.method.as_str().to_lowercase(): {
                "operationId": route.handler_name,
                "summary": route.summary.as_deref().unwrap_or(""),
                "description": route.description.as_deref().unwrap_or(""),
                "parameters": route.parameters.iter().map(|p| {
                    json!({
                        "name": p.name,
                        "in": match p.source {
                            ParameterSource::Path => "path",
                            ParameterSource::Query => "query",
                            ParameterSource::Header => "header",
                            ParameterSource::Cookie => "cookie",
                        },
                        "required": p.required,
                        "schema": json_schema_to_openapi_schema(&p.schema),
                        "description": p.description.as_deref().unwrap_or(""),
                    })
                }).collect::<Vec<_>>(),
                "requestBody": route.body_schema.as_ref().map(|schema| {
                    json!({
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": json_schema_to_openapi_schema(schema)
                            }
                        }
                    })
                }),
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": route.response_schema.as_ref()
                                    .map(|s| json_schema_to_openapi_schema(s))
                                    .unwrap_or(json!({}))
                            }
                        }
                    },
                    "422": {
                        "description": "Validation error",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/ValidationError"
                                }
                            }
                        }
                    }
                }
            }
        });

        paths[&route.path] = path_item;
    }

    json!({
        "openapi": "3.1.0",
        "info": {
            "title": app.title.as_deref().unwrap_or("Spikard API"),
            "version": app.version.as_deref().unwrap_or("1.0.0"),
            "description": app.description.as_deref().unwrap_or(""),
        },
        "paths": paths,
        "components": {
            "schemas": {
                "ValidationError": {
                    "type": "object",
                    "required": ["detail"],
                    "properties": {
                        "detail": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "required": ["type", "loc", "msg"],
                                "properties": {
                                    "type": { "type": "string" },
                                    "loc": { "type": "array", "items": { "type": "string" } },
                                    "msg": { "type": "string" },
                                    "input": {},
                                    "ctx": { "type": "object" }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}
```

### 4.4 Python API

```python
from spikard import Spikard

app = Spikard(
    title="My API",
    version="1.0.0",
    description="A Spikard-powered API",
    openapi_url="/openapi.json",  # Enable OpenAPI endpoint
    docs_url="/docs",              # Enable Swagger UI
    redoc_url="/redoc",            # Enable ReDoc
)

# Serve OpenAPI spec
# GET /openapi.json -> returns OpenAPI 3.1 JSON spec
# GET /docs -> Swagger UI
# GET /redoc -> ReDoc documentation
```

### 4.5 TypeScript API

```typescript
import { Spikard } from 'spikard';

const app = new Spikard({
  title: 'My API',
  version: '1.0.0',
  description: 'A Spikard-powered API',
  openApiUrl: '/openapi.json',
  docsUrl: '/docs',
  redocUrl: '/redoc',
});
```

## 5. Semantic Conventions and Custom Attributes

### 5.1 Standard HTTP Semantic Conventions

Spikard automatically adds these attributes to HTTP server spans:

| Attribute | Type | Example | Source |
|-----------|------|---------|--------|
| `http.request.method` | string | `"GET"` | semconv::trace::HTTP_REQUEST_METHOD |
| `http.route` | string | `"/users/{user_id}"` | semconv::trace::HTTP_ROUTE |
| `http.response.status_code` | int | `200` | semconv::trace::HTTP_RESPONSE_STATUS_CODE |
| `server.address` | string | `"api.example.com"` | semconv::trace::SERVER_ADDRESS |
| `server.port` | int | `3000` | semconv::trace::SERVER_PORT |
| `url.path` | string | `"/users/123"` | semconv::trace::URL_PATH |
| `url.query` | string | `"page=1&limit=10"` | semconv::trace::URL_QUERY |
| `user_agent.original` | string | `"Mozilla/5.0..."` | semconv::trace::USER_AGENT_ORIGINAL |

### 5.2 Spikard-Specific Attributes

Custom attributes added by Spikard:

| Attribute | Type | Example | Description |
|-----------|------|---------|-------------|
| `spikard.handler.name` | string | `"get_user"` | Handler function name |
| `spikard.handler.language` | string | `"python"` | Binding language |
| `spikard.handler.path_params` | object | `{"user_id": "123"}` | Extracted path parameters |
| `spikard.handler.query_params` | object | `{"page": 1}` | Extracted query parameters |
| `spikard.validation.duration_us` | int | `42` | Validation time in microseconds |
| `spikard.response.body_size` | int | `1024` | Response body size in bytes |

### 5.3 User-Defined Attributes

Users can add custom attributes in handlers:

```python
from spikard import get
from opentelemetry import trace

@get("/users/{user_id:int}")
async def get_user(user_id: int) -> dict:
    span = trace.get_current_span()

    # Add custom attributes
    span.set_attribute("business.user_tier", "premium")
    span.set_attribute("business.feature_flag.new_ui", True)

    return {"id": user_id}
```

## 6. Performance Impact

### 6.1 Feature Flag Overhead

| Configuration | Overhead | Notes |
|---------------|----------|-------|
| `otel` feature disabled | **0ns** | Code is not compiled in |
| `otel` enabled, no exporter | **~5-10ns** | Null checks only |
| `otel` enabled, sampling 100% | **~200-500ns** | Span creation + attributes |
| `otel` enabled, sampling 10% | **~50-100ns** | Sampling decision + occasional span |

### 6.2 Trace Export Overhead

- **Batch export** (recommended): ~1-2ms per batch of 512 spans
- **Background thread**: Zero impact on request latency
- **OTLP/gRPC**: Most efficient, ~100 bytes per span

### 6.3 Recommendations

1. **Production**: Enable `otel` feature, use 1-10% sampling
2. **Staging**: Enable `otel`, use 50-100% sampling
3. **Development**: Disable `otel` for fastest iteration
4. **CI/CD**: Disable `otel` unless testing observability

## 7. Implementation Phases

### Phase 1: Core OTEL Integration (Week 1)
- [ ] Add `opentelemetry` dependencies with feature flag
- [ ] Integrate `axum-tracing-opentelemetry` middleware
- [ ] Implement automatic HTTP span creation
- [ ] Add semantic conventions for HTTP requests

### Phase 2: Handler Instrumentation (Week 2)
- [ ] Create handler-level spans with rich context
- [ ] Extract and record path/query parameters
- [ ] Record response attributes (status, size)
- [ ] Add Python binding integration with `pyo3-opentelemetry`

### Phase 3: Cross-Language Context Propagation (Week 3)
- [ ] Implement `pypropagate` macro for Python handlers
- [ ] Add TypeScript/Node.js context propagation
- [ ] Test trace continuity across Rust → Python/TypeScript
- [ ] Document user-facing APIs

### Phase 4: OpenAPI Generation (Week 4)
- [ ] Implement JSON Schema → OpenAPI 3.1 conversion
- [ ] Generate OpenAPI spec at startup (cached)
- [ ] Add `/openapi.json` endpoint
- [ ] Integrate Swagger UI at `/docs`
- [ ] Integrate ReDoc at `/redoc`

### Phase 5: Configuration and Docs (Week 5)
- [ ] Python API for observability configuration
- [ ] TypeScript API for observability configuration
- [ ] Environment variable support
- [ ] Comprehensive documentation with examples
- [ ] Migration guides for FastAPI/NestJS users

## 8. Testing Strategy

### 8.1 OTEL Integration Tests

```rust
#[cfg(all(test, feature = "otel"))]
mod otel_tests {
    use super::*;
    use opentelemetry::global;
    use opentelemetry_sdk::export::trace::SpanData;

    #[tokio::test]
    async fn test_handler_span_created() {
        // Setup in-memory exporter
        let (tracer, spans) = setup_test_tracer();

        // Make request
        let response = test_client
            .get("/users/123")
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);

        // Verify span was created
        let exported_spans = spans.lock().unwrap();
        assert_eq!(exported_spans.len(), 2); // HTTP span + handler span

        let handler_span = &exported_spans[1];
        assert_eq!(handler_span.name, "spikard.handler");
        assert_eq!(
            handler_span.attributes.get("spikard.handler.name"),
            Some(&"get_user".into())
        );
        assert_eq!(
            handler_span.attributes.get("http.route"),
            Some(&"/users/{user_id}".into())
        );
    }
}
```

### 8.2 Context Propagation Tests

```python
# packages/python/tests/test_otel_propagation.py
import pytest
from opentelemetry import trace
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import SimpleSpanProcessor
from opentelemetry.sdk.trace.export.in_memory_span_exporter import InMemorySpanExporter

def test_python_handler_context_propagation(test_client):
    """Verify trace context propagates from Rust to Python handler."""
    exporter = InMemorySpanExporter()
    provider = TracerProvider()
    provider.add_span_processor(SimpleSpanProcessor(exporter))
    trace.set_tracer_provider(provider)

    response = test_client.get("/users/123")
    assert response.status_code == 200

    spans = exporter.get_finished_spans()

    # Should have at least 2 spans: Rust HTTP span + Python handler span
    assert len(spans) >= 2

    # Verify parent-child relationship
    rust_span = next(s for s in spans if s.name == "spikard.handler")
    python_span = next(s for s in spans if s.name == "fetch_user_from_db")

    assert python_span.parent.span_id == rust_span.context.span_id
```

## 9. Benefits

### 9.1 Full-Stack Observability
- **Single trace** from HTTP request → Rust routing → Python/TypeScript handler → database
- **No manual propagation** - context automatically flows across language boundaries
- **Rich context** - All parameters, validation results, and custom attributes in one place

### 9.2 Performance Debugging
- Identify slow handlers with precise timing
- Measure validation overhead
- Track serialization/deserialization costs
- Spot N+1 query patterns in user code

### 9.3 Compliance and Auditing
- Full audit trail of all requests
- Track parameter values through the stack
- Correlate errors with specific requests
- Retention policies via OTEL backend

### 9.4 Developer Experience
- **OpenAPI docs** - Always up-to-date, generated from schemas
- **Swagger UI** - Interactive API testing
- **ReDoc** - Beautiful, responsive documentation
- **Type-safe** - OpenAPI matches runtime validation exactly

## 10. References

### Crates
- [opentelemetry-rust](https://github.com/open-telemetry/opentelemetry-rust) - Official Rust implementation
- [axum-tracing-opentelemetry](https://github.com/davidB/tracing-opentelemetry-instrumentation-sdk) - Axum integration
- [pyo3-opentelemetry](https://github.com/rigetti/pyo3-opentelemetry) - Python context propagation
- [utoipa](https://github.com/juhaku/utoipa) - OpenAPI generation (alternative approach)

### Specifications
- [OpenTelemetry Semantic Conventions](https://opentelemetry.io/docs/specs/semconv/) - Standard attributes
- [OpenAPI 3.1 Specification](https://spec.openapis.org/oas/v3.1.0) - API documentation format
- [JSON Schema 2020-12](https://json-schema.org/draft/2020-12/json-schema-core.html) - Schema format

### Examples
- [rust-tonic-tracing-otel](https://github.com/sdd/rust-tonic-tracing-otel) - PyO3 + OTEL example

---

**Key Takeaway:** By integrating OpenTelemetry in Rust with context propagation to Python/TypeScript, and generating OpenAPI from our existing JSON Schemas, we get production-grade observability and documentation with minimal overhead and zero user friction.
