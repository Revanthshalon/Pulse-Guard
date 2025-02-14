use access_grid::create_service_routes;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
pub async fn health_check() {
    let service_routes = create_service_routes();
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
