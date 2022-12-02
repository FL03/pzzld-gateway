/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{api::Api, Context, Settings};

use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Application {
    pub ctx: Context,
}

impl Application {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }
    pub fn api(&self) -> Api {
        Api::from(self.ctx.clone())
    }
    pub fn with_logging(&mut self) -> &Self {
        self.ctx.settings.logger.setup(None);
        tracing_subscriber::fmt::init();
        self
    }
    pub async fn run(&self) -> BoxResult {
        tracing::info!("{}", self.ctx.settings.server.clone());
        self.api().run().await?;
        Ok(())
    }
}

impl std::convert::From<Settings> for Application {
    fn from(settings: Settings) -> Self {
        Self::new(Context::new(settings))
    }
}

impl std::convert::From<Context> for Application {
    fn from(data: Context) -> Self {
        Self::new(data)
    }
}

pub enum State {
    Connect { name: String, endpoint: String },
    Idle,
}
