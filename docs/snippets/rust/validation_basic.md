```rust
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
struct Payment {
    id: String,
    amount: f64,
}

app.route(
    post("/payments").request_body::<Payment>().response_body::<Payment>(),
    |ctx: Context| async move {
        let payment: Payment = ctx.json()?;
        Ok(Json(payment))
    },
)?;
```
