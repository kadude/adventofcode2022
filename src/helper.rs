use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(filename: &str) -> Vec<String> {
    let reader = BufReader::new(File::open(format!("./test_datasets/{}", filename)).expect("no such file"));
    reader.lines().map(|l| l.unwrap()).collect()
}

