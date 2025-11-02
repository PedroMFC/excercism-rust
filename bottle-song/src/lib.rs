const NUM_WORDS: [&str; 11] = [
    "no", "One", "Two", "Three", "Four",
    "Five", "Six", "Seven", "Eight", "Nine", "Ten"
];

pub fn verse(number: u32) -> String {
   format!(
            "{} green {} hanging on the wall,\n{} green {} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {} green {} hanging on the wall.",
            NUM_WORDS[(number) as usize],
            if number == 1 { "bottle" } else { "bottles" },
            NUM_WORDS[(number) as usize],
            if number == 1 { "bottle" } else { "bottles" },
            NUM_WORDS[(number - 1) as usize].to_lowercase(),
           if number - 1 == 1 { "bottle" } else { "bottles" },
        )
}

    
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result = String::new();
    for i in 0..take_down {
        if i > 0 {
            result.push_str("\n\n");
        }
        result.push_str(&verse(start_bottles - i));
    }
    result
}

// NOTE: Dictionary can be implemented as a match statement
// fn string_for_num(n: u32) -> String {
//     match n {
//         10 => "Ten",
//         9 => "Nine",
//         8 => "Eight",
//         7 => "Seven",
//         6 => "Six",
//         5 => "Five",
//         4 => "Four",
//         3 => "Three",
//         2 => "Two",
//         1 => "One",
//         0 => "No",
//         _ => "what?"
//     }.to_string()
// }