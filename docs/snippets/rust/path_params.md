```rust
app.route(get("/orders/:order_id"), |ctx: Context| async move {
    let id = ctx.path_param("order_id").unwrap_or("0");
    let details: serde_json::Value = ctx.query().unwrap_or_default();
    Ok(Json(json!({
        "id": id.parse::<i64>().unwrap_or_default(),
        "details": details.get("details").and_then(|d| d.as_bool()).unwrap_or(false)
    })))
})?;
```
