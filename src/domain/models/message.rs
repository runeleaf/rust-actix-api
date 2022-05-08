use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
enum MessageStatus {
    Pending(String),
    Active(String),
    Closed(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Message {
    pub fn new(title: String, body: String, published: bool) -> Self {
        Self {
            id: Default::default(),
            title: title,
            body: body,
            published: published,
            created_at: Some(Utc::now().naive_local()),
            updated_at: Some(Utc::now().naive_local()),
        }
    }
}
