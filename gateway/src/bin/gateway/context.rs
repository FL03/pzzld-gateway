/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::settings::Settings;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context {
    pub settings: Settings
}

impl Context {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }
}
