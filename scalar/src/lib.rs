// Length	Signed	Unsigned  Range
// 8-bit	i8	    u8       -128 to 127 or 0 to 255
// 16-bit	i16	    u16      -32768 to 32767 or 0 to 65535
// 32-bit	i32	    u32      -2147483648 to 2147483647 or 0 to 4294967295
// 64-bit	i64	    u64      -9223372036854775808 to 9223372036854775807 or 0 to 18446744073709551615
// 128-bit	i128	u128     -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727 or 0 to 340282366920938463463374607431768211455

// The return keyword is not required to return a variable out of a function. You can just write the expression to return without a semicolon at the end

pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}
