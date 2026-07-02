```rust
app.route(get("/health"), |_ctx: Context| async {
    Ok(Json(serde_json::json!({"status": "ok"})))
})?;
```
