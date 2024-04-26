pub fn sum(a: &[i32]) -> i32 {
    let mut res: i32 = 0;

    for num in a{
        res += num;
    }

    return res;
}

pub fn thirtytwo_tens() -> [i32; 32]{
    return [10; 32];
}