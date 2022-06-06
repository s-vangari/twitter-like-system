use chrono::Utc;
use uuid::Uuid;

use crate::db::message::*;
use crate::db::content::post_content;
use crate::db::content_entity::Content;
use crate::db::db_connection::get_connection;
use crate::db::error::DBError;
use crate::db::message_entity::Message;

pub fn add_message(new_message: Message, data: String) -> Result<Message, DBError> {
    let db_connection = get_connection();
    post_message(new_message, &db_connection)
        .and_then(|m| {
            let c = Content {
                id: Uuid::new_v4(),
                message_id: m.id,
                data,
                created_datetime: Utc::now().naive_utc(),
                modified_datetime: Utc::now().naive_utc(),
            };
            post_content(c, &db_connection)
                // just returning message as is for success
                .map(|_c| m)
        })
}


/**
Return list of message with limit passed in from API layer
Default sort order by desc
 **/
pub fn get_all_messages(limit: i64) -> Result<Vec<Message>, DBError> {
    let conn = get_connection();
    get_message_list(limit, &conn)
}


#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use crate::db::message_entity::Message;
    use crate::service::message::{add_message, get_all_messages};

    #[test]
    fn test_add_message() {
        let m = Message {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            is_reply: false,
            reply_to: Option::from(Uuid::new_v4()),
            created_datetime: Utc::now().naive_utc(),
            modified_datetime: Utc::now().naive_utc(),
        };

        add_message(m, "Hello".to_string()).expect("db insert fail");
    }

    #[test]
    fn test_get_message_list() {
        let m = Message {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            is_reply: false,
            reply_to: Option::from(Uuid::new_v4()),
            created_datetime: Utc::now().naive_utc(),
            modified_datetime: Utc::now().naive_utc(),
        };

        add_message(m, "Hello".to_string()).expect("db insert fail");
        let messages = get_all_messages(5).expect("db fetch fail");
        println!("print list {:?}", messages);
        assert!(messages.len() <= 5);
    }
}