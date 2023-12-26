use crate::user::{User, UserName};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::mysql::MySqlPool;

#[async_trait]
pub trait IUserRepository: Clone {
    async fn save(&self, user: User) -> Result<()>;
    async fn find(&self, name: UserName) -> Result<Option<User>>;
}

#[derive(Clone, Debug)]
pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    pub async fn new() -> Result<Self> {
        dotenvy::dotenv().expect("failed to read .env file");
        let url = std::env::var("DATABASE_URL").expect("env DATABASE_URL must be set");
        let pool = MySqlPool::connect(&url)
            .await
            .expect("cannot connect to the database");
        Ok(Self { pool })
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn save(&self, user: User) -> Result<()> {
        sqlx::query!(
            "REPLACE INTO users VALUES (?, ?);",
            user.id().value(),
            user.name().value()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find(&self, name: UserName) -> Result<Option<User>> {
        let res = sqlx::query!(
            "SELECT id, name FROM users WHERE users.name = ?;",
            name.value()
        )
        .fetch_optional(&self.pool)
        .await?;
        if res.is_none() {
            return Ok(None);
        }

        todo!("User生成");
        println!("{:?}", res.unwrap().id);
        Ok(None)
    }
}
