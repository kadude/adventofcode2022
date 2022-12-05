use crate::helper;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let lines = helper::read_lines("day5_data.txt");
    // let lines = helper::read_lines("day5_testdata.txt");

    let mut arr: [Vec<char>; 9] = Default::default();
    populate_array(&lines, &mut arr);

    let mut ooo = convert_array(&mut arr);

    lines.iter().for_each(|line| {
        if !line.starts_with("move") {
            return;
        };

        let mut a = line.split(|c| c == ' ')
            .filter(|&s| {
                match s.parse::<usize>() {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }).map(|s| s.parse::<usize>().unwrap());
        let moves = a.next().unwrap();
        let from = a.next().unwrap();
        let to = a.next().unwrap();

        for _ in 0..moves {
            let out = ooo[from - 1].pop().unwrap();
            ooo[to - 1].push(out);
        }
    });

    println!("Day 5 Part 1: {:?}", create_string(&mut ooo));
}


fn part2() {
    let lines = helper::read_lines("day5_data.txt");
    // let lines = helper::read_lines("day5_testdata.txt");

    let mut arr: [Vec<char>; 9] = Default::default();
    populate_array(&lines, &mut arr);

    let mut ooo = convert_array(&mut arr);

    lines.iter().for_each(|line| {
        if !line.starts_with("move") {
            return;
        };

        let mut a = line.split(|c| c == ' ')
            .filter(|&s| {
                match s.parse::<usize>() {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }).map(|s| s.parse::<usize>().unwrap());
        let moves = a.next().unwrap();
        let from = a.next().unwrap();
        let to = a.next().unwrap();

        let mut temp = Vec::new();
        for _ in 0..moves {
            let out = ooo[from - 1].pop().unwrap();
            temp.push(out);
        }
        temp.reverse();
        ooo[to - 1].extend(temp);
    });

    println!("Day 5 Part 2: {:?}", create_string(&mut ooo));
}

fn create_string(ooo: &mut Vec<Vec<char>>) -> String {
    ooo.iter().map(|x| x.last().unwrap_or(&' ').to_string()).filter(|x| x != " ").collect::<Vec<_>>().join("")
}

fn convert_array(arr: &mut [Vec<char>; 9]) -> Vec<Vec<char>> {
    arr.iter()
        .map(
            |vec| vec.iter().rev().filter(|&&c| c != ' ')
                .map(|c| *c).collect::<Vec<char>>()
        ).collect::<Vec<Vec<char>>>()
}

fn populate_array(lines: &Vec<String>, arr: &mut [Vec<char>; 9]) {
    lines.iter().for_each(|line| {
        if !line.contains('[') { return; }
        let mut cs = line.chars();
        cs.next();
        let mut asd = cs.step_by(4);
        let mut x = 0;
        while let Some(cc) = asd.next() {
            arr[x].push(cc);
            x = x + 1;
        }
    });
}
