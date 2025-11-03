pub fn is_prime(n: u32) -> bool {
    !(2..(n as f32).sqrt() as u32 + 1).any(|x| n % x == 0)
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

