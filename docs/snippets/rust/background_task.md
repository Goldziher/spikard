```rust
use spikard::prelude::*;
use tokio::task;

app.route(post("/send-email"), |ctx: Context| async move {
    let email = ctx.body_as::<EmailRequest>().await?;

    // Spawn background task without waiting for completion
    task::spawn(async move {
        if let Err(e) = send_email_internal(&email).await {
            eprintln!("Email send failed: {}", e);
        }
    });

    Ok(Json(json!({
        "status": "queued",
        "message": "Email will be sent shortly"
    })))
})?;

async fn send_email_internal(email: &EmailRequest) -> Result<(), Box<dyn std::error::Error>> {
    // Simulate async email sending
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("Email sent to: {}", email.to);
    Ok(())
}

#[derive(serde::Deserialize)]
struct EmailRequest {
    to: String,
    subject: String,
}
```
