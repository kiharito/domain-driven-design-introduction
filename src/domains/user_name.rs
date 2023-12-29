use anyhow::ensure;

#[derive(Clone, Debug, PartialEq)]
pub struct UserName {
    value: String,
}

impl UserName {
    pub fn new(value: &str) -> anyhow::Result<Self> {
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
