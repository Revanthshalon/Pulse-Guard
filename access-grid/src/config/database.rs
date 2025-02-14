use std::{env, time::Duration};

use tracing::info;

/// The database configuration.
///
/// This struct keeps all settings specific to the database – currently that is the database URL to use to connect to the database
/// but more might be added in the future.
///
/// ```rust
/// #[derive(Deserialize, Clone, Debug)]
/// pub struct Config {
///     #[serde(default)]
///     pub server: ServerConfig,
///     pub database: DatabaseConfig,
///     // add your config settings here…
/// }
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct DatabaseConfig {
    /// The host of the database connection, e.g. localhost
    host: String,

    /// The port of the database connection, e.g. 5432
    port: u16,

    /// The username to access the database
    username: String,

    /// The password to access the database
    password: String,

    /// The database name to connect
    database: String,

    /// Max. Connection Settings, e.g. 100
    max_connections: u32,

    /// Connection timeout settings (in seconds), e.g. 30
    connection_timeout: u64,
}

impl DatabaseConfig {
    pub fn new(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        database: &str,
        max_connections: u32,
        connection_timeout: u64,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            username: username.to_string(),
            password: password.to_string(),
            database: database.to_string(),
            max_connections,
            connection_timeout,
        }
    }
    pub fn load_from_env() -> Self {
        let host = env::var("AG_HOST")
            .expect("Missing Environment Variable `AG_HOST`. Database host must be specified.");

        let port = env::var("AG_DATABASE_PORT")
            .expect(
                "Missing Environment Variable `AG_DATABASE_PORT`. Database port must be specified.",
            )
            .parse::<u16>()
            .expect("Port must be specified as a number");

        let username = env::var("AG_USERNAME").expect(
            "Missing Environment Variable `AG_USERNAME`. Database username must be specified.",
        );

        let password = env::var("AG_PASSWORD").expect(
            "Missing Environment Variable `AG_PASSWORD`. Database password must be specified.",
        );

        let database = env::var("AG_DATABASE")
            .expect("Missing Environment Variable `AG_DATABASE`. Database name must be specified.");

        let max_connections = match env::var("AG_MAX_CONNECTIONS") {
            Ok(value) => value
                .parse::<u32>()
                .expect("Max Connections specified must be a number"),
            Err(_) => {
                info!("Missing Max Connection. Defaulting to 30");
                30
            }
        };

        let connection_timeout = match env::var("AG_CONNECTION_TIMEOUT") {
            Ok(value) => value
                .parse::<u64>()
                .expect("Connection Timeout must be specified as a number"),
            Err(_) => {
                info!("Missing Connection Timeout. Defaulting to 100 seconds");
                100
            }
        };

        Self {
            host,
            port,
            username,
            password,
            database,
            max_connections,
            connection_timeout,
        }
    }

    pub fn url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }

    pub fn max_connections(&self) -> u32 {
        self.max_connections
    }

    pub fn connection_timeout(&self) -> Duration {
        Duration::from_secs(self.connection_timeout)
    }
}
