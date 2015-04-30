pub fn is_prime(x: i32) -> bool {
    if x == 0 || x == 1 || x == 2 || x == 3 || x == 5 || x == 7 {
        return true;
    }

    let upper_limit: i32 = (x as f64).sqrt().ceil() as i32;
    let mut sieve = (1..upper_limit).filter(|x| { x % 2 == 1 }).collect::<Vec<i32>>();

    let mut result: bool;
    for num in sieve {
       result = prime_test(num, &x, &mut sieve);
        if result {
            break;
        }
    }
    result
}

pub fn prime_test(num: i32, x: &i32, sieve: &mut Vec<i32>) -> bool {
    // check if this specific number in sieve is prime
    for i in (1..num).collect::<Vec<i32>>() {
        if num % i != 0 {
            // if it's not a prime, remove it and all multiples from sieve
            for multiple in (i..x + 1).step_by(i).collect::<Vec<i32>>() {
                if multiple == *x {
                    return true
                }
                sieve.iter().position(|x| *x == multiple).map(|element| &sieve.remove(element));
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
    assert!(is_prime(17));
}
