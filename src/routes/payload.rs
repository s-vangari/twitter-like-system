use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::db::content_entity::Content;
use crate::db::message_entity::Message;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageRequestBody {
    pub is_reply: bool,
    pub user_id: Uuid,
    // This user_id field must be extracted from User System / Authorization token
    pub reply_to: Option<Uuid>,
    pub data: String,
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponseBody {
    pub id: Uuid,
    pub user_id: Uuid,
    pub is_reply: bool,
    pub reply_to: Option<Uuid>,
    pub created_datetime: NaiveDateTime,
    pub modified_datetime: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentResponseBody {
    pub id: Uuid,
    pub message_id: Uuid,
    pub data: String,
    pub created_datetime: NaiveDateTime,
    pub modified_datetime: NaiveDateTime,
}


impl From<MessageRequestBody> for Message {
    fn from(mess_req: MessageRequestBody) -> Self {
        Message {
            id: Uuid::new_v4(),
            user_id: mess_req.user_id,
            is_reply: mess_req.is_reply,
            reply_to: mess_req.reply_to,
            created_datetime: Utc::now().naive_utc(),
            modified_datetime: Utc::now().naive_utc(),
        }
    }
}

impl From<Message> for MessageResponseBody {
    fn from(message: Message) -> Self {
        MessageResponseBody {
            id: message.id,
            user_id: message.user_id,
            is_reply: message.is_reply,
            reply_to: message.reply_to,
            created_datetime: message.created_datetime,
            modified_datetime: message.modified_datetime,
        }
    }
}

impl From<Content> for ContentResponseBody {
    fn from(c: Content) -> Self {
        ContentResponseBody {
            id: Default::default(),
            message_id: Default::default(),
            data: "".to_string(),
            created_datetime: c.created_datetime,
            modified_datetime: c.modified_datetime,
        }
    }
}