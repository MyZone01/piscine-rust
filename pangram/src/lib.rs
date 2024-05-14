pub fn is_pangram(s: &str) -> bool {
    if s.len() < 26 {
        return false;
    }
    let low = s.to_lowercase();
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .all(|c| low.contains(c))
}
