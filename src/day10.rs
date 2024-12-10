use std::{collections::HashSet, mem::MaybeUninit};

// const D: usize = 8;
const D: usize = 55;

#[aoc(day10,part1)]
pub fn part1(input: &str) -> usize {
    let mut peaks = vec![];
    #[allow(invalid_value)]
    let mut map = [[unsafe { MaybeUninit::<u32>::uninit().assume_init() }; D]; D];

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.char_indices() {
            map[row][col] = ch as u32 - 48;
            if map[row][col] == 9 { peaks.push((row, col)); }
        }
    }
    
    let mut sum = 0;
    for peak in peaks {
        let mut descent = vec![peak];
        let mut seen = HashSet::new();
        while let Some((row, col)) = descent.pop() {
            if map[row][col] == 0 { seen.insert((row, col)); continue; }
            let target = map[row][col]-1;
            if row > 0 && map[row-1][col] == target {
                descent.push((row-1, col));
            }
            if col > 0 && map[row][col-1] == target {
                descent.push((row, col-1));
            }
            if row < D-1 && map[row+1][col] == target {
                descent.push((row+1, col));
            }
            if col < D-1 && map[row][col+1] == target{
                descent.push((row, col+1));
            }
        }
        sum += seen.len();
    }
    sum
}


#[aoc(day10,part2)]
pub fn part2(input: &str) -> usize {
    let mut peaks = vec![];
    let mut trial_heads = vec![];
    #[allow(invalid_value)]
    let mut map = [[unsafe { MaybeUninit::<u32>::uninit().assume_init() }; D]; D];

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.char_indices() {
            map[row][col] = ch as u32 - 48;
            if map[row][col] == 0 { trial_heads.push((row, col)); }
            if map[row][col] == 9 { peaks.push((row, col)); }
        }
    }
    
    let mut sum = 0;
    for peak in peaks {
        let mut walked = [[0; D]; D];
        let mut descent = vec![peak];
        while let Some((row, col)) = descent.pop() {
            if map[row][col] == 0 { continue; }
            let target = map[row][col]-1;
            if row > 0 && map[row-1][col] == target {
                walked[row-1][col] += 1;
                descent.push((row-1, col));
            }
            if col > 0 && map[row][col-1] == target {
                walked[row][col-1] += 1;
                descent.push((row, col-1));
            }
            if row < D-1 && map[row+1][col] == target {
                walked[row+1][col] += 1;
                descent.push((row+1, col));
            }
            if col < D-1 && map[row][col+1] == target{
                walked[row][col+1] += 1;
                descent.push((row, col+1));
            }
        }

        sum += trial_heads.iter().map(|(row, col)| walked[*row][*col]).sum::<usize>()
    }
    sum
}
