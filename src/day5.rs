#[aoc(day5, part1)]
pub fn one(input: &str) -> usize {
    let mut cannot_precede = [[false; 100]; 100];
    let (rules, printing) = input.split_once("\n\n").unwrap();
    for l in rules.lines() {
        let (fst, snd) = l.split_once('|').unwrap();
        let fst = fst.parse::<usize>().unwrap();
        let snd = snd.parse::<usize>().unwrap();
        cannot_precede[fst][snd]= true;
    }
    let mut sum = 0;
    for l in printing.lines() {
        let pages = l.split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let mut printed = vec![];
        for &page in &pages {
            if printed.iter().any(|&prev| cannot_precede[page][prev]) {
                break;
            }
            printed.push(page)
        }
        if printed.len() != pages.len() {continue;}
        sum += printed[printed.len()/2];
    }
    sum
}

#[aoc(day5, part2)]
pub fn two(input: &str) -> usize {
    let mut cannot_precede = [[false; 100]; 100];
    let (rules, printing) = input.split_once("\n\n").unwrap();
    for l in rules.lines() {
        let (fst, snd) = l.split_once('|').unwrap();
        let fst = fst.parse::<usize>().unwrap();
        let snd = snd.parse::<usize>().unwrap();
        cannot_precede[fst][snd]= true;
    }
    let mut sum = 0;
    for l in printing.lines() {
        let pages = l.split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let mut printed = vec![];
        let mut modified = false;
        for &page in &pages {
            let mut insert_at = None;
            for pos in 0..printed.len() {
                if cannot_precede[page][printed[pos]] {
                    insert_at = Some(pos);
                    break;
                }
            }
            if let Some(pos) = insert_at {
                printed.insert(pos, page);
            } else {
                printed.push(page);
            }
            modified |= insert_at.is_some();
        }
        if modified {
            sum += printed[printed.len()/2];
        }
    }
    sum
}
