use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct App {
    pub port: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Logging {
    pub filters: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Configuration {
    pub app: App,
    pub database: Database,
    pub logging: Logging,
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        /*
            evaluation order: later overrides
            1. default file
            2. profile file
            3. local file (don't push)
            4. env vars

            Option => optional, else required
        */

        let mut s = Config::new();

        let config_path = env::var("APP_CONFIG_PATH").unwrap_or_else(|_| "config".into());

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name(&format!("{}/default", config_path)))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let profile = env::var("APP_PROFILE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("{}/{}", config_path, profile)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name(&format!("{}/local", config_path)).required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DATABASE.URL=abc ./target/app` would set the `url` key in the database struct
        s.merge(Environment::with_prefix("APP"))?;

        // deserialize
        s.try_into()
    }
}
