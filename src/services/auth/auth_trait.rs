pub trait AuthTrait {
    fn new(user_id: u8) -> Self;
    fn create_token(&self) -> String;
    fn verify_token(&self) -> bool;
}