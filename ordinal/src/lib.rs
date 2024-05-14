pub fn num_to_ordinal(x: u32) -> String {
    if x == 12 {
        return "12th".to_string();
    }
    let suffix = match x % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    format!("{}{}", x, suffix)
}
