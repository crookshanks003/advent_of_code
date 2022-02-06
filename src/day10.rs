use std::collections::HashMap;
use std::fs;

pub fn get_result() {
    let inp = fs::read_to_string("src/day10.input").expect("Error in file opening");
    let content: Vec<&str> = inp.split("\n").filter(|l| l.len() > 0).collect();

    let pairs: HashMap<char, (char, u32)> = HashMap::from([
        ('}', ('{', 1197)),
        (')', ('(', 3)),
        ('>', ('<', 25137)),
        (']', ('[', 57)),
    ]);
    let mut syntax_score: u32 = 0;
    let mut autocomplete_score: Vec<u64> = Vec::new();

    'outer: for line in content.iter() {
        let mut unclosed: Vec<char> = Vec::new();
        for ch in line.chars() {
            match ch {
                '{' | '[' | '<' | '(' => {
                    unclosed.push(ch);
                }
                '}' | ']' | '>' | ')' => {
                    let unclosed_char = unclosed.pop().unwrap();
                    let (pair, val) = pairs.get(&ch).unwrap();
                    if &unclosed_char == pair {
                    } else {
                        syntax_score += *val;
                        continue 'outer;
                    };
                }
                _ => (),
            }
        }
        let mut score = 0;
        for ch in unclosed.iter().rev() {
            let value = match ch {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => {println!("{}", ch); 0},
            };
            score = (score*5) + value;
        }
        autocomplete_score.push(score);
    }
    autocomplete_score.sort();

    println!("{}", syntax_score);
    println!("{}", autocomplete_score[(autocomplete_score.len()-1)/2]);
}
