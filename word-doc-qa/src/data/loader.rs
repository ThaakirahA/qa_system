// src/data/loader.rs
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Load a text file line by line into a vector of strings
pub fn load_text_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect()
}

/// Example function: load all training data from a folder
pub fn load_dataset_from_folder(folder_path: &str) -> Vec<String> {
    use std::fs;

    let mut dataset = Vec::new();

    for entry in fs::read_dir(folder_path).expect("Could not read folder") {
        let entry = entry.expect("Could not get entry");
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "txt") {
            let texts = load_text_file(path.to_str().unwrap());
            dataset.extend(texts);
        }
    }

    dataset
}