use crate::program::Program;
use crate::repositories::user_repository::UserRepository;
use anyhow::Result;

mod program;
mod domains {
    pub mod user;
    pub mod user_id;
    pub mod user_name;
    pub mod user_service;
}

mod repositories {
    pub mod user_repository;
    pub mod user_repository_interface;
}

#[tokio::main]
async fn main() -> Result<()> {
    Program::new(UserRepository::new().await)
        .create_user("test8")
        .await?;
    Ok(())
}
