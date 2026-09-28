//! Drives the rate limiter to its boundary through the full `router()`.
//!
//! The limits are read from `RWA_RATE_LIMIT_PER_SECOND` and
//! `RWA_RATE_LIMIT_BURST` when the router is built, so each test configures
//! them explicitly. Env access is serialized with a lock and restored before
//! any request is sent.

use std::net::SocketAddr;
use std::sync::Mutex;

use axum::{
    body::{to_bytes, Body},
    extract::ConnectInfo,
    http::{Request, StatusCode},
    response::Response,
    Router,
};
use tower::ServiceExt as _;

use crate::indexer::AppState;

use super::router;

static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Build a router whose limiter allows `burst` requests and then refills one
/// request every `per_second`-th of a second (a very slow refill here, so the
/// boundary is deterministic).
fn router_with_limits(per_second: u64, burst: u32) -> Router {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    std::env::set_var("RWA_RATE_LIMIT_PER_SECOND", per_second.to_string());
    std::env::set_var("RWA_RATE_LIMIT_BURST", burst.to_string());
    let app = router(AppState::for_test_empty());
    std::env::remove_var("RWA_RATE_LIMIT_PER_SECOND");
    std::env::remove_var("RWA_RATE_LIMIT_BURST");
    app
}

async fn get_from(app: &Router, ip: [u8; 4], uri: &str) -> Response {
    let mut request = Request::builder().uri(uri).body(Body::empty()).unwrap();
    request
        .extensions_mut()
        .insert(ConnectInfo(SocketAddr::from((ip, 54321))));
    app.clone().oneshot(request).await.unwrap()
}

#[tokio::test]
async fn requests_up_to_the_burst_succeed_and_the_next_is_throttled() {
    let burst = 3;
    let app = router_with_limits(1, burst);

    for n in 1..=burst {
        let response = get_from(&app, [10, 0, 0, 1], "/version").await;
        assert_eq!(response.status(), StatusCode::OK, "request {n} within burst");
    }

    let throttled = get_from(&app, [10, 0, 0, 1], "/version").await;
    assert_eq!(throttled.status(), StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn throttled_response_has_status_and_a_non_empty_body() {
    let app = router_with_limits(1, 1);

    assert_eq!(
        get_from(&app, [10, 0, 0, 2], "/version").await.status(),
        StatusCode::OK
    );
    let throttled = get_from(&app, [10, 0, 0, 2], "/version").await;

    assert_eq!(throttled.status(), StatusCode::TOO_MANY_REQUESTS);
    let body = to_bytes(throttled.into_body(), usize::MAX).await.unwrap();
    assert!(!body.is_empty(), "a throttled response explains itself");
}

#[tokio::test]
async fn limit_is_tracked_per_client_ip() {
    let app = router_with_limits(1, 1);

    assert_eq!(
        get_from(&app, [10, 0, 0, 3], "/version").await.status(),
        StatusCode::OK
    );
    assert_eq!(
        get_from(&app, [10, 0, 0, 3], "/version").await.status(),
        StatusCode::TOO_MANY_REQUESTS
    );
    // A different client still has its own full bucket.
    assert_eq!(
        get_from(&app, [10, 0, 0, 4], "/version").await.status(),
        StatusCode::OK
    );
}
