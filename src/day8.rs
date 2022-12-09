use crate::helper;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let lines = helper::read_lines("day8_data.txt");
    // let lines = helper::read_lines("day8_testdata.txt");

    let grid = lines.iter().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
    let mut counter = 0;
    grid.iter().enumerate().for_each(|(row_index, line)| {
        line.iter().enumerate().for_each(|(col_index, &k)| {
            if col_index == 0 || col_index == line.len() - 1 || row_index == 0 || row_index == lines.len() - 1 {
                counter += 1;
            } else {
                if (0..col_index).all(|x| grid[row_index][x] < k)
                    || (col_index + 1..line.len()).all(|x| grid[row_index][x] < k)
                    || (0..row_index).all(|x| grid[x][col_index] < k)
                    || (row_index + 1..grid.len()).all(|x| grid[x][col_index] < k)
                {
                    counter += 1;
                }
            };
        });
    });

    println!("Day 8 Part 1: {:?}", counter);
}

fn part2() {
    let lines = helper::read_lines("day8_data.txt");
    // let lines = helper::read_lines("day8_testdata.txt");

    let grid = lines.iter().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
    let max = grid.iter().enumerate().map(|(row_index, line)| {
        line.iter().enumerate().map(|(col_index, &k)| {
            let mut left = 0;
            let mut right = 0;
            let mut up = 0;
            let mut down = 0;
            for x in (0..col_index).rev() {
                left += 1;
                if grid[row_index][x] >= k {
                    break;
                }
            }
            for x in (col_index + 1)..line.len() {
                right += 1;
                if grid[row_index][x] >= k {
                    break;
                };
            }
            for x in (0..row_index).rev() {
                up += 1;
                if grid[x][col_index] >= k {
                    break;
                }
            }
            for x in (row_index + 1)..grid.len() {
                down += 1;
                if grid[x][col_index] >= k {
                    break;
                }
            }
            left * right * up * down
        }).max().unwrap()
    }).max().unwrap();

    println!("Day 8 Part 2: {:?}", max);
}
