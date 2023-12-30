#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
