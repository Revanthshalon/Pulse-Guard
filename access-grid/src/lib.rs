use config::AccessGridConfig;
use db::DbService;
use state::AppState;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod entities;
mod errors;
mod handlers;
mod middlewares;
mod repositories;
mod routes;
mod state;

pub use self::routes::create_service_routes;

pub async fn run_access_grid_service() -> Result<(), Box<dyn std::error::Error>> {
    // Tracing Setup
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().pretty().json())
        .init();

    let ag_config = AccessGridConfig::load_config();

    let db_config = DbService::init(ag_config.database())
        .await
        .expect("🔥 Error connecting to the Database");

    let app_state = AppState::builder()
        .with_db_service(db_config.get_connection_pool())
        .build()?;

    let service_routes = create_service_routes(app_state);

    let listener = TcpListener::bind(ag_config.server().addr())
        .await
        .expect("🔥 Error binding to listener port");

    axum::serve(listener, service_routes.into_make_service())
        .await
        .expect("🔥 Error starting up service");

    Ok(())
}
