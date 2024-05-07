pub fn first_subword(s: String) -> String {
    let mut first = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_lowercase() || (c.is_ascii_uppercase() && i == 0) {
            first.push(c);
        } else {
            break;
        }
    }
    first
}
