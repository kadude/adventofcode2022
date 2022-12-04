use std::collections::HashSet;
use std::ops::RangeInclusive;
use crate::helper;

pub fn run() {
    part1();
    part2();
}

fn combine_ranges(x: &String) -> (RangeInclusive<usize>, RangeInclusive<usize>, usize) {
    let split = x.split_once(",").unwrap();
    let first = split.0.split_once("-").unwrap();
    let second = split.1.split_once("-").unwrap();
    let r1 = first.0.parse::<usize>().unwrap()..=first.1.parse::<usize>().unwrap();
    let r2 = second.0.parse::<usize>().unwrap()..=second.1.parse::<usize>().unwrap();
    let union_len = r1.clone().chain(r2.clone()).collect::<HashSet<_>>().len();
    (r1, r2, union_len)
}

fn part1() {
    let lines = helper::read_lines("day4_data.txt");
    // let lines = helper::read_lines("day4_testdata.txt");
    let answer = lines.iter().map(|x| {
        let (r1, r2, union_len) = combine_ranges(x);
        union_len == r1.count() || union_len == r2.count()
    }).filter(|&x| x).count();

    println!("Day 4 Part 1: {:?}", answer);
}

fn part2() {
    let lines = helper::read_lines("day4_data.txt");
    // let lines = helper::read_lines("day4_testdata.txt");
    let answer = lines.iter().map(|x| {
        let (r1, r2, union_len) = combine_ranges(x);
        union_len < r1.count() + r2.count()
    }).filter(|&x| x).count();
    println!("Day 4 Part 2: {:?}", answer);
}
