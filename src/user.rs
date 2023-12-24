use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn new(name: UserName) -> Self {
        Self { id: UserId::new(&Uuid::new_v4().to_string()), name }
    }
}

#[derive(Debug, PartialEq)]
pub struct UserId {
    value: String,
}

impl UserId {
    pub fn new(value: &str) -> Self {
        Self { value: value.to_string() }
    }
}

#[derive(Debug, PartialEq)]
pub struct UserName {
    value: String,
}

impl UserName {
    pub fn new(value: &str) -> Self {
        Self { value: value.to_string() }
    }
}
