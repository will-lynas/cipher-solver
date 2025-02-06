use crate::lowercase_string::LowercaseString;
use crate::utils;

/// Solves a Caesar cipher using statistical analysis.
///
/// # Example
/// ```
/// use cipher_solver::caesar;
///
/// let text = "The quick brown fox jumps over the lazy dog";
/// let encrypted = caesar::encrypt(text, 3);
/// let solved = caesar::solve(&encrypted);
/// assert_eq!(solved, "thequickbrownfoxjumpsoverthelazydog");
/// ```
#[must_use]
pub fn solve(text: &str) -> String {
    let text = LowercaseString::normalize(text);
    (0..26)
        .map(|shift| {
            let shifted = text.caesar_shift(shift);
            (utils::chi_squared_english_score(&shifted), shifted)
        })
        .min_by(|(score1, _), (score2, _)| score1.total_cmp(score2))
        .map(|(_, text)| text.to_string())
        .unwrap()
}

/// Encrypts a message using a Caesar cipher with a given shift.
/// Punctuation and whitespace are removed.
///
/// # Example
/// ```
/// use cipher_solver::caesar;
///
/// let text = "hello world";
/// let encrypted = caesar::encrypt(text, 3);
/// assert_eq!(encrypted, "khoorzruog");
/// ```
#[must_use]
pub fn encrypt(text: &str, shift: i32) -> String {
    LowercaseString::normalize(text)
        .caesar_shift(shift)
        .to_string()
}

/// Decrypts a message using a Caesar cipher with a given shift.
/// Punctuation and whitespace are removed.
///
/// # Example
/// ```
/// use cipher_solver::caesar;
///
/// let text = "khoorzruog";
/// let decrypted = caesar::decrypt(text, 3);
/// assert_eq!(decrypted, "helloworld");
/// ```
#[must_use]
pub fn decrypt(text: &str, shift: i32) -> String {
    encrypt(text, 26 - shift)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let tests = [
            "I met a traveller from an antique land",
            "Who said, two vast and trunkless legs of stone ",
            "Stand in the desert. Near them, on the sand,",
        ];
        for test in tests {
            let normalized = LowercaseString::normalize(test);
            let shifted = encrypt(test, 3);
            let solved = solve(&shifted);
            assert_eq!(solved, normalized.to_string());
        }
    }

    #[test]
    fn test_encrypt_decrypt() {
        let original = "The quick brown fox jumps over the lazy dog";
        let normalized = LowercaseString::normalize(original);
        let shift = 7;
        let encrypted = encrypt(original, shift);
        let decrypted = decrypt(&encrypted, shift);
        assert_eq!(decrypted, normalized.to_string());
    }
}
