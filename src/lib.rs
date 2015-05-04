 #![feature(step_by)]
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
    // let mut sieve = (1..upper_limit).filter(|x| { x % 2 == 1 && *x != 2 }).collect::<Vec<i32>>();

    // let mut result: bool = false;
    for i in 2..upper_limit {
        if x % i == 0 {
            // result = prime_test(i, &x, &mut sieve);
            return false;
        }
    }
    true
}

pub fn prime_test(num: i32, x: &i32, sieve: &mut Vec<i32>) -> bool {
    // all numbers in the sieve should be prime
    // start at the first prime and remove all of it's multiples
    // (if we run across our target number during this^ process then it is not prime
    // and we can return false)
    // if we hit our target and it hasn't been removed then we can assume it's prime
    for i in (2..num).collect::<Vec<i32>>() {
        if num % i != 0 {
            for multiple in (i..x + 1).step_by(i).collect::<Vec<i32>>() {
                if multiple == *x {
                    return false;
                }
                if sieve.contains(&multiple) {
                    sieve.remove(multiple as usize);
                }
            }
        } else {
            if x % i == 0 {
                return true;
            }
        }
    }
    false
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
