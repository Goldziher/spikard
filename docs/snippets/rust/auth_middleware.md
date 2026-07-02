```rust
use spikard::prelude::*;
use tower_http::auth::RequireAuthorizationLayer;

let mut app = App::new();

app.layer(RequireAuthorizationLayer::bearer("dev-token"));
```
