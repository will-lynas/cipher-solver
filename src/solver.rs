use crate::lowercase_string::LowercaseString;
use crate::utils;

pub struct Solver;

impl Solver {
    /// Solves a Caesar cipher using statistical analysis.
    ///
    /// # Example
    /// ```
    /// use cipher_solver::Solver;
    ///
    /// let text = "The quick brown fox jumps over the lazy dog";
    /// let encrypted = Solver::encrypt_caesar(text, 3);
    /// let solved = Solver::solve_caesar(&encrypted);
    /// assert_eq!(solved, "thequickbrownfoxjumpsoverthelazydog");
    /// ```
    #[must_use]
    pub fn solve_caesar(text: &str) -> String {
        let text = LowercaseString::coerce(text);
        (0..26)
            .map(|shift| {
                let shifted = text.caesar_shift(shift);
                (utils::english_score(&shifted), shifted)
            })
            .min_by(|(score1, _), (score2, _)| score1.total_cmp(score2))
            .map(|(_, text)| text.as_ref().to_string())
            .unwrap()
    }

    /// Encrypts a message using a Caesar cipher with a given shift.
    /// Punctuation and whitespace are removed.
    ///
    /// # Example
    /// ```
    /// use cipher_solver::Solver;
    ///
    /// let text = "hello world";
    /// let encrypted = Solver::encrypt_caesar(text, 3);
    /// assert_eq!(encrypted, "khoorzruog");
    /// ```
    #[must_use]
    pub fn encrypt_caesar(text: &str, shift: i32) -> String {
        LowercaseString::coerce(text)
            .caesar_shift(shift)
            .to_string()
    }

    /// Decrypts a message using a Caesar cipher with a given shift.
    /// Punctuation and whitespace are removed.
    ///
    /// # Example
    /// ```
    /// use cipher_solver::Solver;
    ///
    /// let text = "khoorzruog";
    /// let decrypted = Solver::decrypt_caesar(text, 3);
    /// assert_eq!(decrypted, "helloworld");
    /// ```
    #[must_use]
    pub fn decrypt_caesar(text: &str, shift: i32) -> String {
        Self::encrypt_caesar(text, 26 - shift)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_caesar() {
        let tests = [
            "I met a traveller from an antique land",
            "Who said, two vast and trunkless legs of stone ",
            "Stand in the desert. Near them, on the sand,",
        ];
        for test in tests {
            let coerced = LowercaseString::coerce(test);
            let shifted = Solver::encrypt_caesar(test, 3);
            let solved = Solver::solve_caesar(&shifted);
            assert_eq!(solved, coerced.as_ref());
        }
    }

    #[test]
    fn test_encrypt_decrypt() {
        let original = "The quick brown fox jumps over the lazy dog";
        let coerced = LowercaseString::coerce(original);
        let shift = 7;
        let encrypted = Solver::encrypt_caesar(original, shift);
        let decrypted = Solver::decrypt_caesar(&encrypted, shift);
        assert_eq!(decrypted, coerced.as_ref());
    }
}
