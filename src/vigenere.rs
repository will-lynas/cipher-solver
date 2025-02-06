use crate::lowercase_string::LowercaseString;

fn apply(text: &str, keyword: &str, decrypt: bool) -> String {
    let text = LowercaseString::coerce(text);
    let keyword = LowercaseString::coerce(keyword);
    let text_indices = text.to_indices();
    let key_indices = keyword.to_indices();
    let key_len = key_indices.len();

    if key_len == 0 {
        return text.to_string();
    }

    LowercaseString::from_indices(
        text_indices
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                let k = key_indices[i % key_len];
                let shift = if decrypt { 26 - k } else { k };
                (c + shift) % 26
            })
            .collect(),
    )
    .to_string()
}

/// Encrypts a message using a Vigenère cipher with a given keyword.
/// Punctuation and whitespace are removed.
///
/// # Example
/// ```
/// use cipher_solver::vigenere;
///
/// let text = "hello world";
/// let encrypted = vigenere::encrypt(text, "key");
/// assert_eq!(encrypted, "rijvsuyvjn");
/// ```
#[must_use]
pub fn encrypt(text: &str, keyword: &str) -> String {
    apply(text, keyword, false)
}

/// Decrypts a message using a Vigenère cipher with a given keyword.
/// Punctuation and whitespace are removed.
///
/// # Example
/// ```
/// use cipher_solver::vigenere;
///
/// let text = "rijvsuyvjn";
/// let decrypted = vigenere::decrypt(text, "key");
/// assert_eq!(decrypted, "helloworld");
/// ```
#[must_use]
pub fn decrypt(text: &str, keyword: &str) -> String {
    apply(text, keyword, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let original = "The quick brown fox jumps over the lazy dog";
        let coerced = LowercaseString::coerce(original);
        let keyword = "secret";
        let encrypted = encrypt(original, keyword);
        let decrypted = decrypt(&encrypted, keyword);
        assert_eq!(decrypted, coerced.to_string());

        let encrypted_empty = encrypt(original, "");
        assert_eq!(encrypted_empty, coerced.to_string());
        let decrypted_empty = decrypt(&encrypted_empty, "");
        assert_eq!(decrypted_empty, coerced.to_string());
    }
}
