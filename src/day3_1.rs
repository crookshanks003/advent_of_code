use std::fs;

pub fn get_result() {
    let inp = fs::read_to_string("src/day3.input").expect("Cant read the file");
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
            count_vec[stop + (i % stop )] += 1;
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
    println!("result:{}", isize::from_str_radix(&gamma, 2).unwrap()*isize::from_str_radix(&epsilon, 2).unwrap());
}
