---
id: rust_middleware_basic
language: rust
title: Middleware Basic
tags:
  - rust
---

```rust
use tower_http::trace::TraceLayer;

let mut app = App::new();
app.layer(TraceLayer::new_for_http());
```
