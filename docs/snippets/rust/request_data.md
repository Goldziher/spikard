```rust
app.route(post("/orders/:order_id"), |ctx: Context| async move {
    let mut order: serde_json::Value = ctx.json()?;
    let id = ctx.path_param("order_id").unwrap_or("0");
    let verbose: serde_json::Value = ctx.query().unwrap_or_default();
    if let Some(map) = order.as_object_mut() {
        map.insert("id".into(), serde_json::json!(id.parse::<i64>().unwrap_or_default()));
        map.insert("verbose".into(), verbose.get("verbose").cloned().unwrap_or(serde_json::json!(false)));
    }
    Ok(Json(order))
})?;
```
