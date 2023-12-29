use crate::domains::user::User;
use crate::repositories::user_repository_interface::IUserRepository;

pub struct UserService<R: IUserRepository> {
    user_repository: R,
}

impl<R: IUserRepository> UserService<R> {
    pub fn new(user_repository: &R) -> Self {
        Self {
            user_repository: user_repository.clone(),
        }
    }
    pub async fn exists(&self, user: &User) -> anyhow::Result<bool> {
        let found = self.user_repository.find(user.name()).await?;
        Ok(found.is_some())
    }
}
