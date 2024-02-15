use rocket::serde::json::Json;
use rocket::Request;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Error {
    message: String,
    description: Option<String>,
    code: u16,
}

impl Error {
    fn new(message: String, description: Option<String>, code: u16) -> Error {
        Error {
            message,
            description,
            code,
        }
    }
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<Error> {
    Json(Error::new(
        "Resource not found".to_string(),
        Some(format!(
            "The requested resource {} was not found",
            req.uri().path()
        )),
        404,
    ))
}

#[catch(422)]
pub fn unprocessable_entity() -> Json<Error> {
    Json(Error::new(
        "Unprocessable entity".to_string(),
        Some(
            "The request was well-formed but was unable to be followed due to semantic errors"
                .to_string(),
        ),
        422,
    ))
}
