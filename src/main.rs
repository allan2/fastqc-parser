use std::{fs, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
struct Summary {
    pass: bool,
    filename: String,
}

fn main() {
    let data_dir = Path::new("data");

    let paths = fs::read_dir(data_dir).unwrap();

    // Check all paths in the data directory.
    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
    println!("Hello, world!");
}
