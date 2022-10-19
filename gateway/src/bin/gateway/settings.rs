/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::{prelude::{config::{Config, Environment}, Logger, S3Region, Server}, ConfigResult, collect_config_files};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub gateway: S3Region,
    pub logger: Logger,
    pub server: Server
}

impl Settings {
    pub fn new(gateway: S3Region, logger: Logger, server: Server) -> Self {
        Self { gateway, logger, server}
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder();

        builder = builder.add_source(collect_config_files("**/default.config.*", true));
        builder = builder.add_source(collect_config_files("**/*.config.*", false));
        builder = builder.add_source(Environment::default().separator("__"));

        builder
            .build()
            .expect("Failed to build the configuration...")
            .try_deserialize()

    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::build().expect("Configuration failed...")
    }
}