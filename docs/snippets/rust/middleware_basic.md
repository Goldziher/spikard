```rust
use tower_http::trace::TraceLayer;

let mut app = App::new();
app.layer(TraceLayer::new_for_http());
```
