use crate::infrastructure::repository::message_repository::{
    MessageRepository, MessageRepositoryImpl,
};
use diesel::pg::PgConnection;
// use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct RequestContext {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl RequestContext {
    pub fn new() -> RequestContext {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create DB connection pool.");
        RequestContext { pool }
    }

    pub fn message_repository(&self) -> impl MessageRepository {
        MessageRepositoryImpl {
            pool: Box::new(self.pool.to_owned()),
        }
    }
}
