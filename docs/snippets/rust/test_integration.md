```rust
use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use axum_test::TestServer;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct User {
    id: i64,
    name: String,
}

type UsersDb = Arc<Mutex<HashMap<i64, User>>>;

/// Test complete user creation and retrieval workflow.
#[tokio::test]
async fn test_user_workflow() {
    let users_db: UsersDb = Arc::new(Mutex::new(HashMap::new()));
    let db_clone = users_db.clone();

    let app = Router::new()
        .route(
            "/users",
            post({
                let db = users_db.clone();
                move |Json(payload): Json<serde_json::Value>| {
                    let db = db.clone();
                    async move {
                        let mut users = db.lock().expect("lock poisoned");
                        let id = (users.len() + 1) as i64;
                        let user = User {
                            id,
                            name: payload["name"].as_str().unwrap_or("").to_string(),
                        };
                        users.insert(id, user.clone());
                        Json(user)
                    }
                }
            }),
        )
        .route(
            "/users/:id",
            get({
                let db = db_clone;
                move |Path(user_id): Path<i64>| {
                    let db = db.clone();
                    async move {
                        let users = db.lock().expect("lock poisoned");
                        match users.get(&user_id) {
                            Some(user) => Json(json!(user)),
                            None => Json(json!({"error": "Not found"})),
                        }
                    }
                }
            }),
        );

    let server = TestServer::new(app).expect("failed to create test server");

    // Create user
    let create_response = server
        .post("/users")
        .json(&json!({"name": "Alice"}))
        .await;

    assert_eq!(create_response.status_code(), 200);
    let user: User = create_response.json();
    assert_eq!(user.name, "Alice");

    // Retrieve user
    let get_response = server.get(&format!("/users/{}", user.id)).await;
    assert_eq!(get_response.status_code(), 200);
    let retrieved: User = get_response.json();
    assert_eq!(retrieved, user);
}
```
