pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut skip = 0;

    for ch in s.chars() {
        if skip == 0 && ch != '+' {
            if ch == '-' {
                result.pop();
            } else {
                result.push(ch)
            }
        } else if ch == '+' {
            skip += 1;
        } else {
            skip -= 1;
        }
    }

    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        let res = operation(s.to_string());
        *s = res;
    }
}

pub fn operation(s: String) -> String {
    let mut res = String::new();

    if s.contains('+') {
        let operators: Vec<i32> = s.split('+').map(|s| s.parse::<i32>().unwrap()).collect();
        let ires = operators[0] + operators[1];
        res = ires.to_string()
    } else if s.contains('-') {
        let operators: Vec<i32> = s.split('-').map(|s| s.parse::<i32>().unwrap()).collect();
        let ires = operators[0] - operators[1];
        res = ires.to_string()
    }

    res
}
