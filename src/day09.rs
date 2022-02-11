use std::{fs, collections::HashSet};

trait GetNeighbours {
    fn get_neighbours(&self, i: usize, j: usize, length: usize) -> Vec<(usize, usize)>;
}
//(row,col)
impl GetNeighbours for Vec<Vec<u32>> {
    fn get_neighbours(&self, i: usize, j: usize, length: usize) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        let l = self.len();
        if let Some(x) = j.checked_sub(1) {
            result.push((i, x));
        }
        if let Some(x) = i.checked_sub(1) {
            result.push((x, j));
        }
        if j + 1 < length {
            result.push((i, j + 1));
        }
        if i + 1 < l {
            result.push((i + 1, j));
        }
        result
    }
}

pub fn get_result() {
    let inp = fs::read_to_string("src/input/day9.input").expect("Error in file opening");
    let content: Vec<Vec<u32>> = inp
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let length = content[0].len();

    let mut low_points: Vec<(usize, usize)> = Vec::new();

    for (i, line) in content.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            let neighbours = content.get_neighbours(i, j, length);
            if neighbours.iter().all(|(i, j)| content[*i][*j] > *val) {
                low_points.push((i, j));
            }
        }
    }
    println!("{}", low_points.len());

    let mut basins: Vec<usize> = Vec::new();
    for (i,j) in low_points.iter(){
        let mut visited:HashSet<(usize, usize)> = HashSet::new();
        let mut to_visit: Vec<(usize, usize)> = Vec::from(content.get_neighbours(*i,*j, length));
        while !to_visit.is_empty() {
            let (x,y) = to_visit.pop().unwrap();
            if content[x][y] == 9 {
                continue;
            }
            for (r,c) in content.get_neighbours(x,y,length).iter() {
                if content[*r][*c] != 9 && !visited.contains(&(*r,*c)) {
                    to_visit.push((*r,*c));
                }
            }
            visited.insert((x,y));
        }
        basins.push(visited.len());
    }
    basins.sort_by(|a,b| b.cmp(a));
    let product: usize = basins.iter().take(3).product();
    println!("{}", product);
}
