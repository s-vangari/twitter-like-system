use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

// TODO: setup connection pool
pub fn get_connection() -> PgConnection {
    // TODO: Return appropriate error type
    let db_user = env::var("DB_HOST").expect("ENV var DB_HOST not set");
    let db_pass = env::var("DB_PASS").expect("ENV var DB_PASS not set");
    let db_name = env::var("DB_NAME").expect("ENV var DB_NAME not set");
    let db_port = env::var("DB_PORT").expect("ENV var DB_PORT not set");
    let database_url = format!("postgres://{}:{}@{}/{}", db_user, db_pass, db_name, db_port);
    PgConnection::establish(database_url.as_str())
        .expect(&format!("Error connecting to {}", database_url))
}