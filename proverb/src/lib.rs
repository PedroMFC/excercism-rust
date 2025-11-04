pub fn verse(n: &str, m: &str) -> String {
    format!("For want of a {n} the {m} was lost.\n")
}

pub fn final_verse(n: &str) -> String {
    format!("And all for the want of a {n}.")
}

pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();

    if list.is_empty() {
        return result;
    }
    (0..list.len()-1).for_each(|i| {
        result.push_str(&verse(list[i], list[i+1]));
    });

    result.push_str(&final_verse(list[0]));
    result
}
