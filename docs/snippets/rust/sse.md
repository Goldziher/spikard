```rust
use spikard::prelude::*;
use tokio_stream::StreamExt;

app.route(get("/events"), |_ctx: Context| async move {
    let stream = tokio_stream::iter(0..3).map(|i| {
        format!("data: {}\n\n", serde_json::json!({"tick": i}))
    });
    Ok(StreamingBody::new(stream).with_header("content-type", "text/event-stream"))
})?;
```
