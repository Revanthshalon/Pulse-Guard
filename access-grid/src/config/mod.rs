#![allow(unused)]

mod database;
mod environment;
mod server;

use core::panic;
use std::env;

use environment::Environment;
use tracing::info;

pub use self::database::DatabaseConfig;
pub use self::server::ServerConfig;

/// The application configuration.
///
/// This struct is the central point for the entire application configuration. It holds the [`ServerConfig`] as well as [`DatabaseConfig`]and can be extended with any application-specific configuration settings that will be read from the main `app.toml` and the environment-specific configuration files.
#[derive(Debug, Clone)]
pub struct AccessGridConfig {
    /// the server configuration: [`ServerConfig`]
    server: ServerConfig,
    /// the database configuration: [`DatabaseConfig`]
    database: DatabaseConfig,
}

impl AccessGridConfig {
    pub fn load_config() -> Self {
        match Environment::get_env() {
            Environment::Test => {
                info!("Loading test configuration");
                let res = dotenvy::from_filename_override(".env.test");
                if let Err(e) = res {
                    panic!("Error Loading Config File: {e}");
                }
            }
            Environment::Development => {
                info!("Loading development configuration");
                let res = dotenvy::from_filename_override(".env");
                if let Err(e) = res {
                    panic!("Error Loading Config File: {e}");
                }
            }
            Environment::Production => {
                info!("Loading production configuration");
            }
        };

        let server_config = ServerConfig::load_from_env();
        let database_config = DatabaseConfig::load_from_env();

        Self {
            server: server_config,
            database: database_config,
        }
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }
}

#[cfg(test)]
mod tests {
    use std::{
        env,
        net::{IpAddr, Ipv4Addr},
    };

    use figment::Figment;

    use super::*;

    #[test]
    fn test_load_config_development() {
        let ag_config = AccessGridConfig::load_config();

        let config = AccessGridConfig {
            server: ServerConfig::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000),
            database: DatabaseConfig::new(
                "localhost",
                5433,
                "development",
                "password",
                "access_grid",
                100,
                30,
            ),
        };

        assert_eq!(config.server(), ag_config.server());
        assert_eq!(config.database(), ag_config.database());
    }

    #[test]
    fn test_load_config_test() {
        env::set_var("AG_ENVIRONMENT", "test");

        let ag_config = AccessGridConfig::load_config();

        let config = AccessGridConfig {
            server: ServerConfig::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000),
            database: DatabaseConfig::new("localhost", 5433, "test", "password", "test", 1, 5),
        };

        assert_eq!(config.server(), ag_config.server());
        assert_eq!(config.database(), ag_config.database());
    }

    #[test]
    fn test_load_config_prod() {
        env::set_var("AG_ENVIRONMENT", "prod");

        let ag_config = AccessGridConfig::load_config();

        let config = AccessGridConfig {
            server: ServerConfig::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000),
            database: DatabaseConfig::new(
                "production",
                5432,
                "access_grid",
                "password",
                "access_grid",
                100,
                30,
            ),
        };

        assert_eq!(config.server(), ag_config.server());
        assert_eq!(config.database(), ag_config.database());
    }
}
