---
id: rust_routing_basic
language: rust
title: Routing Basic
tags:
  - rust
---

```rust
use spikard::prelude::*;

let mut app = App::new();

app.route(get("/health"), |_ctx: Context| async { Ok(Json(json!({"status": "ok"}))) })?;
app.route(post("/users"), |ctx: Context| async move {
    let user: serde_json::Value = ctx.json()?;
    Ok(Json(user))
})?;
```
