// ## filter_table

// ### Instructions

// - Define the associated functions for the `Table` struct:
//   - `new`: which creates a new empty table.
//   - `add_rows`: which adds a new row to the table from a slice of strings.
//   - `filter_cols`: which receives a closure and returns a table with all the columns that yielded true when applying that closure. The closure will receive a `&str` and return a `bool` value.
//   - `filter_rows`: which receives a closure and returns a table with all the rows that yielded true when applied to the elements of the selected column. The closure will receive a `&str` and return a `bool` value.

// ### Expected functions and Structures

// ```rust
// #[derive(Clone, Debug, PartialEq)]
// pub struct Table {
// 	pub headers: Vec<String>,
// 	pub body: Vec<Vec<String>>,
// }

// impl Table {
// 	pub fn new() -> Table {

// 	}

// 	pub fn add_row(&mut self, row: &[String]) {

// 	}

// 	pub fn filter_col(&self, filter: T) -> Option<Self> {

// 	}

// 	pub fn filter_row(&self, col_name: &str, filter: T) -> Option<Self> {

// 	}
// }
// ```

// ### Usage

// Here is a possible program to test your function:

// ```rust
// use filter_table::*;

// fn main() {
//     let mut table = Table::new();
//     table.headers = vec![
//         "Name".to_string(),
//         "Last Name".to_string(),
//         "ID Number".to_string(),
//     ];
//     table.add_row(&[
//         "Adam".to_string(),
//         "Philips".to_string(),
//         "123456789".to_string(),
//     ]);
//     table.add_row(&[
//         "Adamaris".to_string(),
//         "Shelby".to_string(),
//         "1111123456789".to_string(),
//     ]);
//     table.add_row(&[
//         "Ackerley".to_string(),
//         "Philips".to_string(),
//         "123456789".to_string(),
//     ]);
//     let filter_names = |col: &str| col == "Name";
//     println!("{:?}", table.filter_col(filter_names));

//     let filter_philips = |lastname: &str| lastname == "Philips";
//     println!("{:?}", table.filter_row("Last Name", filter_philips));
// }
// ```

// And its output:

// ```console
// $ cargo run
// Some(Table { headers: ["Name"], body: [["Adam"], ["Adamaris"], ["Ackerley"]] })
// Some(Table { headers: ["Name", "Last Name", "ID Number"], body: [["Adam", "Philips", "123456789"], ["Ackerley", "Philips", "123456789"]] })
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

    pub fn filter_col(&self, filter: impl Fn(&str) -> bool) -> Option<Self> {
        let mut new_table = Table::new();
        let mut new_headers = Vec::new();
        let mut indices = Vec::new();

        for (i, header) in self.headers.iter().enumerate() {
            if filter(header) {
                new_headers.push(header.clone());
                indices.push(i);
            }
        }

        if new_headers.is_empty() {
            None
        } else {
            new_table.headers = new_headers;
            new_table.body = self
                .body
                .iter()
                .map(|row| indices.iter().map(|&i| row[i].clone()).collect())
                .collect();
            Some(new_table)
        }
    }

    pub fn filter_row(&self, col_name: &str, filter: impl Fn(&str) -> bool) -> Option<Self> {
        let col_index = self.headers.iter().position(|header| header == col_name);

        if let Some(col_index) = col_index {
            let new_headers = self.headers.clone();
            let mut new_body = self.body.clone();
            new_body.retain(|row| filter(&row[col_index]));
            Some(Table {
                headers: new_headers,
                body: new_body,
            })
        } else {
            None
        }
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}
