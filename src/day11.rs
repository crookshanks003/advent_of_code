use std::{collections::HashSet, fs};

const L: usize = 10;

fn get_side_neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    if let Some(i) = x.checked_sub(1) {
        result.push((i, y));
    }
    if x + 1 < L {
        result.push((x + 1, y));
    }
    result
}

fn get_all_neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    result.extend(get_side_neighbours(x, y));
    if let Some(j) = y.checked_sub(1) {
        result.push((x, j));
        result.extend(get_side_neighbours(x, j));
    }
    if y + 1 < L {
        result.push((x, y + 1));
        result.extend(get_side_neighbours(x, y + 1));
    }
    result
}

pub fn get_result() {
    let inp = fs::read_to_string("src/input/day11.input").expect("Error in file opening");
    let mut content: Vec<Vec<u32>> = inp
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();
    let mut flash_count = 0;

    loop {
        let mut to_visit: Vec<(usize, usize)> = (0..10)
            .into_iter()
            .map(|x| {
                (0..10)
                    .into_iter()
                    .map(|y| (x, y))
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect();
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();

        while !to_visit.is_empty() {
            let (x, y) = to_visit.pop().unwrap();
            if content[x][y] == 9 && !flashed.contains(&(x, y)) {
                content[x][y] = 0;
                to_visit.extend(get_all_neighbours(x, y));
                flashed.insert((x, y));
            } else if content[x][y] == 0 {
                if !flashed.contains(&(x, y)) {
                    content[x][y] += 1;
                }
            } else {
                content[x][y] += 1;
            }
        }
        flash_count += 1;
        if flashed.len() == 100 {
            break;
        }
    }
    println!("{}", flash_count);
}
