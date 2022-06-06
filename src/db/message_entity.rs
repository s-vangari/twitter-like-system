use crate::db::schema::messages::message;
use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "message"]
pub struct Message {
    pub id: Uuid,
    pub user_id: Uuid,
    pub is_reply: bool,
    pub reply_to: Option<Uuid>,
    pub created_datetime: NaiveDateTime,
    pub modified_datetime: NaiveDateTime,
}