/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut result = false;
    let mut digits: Vec<char> = code.chars().filter(|c| *c != ' ').collect();
    let no_digit_present = digits.iter().any(|c| !c.is_ascii_digit());

    if digits.len() > 1 && !no_digit_present {
        let size = digits.len();
        for i in (2..=size).step_by(2) {
            let mut value = digits[size - i].to_digit(10).unwrap();
            value *= 2;
            if value > 9 {
                value -= 9;
            }
            digits[size - i] = value.to_string().chars().next().unwrap();
        }

        result = digits.iter().map(|c| c.to_digit(10).unwrap()).sum::<u32>() % 10 == 0;
    }

    result
}
