pub fn stars(n: u32) -> String {
    "*".to_string().repeat(u32::pow(2, n) as usize)
}
