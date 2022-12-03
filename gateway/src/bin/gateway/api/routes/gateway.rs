/*
    Appellation: gateway <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Context;
use axum::{extract::Path, routing::get, Extension, Json, Router};
use scsys::prelude::{BoxResult, Message, Timestamp};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GatewayRouter(pub String);

impl GatewayRouter {
    pub fn new() -> Self {
        Self("/gateway".to_string())
    }
    pub fn endpoint(&self, path: Option<&str>) -> String {
        let endpoint = format!("{}/{}", self.0, path.unwrap_or_default());
        endpoint
    }
    pub fn router(&self) -> Router {
        Router::new()
            .route("/gateway", get(landing))
            .route("/gateway/info", get(gateway_info))
    }
}

pub async fn landing() -> Json<Message> {
    let payload = json!({ "message": "S3 Gateway" });
    let msg = Message::from(payload);
    Json(msg)
}

pub async fn gateway_info(Extension(ctx): Extension<Context>) -> Json<Message> {
    let payload = json!({ "endpoint": ctx.cnf.gateway.endpoint, "region": ctx.cnf.gateway.region });
    let msg = Message::from(payload);
    Json(msg)
}

pub async fn list_buckets(
    Extension(ctx): Extension<Context>,
    Path(name): Path<String>,
) -> Json<Value> {
    let bucket = ctx.bucket(name.as_str()).expect("");
    let objects = crate::fetch_bucket_contents(bucket, "/", None)
        .await
        .unwrap_or_default();
    let names = crate::collect_obj_names(objects).await;
    let payload = json!({"name": name, "data": names});
    Json(payload)
}

pub async fn list_bucket_contents() -> Json<Value> {
    let payload = json!({"": ""});
    Json(payload)
}
