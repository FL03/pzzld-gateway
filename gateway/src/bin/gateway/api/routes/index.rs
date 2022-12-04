/*
    Appellation: index <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Context;
use axum::{routing::get, Extension, Json, Router};
use scsys::prelude::Message;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BaseRouter;

impl BaseRouter {
    pub fn new() -> Self {
        Self
    }
    pub fn router(&self) -> Router {
        Router::new()
            .route("/", get(landing))
            .route("/info/gateway", get(gateway_info))
            .route("/info/gateway/s3", get(s3_info))
    }
}

// Base path for the S3 Gateway
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Landing", body = Json<Message>)
    )
)]
pub async fn landing() -> Json<Message> {
    let payload = json!({ "message": "Welcome to the pzzld-gateway" });
    let msg = Message::from(payload);
    Json(msg)
}

#[utoipa::path(
    get,
    path = "/info/gateway",
    responses(
        (status = 200, description = "Landing", body = Json<Message>)
    )
)]
pub async fn gateway_info(Extension(ctx): Extension<Context>) -> Json<Message> {
    let msg = Message::from(json!({
        "endpoint": ctx.cnf.gateway.endpoint,
        "region": ctx.cnf.gateway.region
    }));
    Json(msg)
}

#[utoipa::path(
    get,
    path = "/info/gateway/s3",
    responses(
        (status = 200, description = "S3 Provider Settings", body = Json<Message>)
    )
)]
pub async fn s3_info(Extension(ctx): Extension<Context>) -> Json<Message> {
    let payload = json!({ "endpoint": ctx.cnf.gateway.endpoint, "region": ctx.cnf.gateway.region });
    let msg = Message::from(payload);
    Json(msg)
}
