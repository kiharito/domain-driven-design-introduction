#[derive(Debug, PartialEq)]
pub struct FullName {
    first_name: String,
    last_name: String,
}

impl FullName {
    pub fn new(first_name: &str, last_name: &str) -> Self {
        Self { first_name: first_name.to_string(), last_name: last_name.to_string() }
    }
    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }
    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let suzuki_taro = FullName::new("Taro", "Suzuki");
        let suzuki_taro2 = FullName::new("Taro", "Suzuki");
        assert_eq!(suzuki_taro, suzuki_taro2);

        let suzuki_jiro = FullName::new("Jiro", "Suzuki");
        assert_ne!(suzuki_taro, suzuki_jiro);

        let tanaka_taro = FullName::new("Taro", "Tanaka");
        assert_ne!(suzuki_taro, tanaka_taro);
    }
}
