use std::collections::LinkedList;

#[aoc(day9,part1)]
pub fn part1(input: &str) -> u64 {
    let mut disk = LinkedList::new();
    let mut index = 0;
    let mut is_file = 1;
    for ch in input.chars() {
        disk.push_back((index*is_file, ch as u64 - 48));
        index += is_file;
        is_file = 1-is_file;
    }

    let (mut sum, mut i) = disk.pop_front().unwrap(); // index 0;
    while let Some(space) = disk.pop_front() {
        match space {
            (0, free) => {
                while let Some((0, _)) = disk.back() { disk.pop_back(); }
                if let Some(data) = disk.pop_back() {
                    if data.0 == 0 { continue; }
                    if data.1 == free {
                        sum += (i..i+data.1).map(|mul| mul*data.0).sum::<u64>();
                        i += data.1;
                    } else if data.1 > free {
                        sum += (i..i+free).map(|mul| mul*data.0).sum::<u64>();
                        i += free;
                        disk.push_back((data.0, data.1-free));
                    } else {
                        sum += (i..i+data.1).map(|mul| mul*data.0).sum::<u64>();
                        i += data.1;
                        disk.push_front((0, free-data.1));
                    }
                }
            },
            data => {
                sum += (i..i+data.1).map(|mul| mul*data.0).sum::<u64>();
                i += data.1;
            }
        }
    }
    sum
}

#[aoc(day9,part2)]
pub fn part2(input: &str) -> u64 {
    let mut index = 0;
    let mut disk = vec![];
    let mut is_file = true;
    for ch in input.chars() {
        if is_file {
            disk.push((Some(index), ch as u64 - 48));
            index += 1;
        } else {
            disk.push((None, ch as u64 - 48));
        }
        is_file = !is_file;
    }
    
    // DEFRAG
    let mut curr = disk.len()-1;
    while curr > 0 {
        let moving = disk[curr];
        if let Some((seek, block)) = disk.iter_mut()
            .take(curr)
            .enumerate()
            .skip_while(|(_, block)| block.0.is_some() || block.1 < moving.1)
            .next() {
                block.1 -= moving.1;
                disk[curr].0 = None;
                disk.insert(seek, moving);
        }
        curr -= 1;
        while disk[curr].0.is_none() {
            curr -= 1;
        }
    }
    
    let mut i = 0;
    let mut sum = 0;
    for (number, size) in disk.iter() {
        if let Some(n) = number {
            sum += (i..i+size).map(|mul| mul*n).sum::<u64>();
        }
        i += size;
    }
    sum
}