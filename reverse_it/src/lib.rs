pub fn reverse_it(v: i32) -> String {
    let mut rev = v;
    let mut result = String::new();
    let is_negative = rev < 0;
    if is_negative {
        rev = -rev;
        result.push('-');
    }
    if rev == 0 {
        return "00".to_string();
    }
    while rev > 0 {
        let digit = rev % 10;
        result.push_str(&digit.to_string());
        rev /= 10;
    }
    if is_negative {
        result + &(-v).to_string()
    } else {
        result + &v.to_string()
    }
}