use std::collections::HashSet;
use std::fs;

struct Board {
    board: Vec<HashSet<u32>>,
    won: bool,
}

impl Board {
    fn new(board: Vec<HashSet<u32>>) -> Board {
        Board { board, won: false }
    }
}

pub fn get_result() {
    let inp = fs::read_to_string("src/day4.input").expect("Cant read the file");

    let content = inp
        .split("\n\n")
        .filter(|line| line.len() > 0)
        .collect::<Vec<&str>>();

    let moves: Vec<u32> = content[0].split(",").map(|n| n.parse().unwrap()).collect();
    let mut boards: Vec<Board> = Vec::new();

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
        boards.push(Board::new(sets));
    }

    let mut required_board = 0;
    let mut mo = 0;
    // 'moves: for m in &moves {
    //     for (i, b) in boards.iter_mut().enumerate() {
    //         for h in &mut b.board {
    //             if h.remove(m) {
    //                 if h.is_empty() {
    //                     required_board = i;
    //                     mo = *m;
    //                     break 'moves;
    //                 }
    //             }
    //         }
    //     }
    // }
    // let sum: u32 = HashSet::<&u32>::from_iter(boards[required_board].iter().flatten())
    //     .into_iter()
    //     .sum();
    // println!("{}", sum * mo);

    let length = boards.len();
    let mut count = 0;

    'moves: for m in &moves {
        for (i, b) in boards.iter_mut().enumerate() {
            for h in &mut b.board {
                if h.remove(m) {
                    if b.won {
                        continue;
                    }
                    if h.is_empty() {
                        count += 1;
                        b.won = true;
                        if count == length {
                            required_board = i;
                            mo = *m;
                            break 'moves;
                        }
                    }
                }
            }
        }
    }
    let sum: u32 = HashSet::<&u32>::from_iter(boards[required_board].board.iter().flatten())
        .into_iter()
        .sum();

    // i have no idea why there is still one mo value left in vec
    println!("{}", mo * (sum-mo));
}
