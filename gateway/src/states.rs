/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::prelude::{fnl_remove, Message, StatePack};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

pub type State = scsys::prelude::State<States>;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum States {
    Error = 0,
    #[default]
    Idle = 1,
    Connect = 2,
}

impl States {
    pub fn into_message(&self) -> Message<Self> {
        (*self).into()
    }
    pub fn boxed(&self) -> Box<&Self> {
        Box::new(self)
    }
    pub fn shared(&self) -> Arc<Self> {
        Arc::new(*self)
    }
}

impl From<States> for i64 {
    fn from(data: States) -> Self {
        data as i64
    }
}

impl From<i64> for States {
    fn from(data: i64) -> Self {
        match data {
            0 => Self::Error,
            1 => Self::Idle,
            2 => Self::Connect,
            _ => Self::Error,
        }
    }
}

impl TryInto<Value> for States {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_into(self) -> Result<Value, <States as TryInto<Value>>::Error> {
        let res = serde_json::to_value(State::new(None, None, Some(self)))?;
        Ok(res)
    }
}

impl TryInto<Message> for States {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_into(self) -> Result<Message, <States as TryInto<Message>>::Error> {
        let res: Value = self.try_into()?;
        Ok(Message::from(res))
    }
}

impl StatePack for States {}

impl std::fmt::Display for States {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", fnl_remove(serde_json::to_string(&self).unwrap()))
    }
}

#[cfg(test)]
pub mod tests {}
