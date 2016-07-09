
fn prime_factors(num: i32) -> Vec<i32> {

	let mut prime_factors = Vec::new();

	if num % 2 == 0 && num > 2 {
        prime_factors.push(2);
        let n = num / 2;
        if n > 1 {
        	prime_factors.push(n)
        }
    } else if num > 1 {
        prime_factors.push(num)
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
