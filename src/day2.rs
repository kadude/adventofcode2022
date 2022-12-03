use std::fs;
use std::io::stderr;

use crate::helper;

const WIN_POINTS: u8 = 6;
const LOSS_POINTS: u8 = 0;
const DRAW_POINTS: u8 = 3;
const SCISSORS_POINTS: u8 = 3;
const PAPER_POINTS: u8 = 2;
const ROCK_POINTS: u8 = 1;

const ELF_GUIDE: [&str; 3] = ["A", "B", "C"];
const OUR_GUIDE: [&str; 3] = ["X", "Y", "Z"];
const POINT_GUIDE: [u8; 3] = [ROCK_POINTS, PAPER_POINTS, SCISSORS_POINTS];

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let lines = helper::read_lines("day2_data.txt");
    // let lines = helper::read_lines("day2_testdata.txt");
    let score = lines.iter().map(|line| {
        let mut split_whitespace = {
            line.split_whitespace()
        };
        get_score(split_whitespace.next().unwrap(), split_whitespace.next().unwrap())
    }).sum::<usize>();

    println!("Part 1: {:?}", score);
}

fn get_score(elf: &str, me: &str) -> usize {
    let my_choice = OUR_GUIDE.iter().position(|&a| a == me).unwrap();
    let elf_choice = ELF_GUIDE.iter().position(|&a| a == elf).unwrap();

    let result: usize = if (my_choice > elf_choice && !(my_choice == 2 && elf_choice == 0)) || (my_choice == 0 && elf_choice == 2) {
        WIN_POINTS
    } else if my_choice == elf_choice {
        DRAW_POINTS
    } else /* if (my_choice < elf_choice && !(my_choice == 0 && elf_choice == 2)) || (my_choice == 2 && elf_choice == 0)*/ {
        LOSS_POINTS
    } as usize + POINT_GUIDE[my_choice] as usize;

    result
}

pub fn part2() {
    let lines = helper::read_lines("day2_data.txt");
    // let lines = helper::read_lines("day2_testdata.txt");
    let score = lines.iter().map(|line| {
        let mut split_whitespace = {
            line.split_whitespace()
        };
        get_score2(split_whitespace.next().unwrap(), split_whitespace.next().unwrap())
    }).sum::<usize>();

    println!("Part 2: {:?}", score);
}

fn get_score2(elf: &str, requred_outcome: &str) -> usize {
    let elf_choice = ELF_GUIDE.iter().position(|&a| a == elf).unwrap();
    let mut guide = OUR_GUIDE.clone();
    for _ in 0..(elf_choice + match requred_outcome {
        "X" => 2,
        "Y" => 0,
        "Z" => 1,
        _ => { panic!("ERROR!") }
    }) {
        guide.rotate_left(1);
    };
    let my_choice: &str = guide[0];
    let result = match requred_outcome {
        "X" => LOSS_POINTS,
        "Y" => DRAW_POINTS,
        "Z" => WIN_POINTS,
        _ => { panic!("ERROR!") }
    } as usize + POINT_GUIDE[OUR_GUIDE.iter().position(|&a| a == my_choice).unwrap()] as usize;
    result
}
