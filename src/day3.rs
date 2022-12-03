use crate::helper;

static CHARS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
];

pub fn run() {
    part1();
    part2();
}

fn get_priority(item: char) -> usize {
    CHARS.iter()
        .position(|&c| c == item.to_ascii_lowercase()).unwrap()
        + if item.is_ascii_uppercase() { 27 } else { 1 }
}

fn part1() {
    let lines = helper::read_lines("day3_data.txt");
    // let lines = helper::read_lines("day3_testdata.txt");

    let priority_sum: usize = lines.iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(part1, part2)| part1.chars().find(|c| part2.contains(*c)).unwrap())
        .map(|x| get_priority(x)).sum();

    println!("Day 3 Part 1: {:?}", priority_sum);
}

fn part2() {
    let lines = helper::read_lines("day3_data.txt");
    // let lines = helper::read_lines("day3_testdata.txt");

    let priority_sum: usize = lines
        .chunks(3)
        .map(|x| {
            if let Some(found) = CHARS.iter().find(|&&c| x[0].contains(c) && x[1].contains(c) && x[2].contains(c)) {
                *found
            } else {
                CHARS.into_iter().map(|c| c.to_ascii_uppercase()).find(|&c| x[0].contains(c) && x[1].contains(c) && x[2].contains(c)).unwrap()
            }
        }).map(|x| get_priority(x)).sum();

    println!("Day 3 Part 2: {:?}", priority_sum);
}
