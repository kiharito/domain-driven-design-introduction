use crate::domains::user::User;
use crate::domains::user_name::UserName;
use async_trait::async_trait;

#[async_trait]
pub trait IUserRepository: Clone {
    async fn save(&self, user: User) -> anyhow::Result<()>;
    async fn find(&self, name: UserName) -> anyhow::Result<Option<User>>;
}
