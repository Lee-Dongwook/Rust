use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use crate::types::{
    account::{Account, AccountId},
    answer::{Answer, AnswerId, NewAnswer},
    question::{NewQuestion, Question, QuestionId},
}

use handle_errors::Error;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = match PgPoolOptions::new().max_connections(5).connect(db_url).await {
            Ok(pool) => pool,
            Err(e) => panic!("Cannot connect to the database: {}", e),
        }

        OK(Store {
            connection: db_pool,
        })
    }
}