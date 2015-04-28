pub fn is_prime(x: i32) -> bool {
    if x == 2 || x == 1 {
        return true;
    }
    let upper_limit: i32 = (x as f64).sqrt().ceil() as i32;

    for i in 2..upper_limit {
        if x % i == 0 && i != x {
            return false;
        }
    }
    true
}

#[test]
fn basic_prime() {
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(!is_prime(8));
    assert!(is_prime(17));
}
