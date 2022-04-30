#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
// Include wordnet bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Third Party Crates
use log::{debug, error, info, warn};

// Standard Library
use std::ffi::{CStr, CString};
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
            indices.insert(word_type, filepath);
        }
    }

    // Initialize the wordnet data files
    let mut data = HashMap::new();
    for word_type in word_types {
        let filename = format!("{}.{}", data_prefix, word_type);
        let filepath = format!("{}/{}", path, filename);
        let file = Path::new(&filepath);
        if file.exists() {
            data.insert(word_type, filepath);
        }
    }

    Wordnet { indices, data }
}

/// Find all senses for a word
//pub fn senses(wordnet: Wordnet, word: &str) -> Vec<String> {
    //let mut senses = vec![];

    //// Search database
//}

fn main() {
    //println!("Hello, world!");

    unsafe {
        wninit();
        println!("Wordnet database has been initialized");

        //let word = CString::new("bank").unwrap();
        //let mut typess: &'static [u8;2usize] = b"s\0";
        let mut sensetype = 's' as libc::c_char;
        //let sensetypeptr: *const i8 = sensetype.as_ptr().cast();
        //let typess = std::ptr::addr_of!(sensetype);

        //let sstype = getsstype(typess);
        let sstype = getsstype(&mut sensetype);
        println!("sstype: {}", sstype);
    }
}
