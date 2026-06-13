```mermaid
graph TD
  subgraph Client
    A[HTTP Client]
  end

  subgraph RustCore["Rust Core (Axum)"]
    B["Router &amp;<br/>Handler Dispatch"]
    C["Middleware Stack<br/>(Compression, Rate Limit,<br/>CORS, Auth)"]
    D["Validation Engine<br/>(Schema, Constraints)"]
    E["Lifecycle Hooks<br/>(Request, Pre-handler,<br/>Response, Error)"]
    F["WebSocket &amp; SSE"]
    G["FFI Bridge &amp;<br/>Handler Invocation"]
  end

  subgraph ErrorTypes["Error Hierarchy"]
    H1["NotFound 404"]
    H2["ValidationError 422"]
    H3["Unauthorized 401"]
    H4["Forbidden 403"]
    H5["RateLimited 429"]
    H6["Conflict 409"]
    H7["InternalError 500"]
  end

  subgraph Bindings["Language Bindings (15 total)"]
    P["Python<br/>PyO3 + msgspec"]
    N["Node/TS<br/>NAPI-RS + Zod"]
    R["Ruby<br/>Magnus"]
    PH["PHP<br/>ext-php-rs"]
    E["Elixir<br/>Rustler"]
    GO["Go<br/>cgo"]
    J["Java<br/>JNI/Panama"]
    CS["C#<br/>P/Invoke"]
    K["Kotlin<br/>JNI"]
    D["Dart<br/>FFI"]
    SW["Swift<br/>C FFI"]
    Z["Zig<br/>C ABI"]
    CR["C<br/>C ABI"]
    W["WASM<br/>Type Stubs"]
    RL["R<br/>Rdbi"]
  end

  subgraph TestClient["Testing"]
    TC["TestClient<br/>(In-process)"]
  end

  A --> B
  B --> C
  C --> D
  D --> E
  E --> F
  F --> G
  G --> H1 &amp; H2 &amp; H3 &amp; H4 &amp; H5 &amp; H6 &amp; H7
  G --> P &amp; N &amp; R &amp; PH &amp; E &amp; GO &amp; J &amp; CS &amp; K &amp; D &amp; SW &amp; Z &amp; CR &amp; W &amp; RL
  TC -.-> G
```
