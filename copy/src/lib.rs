pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let split = a.split_whitespace();
    let mut result = String::new();

    for num in split {
        let num: f64 = num.parse().expect("not a number");
        result.push_str(&(num.exp().to_string() + &String::from(" ")));
    }

    result.pop();

    (a.clone(), result.clone())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let c = b.clone();
    (b, c.iter().map(|elem| (elem.abs() as f64).ln()).collect())
}
