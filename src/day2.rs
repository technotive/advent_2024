#[aoc(day2, part1)]
pub fn one(input: &str) -> u32 {
    let mut safe = 0;
    for ln in input.lines() {
        let curr = ln.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let next = curr.clone().skip(1);
        safe += is_safe(curr, next);
    }
    safe
}

#[aoc(day2, part2)]
pub fn two(input: &str) -> u32 {
    let mut safe = 0;
    for ln in input.lines() {
        let curr = ln.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let next = curr.clone().skip(1);
        if is_safe(curr, next) == 1 { safe += 1; continue; }
        let levels = ln.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        for i in 0..levels.len() {
            let mut levels = levels.clone();
            levels.remove(i);
            let curr = levels.into_iter();
            let next = curr.clone().skip(1);
            if is_safe(curr, next) == 1 { safe += 1; break; }
        }
    }
    safe
}

fn is_safe(curr: impl Iterator<Item = i32>, next: impl Iterator<Item = i32>) -> u32 {
    let mut prev = None;
    for (c, n) in curr.zip(next) {
        if n.abs_diff(c) > 3 || n == c
        || prev.is_some() && prev.unwrap()*(n-c) < 0 {
            return 0;
        }
        prev = Some(n-c);
    }
    return 1;
}