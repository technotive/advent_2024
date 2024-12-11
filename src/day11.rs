use std::collections::HashMap;

#[aoc(day11,part1)]
pub fn part1(input: &str) -> u64 {
    let mut mem = vec![HashMap::new(); 26];
    // let mut mem = HashMap::new();
    input.split_whitespace()
        .map(|s| fast_cycles(s.parse::<u64>().unwrap(), 25, &mut mem))
        .sum()
}
#[aoc(day11,part2)]
pub fn part2(input: &str) -> u64 {
    let mut mem = vec![HashMap::new(); 76];
    // let mut mem = HashMap::new();
    input.split_whitespace()
        .map(|s| fast_cycles(s.parse::<u64>().unwrap(), 75, &mut mem))
        .sum()
}

fn fast_even_digits(n: u64) -> Option<(u64, u64)> {
    if let Some(l) = n.checked_ilog10() {
        if l & 1 == 0 {
            None
        } else {
            let mut d = 0;
            let mut n = n;
            for i in 0..((l+1) >> 1) {
                d += (n % 10) * 10u64.pow(i);
                n /= 10;
            }
            Some((d, n))
        }
    } else {
        None
    }
}
fn fast_cycles(n: u64, count: usize, mem: &mut Vec<HashMap<u64, u64>>) -> u64 {
    if count == 0 { return 1 }
    if let Some(c) = mem[count].get(&n) { return *c }
    let c = match n {
        0 => fast_cycles(1, count-1, mem),
        x => {
            if let Some((l, r)) = fast_even_digits(x) {
                fast_cycles(l, count-1, mem) +
                fast_cycles(r, count-1, mem)
            } else {
                fast_cycles(x * 2024, count-1, mem)
            }
        }
    };
    mem[count].insert(n, c);
    c
}