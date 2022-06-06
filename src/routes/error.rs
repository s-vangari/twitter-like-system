use rocket::http::Status;
use rocket::{Request, response};
use rocket::response::{Responder, status};
use rocket::serde::json::Json;
use crate::db::error::DBError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum APIErrorResponse {
    // TODO: Add validation errors and other business logic related error responses
    ServiceError(DBError),
    UnhandledError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}


impl<'r> Responder<'r, 'static> for APIErrorResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match self {
            // TODO: Return appropirate responses
            APIErrorResponse::ServiceError(err) => match err {
                DBError::NoRecordFound |
                DBError::UnhandledDBError(_) => status::Custom(
                    Status::InternalServerError,
                    Json(
                        serde_json::to_value(ErrorResponse {
                            code: "INTERNAL_SERVER_ERROR".to_string(),
                            message: "Something unexpected happened.".to_string(),
                        }).unwrap())).respond_to(req),
            },
            APIErrorResponse::UnhandledError => status::Custom(
                Status::InternalServerError,
                Json(
                    serde_json::to_value(ErrorResponse {
                        code: "INTERNAL_SERVER_ERROR".to_string(),
                        message: "Something unexpected happened.".to_string(),
                    }).unwrap())).respond_to(req)
        }
    }
}