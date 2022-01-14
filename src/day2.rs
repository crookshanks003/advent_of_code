use std::fs;

fn main() {
    let inp = fs::read_to_string("src/day2.input").expect("Cant read the file");
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    let content = inp
        .split("\n")
        .filter(|word| word.len() > 0)
        .collect::<Vec<&str>>();

    for word in content {
        let splt = word.split(" ").collect::<Vec<&str>>();
        let num = splt[1].parse::<i32>().expect("Not a number");
        if splt[0] == "forward" {
            horizontal += num;
            depth += aim * num;
        } else if splt[0] == "down" {
            aim += num;
        } else if splt[0] == "up" {
            aim -= num;
        }
    }
    println!("{}", depth * horizontal);
}
