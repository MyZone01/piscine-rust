pub fn first_subword(s: String) -> String {
    let mut first = String::new();
    for (i, c) in s.chars().into_iter().enumerate() {
        if (c >= 'a' && c <= 'z') ||  (c >= 'A' && c <= 'Z' && i == 0) {
            first.push(c);
        } else {
            break;
        }
    }
    first
}