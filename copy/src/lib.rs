pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    (
        a.clone(),
        a.split_whitespace()
            .map(|n| n.parse::<f64>().unwrap().exp().to_string())
            .collect::<Vec<String>>()
            .join(" "),
    )
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let c = b.clone();
    (b, c.iter().map(|n| (n.abs() as f64).ln()).collect())
}
