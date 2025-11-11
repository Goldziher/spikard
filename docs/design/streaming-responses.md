# Streaming Responses Implementation Guide

**Date:** November 2025
**Status:** 🟡 Design Complete, Implementation Pending
**Related Docs:** [architecture.md](./architecture.md), [middleware-lifecycle-optimization.md](./middleware-lifecycle-optimization.md)

## Executive Summary

This document provides a concrete implementation plan for streaming large HTTP responses in Spikard, leveraging Axum's native `Body::from_stream()` capability. Streaming responses enable efficient handling of large datasets, file downloads, and real-time data feeds without loading entire payloads into memory.

**Key Design Principles:**
- ✅ Leverage Axum's `Body::from_stream()` for chunked transfer encoding
- ✅ Support streaming from async iterators/generators in all language bindings
- ✅ Zero-copy streaming where possible
- ✅ Automatic backpressure handling via Tokio streams
- ✅ Idiomatic APIs for Python (async generators), Node.js (async iterators), Ruby (Enumerator)

## Overview

### Goals

1. Enable handlers to return streaming responses for large data
2. Support chunked transfer encoding automatically via Axum
3. Provide idiomatic streaming APIs for Python, TypeScript, and Ruby
4. Maintain memory efficiency - stream data instead of buffering
5. Support arbitrary content types (JSON lines, CSV, binary, etc.)
6. Integrate seamlessly with existing Handler trait

### Non-Goals

