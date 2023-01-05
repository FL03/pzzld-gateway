/*
    Appellation: gateway <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::config::{GatewayConfig, GatewayCreds, S3Region};
use s3::{creds::Credentials, error::S3Error, Bucket, Region};
use scsys::prelude::Contextual;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Gateway {
    pub cnf: GatewayConfig,
}

impl Gateway {
    pub fn new(creds: GatewayCreds, region: S3Region) -> Self {
        let cnf = GatewayConfig::new(
            creds.access_key.clone(),
            creds.secret_key,
            region.endpoint(),
            region.region(),
        );
        Self::from(cnf)
    }
    pub fn credentials(&self) -> Credentials {
        let cred = GatewayCreds::new(self.cnf.access_key.clone(), self.cnf.secret_key.clone());
        cred.try_into().ok().unwrap()
    }
    pub fn region(&self) -> Region {
        Region::Custom {
            endpoint: self.cnf.endpoint.clone(),
            region: self.cnf.region.clone(),
        }
    }
    pub fn bucket(&self, name: &str) -> Result<Bucket, S3Error> {
        Bucket::new(name, self.region(), self.credentials())
    }
}

impl Contextual for Gateway {
    type Cnf = GatewayConfig;

    type Ctx = Self;

    fn context(&self) -> &Self::Ctx {
        self
    }
}

impl From<GatewayConfig> for Gateway {
    fn from(cnf: GatewayConfig) -> Self {
        Self { cnf }
    }
}

impl From<&S3Region> for Gateway {
    fn from(value: &S3Region) -> Self {
        Self::new(Default::default(), value.clone())
    }
}

impl From<&Gateway> for Credentials {
    fn from(value: &Gateway) -> Self {
        value.credentials()
    }
}

impl From<&Gateway> for Region {
    fn from(value: &Gateway) -> Self {
        value.region()
    }
}

impl Default for Gateway {
    fn default() -> Self {
        Self::from(GatewayConfig::build().ok().unwrap())
    }
}

impl std::fmt::Debug for Gateway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

impl std::fmt::Display for Gateway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
