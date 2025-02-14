#![allow(unused)]

use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    config::DatabaseConfig,
    db::DbService,
    errors::{AccessGridErrors, AccessGridResult},
};

pub type SharedState = Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    db_service: DbService,
}

impl AppState {
    pub fn builder() -> AppStateBuilder {
        AppStateBuilder::default()
    }
}

#[derive(Default)]
pub struct AppStateBuilder {
    db_service: Option<DbService>,
}

impl AppStateBuilder {
    pub fn with_db_service(mut self, pool: PgPool) -> Self {
        self.db_service = Some(DbService::with_pool(pool));
        self
    }

    pub fn build(self) -> AccessGridResult<AppState> {
        let db_service = match self.db_service {
            Some(db_service) => db_service,
            None => {
                return Err(AccessGridErrors::Configuration(
                    "db_service should be initialized before building the AppState".to_string(),
                ))
            }
        };
        Ok(AppState { db_service })
    }
}
