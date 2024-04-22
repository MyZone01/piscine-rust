pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initials = Vec::<String>::new();
    for name in names {
        let mut initials_str = String::new();
        let mut words = name.split_whitespace();
        while let Some(word) = words.next() {
            let mut chars = word.chars();
            if let Some(initial) = chars.next() {
                initials_str.push(initial);
                initials_str.push_str(". ");
            }
        }
        initials.push(initials_str.trim().to_string());
    }
    initials
}