pub fn reply(message: &str) -> &str {
    let is_uppercase = message.chars().filter(|c| c.is_alphanumeric()).all(|c| c.is_uppercase() || c.is_numeric()) && message.chars().any(|c| c.is_alphabetic());
    match message.chars().filter(|c| !c.is_whitespace()).collect::<String>().ends_with('?') {
        true =>  match is_uppercase{
            true => "Calm down, I know what I'm doing!",
            false => "Sure.",
        },
        false => match is_uppercase {
            true => "Whoa, chill out!",
            false => match message.chars().filter(|c| !c.is_whitespace()).collect::<String>().is_empty() {
                true => "Fine. Be that way!",
                false => "Whatever.",
            },
        },
    }
}
