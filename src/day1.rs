#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut left = [0; 1000];
    let mut right = [0; 1000];
    let mut i = 0;
    for ln in input.lines() {
        left[i] = ln[0..5].parse::<u32>().unwrap();
        right[i] = ln[8..13].parse::<u32>().unwrap();
        i += 1;
    }
    left.sort();
    right.sort();
    (0..left.len()).map(|i| left[i].abs_diff(right[i])).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut left = [0; 1000];
    let mut right = [0; 100000];
    let mut i = 0;
    for ln in input.lines() {
        left[i] = ln[0..5].parse::<usize>().unwrap();
        right[ln[8..13].parse::<usize>().unwrap()] += 1;
        i += 1;
    }
    (0..left.len()).map(|i| left[i] * right[left[i]]).sum()
}