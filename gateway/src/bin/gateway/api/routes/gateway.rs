/*
    Appellation: index <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use axum::{routing::get, Json, Router};
use scsys::{BoxResult, Timestamp};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

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
        let basepath = "/gateway".to_string();
        Router::new()
            .route(self.endpoint(None).as_str(), get(landing))
    }
}

pub async fn landing() -> Json<Value> {
    let timestamp = Timestamp::default();
    Json(json!({"timestamp": timestamp}))
}
