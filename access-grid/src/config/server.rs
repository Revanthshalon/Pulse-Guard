#![allow(unused)]

use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

/// The server configuration.
///
/// This struct keeps all settings specific to the server – currently that is the interface the server binds to
/// but more might be added in the future. The struct is provided pre-defined by Gerust and cannot be changed. It
/// **must** be used for the `server` field in the application-specific [`Config`] struct:
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
pub struct ServerConfig {
    /// The port to bind to, e.g. 3000
    port: u16,

    /// The ip to bind to, e.g. 127.0.0.1
    ip: IpAddr,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 3000,
        }
    }
}

impl ServerConfig {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self { ip, port }
    }

    pub fn load_from_env() -> Self {
        let ip = env::var("AG_IP")
            .expect("Missing Environment Variable `AG_IP`. IP must be specified.")
            .parse::<IpAddr>()
            .expect("IP specified should be in the format `127.0.0.1`");
        let port = env::var("AG_PORT")
            .expect("Missing Environment Variable `AG_PORT`. Port must be specified.")
            .parse::<u16>()
            .expect("Port specified should be a number");

        Self { ip, port }
    }
    /// Returns the full address the server binds to, including both the ip and port.
    ///
    /// This can be used when creating a TCP Listener:
    ///
    /// ```rust
    /// let config: Config = load_config(Environment::Development);
    /// let listener = TcpListener::bind(&config.server.addr).await?;
    /// serve(listener, app.into_make_service()).await?;
    ///  ```
    pub fn addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }
}
