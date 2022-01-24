use std::collections::HashMap;
use std::fs;

pub fn get_result() {
    let inp = fs::read_to_string("src/day8.input").expect("Error in file opening");
    let lines: Vec<(&str, Vec<&str>)> = inp
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| l.split_once(" | ").unwrap())
        .map(|(k, v)| (k, v.split_whitespace().collect()))
        .collect();

    let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    let segments = HashMap::from([
        ("abcefg","0"),
        ("cf","1"),
        ("acdeg", "2"),
        ("acdfg", "3"),
        ("bcdf", "4"),
        ("abdfg", "5"),
        ("abdefg", "6"),
        ("acf", "7"),
        ("abcdefg", "8"),
        ("abcdfg", "9"),
    ]);

    let mut sum = 0;

    for (obs, out) in lines.iter() {
        let mut matched: HashMap<char, char> = HashMap::new();
        let one = obs.split_whitespace().find(|l| l.len() == 2).unwrap();
        let four = obs.split_whitespace().find(|l| l.len() == 4).unwrap();
        for ch in chars {
            let count = obs.matches(ch).count();
            match count {
                4 => matched.insert(ch, 'e'),
                6 => matched.insert(ch, 'b'),
                9 => matched.insert(ch, 'f'),
                8 if one.contains(ch) => matched.insert(ch, 'c'),
                8 => matched.insert(ch, 'a'),
                7 if four.contains(ch) => matched.insert(ch, 'd'),
                7 => matched.insert(ch, 'g'),
                _ => None,
            };
        }

        let mut number = String::new();
        
        for val in out.iter() {
            let mut correct_val = val.chars().map(|ch| *matched.get(&ch).unwrap()).collect::<Vec<char>>();
            correct_val.sort();
            number +=  segments.get(&String::from_iter(correct_val.iter())[..]).unwrap();
        }
        sum += number.parse::<u32>().unwrap();
    }

    println!("{}", sum);
}
