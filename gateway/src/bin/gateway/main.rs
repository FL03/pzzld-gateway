/*
    Appellation: scsys-gateway <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{settings::*, states::*};

pub mod api;
pub(crate) mod settings;
pub(crate) mod states;

use scsys::BoxResult;
use std::sync::Arc;

#[tokio::main]
async fn main() -> BoxResult {
    let mut app = Application::default();
    app.start().await?;

    Ok(())
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Application {
    pub ctx: Context,
    pub state: Arc<State>,
}

impl Application {
    pub fn new(ctx: Context, state: Arc<State>) -> Self {
        Self { ctx, state }
    }
    pub fn api(&self) -> api::Api {
        api::Api::from(self.ctx.clone())
    }
    pub fn with_logging(&mut self) -> &Self {
        self.ctx.cnf.logger.setup(None);
        tracing_subscriber::fmt::init();
        self
    }
    pub async fn run(&self) -> BoxResult {
        tracing::info!("{}", self.ctx.cnf.server.clone());
        self.api().run().await?;
        Ok(())
    }
    pub async fn start(&mut self) -> BoxResult {
        self.with_logging();
        tracing::info!(
            "Success: Initialized tracing for the gateway; awaiting system initialization"
        );
        self.run().await?;

        Ok(())
    }
}

impl std::convert::From<Settings> for Application {
    fn from(settings: Settings) -> Self {
        Self::from(Context::new(settings))
    }
}

impl std::convert::From<Context> for Application {
    fn from(data: Context) -> Self {
        Self::new(data, State::default().shared())
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.ctx).unwrap())
    }
}
