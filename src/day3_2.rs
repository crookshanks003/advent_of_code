use std::fs;

fn get_required_vec<'a>(v: Vec<&'a str>, r: &str, i:usize) -> Vec<&'a str> {
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
        if zero.len() > one.len(){
            one
        } else {
            zero
        }
    }
}

pub fn get_result() {
    let inp = fs::read_to_string("src/day3.input").expect("Cant read the file");
    let mut o2 = inp
        .split("\n")
        .filter(|word| word.len() > 0)
        .collect::<Vec<&str>>();

    let mut co2 = o2.clone();

    let mut i: usize = 0;

    while o2.len() > 1 {
        o2 = get_required_vec(o2,"max", i);
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
