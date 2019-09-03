use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};
use serde_derive::{Serialize, Deserialize};

pub struct Authorization(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: u8,
    pub exp: usize
}

#[derive(Debug)]
pub enum AuthorizationError {
    Missing,
    Invalid,
}

fn is_valid(key: &str) -> bool {
    if key == "key" {
        return true;
    }

    false
}

impl<'a, 'r> FromRequest<'a, 'r> for Authorization {
    type Error = AuthorizationError;

   fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::Forbidden, AuthorizationError::Missing)),
            1 if is_valid(keys[0]) => Outcome::Success(Authorization(keys[0].to_string())),
            1 => Outcome::Failure((Status::Forbidden, AuthorizationError::Invalid)),
            _ => Outcome::Failure((Status::Forbidden, AuthorizationError::Invalid)),
        }
    }
}