pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..((n as f64).sqrt() + 1.0) as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    if n < 1 {
         2
    } else {
        let prev_prime = nth(n - 1);
        let mut candidate = prev_prime + 1;
        loop {
            if is_prime(candidate) {
                return candidate;
            }
            candidate += 1;
        }
    }
}

