use axum::body::Body;
use axum::http::{Request, StatusCode};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use spikard::{App, RequestContext, post};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
struct Greeting {
    message: String,
}

#[tokio::test]
async fn app_builds_router_and_test_server_can_call_routes() {
    let mut app = App::new();
    app.route(
        post("/hello").request_body::<Greeting>().response_body::<Greeting>(),
        |ctx: RequestContext| async move {
            let body: Greeting = ctx.json()?;
            let response = serde_json::to_value(body).unwrap();
            Ok(axum::http::Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(response.to_string()))
                .unwrap())
        },
    )
    .unwrap();

    let server = spikard::testing::TestServer::from_app(app).expect("server");
    let request = Request::builder()
        .method("POST")
        .uri("http://localhost/hello")
        .header("content-type", "application/json")
        .body(Body::from(json!({"message":"hi"}).to_string()))
        .unwrap();

    let snapshot = server.call(request).await.expect("call");
    assert_eq!(snapshot.status, 200);
    assert_eq!(snapshot.json().unwrap(), json!({"message":"hi"}));
}
