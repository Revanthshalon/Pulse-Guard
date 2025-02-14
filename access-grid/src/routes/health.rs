use axum::{routing::get, Router};

use crate::handlers::health;

pub fn create_health_routes() -> Router {
    Router::new().route("/", get(health::get_service_health))
}
