use uuid::Uuid;
use crate::db::content_entity::Content;
use crate::db::error::DBError;
use crate::db::content::get_content;
use crate::db::db_connection::get_connection;


pub fn get_content_by_message_id(mes_id: Uuid) -> Result<Content, DBError> {
    let conn = get_connection();
    get_content(mes_id, &conn)
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;
    use crate::db::content::get_content;
    use crate::db::message_entity::Message;
    use crate::service::content::get_content_by_message_id;
    use crate::service::message::add_message;

    #[test]
    fn test_get_content() {
        let m = Message {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            is_reply: false,
            reply_to: Option::from(Uuid::new_v4()),
            created_datetime: Utc::now().naive_utc(),
            modified_datetime: Utc::now().naive_utc(),
        };

        let message = add_message(m, "Hello".to_string()).expect("db insert fail");
        let content = get_content_by_message_id(message.id).expect("Error or getting content");
        println!("Content {:?}", content);
    }
}