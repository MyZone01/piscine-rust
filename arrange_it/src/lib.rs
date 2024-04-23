pub fn arrange_phrase(phrase: &str) -> String {
    let words: Vec<&str> = phrase.split(' ').collect();
    let mut res = Vec::<String>::with_capacity(words.len());

    words.iter().for_each(|word| {
        res.push(word.to_string());
        res.sort_by(|a, b| {
            let a = a.chars().find(|c| c.is_digit(10)).unwrap();
            let b = b.chars().find(|c| c.is_digit(10)).unwrap();
            a.cmp(&b)
        });
    });

    res.iter_mut().for_each(|word| {
        *word = word.chars().filter(|c| !c.is_digit(10)).collect::<String>();
    });

    res.join(" ").to_string()
}
