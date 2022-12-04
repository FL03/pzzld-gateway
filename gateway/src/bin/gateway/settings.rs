/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use pzzld_gateway::gateways::GatewayConfig;
use s3::{error::S3Error, Bucket};
use scsys::prelude::{
    config::{Config, Environment},
    try_collect_config_files, ConfigResult, Logger, Server,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context {
    pub cnf: Settings,
}

impl Context {
    pub fn new(cnf: Settings) -> Self {
        Self { cnf }
    }
    pub fn credentials(&self) -> s3::creds::Credentials {
        self.cnf.gateway.credentials()
    }
    pub fn region(&self) -> s3::Region {
        self.cnf.gateway.region()
    }
    pub fn bucket(&self, name: &str) -> Result<Bucket, S3Error> {
        Bucket::new(name, self.region(), self.credentials())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub gateway: GatewayConfig,
    pub logger: Logger,
    pub server: Server,
}

impl Settings {
    pub fn new(gateway: GatewayConfig, logger: Logger, server: Server) -> Self {
        Self {
            gateway,
            logger,
            server,
        }
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder()
            .add_source(Environment::default().prefix("S3").separator("_"))
            .set_default("gateway.access_key", "")?
            .set_default("gateway.secret_key", "")?
            .set_default("gateway.endpoint", "https://gateway.storjshare.io")?
            .set_default("gateway.region", "us-east-1")?
            .set_default("logger.level", "info")?
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 9000)?;

        match try_collect_config_files("**/Gateway.*", false) {
            Err(_) => {}
            Ok(v) => {
                builder = builder.add_source(v);
            }
        };
        match std::env::var("S3_ACCESS_KEY") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("gateway.access_key", v)?;
            }
        };

        match std::env::var("S3_SECRET_KEY") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("gateway.secret_key", v)?;
            }
        };

        match std::env::var("RUST_LOG") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("logger.level", v)?;
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
        let d = Self::new(
            Default::default(),
            Default::default(),
            Server::new("127.0.0.1".to_string(), 9000),
        );
        Self::build().unwrap_or(d)
    }
}
