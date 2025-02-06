use crate::lowercase_string::LowercaseString;

fn apply(text: &str, keyword: &str, decrypt: bool) -> Option<String> {
    let text = LowercaseString::normalize(text);
    let keyword = LowercaseString::normalize(keyword);
    let text_indices = text.to_indices();
    let key_indices = keyword.to_indices();
    let key_len = key_indices.len();

    if key_len == 0 {
        return None;
    }

    Some(
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
        .to_string(),
    )
}

/// Encrypts a message using a Vigenère cipher with a given keyword.
/// Punctuation and whitespace are removed.
///
/// # Example
/// ```
/// use cipher_solver::vigenere;
///
/// let text = "hello world";
/// let encrypted = vigenere::encrypt(text, "key").unwrap();
/// assert_eq!(encrypted, "rijvsuyvjn");
/// ```
#[must_use]
pub fn encrypt(text: &str, keyword: &str) -> Option<String> {
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
/// let decrypted = vigenere::decrypt(text, "key").unwrap();
/// assert_eq!(decrypted, "helloworld");
/// ```
#[must_use]
pub fn decrypt(text: &str, keyword: &str) -> Option<String> {
    apply(text, keyword, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let original = "The quick brown fox jumps over the lazy dog";
        let normalized = LowercaseString::normalize(original);
        let keyword = "secret";
        let encrypted = encrypt(original, keyword).unwrap();
        let decrypted = decrypt(&encrypted, keyword);
        assert_eq!(decrypted, Some(normalized.to_string()));

        let encrypted_empty = encrypt(original, "");
        assert_eq!(encrypted_empty, None);

        let decrypted_empty = decrypt(original, "");
        assert_eq!(decrypted_empty, None);
    }
}
