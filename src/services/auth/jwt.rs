use jsonwebtoken::{encode, Header};
use serde_derive::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::env;
use crate::services::auth::auth_trait::AuthTrait;


#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: u8,
    pub created_at: i64,
}

impl AuthTrait for Claims {
    fn new(user_id: u8) -> Claims {
        Claims {
            user_id,
            created_at: DateTime::timestamp(&Utc::now())
        }
    }

    fn create_token(&self) -> String {
        let secret_key = env::var("SECRET_KEY").expect("NO SECRET KEY AVAILABLE");

        let token = encode(
            &Header::default(),
            self,
            secret_key.as_ref()).unwrap();

        token
    }

    fn verify_token(&self) -> bool {
        true
    }
}