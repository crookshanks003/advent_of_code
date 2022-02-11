use std::{collections::HashSet, fs};

pub fn get_result() {
    let inp = fs::read_to_string("src/input/day13.input").expect("Cant read the file");
    let (content, fold_content) = inp.split_once("\n\n").unwrap();

    let mut points: HashSet<(usize, usize)> = content
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let fold: Vec<(&str, usize)> = fold_content
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| {
            let (axis, val) = l.split_once("=").unwrap();
            return (
                axis.split_whitespace().collect::<Vec<&str>>()[2],
                val.parse().unwrap(),
            );
        })
        .collect();

    let mut max_x = 0;
    let mut max_y = 0;
    points.iter().for_each(|(x, y)| {
        if x > &max_x {
            max_x = *x;
        }
        if y > &max_y {
            max_y = *y;
        }
    });

    for (axis, val) in fold.iter() {
        if axis == &"y" {
            points = points
                .iter()
                .map(|(x, y)| match val.checked_sub(*y) {
                    Some(_) => (*x, *y),
                    None => (*x, (2 * val) - y),
                })
                .collect();
        } else if axis == &"x" {
            points = points
                .iter()
                .map(|(x, y)| match val.checked_sub(*x) {
                    Some(_) => (*x, *y),
                    None => ((2 * val) - x, *y),
                })
                .collect();
        }
    }

    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in points.iter() {
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }

    let mut code = vec![vec![" "; max_x + 1]; max_y + 1];

    for (x, y) in points {
        code[y][x] = "#";
    }
    for line in code {
        println!("{}", String::from_iter(line.into_iter()));
    }
}
