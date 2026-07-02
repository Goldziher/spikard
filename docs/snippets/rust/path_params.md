```rust
app.route(get("/orders/:order_id"), |ctx: Context| async move {
    let order_id: i64 = ctx.path_param::<String>("order_id")?
        .parse()
        .map_err(|_| Error::BadRequest("order_id must be a valid number"))?;

    #[derive(serde::Deserialize, Default)]
    struct DetailsQuery {
        details: Option<bool>,
    }
    let query: DetailsQuery = ctx.query().unwrap_or_default();

    Ok(Json(json!({
        "id": order_id,
        "details": query.details.unwrap_or(false)
    })))
})?;
```
