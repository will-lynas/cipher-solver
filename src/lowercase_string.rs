#[derive(Debug, Clone, PartialEq)]
pub struct LowercaseString(String);

impl LowercaseString {
    pub fn coerce(s: &str) -> Self {
        Self(
            s.chars()
                .filter(|c| c.is_ascii_alphabetic())
                .map(|c| c.to_ascii_lowercase())
                .collect(),
        )
    }
}

impl AsRef<str> for LowercaseString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowercase_string_coerce() {
        assert_eq!(LowercaseString::coerce("Hello123").as_ref(), "hello");
        assert_eq!(LowercaseString::coerce("ABC def!").as_ref(), "abcdef");
        assert_eq!(LowercaseString::coerce("").as_ref(), "");
    }
}
