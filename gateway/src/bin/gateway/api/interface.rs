/*
    Appellation: interface <api>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{routes, ApiDoc};
use crate::{Context, Settings};
use axum::{Extension, Router, Server};
use http::header::{HeaderName, AUTHORIZATION};
use scsys::prelude::BoxResult;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Api {
    pub address: SocketAddr,
    pub ctx: Context,
}

impl Api {
    pub fn new(address: SocketAddr, ctx: Context) -> Self {
        Self { address, ctx }
    }
    pub fn address(&self) -> SocketAddr {
        self.ctx.clone().cnf.server.address()
    }
    pub async fn client(&self) -> Router {
        let sensitive_headers = std::iter::once(AUTHORIZATION);
        let custom_headers = HeaderName::from_static("x-request-id");
        let mut router = Router::new();
        router =
            router.merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()));
        router = router
            .merge(routes::index::BaseRouter::new().router())
            .merge(routes::s3::S3Router::new().router());
        router = router
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            )
            .layer(CompressionLayer::new())
            .layer(PropagateHeaderLayer::new(custom_headers))
            .layer(SetSensitiveHeadersLayer::new(sensitive_headers))
            .layer(Extension(self.ctx.clone()));
        router
    }
    /// Implements a graceful shutdown when users press CTRL + C
    pub async fn graceful_shutdown(&self) -> () {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to shutdown the server...");
        tracing::info!("Signal graceful shutdown...");
    }
    /// Quickly run the api
    pub async fn run(&self) -> BoxResult {
        let client = self.client().await;
        Server::bind(&self.address())
            .serve(client.into_make_service())
            .with_graceful_shutdown(self.graceful_shutdown())
            .await?;
        Ok(())
    }
}

impl std::convert::From<Context> for Api {
    fn from(ctx: Context) -> Self {
        let address = ctx.clone().cnf.server.address();
        Self::new(address, ctx)
    }
}

impl std::convert::From<Settings> for Api {
    fn from(settings: Settings) -> Self {
        Self::from(Context::new(settings))
    }
}

impl std::fmt::Display for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "View the application locally at http://localhost:{}",
            self.ctx.cnf.server.port
        )
    }
}
