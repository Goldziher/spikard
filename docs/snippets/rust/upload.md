```rust
use spikard::prelude::*;
use spikard::UploadFile;
use std::path::Path;
use tokio::fs;
use uuid::Uuid;

// Basic upload handler
app.route(post("/upload"), |ctx: Context| async move {
    let upload: UploadFile = ctx.json()?;
    Ok(Json(json!({ "filename": upload.filename, "size": upload.size })))
})?;

// Complete upload handler with validation and storage
app.route(post("/upload/complete"), |ctx: Context| async move {
    let upload: UploadFile = ctx.json()?;

    // Validate file size (10MB limit)
    const MAX_SIZE: usize = 10 * 1024 * 1024;
    if upload.content.len() > MAX_SIZE {
        return Err(format!("File size {} exceeds {} bytes", upload.content.len(), MAX_SIZE).into());
    }

    // Validate MIME type
    let allowed_types = vec!["image/jpeg", "image/png", "image/gif", "application/pdf"];
    if !allowed_types.contains(&upload.content_type.as_str()) {
        return Err(format!("File type {} not allowed", upload.content_type).into());
    }

    // Prevent path traversal - sanitize filename
    let safe_filename = Path::new(&upload.filename)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid filename")?;
    let unique_filename = format!("{}_{}", Uuid::new_v4(), safe_filename);

    // Save to local filesystem
    let upload_dir = Path::new("/var/uploads");
    fs::create_dir_all(upload_dir).await?;
    let file_path = upload_dir.join(&unique_filename);

    fs::write(&file_path, &upload.content).await?;

    Ok(Json(json!({
        "filename": safe_filename,
        "stored_as": unique_filename,
        "size": upload.content.len(),
        "content_type": upload.content_type,
        "url": format!("/files/{}", unique_filename)
    })))
})?;

// Upload to S3/cloud storage
app.route(post("/upload/s3"), |ctx: Context| async move {
    use aws_sdk_s3::Client;

    let upload: UploadFile = ctx.json()?;
    let config = aws_config::load_from_env().await;
    let s3_client = Client::new(&config);
    let bucket_name = "my-uploads-bucket";

    // Validate and sanitize
    let safe_filename = Path::new(&upload.filename)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid filename")?;
    let s3_key = format!("uploads/{}/{}", Uuid::new_v4(), safe_filename);

    // Upload to S3
    s3_client
        .put_object()
        .bucket(bucket_name)
        .key(&s3_key)
        .body(upload.content.into())
        .content_type(&upload.content_type)
        .send()
        .await?;

    Ok(Json(json!({
        "filename": safe_filename,
        "s3_key": s3_key,
        "url": format!("https://{}.s3.amazonaws.com/{}", bucket_name, s3_key)
    })))
})?;
```
