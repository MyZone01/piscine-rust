pub fn add_curry(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

pub fn twice(f: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    move |x| f(f(x))
}
