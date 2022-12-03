/*
    Appellation: scsys-gateway <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{states::*, interface::*, settings::*};
pub mod api;
pub(crate) mod states;
pub(crate) mod interface;
pub(crate) mod settings;

use pzzld_gateway::gateways::{Gateway, GatewayCreds, S3Region};
use s3::serde_types::ListBucketResult;
use scsys::BoxResult;

#[tokio::main]
async fn main() -> BoxResult {
    let region = S3Region::from(("https://gateway.storjshare.io", "us-east-1"));
    let mut creds = GatewayCreds::default();
    creds.from_env(Some("STORJ_ACCESS_KEY"), Some("STORJ_SECRET_KEY"))?;

    let gateway = Gateway::new(creds.clone(), region);
    let objects = fetch_bucket_contents(&gateway, "scsys", "/lib/documents/research", Some("/".to_string())).await?;
    let _names = collect_obj_names(objects.clone()).await;
    // println!("{:?}", objects);

    let mut app = Application::default();
    app.with_logging();
    app.run().await?;

    Ok(())
}

pub async fn collect_obj_names(objects: Vec<ListBucketResult>) -> Vec<String> {
    tracing::info!("Collecting information on the given data...");
    objects.iter().map(|i| i.clone().name).collect::<Vec<String>>()
}

pub async fn fetch_bucket_contents(gateway: &Gateway, name: &str, prefix: &str, delim: Option<String>) -> BoxResult<Vec<ListBucketResult>> {
    let res = gateway.bucket(name)?.list(prefix.to_string(),delim).await?;
    Ok(res)
}

pub async fn fetch_bucket(gateway: &Gateway, name: &str) -> BoxResult<s3::Bucket> {
    Ok(gateway.bucket(name)?)
} 
