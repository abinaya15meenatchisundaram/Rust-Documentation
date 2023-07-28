use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file=File::create("output.txt")
        .expect("Could not create A file");
    file.write_all(b"welcome to my repo")
        .expect("Cannot write to file");
}
