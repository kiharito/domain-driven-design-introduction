use anyhow::{ensure, Result};
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn new(name: UserName) -> Self {
        Self {
            id: UserId::new(&Uuid::new_v4().to_string()),
            name,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UserId {
    value: String,
}

impl UserId {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct UserName {
    value: String,
}

impl UserName {
    pub fn new(value: &str) -> Result<Self> {
        ensure!(value.chars().count() >= 3, "ユーザー名は3文字以上です");
        Ok(Self {
            value: value.to_string(),
        })
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_user_name() {
        let user_name = UserName::new("abc").unwrap();
        assert_eq!(user_name.value, "abc");
    }

    #[test]
    fn test_invalid_user_name() {
        let result = UserName::new("ab");
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "ユーザー名は3文字以上です"
        );
    }
}
