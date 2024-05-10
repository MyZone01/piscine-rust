// Instructions
// Your task is to implement the trait AppendStr for the type StringValue.

// The trait AppendStr has the following functions:

// append_str: that appends the string to the value.
// append_number: that appends the number to the value.
// remove_punctuation_marks: that removes punctuation from the value (., ,, ? and !).
// Expected Function
// #[derive(Clone)]
// pub struct StringValue {
//     pub value: String,
// }

// pub trait AppendStr {
//     fn append_str(&mut self, str_to_append: String) -> Self;

//     fn append_number(&mut self, nb_to_append: f64) -> Self;

//     fn remove_punctuation_marks(&mut self) -> Self;
// }

// impl AppendStr for StringValue {
// }
// Usage
// Here is a program to test your function.

// use easy_traits::*;

// fn main() {
//     let mut str_aux = StringValue {
//         value: String::from("hello"),
//     };

//     println!("Before append: {}", str_aux.value);

//     str_aux.append_str(String::from(" there!"));
//     println!("After append: {}", str_aux.value);

//     str_aux.remove_punctuation_marks();
//     println!("After removing punctuation: {}", str_aux.value);
// }
// And its output

// $ cargo run
// Before append: hello
// After append: hello there!
// After removing punctuation: hello there
// $

#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String) -> Self;

    fn append_number(&mut self, nb_to_append: f64) -> Self;

    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, str_to_append: String) -> Self {
        self.value.push_str(&str_to_append);
        self.clone()
    }
    fn append_number(&mut self, nb_to_append: f64) -> Self {
        self.append_str(nb_to_append.to_string())
    }
    fn remove_punctuation_marks(&mut self) -> Self {
        self.value = self
            .value
            .replace(|c: char| char::is_ascii_punctuation(&c) && c != '-', "");
        self.clone()
    }
}
