/*
    Appellation: gateway <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Context;
use axum::{extract::Path, routing::get, Extension, Json, Router};
use scsys::{BoxResult, Timestamp};
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
            .route("/gateway/info/region", get(region))
    }
}

pub async fn landing() -> Json<Value> {
    let timestamp = Timestamp::default();
    Json(json!({ "timestamp": timestamp }))
}

pub async fn region(Extension(ctx): Extension<Context>) -> Json<Value> {
    let payload = json!({ "region": ctx.settings.gateway });
    Json(payload)
}

pub async fn list_buckets(
    Extension(ctx): Extension<Context>,
    Path(name): Path<String>,
) -> Json<Value> {
    let data = Vec::<String>::new();
    let payload = json!({"name": name, "data": data});
    Json(payload)
}
