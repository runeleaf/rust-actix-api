use crate::domain::models::message::Message;
use crate::infrastructure::repository::message_repository::MessageRepository;
use anyhow::Result;

pub fn handle(repository: impl MessageRepository, message: Message, id: i32) -> Result<Message> {
    repository.update(message, id)
}
