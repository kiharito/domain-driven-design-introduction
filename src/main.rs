use crate::program::Program;
use crate::repository::UserRepository;
use anyhow::Result;

mod program;
mod repository;
mod user;
mod user_service;

#[tokio::main]
async fn main() -> Result<()> {
    Program::new(UserRepository::new().await?)
        .create_user("test")
        .await?;
    Ok(())
}
