use std::collections::{HashMap};
use std::ops::AddAssign;

use crate::helper;

pub fn run() {
    part1();
    part2();
}

fn do_file_calc(lines: Vec<String>) -> HashMap<String, usize> {
    let mut dirstack: Vec<String> = Vec::new();
    dirstack.push("/".to_string());
    let mut dirsize_map: HashMap<String, usize> = HashMap::new();

    lines.iter().for_each(|line| {
        if line.starts_with("$ ") {
            match line {
                x if x.contains("cd ..") => {
                    dirstack.pop();
                }
                x if x.contains("cd /") => {
                    dirstack.clear();
                    dirstack.push("/".to_string());
                }
                x if x.contains("cd ") => {
                    let dir = line.rsplit_once(' ').unwrap().1.to_string();
                    let parent = dirstack.last().unwrap().to_owned();
                    dirstack.push([parent, dir].join("")); // join parent + dir to remedy similar dir-names
                }
                _ => {}
            }
        } else if line.as_bytes()[0].is_ascii_digit() {
            let size = line.split_once(' ').unwrap().0.parse::<usize>().unwrap();
            dirstack.iter().for_each(|d| {
                dirsize_map.entry(d.to_owned()).or_insert(0).add_assign(size);
            });
        };
    });
    dirsize_map
}

fn part1() {
    let lines = helper::read_lines("day7_data.txt");
    // let lines = helper::read_lines("day7_testdata.txt");
    let dirsize_map = do_file_calc(lines);
    println!("Day 7 Part 1: {:?}", dirsize_map.iter().map(|(_, size)| *size).filter(|&size| size <= 100000).sum::<usize>());
}

const MAX_SIZE: usize = 70000000;
const NEEDED_TOTAL_SPACE: usize = 30000000;
fn part2() {
    let lines = helper::read_lines("day7_data.txt");
    // let lines = helper::read_lines("day7_testdata.txt");
    let dirsize_map = do_file_calc(lines);
    let used_space = dirsize_map.get("/").unwrap();
    let available_space = MAX_SIZE - used_space;
    let space_to_clear = NEEDED_TOTAL_SPACE - available_space;
    println!("Day 7 Part 2: {:?}", dirsize_map.iter().map(|(_, size)| *size).filter(|&size| size > space_to_clear).min().unwrap());
}
