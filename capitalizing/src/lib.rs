pub fn capitalize_first(input: &str) -> String {
    // let mut result = String::new();

    // for (i, c) in input.chars().enumerate() {
    //     if i == 0 {
    //         result.push(c.to_uppercase().next().unwrap());
    //     } else {
    //         result.push(c);
    //     }
    // }

    // result
    if input.is_empty() {
        return input.to_string();
    }
    input.chars().next().unwrap().to_uppercase().to_string() + &input[1..]
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_whitespace() {
            capitalize_next = true;
            result.push(c);
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

pub fn change_case(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars() {
        match c {
            c if c.is_lowercase() => result.push(c.to_uppercase().next().unwrap()),
            c if c.is_uppercase() => result.push(c.to_lowercase().next().unwrap()),
            _ => result.push(c),
        }
    }

    result
}
