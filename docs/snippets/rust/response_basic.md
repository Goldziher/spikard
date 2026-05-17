---
id: rust_response_basic
language: rust
title: Response Basic
tags:
  - rust
---

```rust
app.route(get("/health"), |_ctx: Context| async {
    Ok(Json(serde_json::json!({"status": "ok"})))
})?;
```
