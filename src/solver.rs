use crate::lowercase_string::LowercaseString;

const ENGLISH_FREQUENCIES: [f64; 26] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094, 0.06966, 0.00153,
    0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056,
    0.02758, 0.00978, 0.02360, 0.00150, 0.01974, 0.00074,
];

pub struct Solver;

impl Solver {
    fn chi_squared<const N: usize>(observed: &[f64; N], expected: &[f64; N]) -> f64 {
        observed
            .iter()
            .zip(expected.iter())
            .map(|(o, e)| {
                let diff = o - e;
                diff * diff / e
            })
            .sum()
    }

    fn english_score(text: &LowercaseString) -> f64 {
        let observed = text.letter_frequencies();
        Self::chi_squared(&observed, &ENGLISH_FREQUENCIES)
    }

    pub fn solve_caesar(text: &str) -> LowercaseString {
        let text = LowercaseString::coerce(text);
        (0..26)
            .map(|shift| {
                let shifted = text.caesar_shift(shift);
                (Self::english_score(&shifted), shifted)
            })
            .min_by(|(score1, _), (score2, _)| score1.total_cmp(score2))
            .map(|(_, text)| text)
            .unwrap()
    }

    pub fn encrypt_caesar(text: &str, shift: i32) -> String {
        LowercaseString::coerce(text)
            .caesar_shift(shift)
            .as_ref()
            .to_string()
    }

    pub fn decrypt_caesar(text: &str, shift: i32) -> String {
        Self::encrypt_caesar(text, 26 - shift)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chi_squared() {
        let observed = [4.0, 6.0, 8.0];
        let expected = [5.0, 5.0, 8.0];
        let result = Solver::chi_squared(&observed, &expected);
        assert!((result - 0.4).abs() < 1e-10);
    }

    #[test]
    fn test_english_score_sanity() {
        let english_text = LowercaseString::coerce("the quick brown fox jumps over the lazy dog");
        let gibberish = LowercaseString::coerce("zzzzxxxx");

        let english_result = Solver::english_score(&english_text);
        let gibberish_result = Solver::english_score(&gibberish);
        assert!(english_result < gibberish_result);
    }

    #[test]
    fn test_solve_caesar() {
        let tests = [
            (
                "I met a traveller from an antique land",
                "imetatravellerfromanantiqueland",
            ),
            (
                "Who said, two vast and trunkless legs of stone ",
                "whosaidtwovastandtrunklesslegsofstone",
            ),
            (
                "Stand in the desert. Near them, on the sand,",
                "standinthedesertnearthemonthesand",
            ),
        ];
        for (original, expected) in tests {
            let shifted = Solver::encrypt_caesar(original, 3);
            let solved = Solver::solve_caesar(&shifted);
            assert_eq!(solved.as_ref(), expected);
        }
    }

    #[test]
    fn test_encrypt_decrypt() {
        let original = "thequickbrownfoxjumpsoverthelazydog";
        let shift = 7;
        let encrypted = Solver::encrypt_caesar(original, shift);
        let decrypted = Solver::decrypt_caesar(&encrypted, shift);
        assert_eq!(decrypted, original);
    }
}
