/*
    Appellation: buckets <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Context;
use axum::{
    extract::{Path, Query},
    routing::get,
    Extension, Json, Router,
};
use pzzld_gateway::{collect_obj_names, fetch_bucket_contents};
use scsys::prelude::Message;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new()
        .route("/", get(landing))
        .route("/:name", get(fetch_bucket_object_names))
}

// Base path for the S3 Gateway
pub async fn landing(Extension(ctx): Extension<Context>) -> Json<Message> {
    let mut auth = false;
    if ctx.gateway.credentials().access_key.is_some()
        && ctx.gateway.credentials().secret_key.is_some()
    {
        auth = true
    }
    let msg = Message::from(json!({
        "auth": auth,
        "message": "S3 Gateway"
    }));
    Json(msg)
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BucketParams {
    pub delim: Option<String>,
    pub prefix: Option<String>,
}

// Given
pub async fn fetch_bucket_object_names(
    Extension(ctx): Extension<Context>,
    Path(name): Path<String>,
    Query(params): Query<BucketParams>,
) -> Json<Value> {
    let delim = Some(params.delim.unwrap_or_else(|| "/".to_string()));
    let prefix = params.prefix.unwrap_or_else(|| "/".to_string());
    let bucket = ctx.gateway.bucket(name.as_str()).expect("");
    let objects = fetch_bucket_contents(bucket, prefix.as_str(), delim)
        .await
        .unwrap_or_default();
    let names = collect_obj_names(objects).await;
    let payload = json!({"name": name, "data": names});
    Json(payload)
}

pub async fn list_bucket_contents() -> Json<Value> {
    let payload = json!({"": ""});
    Json(payload)
}
