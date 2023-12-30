use crate::domains::user::User;
use crate::domains::user_id::UserId;
use crate::domains::user_name::UserName;
use crate::repositories::user_repository_interface::IUserRepository;
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct InMemoryUserRepository {
    pub store: HashMap<UserId, User>,
}

impl InMemoryUserRepository {
    pub async fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

#[async_trait]
impl IUserRepository for InMemoryUserRepository {
    async fn save(&mut self, user: User) -> anyhow::Result<()> {
        self.store.insert(user.id(), user.clone());
        Ok(())
    }

    async fn find(&self, name: UserName) -> anyhow::Result<Option<User>> {
        let target = self.store.values().find(|&user| user.name() == name);
        match target {
            Some(user) => Ok(Some(user.clone())),
            None => Ok(None),
        }
    }
}
