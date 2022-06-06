use crate::db::schema::messages::content;
use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "content"]
pub struct Content {
    pub id: Uuid,
    pub message_id: Uuid,
    pub data: String,
    pub created_datetime: NaiveDateTime,
    pub modified_datetime: NaiveDateTime,
}