// ## display_table

// ### Instructions

// Define the `Table` struct below, and implement the associated functions `new` and `add_row`. You can see how they should work from the usage.

// Implement the `std::fmt::Display` trait for the `Table` structure so that the table is printed like in the usage. The length of each column must adjust to the longest element of the column, and the element must be centered in the "cell" when possible. If the element cannot be centred exactly, it must be positioned slightly to the left.

// If the table is empty `println!` must not print anything.

// ### Expected functions and Structures

// ```rust
// #[derive(Clone, Debug, PartialEq)]
// pub struct Table {
// 	pub headers: Vec<String>,
// 	pub body: Vec<Vec<String>>,
// }

// impl fmt::Display for Table {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

//     }
// }

// impl Table {
// 	pub fn new() -> Table {

// 	}
// 	pub fn add_row(&mut self, row: &[String]) {

// 	}
// }
// ```

// ### Usage

// Here is a possible program to test your function:

// ```rust
// use display_table::*;

// fn main() {
//     let mut table = Table::new();
//     println!("{}", table);
//     table.headers = vec![
//         String::from("Model"),
//         String::from("Piece N°"),
//         String::from("In Stock"),
//         String::from("Description"),
//     ];
//     table.add_row(&[
//         String::from("model 1"),
//         String::from("43-EWQE304"),
//         String::from("30"),
//         String::from("Piece for x"),
//     ]);
//     table.add_row(&[
//         String::from("model 2"),
//         String::from("98-QCVX5433"),
//         String::from("100000000"),
//         String::from("-"),
//     ]);
//     table.add_row(&[
//         String::from("model y"),
//         String::from("78-NMNH"),
//         String::from("60"),
//         String::from("nothing"),
//     ]);
//     println!("{}", table);
// }

// ```

// And its output:

// ```console
// $ cargo run
// |  Model  |  Piece N°   | In Stock  | Description |
// |---------+-------------+-----------+-------------|
// | model 1 | 43-EWQE304  |    30     | Piece for x |
// | model 2 | 98-QCVX5433 | 100000000 |      -      |
// | model y |   78-NMNH   |    60     |   nothing   |
// $
// ```

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

use std::fmt;

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.headers.is_empty() {
            return Ok(());
        }

        let mut max_len = vec![0; self.headers.len()];
        for (i, header) in self.headers.iter().enumerate() {
            max_len[i] = header.len();
        }

        for row in &self.body {
            for (i, cell) in row.iter().enumerate() {
                if cell.len() > max_len[i] {
                    max_len[i] = cell.len();
                }
            }
        }

        let mut formatted_header = String::new();
        let mut separator = String::new();
        for (i, header) in self.headers.iter().enumerate() {
            let padding = max_len[i] - header.len();
            let left_padding = padding / 2;
            let right_padding = padding - left_padding;
            formatted_header.push_str(&" ".repeat(left_padding));
            formatted_header.push_str(header);
            formatted_header.push_str(&" ".repeat(right_padding));
            formatted_header.push_str(" | ");
            separator.push_str(&"-".repeat(max_len[i]));
            separator.push_str("-+-");
        }
        formatted_header.pop();
        separator.pop();
        separator.pop();

        writeln!(f, "| {}", formatted_header)?;
        writeln!(f, "|-{}|", separator)?;

        for row in &self.body {
            let mut line = String::new();
            for (i, cell) in row.iter().enumerate() {
                let padding = max_len[i] - cell.len();
                let left_padding = padding / 2;
                let right_padding = padding - left_padding;
                line.push_str(&" ".repeat(left_padding));
                line.push_str(cell);
                line.push_str(&" ".repeat(right_padding));
                line.push_str(" | ");
            }
            line.pop();
            writeln!(f, "| {}", line)?;
        }

        Ok(())
    }
}
