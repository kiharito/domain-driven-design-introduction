use crate::user::{User, UserName};

pub trait UserRepository: Clone {
    fn save(&self, user: User);
    fn find(&self, name: UserName) -> Option<User>;
}

#[derive(Clone, Debug)]
pub struct RdsUserRepository {}

impl RdsUserRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserRepository for RdsUserRepository {
    fn save(&self, _user: User) {}

    fn find(&self, _name: UserName) -> Option<User> {
        None
    }
}
