use std::fs;

fn part_1(positions: &Vec<i32>) -> i32 {
    let mid = positions[(positions.len() + 1) / 2];

    println!("mid : {}", mid);

    positions.iter().map(|w| i32::abs(w - mid)).sum()
}

fn part_2(positions: &Vec<i32>) -> i32 {
    //check minimum between floor and ceil manually
    let opt_point = (positions.iter().sum::<i32>() as f32 / positions.len() as f32).floor() as i32;

    positions
        .iter()
        .map(|n| (i32::pow(opt_point - n, 2) + i32::abs(n - opt_point)) / 2)
        .sum()
}

pub fn get_result() {
    let inp = fs::read_to_string("src/input/day7.input").expect("Error in file opening");

    let mut positions: Vec<i32> = inp
        .trim_matches('\n')
        .split(",")
        .filter(|w| w.len() > 0)
        .map(|w| w.parse().unwrap())
        .collect();

    positions.sort_unstable();
    //[0,1,1,2,2,2,4,7,14,16]

    println!("{}", part_2(&positions));
}
