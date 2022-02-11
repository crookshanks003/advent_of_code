use std::fs;

fn main() {
    let mut count = 0;
    let inp = fs::read_to_string("src/input/day1.input").expect("Cant read the file");
    let content = inp
        .split("\n")
        .filter(|word| word.len() > 0)
        .map(|word| word.parse::<i32>().expect("Not a number"))
        .collect::<Vec<i32>>();

    // for i in 0..content.len()-1 {
    //     if content[i] < content[i+1] {
    //         count += 1;
    //     }
    // }

    let curr_sum:i32 = content[0] + content[1] + content[2];
    let mut next_sum: i32;

    for i in 0..content.len() - 3 {
        next_sum = curr_sum - content[i] + content[i + 3];
        if next_sum > curr_sum {
            count += 1;
        }
    }
    println!("{}", count);
}
