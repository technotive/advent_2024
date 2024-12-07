#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for l in input.lines() {
        let (result, numbers) = l.split_once(": ").unwrap();
        let result = result.parse::<u64>().unwrap();
        let numbers = numbers.split(' ').map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
        if recurse_one(&numbers, 1, numbers[0], result) {
            sum += result
        }
    }
    sum
}
fn recurse_one(numbers: &Vec<u64>, index: usize, acc: u64, end: u64) -> bool {
    if acc > end { return false; }
    if let Some(&n) = numbers.get(index) {
        return 
            recurse_one(numbers, index+1, acc+n, end) ||
            recurse_one(numbers, index+1, acc*n, end)
    } else {
        return acc == end
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    let mut sum = 0;
    for l in input.lines() {
        let (result, numbers) = l.split_once(": ").unwrap();
        let result = result.parse::<u64>().unwrap();
        let numbers = numbers.split(' ').map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
        if recurse_two(&numbers, 1, numbers[0], result) {
            sum += result
        }
    }
    sum
}
fn recurse_two(numbers: &Vec<u64>, index: usize, acc: u64, end: u64) -> bool {
    if acc > end { return false; }
    if let Some(&n) = numbers.get(index) {
        return recurse_two(numbers, index+1, acc+n, end)
        || recurse_two(numbers, index+1, acc*n, end)
        || recurse_two(numbers, index+1, concat(acc, n), end)
    } else {
        return acc == end
    }
}
fn concat(acc: u64, n: u64) -> u64 {
    if n < 10 {
        acc * 10 + n
    } else if n < 100 {
        acc * 100 + n
    } else {
        acc * 1000 + n
    }
}