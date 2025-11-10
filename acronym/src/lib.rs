pub fn abbreviate(phrase: &str) -> String {

    phrase.chars().map(|c| if c == '-' || c == '_' || c == ' ' { ' ' } else { c })
    .collect::<String>()
    .split_whitespace()
    .map(|word| {
        if word.chars().all(|c| c.is_uppercase()){
            return word[0..1].to_string();
        }
        let mut chars = word.chars();
        match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
     })
    .collect::<String>()
    .chars()
    .filter(|c| c.is_uppercase())
    .collect::<String>()
}
