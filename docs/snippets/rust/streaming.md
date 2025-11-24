```rust
use spikard::prelude::*;
use tokio_stream::StreamExt;

app.route(get("/stream"), |_ctx: Context| async move {
    let stream = tokio_stream::iter(0..3).map(|i| {
        serde_json::to_vec(&serde_json::json!({ "tick": i })).unwrap()
    });
    Ok(StreamingBody::new(stream))
})?;
```
