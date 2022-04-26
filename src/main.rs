use log::{debug, error, info, warn};
use std::fs::read_dir;

/// Wordnet database dictionary
#[derive(Default, Debug)]
pub struct Wordnet {
}

/// Open a wordnet database directory
pub fn open(path: &str) -> Wordnet {
    // Stub the implementation for now
    let files = std::fs::read_dir(path).unwrap();
    for file in files {
        let entry = file.unwrap();
    }
    Wordnet {}
}

fn main() {
    println!("Hello, world!");
}
