/*
    Appellation: gateway <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{GatewayCreds, S3Region};
use s3::{creds::Credentials, error::S3Error, Bucket, Region};
use scsys::prelude::Server;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GatewaySettings {
    pub creds: GatewayCreds,
    pub endpoint: String,
    pub region: String,
    pub server: Server,
}

pub struct GatewayContext {
    pub settings: GatewaySettings,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Gateway {
    creds: GatewayCreds,
    region: S3Region
}

impl Gateway {
    pub fn new(creds: GatewayCreds, region: S3Region) -> Self {
        Self { creds, region }
    }
    pub fn region(&self) -> Region {
        Region::Custom { endpoint: self.region.endpoint.clone(), region: self.region.region.clone() }
    }
    pub fn bucket(&self, name: &str) -> Result<Bucket, S3Error> {
        Bucket::new(name, self.region(), self.creds.clone().try_into()?)
    }
}

impl std::convert::From<&S3Region> for Gateway {
    fn from(value: &S3Region) -> Self {
        Self::new(Default::default(), value.clone())
    }
}

impl std::convert::From<&Gateway> for Credentials {
    fn from(value: &Gateway) -> Self {
        value.clone().creds.try_into().ok().unwrap()
    }
}

impl std::convert::From<&Gateway> for Region {
    fn from(value: &Gateway) -> Self {
        value.region()
    }
}