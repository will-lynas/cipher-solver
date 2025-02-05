pub fn chi_squared(observed: &[f64], expected: &[f64]) -> f64 {
    assert!(
        observed.len() == expected.len(),
        "Observed and expected arrays must have the same length"
    );

    observed
        .iter()
        .zip(expected.iter())
        .map(|(o, e)| {
            let diff = o - e;
            diff * diff / e
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chi_squared() {
        let observed = vec![4.0, 6.0, 8.0];
        let expected = vec![5.0, 5.0, 8.0];
        let result = chi_squared(&observed, &expected);
        assert!((result - 0.4).abs() < 1e-10);
    }
}
