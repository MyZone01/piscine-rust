pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, needle: &str) -> bool {
    v.contains(needle)
}

pub fn split_at(v: &str, mid: usize) -> (&str, &str) {
    v.split_at(mid)
}

pub fn find(v: &str, needle: char) -> usize {
    match v.find(needle) {
        Some(i) => i,
        None => 0,
    }
}

pub fn find_with_unwrap(v: &str, needle: char) -> usize {
    v.find(needle).unwrap_or(0)
}