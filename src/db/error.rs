use std::fmt::{Display, Formatter, Result};
use diesel::result::Error;


// TODO: Handle appropriate error types

#[derive(Debug, Clone, PartialEq)]
pub enum DBError {
    NoRecordFound,
    UnhandledDBError(String),
}

impl Display for DBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            DBError::NoRecordFound => f.write_str("No Record Found in DB"),
            DBError::UnhandledDBError(ref err) => write!(f, "Unhandled DB Error {}", err),
        }
    }
}

impl From<Error> for DBError {
    fn from(err: Error) -> DBError {
        println!("Diesel DB Error {:?}", err);
        match err {
            Error::NotFound => DBError::NoRecordFound,
            _ => DBError::UnhandledDBError(format!(
                "Unexpected Diesel error occurred: {}",
                &err.to_string()
            )),
        }
    }
}

