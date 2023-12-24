use crate::user::User;

pub struct UserService {}

impl UserService {
    pub fn new() -> Self {
        Self {}
    }
    pub fn exists(&self, _user: User) -> bool {
        false
    }
}
