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

    fn indices(&self) -> Vec<u8> {
        self.0.chars().map(|c| (c as u8) - b'a').collect()
    }

    pub fn letter_counts(&self) -> [usize; 26] {
        let mut counts = [0; 26];
        for idx in self.indices() {
            counts[idx as usize] += 1;
        }
        counts
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

    #[test]
    fn test_indices() {
        assert_eq!(LowercaseString::coerce("abc").indices(), vec![0, 1, 2]);
        assert_eq!(
            LowercaseString::coerce("hello").indices(),
            vec![7, 4, 11, 11, 14]
        );
        assert_eq!(LowercaseString::coerce("").indices(), Vec::<u8>::new());
    }

    #[test]
    fn test_letter_counts() {
        let mut expected = [0; 26];
        expected[7] = 1; // h
        expected[4] = 1; // e
        expected[11] = 2; // l
        expected[14] = 1; // o
        assert_eq!(LowercaseString::coerce("hello").letter_counts(), expected);

        assert_eq!(LowercaseString::coerce("").letter_counts(), [0; 26]);
    }
}
