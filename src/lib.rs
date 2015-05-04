#![feature(step_by)]
#![feature(collections)]
pub fn is_prime(x: &i32) -> bool {
   // Handle small edge cases first
    if *x == 2 {
        return true;
    } else if *x == 0 || *x == 1 {
        return false
    }

    // a number is prime if it is only divisible by 1 & itself
    // once we pass the sqrt of a number, we don't need to proceed checking for divisors
    let upper_limit: i32 = (*x as f64).sqrt().ceil() as i32;

    for i in (2..upper_limit + 1) {
        if *x % i == 0 {
            return false;
        }
    }
    true
}

pub fn prime_sieve(start: i32, end: i32) -> Vec<i32> {
    // all numbers in the sieve should be prime
    // start at the first prime and remove all of it's multiples
    let mut sieve = (start..end).collect::<Vec<_>>();

    if sieve.contains(&0) {
        let index = sieve.position_elem(&0).unwrap();
        sieve.remove(index);
    }
    if sieve.contains(&1) {
        let index = sieve.position_elem(&1).unwrap();
        sieve.remove(index);
    }

    for num in start..end {
        if sieve.contains(&num) && !is_prime(&num) {
            for multiple in (num..end).step_by(num).collect::<Vec<_>>() {
                let index = sieve.binary_search(&multiple).unwrap();
                sieve.remove(index);
            }
        }
    }
    sieve
}

#[test]
fn basic_prime() {
    assert!(is_prime(&3));
    assert!(is_prime(&5));
    assert!(!is_prime(&8));
}
#[test]
fn double_digits(){
    assert!(is_prime(&97));
    assert!(is_prime(&17));
    assert!(!is_prime(&40));
    assert!(!is_prime(&39));
}
#[test]
fn triple_digits() {
    assert!(!is_prime(&100));
}
#[test]
fn sieve_to_ten() {
    assert_eq!(prime_sieve(0, 10), vec![2, 3, 5, 7]);
}
#[test]
fn sieve_to_hunnid(){
    assert_eq!(prime_sieve(0, 100), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37,
    41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]); 
}
