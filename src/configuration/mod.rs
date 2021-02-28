mod environment;
mod http_server_configuration;

use config::{Config, ConfigError, File};
use environment::Environment;
use http_server_configuration::HttpServerConfiguration;
use std::convert::TryInto;

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub http_server: HttpServerConfiguration,
}

impl Configuration {
    pub fn load(overrides: &[(&str, &str)]) -> Result<Configuration, ConfigError> {
        let configuration_directory = std::env::current_dir()
            .expect("Failed to determine current directory.")
            .join("configuration");

        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT.");

        let mut config = Config::default();
        config
            .merge(File::from(configuration_directory.join("default")).required(true))?
            .merge(File::from(configuration_directory.join(environment.as_str())).required(true))?
            .merge(config::Environment::with_prefix("APP").separator("__"))?;

        for &(key, value) in overrides {
            config.set(key, value)?;
        }

        config.try_into()
    }
}
