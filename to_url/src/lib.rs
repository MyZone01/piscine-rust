pub fn to_url(s: &str) -> String {
    s.to_string().replace(' ', "%20")
}
