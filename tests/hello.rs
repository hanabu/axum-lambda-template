//!
//! Since app() can generate axum::Router,
//! you can easily test handler functions using ServiceExt.oneshot()
//!

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use axum_lambda_template::*;
use tower::ServiceExt;

/// Call hello() via Axum router
#[tokio::test]
async fn test_hello() {
    dotenv::dotenv().ok();

    let app = app().await;
    let resp = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    // Check HTTP status code
    assert_eq!(resp.status(), StatusCode::OK);

    // Check body
    let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let text_body = std::str::from_utf8(&body[..]).unwrap();
    assert!(text_body.contains("<p>Hello</p>"));
}
