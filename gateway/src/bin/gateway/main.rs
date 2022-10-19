/*
    Appellation: scsys-gateway <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::BoxResult;
use scsys_gateway::gateways::{Gateway, simple_creds, simple_region};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> BoxResult {
    println!("Hello, world!");
    let access_key = std::env::var("STORJ_ACCESS_KEY")?;
    let secret_key = std::env::var("STORJ_SECRET_KEY")?;
    let endpoint = "https://gateway.storjshare.io";
    let region = "us-east-1";
    let creds = simple_creds(access_key.as_str(), secret_key.as_str());
    
    let gateway = Gateway::new(creds, simple_region(endpoint, region));
    let bucket = gateway.bucket("scsys")?;
    let objects = bucket.list("/lib/documents/research/".to_string(), Some("/".to_string())).await?;
    let object_names = objects.iter().map(|i| i.clone().name ).collect::<Vec<String>>();
    println!("{:?}", objects);
    Ok(())
}



