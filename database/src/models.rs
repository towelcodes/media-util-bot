use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::llm_context)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct LlmContext {
    pub uid: u64,
    pub last_updated: chrono::NaiveDateTime,
    pub context: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::llm_context)]
pub struct NewLlmContext {
    pub uid: u64,
    pub context: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LlmMessage {
    pub role: String,
    pub content: String,
}
