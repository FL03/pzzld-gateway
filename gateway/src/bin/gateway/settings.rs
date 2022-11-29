/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::prelude::{
    collect_config_files,
    config::{Config, Environment},
    ConfigResult, Logger, S3Region, Server,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub gateway: S3Region,
    pub logger: Logger,
    pub server: Server,
}

impl Settings {
    pub fn new(gateway: S3Region, logger: Logger, server: Server) -> Self {
        Self {
            gateway,
            logger,
            server,
        }
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder().add_source(Environment::default().separator("__"));

        builder = builder.add_source(collect_config_files("**/Gateway.*", true));

        builder.build()?.try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        let d = Self::new(Default::default(), Default::default(), Default::default());
        Self::build().unwrap_or(d)
    }
}
