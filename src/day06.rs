use std::{fs, collections::VecDeque};

pub fn get_result() {
    let inp = fs::read_to_string("src/input/day6.input").expect("Cant read file");

    let content: Vec<usize> = inp
        .trim_matches('\n')
        .split(",")
        .filter(|num| num.len() > 0)
        .map(|num| num.parse().unwrap())
        .collect();

    let mut count:VecDeque<u64> = VecDeque::from([0;9]);

    content.iter().for_each(|i| count[*i] += 1);

    for _i in 0..256 {
        let new_timer = count.pop_front().unwrap();
        count[6] += new_timer;
        count.push_back(new_timer);
    }

    println!("{}", count.iter().sum::<u64>());

}
