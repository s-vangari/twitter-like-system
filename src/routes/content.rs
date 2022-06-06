use rocket::serde::json::Json;
use uuid::Uuid;
use crate::routes::error::APIErrorResponse;
use crate::routes::payload::ContentResponseBody;
use crate::service::content::get_content_by_message_id;

#[get("/content/<message_id>", format = "json")]
pub fn get_content(message_id: String) -> Result<Json<ContentResponseBody>, APIErrorResponse> {
    Uuid::parse_str(&message_id)
        .map_err(|_e| APIErrorResponse::UnhandledError)
        .and_then(|message_id| {
            get_content_by_message_id(message_id)
                .map(ContentResponseBody::from)
                .map(Json)
                .map_err(|e|APIErrorResponse::ServiceError(e))
        })

}