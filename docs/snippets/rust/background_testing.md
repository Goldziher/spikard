```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, timeout, Duration};

    #[tokio::test]
    async fn test_background_task_completion() {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(1);

        tokio::spawn(async move {
            sleep(Duration::from_millis(100)).await;
            let _ = tx.send("task_complete".to_string()).await;
        });

        let result = timeout(Duration::from_secs(1), rx.recv())
            .await
            .expect("timeout")
            .expect("channel closed");

        assert_eq!(result, "task_complete");
    }

    #[tokio::test]
    async fn test_multiple_background_tasks() {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<u32>(10);

        for i in 0..5 {
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                sleep(Duration::from_millis(50 * i)).await;
                let _ = tx_clone.send(i).await;
            });
        }
        drop(tx); // Close sender to allow recv to complete

        let mut results = Vec::new();
        while let Ok(value) = timeout(Duration::from_secs(1), rx.recv()).await {
            if let Some(v) = value {
                results.push(v);
            } else {
                break;
            }
        }

        assert_eq!(results.len(), 5);
    }

    #[tokio::test]
    async fn test_task_error_handling() {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Result<String, String>>(1);

        tokio::spawn(async move {
            sleep(Duration::from_millis(50)).await;
            let _ = tx.send(Err("task_failed".to_string())).await;
        });

        let result = timeout(Duration::from_secs(1), rx.recv())
            .await
            .expect("timeout")
            .expect("channel closed");

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "task_failed");
    }
}
```
