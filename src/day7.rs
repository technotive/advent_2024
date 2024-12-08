#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for l in input.lines() {
        let (result, numbers) = l.split_once(": ").unwrap();
        let result = result.parse::<u64>().unwrap();
        let numbers = numbers.split(' ').map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
        if recurse_one_v2(result, &numbers, numbers.len()-1) { sum += result }
    }
    sum
}
fn recurse_one_v2(num: u64, numbers: &Vec<u64>, index: usize) -> bool {
    if index == 0 {
        return numbers[0] == num
    }
    (
        num.checked_sub(numbers[index]).is_some()
        && recurse_one_v2(num-numbers[index], numbers, index-1)
    ) || (
        num % numbers[index] == 0
        && recurse_one_v2(num/numbers[index], numbers, index-1)
    )
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    let mut sum = 0;
    for l in input.lines() {
        let (result, numbers) = l.split_once(": ").unwrap();
        let result = result.parse::<u64>().unwrap();
        let numbers = numbers.split(' ').map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();
        if recurse_two_v2(result, &numbers, numbers.len()-1) {
            sum += result;
        }
    }
    sum
}
fn recurse_two_v2(num: u64, numbers: &Vec<u64>, index: usize) -> bool {
    if index == 0 {
        return numbers[0] == num
    }
    let det = detach(num, numbers[index]);
    (
        num.checked_sub(numbers[index]).is_some()
        && recurse_two_v2(num-numbers[index], numbers, index-1)
    ) || (
        num % numbers[index] == 0
        && recurse_two_v2(num/numbers[index], numbers, index-1)
    ) || (
        det.is_some()
        && recurse_two_v2(det.unwrap(), numbers, index-1)
    )
}
fn detach(num: u64, n: u64) -> Option<u64> {
    if num < n { return None }
    if n < 10 && (num-n)%10 == 0 { return Some((num-n)/10) }
    if n < 100 && (num-n)%100 == 0 { return Some((num-n)/100)}
    if (num-n)%1000 == 0 { return Some((num-n)/1000) }
    None
}