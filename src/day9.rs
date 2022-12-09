use crate::helper;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let lines = helper::read_lines("day9_data.txt");
    // let lines = helper::read_lines("day9_testdata.txt");

    let mut grid = [[false; 1000]; 1000];
    let start = grid.len() / 2;
    let mut head_pos = (start, start);
    let mut tail_pos = (start, start);
    grid[start][start] = true;

    lines.iter().for_each(|line| {
        let (direction, l) = line.split_once(" ").unwrap();
        let length = l.parse::<usize>().unwrap();
        (0..length).for_each(|_| {
            let temp_head_pos = head_pos.clone();
            if direction == "R" {
                head_pos = (head_pos.0 + 1, head_pos.1);
            } else if direction == "U" {
                head_pos = (head_pos.0, head_pos.1 + 1);
            } else if direction == "L" {
                head_pos = (head_pos.0 - 1, head_pos.1);
            } else if direction == "D" {
                head_pos = (head_pos.0, head_pos.1 - 1);
            }

            let x_pos = if head_pos.0 == 0 { 0 } else { head_pos.0 - 1 };
            let y_pos = if head_pos.1 == 0 { 0 } else { head_pos.1 - 1 };

            if !((x_pos..=head_pos.0 + 1).contains(&tail_pos.0) && (y_pos..=head_pos.1 + 1).contains(&tail_pos.1)) {
                tail_pos = temp_head_pos.clone();
            }
            grid[tail_pos.0][tail_pos.1] = true;
        });
    });

    let result = grid.iter().map(|line| line.iter().filter(|&&pos| pos).count()).sum::<usize>();

    println!("Day 9 Part 1: {:?}", result);
}

fn part2() {
    let lines = helper::read_lines("day9_data.txt");
    // let lines = helper::read_lines("day9_testdata.txt");

    let mut grid = [[false; 1000]; 1000];
    let start = grid.len() / 2;
    grid[start][start] = true;

    let mut arr = [(start, start); 10];
    let mut shadow = [(start, start); 10];

    lines.iter().for_each(|line| {
        let (direction, l) = line.split_once(" ").unwrap();
        let length = l.parse::<usize>().unwrap();
        (0..length).for_each(|_| {
            arr.clone().iter().enumerate().for_each(|(i, (x, y))| {
                shadow[i] = (x.clone(), y.clone());
                if i == 0 {
                    if direction == "R" {
                        arr[i] = (x + 1, *y);
                    } else if direction == "U" {
                        arr[i] = (*x, y + 1);
                    } else if direction == "L" {
                        arr[i] = (x - 1, *y);
                    } else if direction == "D" {
                        arr[i] = (*x, y - 1);
                    }
                } else {
                    let (prev_x, prev_y) = arr[i - 1];
                    if !((prev_x - 1..=prev_x + 1).contains(x) && (prev_y - 1..=prev_y + 1).contains(y)) {
                        let new_y = if ((prev_y as i32 - *y as i32) as i32) > 1 {
                            y + 1
                        } else if ((prev_y as i32 - *y as i32) as i32) < -1 {
                            y - 1
                        } else { *y };
                        let new_x = if ((prev_x as i32 - *x as i32) as i32) > 1 {
                            x + 1
                        } else if ((prev_x as i32 - *x as i32) as i32) < -1 {
                            x - 1
                        } else { *x };
                        if new_x == *x && new_y == *y {
                            arr[i] = shadow[i - 1].clone();
                        } else {
                            arr[i] = (
                                if new_x != *x { new_x } else { prev_x },
                                if new_y != *y { new_y } else { prev_y }
                            );
                        }
                    }
                }
            });
            grid[arr[9].0][arr[9].1] = true;
        });
    });
    let result = grid.iter().map(|line| line.iter().filter(|&&pos| pos).count()).sum::<usize>();

    println!("Day 9 Part 2: {:?}", result);
}
