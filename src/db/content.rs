use uuid::Uuid;
use crate::db::content_entity::Content;
use crate::db::error::DBError;
use crate::db::schema::messages::content::message_id;
use diesel::*;
use crate::db::schema::messages::content::dsl::content;

pub fn get_content(mes_id: Uuid, conn: &PgConnection) -> Result<Content, DBError> {
    content.filter(message_id.eq(mes_id))
        .get_result::<Content>(conn)
        .map_err(DBError::from)
}

pub fn post_content(c: Content, conn: &PgConnection) -> Result<Content, DBError> {
    diesel::insert_into(content)
        .values(c)
        .get_result::<Content>(conn)
        .map_err(DBError::from)
}