use crate::program::Program;
use crate::repositories::in_memory_user_repository::InMemoryUserRepository;
use crate::repositories::user_repository::UserRepository;
use anyhow::Result;

mod domains;
mod program;
mod repositories;

#[tokio::main]
async fn main() -> Result<()> {
    Program::new(UserRepository::new().await)
        .create_user("test8")
        .await?;
    Ok(())
}
