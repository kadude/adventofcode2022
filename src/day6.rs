use std::collections::HashSet;
use crate::helper;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let lines = helper::read_lines("day6_data.txt");
    // let lines = helper::read_lines("day6_testdata.txt");
    let line = &lines[0];
    let chars = line.chars().collect::<Vec<char>>();
    for (i, _) in chars.iter().enumerate() {
        let to = i + 4;
        let len = if to >= chars.len() { chars.len() - 1 } else { to };
        let x = &chars[i..len].into_iter().collect::<HashSet<&char>>();
        if x.len() > 3 {
            println!("Day 6 Part 1: {:?}", to);
            break;
        };
    }
}

fn part2() {
    let lines = helper::read_lines("day6_data.txt");
    // let lines = helper::read_lines("day6_testdata.txt");
    let line = &lines[0];
    let chars = line.chars().collect::<Vec<char>>();
    for (i, _) in chars.iter().enumerate() {
        let to = i + 14;
        let len = if to >= chars.len() { chars.len() - 1 } else { to };
        let x = &chars[i..len].into_iter().collect::<HashSet<&char>>();
        if x.len() > 13 {
            println!("Day 6 Part 2: {:?}", to);
            break;
        };
    }
}
