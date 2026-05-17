---
id: rust_auth_middleware
language: rust
title: Auth Middleware
tags:
  - rust
---

```rust
use spikard::prelude::*;
use tower_http::auth::RequireAuthorizationLayer;

let mut app = App::new();

app.layer(RequireAuthorizationLayer::bearer("dev-token"));
```
