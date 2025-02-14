use std::env;

use access_grid::{create_service_routes, AccessGridConfig, AppState};
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
pub async fn health_check() {
    env::set_current_dir("../").ok();
    env::set_var("AG_ENVIRONMENT", "test");

    let app_config = AccessGridConfig::load_config();

    let app_state = AppState::builder()
        .with_db_config(app_config.database())
        .await
        .build()
        .expect("Error building Application Shared State");

    let service_routes = create_service_routes(app_state);
    let response = service_routes
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/access-grid/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK)
}
