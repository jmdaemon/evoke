// Third Party Crates
use log::{debug, error, info, warn};

// Standard Library
use std::fs::read_dir;
use std::path::Path;
use std::collections::HashMap;
use std::process::exit;

/// Wordnet database dictionary
#[derive(Default, Debug)]
pub struct Wordnet<'a> {
    pub indices: HashMap<&'a str, String>,
    pub data: HashMap<&'a str, String>
}

/// Initializes the wordnet database struct
pub fn init(path: &str) -> Wordnet {
    // Stub the implementation for now
    if !Path::new(path).exists() {
        error!("{}", format!("Could not open {}", path));
        exit(0);
    }
    // If the wordnet directory exists
    let index_prefix = "index";
    let data_prefix = "data";
    let word_types = ["adj", "adv", "noun", "sense", "verb" ];

    // Initialize the wordnet indices
    let mut indices = HashMap::new();
    for word_type in word_types {
        let filename = format!("{}.{}", index_prefix, word_type);
        let filepath = format!("{}/{}", path, filename);
        let file = Path::new(&filepath);
        if file.exists() {
            indices.insert(word_type, filename);
        }
    }

    // Initialize the wordnet data files
    let mut data = HashMap::new();
    for word_type in word_types {
        let filename = format!("{}.{}", data_prefix, word_type);
        let filepath = format!("{}/{}", path, filename);
        let file = Path::new(&filepath);
        if file.exists() {
            data.insert(word_type, filename);
        }
    }

    // Open the index files
    // Get the extension [noun, adv, adj]
    // Open the data files
    // Initialize Wordnet with index and data files


    //let files = std::fs::read_dir(path).unwrap();
    //for file in files {
        //let entry = file.unwrap();
    //}
    Wordnet { indices, data }
}

fn main() {
    println!("Hello, world!");
}
