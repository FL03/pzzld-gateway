/*
    Appellation: gateways <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{config::*, gateway::*, utils::*};

pub(crate) mod config;
pub(crate) mod gateway;

pub(crate) mod utils {
    use s3::{creds::Credentials, Region};
    use scsys::prelude::S3Credential;
    
    pub fn convert_credentials(cred: S3Credential) -> Credentials {
        Credentials { 
            access_key: Some(cred.access_key), 
            secret_key: Some(cred.secret_key), 
            security_token: None, 
            session_token: None, 
            expiration: None 
        }
    }

    pub fn simple_creds(access_key: &str, secret_key: &str) -> Credentials {
        Credentials::new(Some(access_key), Some(secret_key), None, None, None).expect("msg")
    }

    pub fn simple_region<T: std::string::ToString>(endpoint: T, region: T) -> Region {
        Region::Custom {
            endpoint: endpoint.to_string(),
            region: region.to_string(),
        }
    }
}