- Server-Sent Events (SSE) - covered in separate design doc
- WebSocket support - covered in separate design doc
- Custom streaming protocols (use Axum's Body abstraction)
- Response compression (handled by tower-http middleware)

## Axum's Streaming Approach

Axum provides first-class support for streaming responses via the `Body` type:

```rust
use axum::{
    body::Body,
    response::Response,
};
use futures::stream::{self, Stream};
use bytes::Bytes;

// Create a response from any stream of Bytes
let stream = stream::iter(vec![
    Ok::<_, std::io::Error>(Bytes::from("chunk1")),
    Ok(Bytes::from("chunk2")),
    Ok(Bytes::from("chunk3")),
]);

let body = Body::from_stream(stream);
let response = Response::new(body);
```

**Key Characteristics:**
- `Body::from_stream()` accepts any `impl Stream<Item = Result<Bytes, E>>` where `E: Into<BoxError>`
- Stream must be `Send + 'static` for multi-threaded runtime
- Axum automatically uses chunked transfer encoding when streaming
- Backpressure is handled automatically by Tokio's stream infrastructure
- Errors in the stream are propagated to the client

## Implementation Strategy

### Phase 1: Rust Core (1-2 days)

**File:** `crates/spikard-http/src/handler.rs` (update)

Extend the Handler trait to support streaming responses:

```rust
use axum::body::Body;
use futures::stream::Stream;
use bytes::Bytes;

/// Result from a streaming handler
pub enum HandlerResponse {
    /// Standard response with body already constructed
    Response(Response<Body>),

    /// Streaming response - body will be constructed from stream
    Stream {
        /// The stream of chunks
        stream: Pin<Box<dyn Stream<Item = Result<Bytes, BoxError>> + Send>>,
        /// HTTP status code
        status: StatusCode,
        /// Response headers
        headers: HeaderMap,
    },
}

impl From<Response<Body>> for HandlerResponse {
    fn from(response: Response<Body>) -> Self {
        HandlerResponse::Response(response)
    }
}

impl HandlerResponse {
    /// Convert to Axum response
    pub fn into_response(self) -> Response<Body> {
        match self {
            HandlerResponse::Response(r) => r,
            HandlerResponse::Stream { stream, status, headers } => {
                let body = Body::from_stream(stream);
                let mut response = Response::new(body);
                *response.status_mut() = status;
                *response.headers_mut() = headers;
                response
            }
        }
    }

    /// Create a streaming response
    pub fn stream<S, E>(stream: S) -> Self
    where
        S: Stream<Item = Result<Bytes, E>> + Send + 'static,
        E: Into<BoxError>,
    {
        let mapped_stream = stream.map(|result| result.map_err(Into::into));
        HandlerResponse::Stream {
            stream: Box::pin(mapped_stream),
            status: StatusCode::OK,
            headers: HeaderMap::new(),
        }
    }

    /// Set status code for streaming response
    pub fn with_status(mut self, status: StatusCode) -> Self {
        if let HandlerResponse::Stream { status: s, .. } = &mut self {
            *s = status;
        }
        self
    }

    /// Add header to streaming response
    pub fn with_header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        if let HandlerResponse::Stream { headers, .. } = &mut self {
            headers.insert(key, value);
        }
        self
    }
}
```

**File:** `crates/spikard-http/src/streaming.rs` (new)

Utilities for common streaming scenarios:

```rust
use bytes::Bytes;
use futures::stream::{Stream, StreamExt};
use serde::Serialize;
use std::io;

/// Stream JSON objects line-by-line (JSON Lines format)
pub fn json_lines_stream<S, T>(stream: S) -> impl Stream<Item = Result<Bytes, io::Error>>
where
    S: Stream<Item = T>,
    T: Serialize,
{
    stream.map(|item| {
        serde_json::to_vec(&item)
            .map(|mut bytes| {
                bytes.push(b'\n');
                Bytes::from(bytes)
            })
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    })
}

/// Stream CSV rows
pub fn csv_stream<S, T>(stream: S) -> impl Stream<Item = Result<Bytes, io::Error>>
where
    S: Stream<Item = T>,
    T: Serialize,
{
    stream.enumerate().map(|(index, item)| {
        let mut writer = csv::Writer::from_writer(Vec::new());

        // Write header on first row
        if index == 0 {
            writer.serialize(&item)?;
        } else {
            writer.serialize(&item)?;
        }

        let bytes = writer.into_inner()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(Bytes::from(bytes))
    })
}

/// Stream file chunks
pub async fn file_stream(path: impl AsRef<Path>) -> io::Result<impl Stream<Item = Result<Bytes, io::Error>>> {
    use tokio::fs::File;
    use tokio_util::io::ReaderStream;

    let file = File::open(path).await?;
    let reader_stream = ReaderStream::new(file);
    Ok(reader_stream.map(|result| result.map(|bytes| bytes.freeze())))
}
```

**File:** `crates/spikard-http/src/server.rs` (update)

Handle streaming responses in the request pipeline:

```rust
async fn handle_request(
    handler: Arc<dyn Handler>,
    req: Request<Body>,
) -> Response<Body> {
    match handler.handle(req).await {
        Ok(handler_response) => handler_response.into_response(),
        Err(e) => error_response(e),
    }
}
```

### Phase 2: Python Bindings (1-2 days)

**File:** `crates/spikard-py/src/response.rs` (update)

Support streaming responses from Python async generators:

```rust
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};
use futures::stream::{Stream, StreamExt};
use bytes::Bytes;

#[pyclass]
pub struct StreamingResponse {
    #[pyo3(get, set)]
    pub status_code: u16,

    #[pyo3(get, set)]
    pub headers: Py<PyDict>,

    /// Python async generator or async iterator
    stream: Py<PyAny>,
}

#[pymethods]
impl StreamingResponse {
    #[new]
    #[pyo3(signature = (stream, *, status_code=200, headers=None))]
    fn new(
        stream: Py<PyAny>,
        status_code: u16,
        headers: Option<Py<PyDict>>,
    ) -> PyResult<Self> {
        let headers = headers.unwrap_or_else(|| {
            Python::with_gil(|py| PyDict::new(py).into())
        });

        Ok(Self {
            status_code,
            headers,
            stream,
        })
    }
}

impl StreamingResponse {
    /// Convert Python async generator to Rust stream
    pub fn to_stream(&self) -> impl Stream<Item = Result<Bytes, BoxError>> + Send + 'static {
        let stream = self.stream.clone();

        async_stream::stream! {
            loop {
                // Get next item from Python async iterator
                let chunk = Python::with_gil(|py| -> PyResult<Option<Bytes>> {
                    let awaitable = stream.call_method0(py, "__anext__")?;

                    // Convert coroutine to future
                    let future = pyo3_async_runtimes::tokio::into_future(awaitable)?;
                    Ok(Some(future))
                })?;

                if let Some(future) = chunk {
                    // Await the future outside GIL
                    match future.await {
                        Ok(py_bytes) => {
                            let bytes = Python::with_gil(|py| {
                                // Handle both str and bytes
                                if let Ok(s) = py_bytes.downcast::<PyString>(py) {
                                    Ok(Bytes::from(s.to_string()))
                                } else if let Ok(b) = py_bytes.downcast::<PyBytes>(py) {
                                    Ok(Bytes::copy_from_slice(b.as_bytes()))
                                } else {
                                    Err("Stream must yield str or bytes")
                                }
                            })?;
                            yield Ok(bytes);
                        }
                        Err(e) => {
                            // Check if it's StopAsyncIteration
                            if Python::with_gil(|py| {
                                e.is_instance_of::<pyo3::exceptions::PyStopAsyncIteration>(py)
                            }) {
                                break;
                            }
                            yield Err(e.into());
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    /// Convert to HandlerResponse
    pub fn to_handler_response(&self) -> PyResult<HandlerResponse> {
        let stream = self.to_stream();
        let status = StatusCode::from_u16(self.status_code)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let mut headers = HeaderMap::new();
        Python::with_gil(|py| {
            for (key, value) in self.headers.bind(py).iter() {
                let key_str: String = key.extract()?;
                let value_str: String = value.extract()?;
                headers.insert(
                    HeaderName::from_bytes(key_str.as_bytes())?,
                    HeaderValue::from_str(&value_str)?,
                );
            }
            Ok::<_, PyErr>(())
        })?;

        Ok(HandlerResponse::Stream {
            stream: Box::pin(stream),
            status,
            headers,
        })
    }
}
```

**File:** `packages/python/spikard/response.py` (update)

```python
from typing import AsyncIterator, Iterator, Union
from collections.abc import AsyncIterable, Iterable

class StreamingResponse:
    """Streaming HTTP response for large data

    Accepts an async generator or async iterator that yields chunks.
    Each chunk can be str (encoded as UTF-8) or bytes.

    Examples:
        Stream JSON lines:
        ```python
        async def generate_data():
            for i in range(1000):
                yield json.dumps({"id": i, "value": i * 2}) + "\\n"

        @app.get("/data")
        async def stream_data():
            return StreamingResponse(
                generate_data(),
                headers={"content-type": "application/x-ndjson"}
            )
        ```

        Stream file chunks:
        ```python
        async def stream_file():
            async with aiofiles.open("large_file.dat", "rb") as f:
                while chunk := await f.read(8192):
                    yield chunk

        @app.get("/download")
        async def download():
            return StreamingResponse(
                stream_file(),
                headers={
                    "content-type": "application/octet-stream",
                    "content-disposition": "attachment; filename=file.dat"
                }
            )
        ```
    """

    def __init__(
        self,
        content: Union[AsyncIterable[Union[str, bytes]], Iterable[Union[str, bytes]]],
        *,
        status_code: int = 200,
        headers: dict[str, str] | None = None,
    ):
        """Create a streaming response

        Args:
            content: Async generator/iterator yielding str or bytes chunks
            status_code: HTTP status code (default 200)
            headers: Response headers
        """
        self.content = content
        self.status_code = status_code
        self.headers = headers or {}

        # Ensure content-type is set
        if "content-type" not in {k.lower() for k in self.headers}:
            self.headers["content-type"] = "application/octet-stream"
```

**Usage Example:**

```python
from spikard import Spikard, StreamingResponse
import json

app = Spikard()

@app.get("/stream-json")
async def stream_json_lines():
    """Stream large dataset as JSON lines"""
    async def generate_data():
        # Simulate fetching data from database in chunks
        for page in range(100):
            records = await fetch_records(page=page, limit=100)
            for record in records:
                yield json.dumps(record) + "\n"

    return StreamingResponse(
        generate_data(),
        headers={"content-type": "application/x-ndjson"}
    )

@app.get("/stream-csv")
async def stream_csv():
    """Stream large dataset as CSV"""
    async def generate_csv():
        # Header row
        yield "id,name,value\n"

        # Data rows
        for page in range(100):
            records = await fetch_records(page=page, limit=100)
            for record in records:
                yield f"{record['id']},{record['name']},{record['value']}\n"

    return StreamingResponse(
        generate_csv(),
        headers={
            "content-type": "text/csv",
            "content-disposition": "attachment; filename=data.csv"
        }
    )

@app.get("/download/{file_id}")
async def download_file(file_id: str):
    """Stream file download"""
    import aiofiles

    file_path = await get_file_path(file_id)
    file_name = file_path.name

    async def stream_file():
        async with aiofiles.open(file_path, "rb") as f:
            while chunk := await f.read(8192):  # 8KB chunks
                yield chunk

    return StreamingResponse(
        stream_file(),
        headers={
            "content-type": "application/octet-stream",
            "content-disposition": f"attachment; filename={file_name}"
        }
    )
```

### Phase 3: TypeScript Bindings (1-2 days)

**File:** `crates/spikard-node/src/response.rs` (update)

```rust
use napi::bindgen_prelude::*;
use napi::JsUnknown;
use futures::stream::Stream;

#[napi]
pub struct StreamingResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    stream: AsyncIterator,
}

#[napi]
impl StreamingResponse {
    #[napi(constructor)]
    pub fn new(
        stream: JsUnknown,
        status_code: Option<u16>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<Self> {
        Ok(Self {
            status_code: status_code.unwrap_or(200),
            headers: headers.unwrap_or_default(),
            stream: AsyncIterator::from_js(stream)?,
        })
    }

    pub fn to_stream(&self) -> impl Stream<Item = Result<Bytes, BoxError>> + Send + 'static {
        // Convert JS async iterator to Rust stream
        self.stream.into_rust_stream()
    }
}
```

**Usage Example:**

```typescript
import { Spikard, StreamingResponse } from '@spikard/node';

const app = new Spikard();

app.get('/stream-json', async () => {
  async function* generateData() {
    for (let page = 0; page < 100; page++) {
      const records = await fetchRecords({ page, limit: 100 });
      for (const record of records) {
        yield JSON.stringify(record) + '\n';
      }
    }
  }

  return new StreamingResponse(generateData(), {
    statusCode: 200,
    headers: { 'content-type': 'application/x-ndjson' }
  });
});

app.get('/download/:fileId', async (request) => {
  const filePath = await getFilePath(request.params.fileId);
  const fileStream = fs.createReadStream(filePath, { highWaterMark: 8192 });

  async function* streamFile() {
    for await (const chunk of fileStream) {
      yield chunk;
    }
  }

  return new StreamingResponse(streamFile(), {
    headers: {
      'content-type': 'application/octet-stream',
      'content-disposition': `attachment; filename=${path.basename(filePath)}`
    }
  });
});
```

### Phase 4: Ruby Bindings (1-2 days)

**File:** `crates/spikard-rb/src/response.rs` (update)

```rust
use magnus::{prelude::*, RHash, Value};

pub struct StreamingResponse {
    status_code: u16,
    headers: HashMap<String, String>,
    enumerator: Opaque<Value>,
}

impl StreamingResponse {
    pub fn to_stream(&self) -> impl Stream<Item = Result<Bytes, BoxError>> + Send + 'static {
        let enumerator = self.enumerator.clone();

        async_stream::stream! {
            loop {
                // Call next on the enumerator
                let result = Ruby::with(|ruby| {
                    let enum_obj = enumerator.get(ruby);
                    enum_obj.funcall::<_, _, Value>("next", ())
                });

                match result {
                    Ok(value) => {
                        let bytes = Ruby::with(|ruby| {
                            if let Ok(s) = value.try_convert::<String>() {
                                Ok(Bytes::from(s))
                            } else if let Ok(b) = value.try_convert::<Vec<u8>>() {
                                Ok(Bytes::from(b))
                            } else {
                                Err("Stream must yield String or byte array")
                            }
                        })?;
                        yield Ok(bytes);
                    }
                    Err(_) => break, // StopIteration
                }
            }
        }
    }
}
```

**Usage Example:**

```ruby
require 'spikard'

app = Spikard::App.new

app.get('/stream-json', handler_name: 'stream_json') do
  # Return an Enumerator that yields chunks
  Enumerator.new do |yielder|
    100.times do |page|
      records = fetch_records(page: page, limit: 100)
      records.each do |record|
        yielder << "#{record.to_json}\n"
      end
    end
  end
end

app.get('/download/:file_id', handler_name: 'download') do |request|
  file_id = request.params['file_id']
  file_path = get_file_path(file_id)

  # Stream file in chunks
  Spikard::StreamingResponse.new(
    File.open(file_path).each(8192), # Read 8KB chunks
    status_code: 200,
    headers: {
      'content-type' => 'application/octet-stream',
      'content-disposition' => "attachment; filename=#{File.basename(file_path)}"
    }
  )
end
```

## Testing Strategy

### Unit Tests (Rust)

**File:** `crates/spikard-http/src/streaming.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream;

    #[tokio::test]
    async fn test_json_lines_stream() {
        let data = vec![
            serde_json::json!({"id": 1, "name": "Alice"}),
            serde_json::json!({"id": 2, "name": "Bob"}),
        ];

        let stream = json_lines_stream(stream::iter(data));
        let chunks: Vec<_> = stream.collect().await;

        assert_eq!(chunks.len(), 2);
        assert!(chunks[0].as_ref().unwrap().ends_with(b"\n"));
    }

    #[tokio::test]
    async fn test_handler_response_stream() {
        let stream = stream::iter(vec![
            Ok::<_, std::io::Error>(Bytes::from("chunk1")),
            Ok(Bytes::from("chunk2")),
        ]);

        let response = HandlerResponse::stream(stream)
            .with_status(StatusCode::OK)
            .with_header(
                HeaderName::from_static("content-type"),
                HeaderValue::from_static("text/plain")
            );

        let axum_response = response.into_response();
        assert_eq!(axum_response.status(), StatusCode::OK);
        assert_eq!(
            axum_response.headers().get("content-type").unwrap(),
            "text/plain"
        );
    }
}
```

### Integration Tests (Python)

**File:** `packages/python/tests/test_streaming.py`

```python
import pytest
from spikard import Spikard, StreamingResponse
import json

@pytest.mark.asyncio
async def test_streaming_json_lines():
    """Test streaming JSON lines response"""
    app = Spikard()

    @app.get("/stream")
    async def stream_data():
        async def generate():
            for i in range(5):
                yield json.dumps({"id": i}) + "\n"

        return StreamingResponse(
            generate(),
            headers={"content-type": "application/x-ndjson"}
        )

    client = app.test_client()
    response = await client.get("/stream")

    assert response.status_code == 200
    assert response.headers["content-type"] == "application/x-ndjson"

    # Parse each line
    lines = response.text.strip().split("\n")
    assert len(lines) == 5
    for i, line in enumerate(lines):
        data = json.loads(line)
        assert data["id"] == i

@pytest.mark.asyncio
async def test_streaming_large_file():
    """Test streaming file download"""
    import tempfile
    import aiofiles

    app = Spikard()

    # Create temp file
    with tempfile.NamedTemporaryFile(delete=False) as f:
        f.write(b"x" * 100000)  # 100KB
        temp_path = f.name

    @app.get("/download")
    async def download():
        async def stream_file():
            async with aiofiles.open(temp_path, "rb") as f:
                while chunk := await f.read(8192):
                    yield chunk

        return StreamingResponse(stream_file())

    client = app.test_client()
    response = await client.get("/download")

    assert response.status_code == 200
    assert len(response.content) == 100000
    assert response.content == b"x" * 100000

@pytest.mark.asyncio
async def test_streaming_error_handling():
    """Test error handling in streams"""
    app = Spikard()

    @app.get("/stream-error")
    async def stream_with_error():
        async def generate():
            yield "chunk1\n"
            raise ValueError("Stream error")
            yield "chunk2\n"

        return StreamingResponse(generate())

    client = app.test_client()

    # Stream should be interrupted on error
    with pytest.raises(Exception):
        response = await client.get("/stream-error")
```

## Performance Considerations

### Memory Efficiency

**Streaming vs Buffering:**

| Approach | Memory Usage | Response Time |
|----------|--------------|---------------|
| Buffered (load entire 100MB file) | ~100MB | High latency before first byte |
| Streaming (8KB chunks) | ~8KB | Low latency, immediate start |

### Backpressure

Axum's streaming implementation automatically handles backpressure:
- If client is slow, Tokio will pause the stream producer
- Prevents unbounded memory growth
- No manual flow control needed

### Chunking Strategy

Recommended chunk sizes:
- **Text data (JSON, CSV):** 4-16KB per chunk
- **Binary files:** 8-64KB per chunk
- **Real-time data:** Smaller chunks for lower latency

## Implementation Checklist

### Rust Core
- [ ] Create `HandlerResponse` enum with Stream variant
- [ ] Implement `HandlerResponse::stream()` constructor
- [ ] Create `crates/spikard-http/src/streaming.rs` with utilities
- [ ] Add `json_lines_stream()` helper
- [ ] Add `csv_stream()` helper
- [ ] Add `file_stream()` helper
- [ ] Update server pipeline to handle streaming responses
- [ ] Write unit tests for streaming utilities
- [ ] Add benchmarks for stream overhead

### Python Bindings
- [ ] Create `StreamingResponse` class in `crates/spikard-py/src/response.rs`
- [ ] Implement async generator to Rust stream conversion
- [ ] Update Python API in `packages/python/spikard/response.py`
- [ ] Add type hints and docstrings
- [ ] Write integration tests for streaming
- [ ] Add example: JSON lines streaming
- [ ] Add example: File download streaming
- [ ] Add example: CSV streaming

### TypeScript Bindings
- [ ] Create `StreamingResponse` class in `crates/spikard-node/src/response.rs`
- [ ] Implement async iterator to Rust stream conversion
- [ ] Add TypeScript types for streaming
- [ ] Write integration tests
- [ ] Add examples

### Ruby Bindings
- [ ] Create `StreamingResponse` class in `crates/spikard-rb/src/response.rs`
- [ ] Implement Enumerator to Rust stream conversion
- [ ] Update Ruby API
- [ ] Write integration tests
- [ ] Add examples

### Documentation
- [ ] Add streaming section to README
- [ ] Create user guide with examples
- [ ] Document chunk size recommendations
- [ ] Add migration guide from buffered responses
- [ ] Document memory characteristics

## References

- **Axum Body::from_stream:** https://docs.rs/axum/latest/axum/body/struct.Body.html#method.from_stream
- **Tokio Streams:** https://docs.rs/tokio-stream/latest/tokio_stream/
- **futures Stream trait:** https://docs.rs/futures/latest/futures/stream/trait.Stream.html
- **async-stream crate:** https://docs.rs/async-stream/latest/async_stream/
- **Python async generators:** https://peps.python.org/pep-0525/
- **TypeScript async iterators:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncIterator

---

**Key Takeaway:** Streaming responses enable memory-efficient handling of large data by leveraging Axum's native `Body::from_stream()` capability, with idiomatic async generator/iterator APIs in Python, TypeScript, and Ruby.
