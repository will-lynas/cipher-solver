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

pub fn chi_squared_english(text: &str) -> f64 {
    let mut counts = [0.0; 26];
    let mut total = 0;

    for c in text.chars() {
        if let Some(idx) = (c.to_ascii_lowercase() as u8)
            .checked_sub(b'a')
            .filter(|&i| i < 26)
        {
            counts[idx as usize] += 1.0;
            total += 1;
        }
    }

    if total == 0 {
        return f64::INFINITY;
    }

    let observed: [f64; 26] = counts.map(|c| c / total as f64);
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
    fn test_chi_squared_english_sanity() {
        let english_text = "the quick brown fox jumps over the lazy dog";
        let gibberish = "zzzzxxxx";

        let english_result = chi_squared_english(english_text);
        let gibberish_result = chi_squared_english(gibberish);
        println!("English result: {}", english_result);
        println!("Gibberish result: {}", gibberish_result);
        assert!(english_result < gibberish_result);
    }

    #[test]
    fn test_chi_squared_english_empty() {
        assert_eq!(chi_squared_english(""), f64::INFINITY);
        assert_eq!(chi_squared_english("123 !@#"), f64::INFINITY);
    }
}
