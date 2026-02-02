```rust
use spikard::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

struct TaskMonitor {
    active_tasks: Arc<AtomicU32>,
    completed_tasks: Arc<AtomicU32>,
}

impl TaskMonitor {
    fn new() -> Self {
        Self {
            active_tasks: Arc::new(AtomicU32::new(0)),
            completed_tasks: Arc::new(AtomicU32::new(0)),
        }
    }

    fn record_start(&self) {
        self.active_tasks.fetch_add(1, Ordering::SeqCst);
    }

    fn record_complete(&self) {
        self.active_tasks.fetch_sub(1, Ordering::SeqCst);
        self.completed_tasks.fetch_add(1, Ordering::SeqCst);
    }
}

let monitor = Arc::new(TaskMonitor::new());

app.route(get("/task-stats"), {
    let monitor = Arc::clone(&monitor);
    |_ctx: Context| async move {
        Ok(Json(json!({
            "active_tasks": monitor.active_tasks.load(Ordering::SeqCst),
            "completed_tasks": monitor.completed_tasks.load(Ordering::SeqCst),
        })))
    }
})?;

app.route(post("/process"), {
    let monitor = Arc::clone(&monitor);
    |ctx: Context| async move {
        let data = ctx.body_as::<ProcessRequest>().await?;

        let monitor_clone = Arc::clone(&monitor);
        tokio::spawn(async move {
            monitor_clone.record_start();
            if let Err(e) = process_data(&data).await {
                eprintln!("Processing failed: {}", e);
            }
            monitor_clone.record_complete();
        });

        Ok(Json(json!({"status": "processing"})))
    }
})?;

async fn process_data(data: &ProcessRequest) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[derive(serde::Deserialize)]
struct ProcessRequest {
    id: String,
}
```
