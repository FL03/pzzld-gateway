/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::prelude::{
    try_collect_config_files,
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
        let mut builder = Config::builder()
            .add_source(Environment::default().separator("__"))
            .set_default("gateway.endpoint", "https://gateway.storjshare.io")?
            .set_default("gateway.region", "us-east-1")?
            .set_default("logger.level", "info")?
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 9000)?;

        match try_collect_config_files("**/Gateway.*", false) {
            Err(_) => {},
            Ok(v) => { builder = builder.add_source(v); }
        };

        match std::env::var("RUST_LOG") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("logger.level", Some(v))?;
            }
        };

        match std::env::var("SERVER_PORT") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("server.port", v)?;
            }
        };

        builder.build()?.try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        let d = Self::new(Default::default(), Default::default(), Server::new("127.0.0.1".to_string(), 9000));
        Self::build().unwrap_or(d)
    }
}
