use std::sync::Arc;

use axum::Router;
use health::create_health_routes;

use crate::state::{AppState, SharedState};

mod health;

pub fn create_service_routes(state: AppState) -> Router {
    let shared_state: SharedState = Arc::new(state);

    let health_routes = Router::new().nest("/health", create_health_routes());

    let merged_routes = Router::new().merge(health_routes);

    Router::new().nest("/access-grid/api", merged_routes)
}
