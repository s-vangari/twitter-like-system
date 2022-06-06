use rocket::serde::json::Json;
use crate::db::message_entity::Message;
use crate::routes::error::APIErrorResponse;
use crate::routes::payload::{MessageRequestBody, MessageResponseBody};
use crate::service::message::{add_message, get_all_messages};

// #[post("/message", data = "<input>", format = "json")]
// pub fn post_message(input: Json<MessageRequestBody>) -> Json<MessageResponseBody> {
//     // TODO: Validate input payload
//     let request_body = input.into_inner();
// }

#[post("/message", data = "<input>", format = "json")]
pub fn post_message(input: Json<MessageRequestBody>) -> Result<Json<MessageResponseBody>, APIErrorResponse> {
    let req_body = input.into_inner();
    // TODO: User Reference for req_body
    add_message(Message::from(req_body.clone()), req_body.data)
        .map(MessageResponseBody::from)
        .map(Json)
        .map_err(|e|APIErrorResponse::ServiceError(e))
}


#[get("/message/list")]
pub fn get_message() -> Result<Json<Vec<MessageResponseBody>>, APIErrorResponse> {
    get_all_messages(5)
        .map(|messages| messages.into_iter().map(|m|MessageResponseBody::from(m)).collect())
        .map(Json)
        .map_err(|e|APIErrorResponse::ServiceError(e))
}


