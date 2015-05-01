pub fn is_prime(x: i32) -> bool {
    // Our sieve goes from 1 to sqrt of x - catch these small cases first to avoid
    // cases where sieve = vec![1] (if x >= 9 sieve will be vec![1, 3] which is ok)
    if x < 9 {
        if x == 0 || x == 1 || x == 2 || x == 3 || x == 5 || x == 7 {
            return true;
        } else {
            return false
        }
    }

    // a number is prime if it is only divisible by 1 & itself
    // once we pass the sqrt of a number, we don't need to proceed checking for divisors
    // so we'll create a sieve of *potential* divisors between 1..√x
    // ( a sieve is essentially all prime numbers up to a certain point
    let upper_limit: i32 = (x as f64).sqrt().ceil() as i32;
    let mut sieve = (1..upper_limit).filter(|x| { x % 2 == 1 && x != 2 }).collect::<Vec<i32>>();

    // check if the numbers in the sieve are divisors for x (i.e. x is prime)
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
    // all numbers in the sieve should be prime
    // start at the first prime and remove all of it's multiples
    // (if we run across our target number during this^ process then it is not prime
    // and we can return false)
    // if we hit our target and it hasn't been removed then we can assume it's prime
    for i in (1..num).collect::<Vec<i32>>() {
        if num % i != 0 {
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
