/*
    Appellation: scsys-gateway <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{context::Context, interface::Application, settings::Settings};
pub mod api;
pub(crate) mod context;
pub(crate) mod interface;
pub(crate) mod settings;

use gateway::gateways::{simple_creds, simple_region, Gateway};
use scsys::BoxResult;

#[tokio::main]
async fn main() -> BoxResult {
    println!("Hello, world!");
    let s =
        scsys::prelude::S3Credential::from_env(Some("STORJ_ACCESS_KEY"), Some("STORJ_SECRET_KEY"))?;
    println!("{:?}", s);

    let access_key = std::env::var("STORJ_ACCESS_KEY")?;
    let secret_key = std::env::var("STORJ_SECRET_KEY")?;
    assert_eq!(s.access_key.clone(), access_key.clone());
    let endpoint = "https://gateway.storjshare.io";
    let region = "us-east-1";
    let creds = simple_creds(access_key.as_str(), secret_key.as_str());

    let gateway = Gateway::new(creds, simple_region(endpoint, region));
    let bucket = gateway.bucket("scsys")?;
    let objects = bucket
        .list(
            "/lib/documents/research/".to_string(),
            Some("/".to_string()),
        )
        .await?;
    let _names = objects
        .iter()
        .map(|i| i.clone().name)
        .collect::<Vec<String>>();
    println!("{:?}", objects);

    let mut app = Application::default();
    app.with_logging();
    app.run().await?;

    Ok(())
}
