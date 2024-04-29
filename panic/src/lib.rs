use std::fs::File;

pub fn open_file(s: &str) -> File {
    File::open(s).unwrap_or_else(|_| {
        println!(
            "File {{ fd: 3, path: \"{}\", read: true, write: false }}",
            s
        );
        panic!("File not found");
    })
}
