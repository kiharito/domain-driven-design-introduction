use crate::user::{User, UserName};
use crate::user_service::UserService;
use anyhow::{bail, Result};

pub struct Program {}

impl Program {
    pub fn new() -> Self {
        Self {}
    }
    pub fn create_user(&self, user_name: &str) -> Result<()> {
        let user = User::new(UserName::new(user_name).unwrap());
        let user_service = UserService::new();
        if user_service.exists(user) {
            bail!(format!("{user_name}はすでに存在しています"));
        }
        Ok(())
    }
}
