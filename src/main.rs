
fn prime_factors(num: i32) -> Vec<i32> {

	let mut prime_factors = Vec::new();
	let mut num = num;
	let mut candidate = 2;

	while num > 1 {
        while num % candidate == 0 {
            prime_factors.push(candidate);
            num /= candidate;
        }
        candidate += 1;
    }

    prime_factors
}

#[test]
fn test_prime_factors_one() {
    assert_eq!(prime_factors(1), []);
}

#[test]
fn test_prime_factors_two() {
    assert_eq!(prime_factors(2), [2]);
}

#[test]
fn test_prime_factors_three() {
    assert_eq!(prime_factors(3), [3]);
}

#[test]
fn test_prime_factors_four() {
    assert_eq!(prime_factors(4), [2,2]);
}

#[test]
fn test_prime_factors_six() {
    assert_eq!(prime_factors(6), [2,3]);
}

#[test]
fn test_prime_factors_eight() {
    assert_eq!(prime_factors(8), [2,2,2]);
}

#[test]
fn test_prime_factors_nine() {
    assert_eq!(prime_factors(9), [3,3]);
}
