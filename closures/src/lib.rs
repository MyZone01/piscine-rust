pub fn first_fifty_even_square() -> Vec<i32> {
    (2..=100)
        .filter_map(|x| if x % 2 == 0 { Some(x * x) } else { None })
        .collect()
}
