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


// COMMUNITY SOLUTION
// pub fn is_valid(code: &str) -> bool {
//     code.chars()
//         .rev()
//         .filter(|c| !c.is_whitespace())
//         .try_fold((0, 0), |(sum, count), val| {
//             val.to_digit(10)
//                 .map(|num| if count % 2 == 1 { num * 2 } else { num })
//                 .map(|num| if num > 9 { num - 9 } else { num })
//                 .map(|num| (num + sum, count + 1))
//         }).map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
// }
