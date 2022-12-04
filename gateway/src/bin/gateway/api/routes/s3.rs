/*
    Appellation: gateway <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{
    api::{collect_obj_names, fetch_bucket_contents},
    Context,
};
use axum::{
    extract::{Path, Query},
    routing::get,
    Extension, Json, Router,
};
use scsys::prelude::Message;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct S3Router;

impl S3Router {
    pub fn new() -> Self {
        Self
    }
    pub fn router(&self) -> Router {
        Router::new()
            .route("/s3", get(landing))
            .route("/s3/bucket/:name", get(fetch_bucket_object_names))
    }
}

// Base path for the S3 Gateway
pub async fn landing(Extension(ctx): Extension<Context>) -> Json<Message> {
    let mut auth = false;
    if ctx.credentials().access_key.is_some() && ctx.credentials().secret_key.is_some() {
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
    pub name: String,
    pub path: Vec<String>,
}

// Given
pub async fn fetch_bucket_object_names(
    Extension(ctx): Extension<Context>,
    Path(name): Path<String>,
    Query(params): Query<BucketParams>,
) -> Json<Value> {
    let bucket = ctx.bucket(name.as_str()).expect("");
    let objects = fetch_bucket_contents(bucket, "/", Some("/".to_string()))
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
