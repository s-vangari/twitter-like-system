use diesel::RunQueryDsl;
use crate::db::error::DBError;
use crate::db::message_entity::Message;
use crate::db::schema::messages::message::dsl::message;
use diesel::*;
use crate::db::schema::messages::message::created_datetime;

/**
Return list of message with limit passed in from API layer
Default sort order by desc
 **/
pub fn get_message_list(limit: i64, conn: &PgConnection) -> Result<Vec<Message>, DBError> {
    message
        .order_by(created_datetime.desc())
        .limit(limit)
        .load(conn)
        .map_err(DBError::from)
}

pub fn post_message(new_message: Message, conn: &PgConnection) -> Result<Message, DBError> {
    diesel::insert_into(message)
        .values(&new_message)
        .get_result::<Message>(conn)
        .map_err(DBError::from)
}
