/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{creds::*, regions::*, settings::*};

pub(crate) mod creds;
pub(crate) mod regions;

pub(crate) mod settings {
    use super::*;
    use s3::{creds::Credentials, Region};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct GatewayConfig {
        pub access_key: String,
        pub(crate) secret_key: String,
        pub endpoint: String,
        pub region: String,
    }

    impl GatewayConfig {
        pub fn new(
            access_key: String,
            secret_key: String,
            endpoint: String,
            region: String,
        ) -> Self {
            Self {
                access_key,
                secret_key,
                endpoint,
                region,
            }
        }
        pub fn partial_env(
            access_key: Option<&str>,
            secret_key: Option<&str>,
            endpoint: String,
            region: String,
        ) -> scsys::BoxResult<Self> {
            let access_key = std::env::var(access_key.unwrap_or("S3_ACCESS_KEY"))?;
            let secret_key = std::env::var(secret_key.unwrap_or("S3_SECRET_KEY"))?;
            Ok(Self::new(access_key, secret_key, endpoint, region))
        }
        pub fn credentials(&self) -> Credentials {
            let cred = GatewayCreds::new(self.access_key.clone(), self.secret_key.clone());
            cred.try_into().ok().unwrap()
        }
        pub fn region(&self) -> Region {
            Region::Custom {
                endpoint: self.endpoint.clone(),
                region: self.region.clone(),
            }
        }
    }

    impl std::convert::From<&GatewayCreds> for GatewayConfig {
        fn from(data: &GatewayCreds) -> Self {
            let region = S3Region::default();
            Self::new(
                data.access_key.clone(),
                data.secret_key.clone(),
                region.endpoint(),
                region.region(),
            )
        }
    }

    impl std::convert::From<&S3Region> for GatewayConfig {
        fn from(data: &S3Region) -> Self {
            let cred = GatewayCreds::default();
            Self::new(
                cred.access_key.clone(),
                cred.secret_key.clone(),
                data.endpoint(),
                data.region(),
            )
        }
    }

    impl Default for GatewayConfig {
        fn default() -> Self {
            Self::partial_env(
                None,
                None,
                "https://gateway.storjshare.io".to_string(),
                "us-east-1".to_string(),
            )
            .ok()
            .unwrap()
        }
    }
}
