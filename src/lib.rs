#![feature(step_by)]
#![feature(collections)]
#![feature(test)]

extern crate test;

pub fn is_prime(x: &i32) -> bool {
   // Handle small edge cases first
    if *x == 2 {
        return true;
    } else if *x == 0 || *x == 1 {
        return false
    }

    // once we pass the sqrt of a number, we don't need to proceed checking for divisors
    let upper_limit: i32 = (*x as f64).sqrt().ceil() as i32;

    for i in (2..upper_limit + 1) {
        if *x % i == 0 {
            return false;
        }
    }
    true
}

pub fn prime_sieve(start: &i32, end: &i32) -> Vec<i32> {
    // create list of all numbers in range that we will then trim into a prime sieve
    let mut sieve = (*start..*end + 1).collect::<Vec<_>>();

    // remove 0 or 1 from list to prevent step_by(0) or step_by(1) disaster
    if sieve.contains(&0) {
        let index = sieve.position_elem(&0).unwrap();
        sieve.remove(index);
    }
    if sieve.contains(&1) {
        let index = sieve.position_elem(&1).unwrap();
        sieve.remove(index);
    }

    for num in (2..*end + 1) {
        // look for primes to start trimming our sieve with
        if is_prime(&num) {
            // remove all multiples of this number that are in the sieve
            for multiple in (num * 2..*end + 1).step_by(num).collect::<Vec<_>>() {
                if sieve.contains(&multiple) {
                    let index = sieve.position_elem(&multiple).unwrap();
                    sieve.remove(index);
                }
            }
        }
    }
    // now let's return our actual sieve
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
    assert_eq!(prime_sieve(&0, &10), vec![2, 3, 5, 7]);
}
#[test]
fn sieve_to_hunnid(){
    assert_eq!(prime_sieve(&0, &100), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37,
    41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]); 
}
#[test]
fn sieve_gross_range() {
    // should be inclusive and handle ranges !starting with 0
    assert_eq!(prime_sieve(&11, &31), vec![11, 13, 17, 19, 23, 29, 31]);
}
#[cfg(test)]
mod tests {
    use super::is_prime;
    use super::prime_sieve;
    use test::Bencher;

    #[bench]
    fn timeout_prime(b: &mut Bencher){
        b.iter(|| is_prime(&2147483647));
    }

    #[bench]
    fn timeout_sieve(b: &mut Bencher){
        b.iter(|| prime_sieve(&0, &10000000));
    }
}
