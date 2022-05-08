use crate::domain::models::message::Message;
use crate::infrastructure::repository::message_repository::MessageRepository;
use anyhow::Result;

pub fn handle(repository: impl MessageRepository, message: Message) -> Result<Message> {
    repository.create(message)
}
