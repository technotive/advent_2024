#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut safe = 0;
    for ln in input.lines() {
        let curr = ln.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let next = curr.clone().skip(1);
        safe += is_safe(curr, next);
    }
    safe
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut safe: u32 = 0;
    for ln in input.lines() {
        let levels = ln.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let mut problem = vec![false; levels.len()];
        
        let mut pass = true;
        let mut prev = None;
        for i in 1..levels.len() {
            let diff = levels[i]-levels[i-1];
            if prev.is_some() && prev.unwrap()*diff < 0 {
                problem[i-2] = true;
            }
            if diff > 3 || diff == 0 || diff < -3 || (prev.is_some() && prev.unwrap()*diff < 0){
                problem[i-1] = true;
                problem[i] = true;
                pass = false;
            }
            prev = Some(diff);
        }
        if pass { safe += 1; continue; }

        for i in 0..levels.len() {
            if !problem[i] { continue; }
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
    let mut prev = 0;
    for d in curr.zip(next).map(|(n, c) |n-c) {
        if d > 3 || d == 0 || d < -3 || prev*d < 0 {
            return 0;
        }
        prev = d;
    }
    return 1;
}