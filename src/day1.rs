use crate::helper;

pub fn run() {
    let lines = helper::read_lines("day1_data.txt");
    // let lines = helper::read_lines("day1_testdata.txt");
    let data = lines.split(|x| x == "")
        .map(|x| x.to_vec())
        .map(|x| x.into_iter().map(|a| a.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|a| a.iter().fold(0, |acc, item| acc + item))
        .collect::<Vec<i32>>();

    let max = data.iter().max().unwrap();
    let elf = data.iter().position(|s| s == max).unwrap();

    println!("Elf {:?} - Carries: {:?}", elf + 1, max);

    let mut data2 = data.clone();
    data2.sort();
    let top3: i32 = [data2.pop().unwrap(), data2.pop().unwrap(), data2.pop().unwrap()].iter().sum();
    println!("Top 3 total: {:?}", top3);
}
