pub fn is_prime(x: i32) -> bool {
   // Handle small edge cases first
    if x == 1 || x == 2 {
        return true;
    } else if x == 0 {
        return false
    }

    // a number is prime if it is only divisible by 1 & itself
    // once we pass the sqrt of a number, we don't need to proceed checking for divisors
    let upper_limit: i32 = (x as f64).sqrt().ceil() as i32;

    for i in 2..upper_limit {
        if x % i == 0 {
            return false;
        }
    }
    true
}

pub fn prime_sieve(start: i32, end: i32) -> Vec<i32> {
    // all numbers in the sieve should be prime
    // start at the first prime and remove all of it's multiples
    let sieve = (start..end).collect::<Vec<_>>();
    sieve
}

#[test]
fn basic_prime() {
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(!is_prime(8));
}
#[test]
fn double_digits(){
    assert!(is_prime(97));
    assert!(is_prime(17));
    assert!(!is_prime(40));
    assert!(!is_prime(39));
}
#[test]
fn triple_digits() {
    assert!(!is_prime(100));
}
#[test]
fn sieve_to_hunnid(){
    assert_eq!(prime_sieve(0, 100), (0..100).filter(|x| is_prime(*x)).collect::<Vec<_>>());
}
