use std::collections::{HashMap, HashSet};

const W: i32 = 50; //12
const H: i32 = 50; //12

#[aoc(day8,part1)]
pub fn part1(input: &str) -> u32 {
    let mut seen = HashSet::new();
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (row, l) in input.lines().enumerate() {
        let row = row as i32;
        for (col, c) in l.char_indices().filter(|(_, c)| *c != '.') {
            let col = col as i32;
            if let Some(vec) = map.get_mut(&c) {
                for i in 0..vec.len() {
                    let d_row = row - vec[i].0;
                    let d_col = col - vec[i].1;
                    let r_1_row = vec[i].0 - d_row;
                    let r_2_row = row + d_row;
                    let r_1_col = vec[i].1 - d_col;
                    let r_2_col = col + d_col;
                    if r_1_row >= 0 && r_1_row < H && r_1_col >= 0 && r_1_col < W {
                        seen.insert((r_1_row, r_1_col));
                    }
                    if r_2_row >= 0 && r_2_row < H && r_2_col >= 0 && r_2_col < W {
                        seen.insert((r_2_row, r_2_col));
                    }
                }
                vec.push((row, col));
            } else {
                map.insert(c, vec![(row, col)]);
            }
        }
    }
    seen.len() as u32
}


#[aoc(day8,part2)]
pub fn part2(input: &str) -> usize {
    let mut seen = HashSet::new();
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (row, l) in input.lines().enumerate() {
        let row = row as i32;
        for (col, c) in l.char_indices() {
            let col = col as i32;
            if c == '.' { continue; }
            if let Some(vec) = map.get_mut(&c) {
                for i in 0..vec.len() {
                    seen.insert((row, col));
                    seen.insert((vec[i].0, vec[i].1));
                    let d_row = row - vec[i].0;
                    let d_col = col - vec[i].1;
                    let mut r_1_row = vec[i].0 - d_row;
                    let mut r_1_col = vec[i].1 - d_col;
                    while r_1_row >= 0 && r_1_col >= 0 && r_1_col < W {
                        seen.insert((r_1_row, r_1_col));
                        r_1_row -= d_row;
                        r_1_col -= d_col;
                    }
                    let mut r_2_row = row + d_row;
                    let mut r_2_col = col + d_col;
                    while r_2_row < H && r_2_col >= 0 && r_2_col < W {
                        seen.insert((r_2_row, r_2_col));
                        r_2_row += d_row;
                        r_2_col += d_col;
                    }
                }
                vec.push((row, col));
            } else {
                map.insert(c, vec![(row, col)]);
            }
        }
    }
    seen.len()
}
