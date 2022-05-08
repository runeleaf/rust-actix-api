use crate::infrastructure::repository::message_repository::MessageRepository;
use anyhow::Result;

pub fn handle(repository: impl MessageRepository, id: i32) -> Result<()> {
    repository.delete(id)
}
