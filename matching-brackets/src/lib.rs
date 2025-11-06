pub fn brackets_are_balanced(string: &str) -> bool {
    let mut char_stack = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => char_stack.push(c),
            ')' | ']' | '}' => {
                if let Some(last) = char_stack.pop() {
                    if (c == ')' && last != '(') || (c == ']' && last != '[') || (c == '}' && last != '{') {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }
    char_stack.is_empty()
}
