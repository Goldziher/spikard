use spikard::ServerConfig;

fn main() {
    let json = r#"{"host": "127.0.0.1", "port": 8000}"#;
    println!("Testing ServerConfig deserialization with JSON: {}", json);
    match serde_json::from_str::<ServerConfig>(json) {
        Ok(cfg) => {
            println!("Success!");
            println!("  host: {}", cfg.host);
            println!("  port: {}", cfg.port);
            println!("  workers: {}", cfg.workers);
            println!("  enable_request_id: {}", cfg.enable_request_id);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
