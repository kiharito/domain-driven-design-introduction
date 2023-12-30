use crate::domains::user::User;
use crate::domains::user_id::UserId;
use crate::domains::user_name::UserName;
use crate::repositories::user_repository_interface::IUserRepository;
use async_trait::async_trait;
use sqlx::MySqlPool;

#[derive(Clone, Debug)]
pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    pub async fn new() -> Self {
        dotenvy::dotenv().expect("failed to read .env file");
        let url = std::env::var("DATABASE_URL").expect("env DATABASE_URL must be set");
        let pool = MySqlPool::connect(&url)
            .await
            .expect("cannot connect to the database");
        Self { pool }
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn save(&mut self, user: User) -> anyhow::Result<()> {
        sqlx::query!(
            "REPLACE INTO users VALUES (?, ?);",
            user.id().value(),
            user.name().value()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find(&self, name: UserName) -> anyhow::Result<Option<User>> {
        let res = sqlx::query!(
            "SELECT id, name FROM users WHERE users.name = ?;",
            name.value()
        )
        .fetch_optional(&self.pool)
        .await?;
        match res {
            Some(record) => {
                let user = User::new(
                    UserId::new(&record.id),
                    UserName::new(&record.name.unwrap())?,
                );
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
}
