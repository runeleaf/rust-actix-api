use crate::domain::models::message::Message;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
pub struct MessageRequest {
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl MessageRequest {
    pub fn of(&self) -> Message {
        Message::new(
            self.title.clone().try_into().unwrap(),
            self.body.clone().try_into().unwrap(),
            self.published.clone(),
        )
    }
}
