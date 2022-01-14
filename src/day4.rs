use std::collections::HashSet;
use std::fs;

pub fn get_result() {
    let inp = fs::read_to_string("src/day4.input").expect("Cant read the file");

    let content = inp
        .split("\n\n")
        .filter(|line| line.len() > 0)
        .collect::<Vec<&str>>();

    let moves: Vec<u32> = content[0].split(",").map(|n| n.parse().unwrap()).collect();
    let mut boards: Vec<Vec<HashSet<u32>>> = Vec::new();

    for board in &content[1..content.len()] {
        let b = board.split("\n").filter(|l| l.len() > 0);
        let groups: Vec<Vec<u32>> = b
            .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
            .collect();
        let mut sets: Vec<HashSet<u32>> = Vec::new();
        for i in 0..5 {
            let mut hash = HashSet::new();
            for j in 0..5 {
                hash.insert(groups[j][i]);
            }
            sets.push(hash);
        }

        for g in groups {
            sets.push(HashSet::from_iter(g));
        }
        boards.push(sets);
    }

    let mut winning_board = 0;
    let mut mo = 0;
    'moves: for m in &moves {
        for (i, b) in boards.iter_mut().enumerate() {
            for h in b {
                if h.remove(m) {
                    if h.is_empty() {
                        winning_board = i;
                        mo = *m;
                        break 'moves;
                    }
                }
            }
        }
    }
    let sum: u32 = HashSet::<&u32>::from_iter(boards[winning_board].iter().flatten())
        .into_iter()
        .sum();
    println!("{}", sum * mo);
}
