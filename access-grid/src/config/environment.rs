use std::env;

use tracing::info;

use crate::errors::AccessGridResult;

/// The environment the application runs in.
///
/// The application can run in 3 different environments: development, production, and test. Depending on the environment, the configuration might be different (e.g. different databases) or the application might behave differently.
#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Production,
    Development,
    Test,
}

impl Environment {
    pub fn get_env() -> Self {
        match env::var("AG_ENVIRONMENT") {
            Ok(value) => match Environment::parse_env(&value) {
                Ok(value) => value,
                Err(e) => panic!("{e}"),
            },
            Err(_) => {
                info!(
                    "Environment Variable `AG_ENVIROMENT` not specified. Defaulting to Development"
                );
                Environment::Development
            }
        }
    }

    fn parse_env(env: &str) -> AccessGridResult<Self> {
        let env = env.to_lowercase();
        match env.as_str() {
            "dev" | "development" => Ok(Environment::Development),
            "test" => Ok(Environment::Test),
            "prod" | "production" => Ok(Environment::Production),
            unknown => Err(crate::errors::AccessGridErrors::Configuration(format!(
                "Unknown environment variable: {}",
                unknown
            ))),
        }
    }
}
