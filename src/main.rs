fn prime_factors(num: i64) -> Vec<i64> {
    vec![]
}

#[test]
fn test_prime_factors_one() {
    assert_eq!(prime_factors(1), []);
}

#[test]
fn test_prime_factors_two() {
    assert_eq!(prime_factors(2), [2]);
}
