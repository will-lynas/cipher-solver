use crate::lowercase_string::LowercaseString;

const ENGLISH_FREQUENCIES: [f64; 26] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094, 0.06966, 0.00153,
    0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056,
    0.02758, 0.00978, 0.02360, 0.00150, 0.01974, 0.00074,
];

pub fn chi_squared<const N: usize>(observed: &[f64; N], expected: &[f64; N]) -> f64 {
    observed
        .iter()
        .zip(expected.iter())
        .map(|(o, e)| {
            let diff = o - e;
            diff * diff / e
        })
        .sum()
}

pub fn chi_squared_english_score(text: &LowercaseString) -> f64 {
    let observed = text.letter_frequencies();
    chi_squared(&observed, &ENGLISH_FREQUENCIES)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chi_squared() {
        let observed = [4.0, 6.0, 8.0];
        let expected = [5.0, 5.0, 8.0];
        let result = chi_squared(&observed, &expected);
        assert!((result - 0.4).abs() < 1e-10);
    }

    #[test]
    fn test_english_score_sanity() {
        let english_text = LowercaseString::normalize("the quick brown fox jumps over the lazy dog");
        let gibberish = LowercaseString::normalize("zzzzxxxx");

        let english_result = chi_squared_english_score(&english_text);
        let gibberish_result = chi_squared_english_score(&gibberish);
        assert!(english_result < gibberish_result);
    }
}
