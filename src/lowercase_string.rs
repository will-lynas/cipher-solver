use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct LowercaseString(Vec<u8>);

impl LowercaseString {
    #[must_use]
    pub fn normalize(s: &str) -> Self {
        Self(
            s.chars()
                .filter(char::is_ascii_alphabetic)
                .map(|c| (c.to_ascii_lowercase() as u8) - b'a')
                .collect(),
        )
    }

    #[must_use]
    pub fn to_indices(&self) -> &[u8] {
        &self.0
    }

    #[must_use]
    pub fn from_indices(indices: Vec<u8>) -> Self {
        Self(indices.into_iter().map(|i| i.rem_euclid(26)).collect())
    }

    #[must_use]
    pub fn letter_counts(&self) -> [usize; 26] {
        let mut counts = [0; 26];
        for &idx in &self.0 {
            counts[idx as usize] += 1;
        }
        counts
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn letter_frequencies(&self) -> [f64; 26] {
        let counts = self.letter_counts();
        let total = self.0.len() as f64;
        let mut frequencies = [0.0; 26];
        if total > 0.0 {
            for (i, &count) in counts.iter().enumerate() {
                frequencies[i] = count as f64 / total;
            }
        }
        frequencies
    }

    #[must_use]
    pub fn caesar_shift(&self, shift: i32) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        let shift = shift.rem_euclid(26) as u8;
        Self(self.0.iter().map(|&i| (i + shift) % 26).collect())
    }
}

impl Display for LowercaseString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for &idx in &self.0 {
            write!(f, "{}", ((idx + b'a') as char))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowercase_string_normalize() {
        assert_eq!(LowercaseString::normalize("Hello123").to_string(), "hello");
        assert_eq!(LowercaseString::normalize("ABC def!").to_string(), "abcdef");
        assert_eq!(LowercaseString::normalize("").to_string(), "");
    }

    #[test]
    fn test_to_indices() {
        assert_eq!(
            LowercaseString::normalize("abc").to_indices(),
            vec![0, 1, 2]
        );
        assert_eq!(
            LowercaseString::normalize("hello").to_indices(),
            vec![7, 4, 11, 11, 14]
        );
        assert_eq!(
            LowercaseString::normalize("").to_indices(),
            Vec::<u8>::new()
        );
    }

    #[test]
    fn test_from_indices() {
        assert_eq!(
            LowercaseString::from_indices(vec![0, 1, 2]).to_string(),
            "abc"
        );
        assert_eq!(
            LowercaseString::from_indices(vec![7, 4, 11, 11, 14]).to_string(),
            "hello"
        );
        assert_eq!(LowercaseString::from_indices(vec![]).to_string(), "");
    }

    #[test]
    fn test_letter_counts() {
        let mut expected = [0; 26];
        expected[7] = 1; // h
        expected[4] = 1; // e
        expected[11] = 2; // l
        expected[14] = 1; // o
        assert_eq!(
            LowercaseString::normalize("hello").letter_counts(),
            expected
        );

        assert_eq!(LowercaseString::normalize("").letter_counts(), [0; 26]);
    }

    #[test]
    fn test_letter_frequencies() {
        let text = LowercaseString::normalize("hello");
        let frequencies = text.letter_frequencies();
        assert!((frequencies[7] - 0.2).abs() < 1e-10); // h: 1/5
        assert!((frequencies[4] - 0.2).abs() < 1e-10); // e: 1/5
        assert!((frequencies[11] - 0.4).abs() < 1e-10); // l: 2/5
        assert!((frequencies[14] - 0.2).abs() < 1e-10); // o: 1/5

        let empty_freqs = LowercaseString::normalize("").letter_frequencies();
        for freq in empty_freqs {
            assert!(freq.abs() < 1e-10);
        }
    }

    #[test]
    fn test_caesar_shift() {
        let text = LowercaseString::normalize("hello");
        assert_eq!(text.caesar_shift(1).to_string(), "ifmmp");
        assert_eq!(text.caesar_shift(2).to_string(), "jgnnq");
        assert_eq!(text.caesar_shift(26).to_string(), "hello");
        assert_eq!(text.caesar_shift(-1).to_string(), "gdkkn");
        assert_eq!(text.caesar_shift(27).to_string(), "ifmmp");
        assert_eq!(
            LowercaseString::normalize("").caesar_shift(1).to_string(),
            ""
        );
    }

    #[test]
    fn test_to_string() {
        let text = LowercaseString::normalize("Hello123");
        assert_eq!(text.to_string(), "hello");
        assert_eq!(LowercaseString::normalize("").to_string(), "");
    }
}
