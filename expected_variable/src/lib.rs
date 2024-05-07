extern crate case;

pub fn expected_variable(s1: &str, s2: &str) -> Option<String> {
    let is_camel_case =
        s1.chars().any(|c| c.is_ascii_uppercase()) && s1.chars().any(|c| c.is_ascii_lowercase());
    let is_snake_case = s1.contains('_');
    if (!is_camel_case && !is_snake_case) || s1.contains(' ') {
        return None;
    }

    let distance = edit_distance::edit_distance(&s1.to_lowercase(), &s2.to_lowercase());
    let resul = 1.0 - (distance as f64 / s2.len().max(s1.len()) as f64);

    if resul > 0.5 {
        Some(format!("{:.0}%", resul * 100.0))
    } else {
        None
    }
}
