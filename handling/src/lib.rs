use std::{fs::OpenOptions, io::Write};

pub fn open_or_create(file: &str, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file)
        .unwrap();

    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("File created"),
        Err(e) => panic!("Error while writing to file: {:?}", e),
    }
}
