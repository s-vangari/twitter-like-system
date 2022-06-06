use rocket::Build;
use serde_json::{json, Value};

mod message;
mod content;
mod payload;
mod error;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "404",
        "reason": "Resource not found."
    })
}

pub fn rocket() -> rocket::Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            routes![hello],
        )
        .mount(
            "/api",
            routes![
                 message::get_message,
                 message::post_message,
                 content::get_content,
            ],
        )
        .register("/", catchers![not_found])
}

#[get("/health")]
fn hello() -> &'static str {
    "Hello, world!"
}
