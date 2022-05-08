use crate::domain::models::message::Message;
use crate::infrastructure::db::schema::messages;
use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

pub trait MessageRepository {
    fn find(&self, message_id: i32) -> Result<Message>;
    fn all(&self) -> Result<Vec<Message>>;
    fn create(&self, message: Message) -> Result<Message>;
    fn update(&self, message: Message, message_id: i32) -> Result<Message>;
    fn delete(&self, message_id: i32) -> Result<()>;
}

#[derive(Debug, Queryable, Clone, Serialize, Deserialize)]
pub struct MessageEntity {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
#[derive(Debug, Insertable, Deserialize)]
#[table_name = "messages"]
pub struct NewMessageEntity {
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[table_name = "messages"]
pub struct EditMessageEntity {
    pub title: String,
    pub body: String,
    pub published: bool,
    pub updated_at: Option<NaiveDateTime>,
}

impl MessageEntity {
    fn from(model: Message) -> NewMessageEntity {
        NewMessageEntity {
            title: model.title,
            body: model.body,
            published: model.published,
            created_at: Some(Utc::now().naive_local()),
            updated_at: Some(Utc::now().naive_local()),
        }
    }

    fn build(model: Message) -> EditMessageEntity {
        EditMessageEntity {
            title: model.title,
            body: model.body,
            published: model.published,
            updated_at: Some(Utc::now().naive_local()),
        }
    }

    fn to(entity: MessageEntity) -> Message {
        Message {
            id: entity.id.to_owned(),
            title: entity.title.to_owned(),
            body: entity.body.to_owned(),
            published: entity.published.to_owned(),
            created_at: entity.created_at.to_owned(),
            updated_at: entity.updated_at.to_owned(),
        }
    }

    fn of(&self) -> Message {
        Message {
            id: self.id,
            title: self.title.to_owned(),
            body: self.body.to_owned(),
            published: self.published.to_owned(),
            created_at: self.created_at.to_owned(),
            updated_at: self.updated_at.to_owned(),
        }
    }
}

pub struct MessageRepositoryImpl {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
}

impl MessageRepository for MessageRepositoryImpl {
    fn find(&self, message_id: i32) -> Result<Message, anyhow::Error> {
        use crate::infrastructure::db::schema::messages::dsl;
        let conn = self.pool.get().expect("failed to get connection");
        let query = dsl::messages.order(messages::id.desc());
        let result = query
            .filter(messages::id.eq(message_id))
            .load::<MessageEntity>(&conn)
            .expect("error find message");
        let message = result.get(0);

        Ok(MessageEntity::to(message.unwrap().clone()))
    }

    fn all(&self) -> Result<Vec<Message>, anyhow::Error> {
        use crate::infrastructure::db::schema::messages::dsl;
        let conn = self.pool.get().expect("failed to get connection");
        let query = dsl::messages.order(messages::id.desc());
        let results = query
            .limit(10)
            .load::<MessageEntity>(&conn)
            .expect("error loading message");

        let r = results.into_iter().map(|e| e.of()).collect();
        Ok(r)
    }

    fn create(&self, message: Message) -> Result<Message, anyhow::Error> {
        use crate::infrastructure::db::schema::messages::dsl::*;
        let new_message_entity = MessageEntity::from(message.clone());

        let conn = self.pool.get().expect("failed to get connection");
        let query: MessageEntity = diesel::insert_into(messages)
            .values(&new_message_entity)
            .get_result(&conn)
            .expect("error insert message");

        log::debug!("{:#?}", query);
        let message = MessageEntity::to(query.clone());
        Ok(message)
    }

    fn update(&self, message: Message, message_id: i32) -> Result<Message, anyhow::Error> {
        use crate::infrastructure::db::schema::messages::dsl::*;
        let edit_message_entity = MessageEntity::build(message.clone());
        let conn = self.pool.get().expect("failed to get connection");
        let query: MessageEntity = diesel::update(messages.filter(id.eq(message_id)))
            .set(edit_message_entity)
            .get_result(&conn)
            .expect("error updadte message");

        log::debug!("{:#?}", query);
        let message = MessageEntity::to(query.clone());

        Ok(message)
    }

    fn delete(&self, message_id: i32) -> Result<(), anyhow::Error> {
        use crate::infrastructure::db::schema::messages::dsl::*;
        let conn = self.pool.get().expect("failed to get connection");
        diesel::delete(messages.filter(id.eq(message_id)))
            .execute(&conn)
            .expect("error destroy message");
        Ok(())
    }
}
