pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut number = n;

    let mut divisor = 2;
    while number > 1 {
        while number % divisor == 0 {
            number /= divisor;
            result.push(divisor);
            divisor = 2;
        }
        divisor += 1;
    }

    result
}
