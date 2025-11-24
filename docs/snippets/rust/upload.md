```rust
use spikard::prelude::*;
use spikard::UploadFile;

app.route(post("/upload"), |ctx: Context| async move {
    let upload: UploadFile = ctx.json()?;
    Ok(Json(json!({ "filename": upload.filename, "size": upload.size })))
})?;
```
