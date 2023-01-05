/*
   Appellation: settings
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use scsys::prelude::config::{Config, Environment};
use scsys::Hash;
use scsys::{
    prelude::{Configurable, Hashable, Logger, Server},
    try_collect_config_files, ConfigResult,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OAuth2Config {
    #[serde(rename = "client_id")]
    pub id: String,
    #[serde(rename = "client_secret")]
    pub secret: String,
    pub redirect: String,
    pub scope: Option<String>,
    pub token: String
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub auth: OAuth2Config,
    pub mode: String,
    pub logger: Logger,
    pub server: Server,
}

impl Settings {
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder();
        // Set defaults
        builder = builder
            .set_default("mode", "production")?
            .set_default("logger.level", "info")?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?;
        // Load in the .env file
        builder = builder.add_source(Environment::default().separator("__"));
        // Load in configuration files following the *.config.* pattern
        if let Ok(v) = try_collect_config_files("**/*.config.*", false) {
            builder = builder.add_source(v);
        }
        // Check for alternative environment variable representations
        if let Ok(v) = std::env::var("CLIENT_ID") {
            builder = builder.set_override("auth.id", v)?;
        }
        if let Ok(v) = std::env::var("CLIENT_SECRET") {
            builder = builder.set_override("auth.secret", v)?;
        }
        if let Ok(v) = std::env::var("RUST_LOG") {
            builder = builder.set_override("logger.level", v)?;
        }
        if let Ok(v) = std::env::var("SERVER_PORT") {
            builder = builder.set_override("server.port", v)?;
        }
        // Attempt to build, then deserialize the configuration
        builder.build()?.try_deserialize()
    }
}

impl Configurable for Settings {
    type Settings = Self;

    fn settings(&self) -> &Self::Settings {
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        if let Ok(v) = Self::build() {
            v
        } else {
            Self {
                auth: Default::default(),
                mode: "production".to_string(),
                logger: Logger::default(),
                server: Server::new("0.0.0.0".to_string(), 8080),
            }
        }
    }
}

impl std::fmt::Display for OAuth2Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
