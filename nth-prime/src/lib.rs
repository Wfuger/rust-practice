pub fn nth(n: u32) -> Option<u32> {
    let mut prime: Option<u32> = None;
    let mut current: u32 = 2;
    let mut count: u32 = 0;
    while count < n {
        if is_prime(current) {
            prime = Some(current);
            count += 1;
        }
        current += 1;
    }
    return prime
}

fn is_prime(num: u32) -> bool {
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    true
}
