use crate::domains::user::User;
use crate::domains::user_name::UserName;
use crate::domains::user_service::UserService;
use crate::repositories::user_repository_interface::IUserRepository;
use anyhow::{bail, Result};

pub struct Program<R: IUserRepository> {
    user_repository: R,
}

impl<R: IUserRepository> Program<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
    pub async fn create_user(&self, user_name: &str) -> Result<()> {
        let user = User::build(UserName::new(user_name)?);
        let user_service = UserService::new(&self.user_repository);
        if user_service.exists(&user).await? {
            bail!(format!("{user_name}はすでに存在しています"));
        }
        self.user_repository.save(user).await?;
        Ok(())
    }
}
