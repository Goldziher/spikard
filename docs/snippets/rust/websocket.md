```rust
use spikard::prelude::*;
use futures::StreamExt;

app.websocket("/ws", |mut socket| async move {
    while let Some(msg) = socket.next().await {
        let text = msg.unwrap_or_default();
        socket.send(text).await.ok();
    }
});
```
