use crate::domains::user_id::UserId;
use crate::domains::user_name::UserName;

#[derive(Debug)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
}

impl User {
    pub fn new(id: UserId, name: UserName) -> Self {
        Self { id, name }
    }
    pub fn build(name: UserName) -> Self {
        Self {
            id: UserId::new(&uuid::Uuid::new_v4().to_string()),
            name,
        }
    }
    pub fn id(&self) -> UserId {
        self.id.clone()
    }
    pub fn name(&self) -> UserName {
        self.name.clone()
    }
}
