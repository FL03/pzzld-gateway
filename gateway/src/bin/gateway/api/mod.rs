/*
    Appellation: api <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{docs::*, interface::*, utils::*};

pub(crate) mod docs;
pub(crate) mod interface;
pub mod routes;
pub mod schemas;

pub(crate) mod utils {
    use pzzld_gateway::gateways::Gateway;
    use s3::serde_types::ListBucketResult;
    use scsys::BoxResult;

    ///
    pub async fn collect_obj_names(objects: Vec<ListBucketResult>) -> Vec<String> {
        tracing::info!("Collecting information on the given data...");
        objects
            .iter()
            .map(|i| i.clone().name)
            .collect::<Vec<String>>()
    }
    ///
    pub async fn fetch_bucket_contents(
        bucket: s3::Bucket,
        prefix: &str,
        delim: Option<String>,
    ) -> BoxResult<Vec<ListBucketResult>> {
        let res = bucket.list(prefix.to_string(), delim).await?;
        Ok(res)
    }
    ///
    pub async fn fetch_bucket(gateway: &Gateway, name: &str) -> BoxResult<s3::Bucket> {
        Ok(gateway.bucket(name)?)
    }
}
