use crate::user::{User, UserName};

pub trait IUserRepository: Clone {
    fn save(&self, user: User);
    fn find(&self, name: UserName) -> Option<User>;
}

#[derive(Clone, Debug)]
pub struct UserRepository {}

impl UserRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl IUserRepository for UserRepository {
    fn save(&self, _user: User) {}

    fn find(&self, _name: UserName) -> Option<User> {
        None
    }
}
