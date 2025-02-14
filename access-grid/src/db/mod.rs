#![allow(unused)]

use sqlx::{postgres::PgPoolOptions, PgPool, PgTransaction};

use crate::{config::DatabaseConfig, errors::AccessGridResult};

#[derive(Debug)]
pub struct DbService {
    pool: PgPool,
}

impl DbService {
    pub async fn init(db_config: &DatabaseConfig) -> AccessGridResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(db_config.max_connections())
            .idle_timeout(db_config.connection_timeout())
            .connect(&db_config.url())
            .await?;

        Ok(Self { pool })
    }

    pub fn get_connection_pool(&self) -> PgPool {
        self.pool.clone()
    }

    pub fn with_pool(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn begin_transaction(pool: PgPool) -> AccessGridResult<PgTransaction<'static>> {
        let tx = pool.begin().await?;
        Ok(tx)
    }
}
