// Instructions
// Implement the struct Color with the associated function swap. This function returns a Color with the matching values swapped.

// Expected functions
// pub struct Color {
//     pub r: u8,
//     pub g: u8,
//     pub b: u8,
//     pub a: u8,
// }

// impl Color {
//     pub fn swap(mut self, first: u8, second: u8) -> Color {

//     }
// }
// Usage
// Here is a program to test your function.

// use rgb_match::*;

// fn main() {
//     let c = Color {
//         r: 255,
//         g: 200,
//         b: 10,
//         a: 30,
//     };

//     println!("{:?}", c.swap(c.r, c.b));
//     println!("{:?}", c.swap(c.r, c.g));
//     println!("{:?}", c.swap(c.r, c.a));
//     println!();
//     println!("{:?}", c.swap(c.g, c.r));
//     println!("{:?}", c.swap(c.g, c.b));
//     println!("{:?}", c.swap(c.g, c.a));
//     println!();
//     println!("{:?}", c.swap(c.b, c.r));
//     println!("{:?}", c.swap(c.b, c.g));
//     println!("{:?}", c.swap(c.b, c.a));
//     println!();
//     println!("{:?}", c.swap(c.a, c.r));
//     println!("{:?}", c.swap(c.a, c.b));
//     println!("{:?}", c.swap(c.a, c.g));
// }
// And its output:

// $ cargo run
// Color { r: 10, g: 200, b: 255, a: 30 }
// Color { r: 200, g: 255, b: 10, a: 30 }
// Color { r: 30, g: 200, b: 10, a: 255 }

// Color { r: 200, g: 255, b: 10, a: 30 }
// Color { r: 255, g: 10, b: 200, a: 30 }
// Color { r: 255, g: 30, b: 10, a: 200 }

// Color { r: 10, g: 200, b: 255, a: 30 }
// Color { r: 255, g: 10, b: 200, a: 30 }
// Color { r: 255, g: 200, b: 30, a: 10 }

// Color { r: 30, g: 200, b: 10, a: 255 }
// Color { r: 255, g: 200, b: 30, a: 10 }
// Color { r: 255, g: 30, b: 10, a: 200 }

// $

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match (first, second) {
            (f, s) if f == self.r && s == self.g => {
                self.r = self.g;
                self.g = f;
            }
            (f, s) if f == self.r && s == self.b => {
                self.r = self.b;
                self.b = f;
            }
            (f, s) if f == self.r && s == self.a => {
                self.r = self.a;
                self.a = f;
            }
            (f, s) if f == self.g && s == self.r => {
                self.g = self.r;
                self.r = f;
            }
            (f, s) if f == self.g && s == self.b => {
                self.g = self.b;
                self.b = f;
            }
            (f, s) if f == self.g && s == self.a => {
                self.g = self.a;
                self.a = f;
            }
            (f, s) if f == self.b && s == self.r => {
                self.b = self.r;
                self.r = f;
            }
            (f, s) if f == self.b && s == self.g => {
                self.b = self.g;
                self.g = f;
            }
            (f, s) if f == self.b && s == self.a => {
                self.b = self.a;
                self.a = f;
            }
            (f, s) if f == self.a && s == self.r => {
                self.a = self.r;
                self.r = f;
            }
            (f, s) if f == self.a && s == self.b => {
                self.a = self.b;
                self.b = f;
            }
            (f, s) if f == self.a && s == self.g => {
                self.a = self.g;
                self.g = f;
            }
            _ => {}
        }
        self
    }
}
