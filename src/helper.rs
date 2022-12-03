use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_lines(filename: &str) -> Vec<String> {
    let reader = BufReader::new(File::open(format!("./test_datasets/{}", filename)).expect("no such file"));
    reader.lines().map(|l| l.unwrap()).collect()
}

