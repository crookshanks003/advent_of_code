use std::fs;

fn part_1(inp: String) {
    let stop = match inp.find("\n") {
        Some(x) => x,
        None => 0,
    };
    let mut count_vec: Vec<u32> = vec![0; stop * 2];
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for (i, ch) in inp.chars().filter(|char| char != &'\n').enumerate() {
        if ch == '1' {
            count_vec[i % stop] += 1;
        } else if ch == '0' {
            count_vec[stop + (i % stop)] += 1;
        }
    }

    for i in 0..stop {
        if count_vec[i] > count_vec[stop + i] {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    println!(
        "result:{}",
        isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap()
    );
}

fn get_required_vec<'a>(v: Vec<&'a str>, r: &str, i: usize) -> Vec<&'a str> {
    let mut zero = Vec::<&str>::new();
    let mut one = Vec::<&str>::new();
    for num in v {
        if num.chars().nth(i).unwrap() == '0' {
            zero.push(num);
        } else {
            one.push(num);
        }
    }
    if r == "max" {
        if zero.len() > one.len() {
            zero
        } else {
            one
        }
    } else {
        if zero.len() > one.len() {
            one
        } else {
            zero
        }
    }
}

fn part_2(inp: String) {
    let mut o2 = inp
        .split("\n")
        .filter(|word| word.len() > 0)
        .collect::<Vec<&str>>();

    let mut co2 = o2.clone();

    let mut i: usize = 0;

    while o2.len() > 1 {
        o2 = get_required_vec(o2, "max", i);
        i += 1;
    }

    i = 0;

    while co2.len() > 1 {
        co2 = get_required_vec(co2, "min", i);
        i += 1;
    }
    println!(
        "{}",
        isize::from_str_radix(&o2[0], 2).unwrap() * isize::from_str_radix(&co2[0], 2).unwrap()
    );
}

pub fn get_result() {
    let inp = fs::read_to_string("src/input/day3.input").expect("Cant read the file");
    part_1(inp);
    // part_2(inp);
}
