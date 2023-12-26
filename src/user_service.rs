use crate::repository::IUserRepository;
use crate::user::User;

pub struct UserService<R: IUserRepository> {
    user_repository: R,
}

impl<R: IUserRepository> UserService<R> {
    pub fn new(user_repository: &R) -> Self {
        Self {
            user_repository: user_repository.clone(),
        }
    }
    pub fn exists(&self, user: &User) -> bool {
        let found = self.user_repository.find(user.name());
        found.is_some()
    }
}
