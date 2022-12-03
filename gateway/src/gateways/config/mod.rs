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
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct GatewayConfig {
        pub access_key: String,
        pub(crate) secret_key: String,
        pub endpoint: String,
        pub region: String
    }

    impl GatewayConfig {
        pub fn new(access_key: String, secret_key: String, endpoint: String, region: String) -> Self {
            Self { access_key, secret_key, endpoint, region, }
        }
        pub fn partial_env(access_key: Option<&str>, secret_key: Option<&str>, endpoint: String, region: String) -> scsys::BoxResult<Self> {
            let access_key = std::env::var(access_key.unwrap_or("S3_ACCESS_KEY"))?;
            let secret_key = std::env::var(secret_key.unwrap_or("S3_SECRET_KEY"))?;
            Ok(Self::new(access_key, secret_key, endpoint, region))
        }
    }

    impl Default for GatewayConfig {
        fn default() -> Self {
            Self::partial_env(None, None, "https://gateway.storjshare.io".to_string(), "us-east-1".to_string()).ok().unwrap()
        }
    }

}
