pub fn fibonacci(n: u32) -> u32 {
    // if n == 0 {
    //   return 0;
    // } else if n == 1 {
    //   return 1;
    // } else {
    //   return fibonacci(n - 1) + fibonacci(n - 2);
    // }

    return match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    };
}
