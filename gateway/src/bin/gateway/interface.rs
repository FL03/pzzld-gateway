/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{api::Api, states::State, Context, Settings};

use scsys::BoxResult;
use std::sync::Arc;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Application {
    pub ctx: Context,
    pub state: Arc<State>,
}

impl Application {
    pub fn new(ctx: Context, state: Arc<State>) -> Self {
        Self { ctx, state }
    }
    pub fn api(&self) -> Api {
        Api::from(self.ctx.clone())
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
