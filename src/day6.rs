const WS: usize = 130;
const HS: usize = 130;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    let (map, mut pos) = FastMap2D::<char, WS, HS>::from_starting(input, '^');
    let mut seen = FastMap2D::<u32, WS, HS>::new(1);
    let mut ct = 1;
    let mut dir = Direction::Up;

    while let Some(next) = dir.next(pos){
        match map.get(next) {
            Some('#') => dir = dir.turn(),
            Some(_) => {
                ct += seen[pos];
                seen[pos] = 0;
                pos = next;
            }
            None => break
        }
    }
    ct
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
    let (mut map, mut pos) = FastMap2D::<char, WS, HS>::from_starting(input, '^');
    let mut dir = Direction::Up;

    let mut cycle = 0;
    while let Some(next) = dir.next(pos){
        match map.get(next) {
            Some('#') => dir = dir.turn(),
            Some('X') => pos = next,
            Some(_) => {
                map[next] = '#';
                if check(&map, pos, dir) { cycle += 1; }
                map[next] = 'X';
                pos = next;
            }
            None => break
        }
    }
    cycle
}

fn check(map: &FastMap2D<char, WS, HS>, mut pos: (usize, usize), mut dir: Direction) -> bool {
    let mut seen = [[[false; 4]; WS]; HS];
    while let Some(next) = dir.next(pos) {
        match map.get(next) {
            Some('#') => dir = dir.turn(),
            Some(_) => pos = next,
            None => return false
        }
        if seen[pos.0][pos.1][dir as usize] {
            return true
        } else {
            seen[pos.0][pos.1][dir as usize] = true;
        }
    }
    return false
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up, Down, Left, Right
}
impl Direction {
    fn next(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => pos.0.checked_sub(1).and_then(|row| Some((row, pos.1))),
            Direction::Down => pos.0.checked_add(1).and_then(|row| Some((row, pos.1))),
            Direction::Left => pos.1.checked_sub(1).and_then(|col| Some((pos.0, col))),
            Direction::Right => pos.1.checked_add(1).and_then(|col| Some((pos.0, col))),
        }
    }
    fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

use std::{mem::MaybeUninit, ops::{Index, IndexMut}, usize};

#[allow(unused)]
pub struct FastMap2D<T, const W: usize, const H: usize> where T: Copy {
    data: [[T; W]; H],
    curr: (usize, usize),
    next: (usize, usize)
}
#[allow(unused)]
impl<T, const W: usize, const H: usize> FastMap2D<T, W, H> where T: Copy {
    pub fn new(default: T) -> Self {
        Self { data: [[default; W]; H], curr: (0, 0), next: (0, 1) }
    }
    pub fn transposed(&self) -> Self where T: Clone {
        let mut buf = self.data.clone();
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                buf[j][i] = self.data[i][j];
            }
        }
        let curr = buf[0][0].clone();
        return Self { data: buf, curr: (0, 0), next: (0, 1) }
    }

    pub fn get(&self, index: (usize, usize)) -> Option<&T> {
        if index.0 >= self.data.len() || index.1 >= self.data[0].len() { return None; }
        Some(&self[index])
    }
}
impl<const W: usize, const H: usize> From<&str> for FastMap2D<char, W, H> {
    fn from(value: &str) -> Self {
        unsafe { 
            #[allow(invalid_value)]
            let mut data = [[MaybeUninit::<char>::uninit().assume_init(); W]; H];
            let mut row = 0;
            for ln in value.lines() {
                for (col, c) in ln.char_indices() {
                    data[row][col] = c;
                }
                row += 1;
            }
            Self { data, curr: (0, 0), next: (0, 1) }
        }
    }
}

impl<T, const W: usize, const H: usize> Iterator for FastMap2D<T, W, H> where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        let mut c = self.next.0+1;
        let mut r = self.next.1;
        if c >= self.data[0].len() { c -= self.data[0].len(); r += 1 }
        if r >= self.data.len() { return None; }
        self.next = (r, c);
        Some(self[current])
    }
}

impl<T, const W: usize, const H: usize> Index<(usize, usize)> for FastMap2D<T, W, H> where T: Copy {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}
impl<T, const W: usize, const H: usize> IndexMut<(usize, usize)> for FastMap2D<T, W, H> where T: Copy {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl<const W: usize, const H: usize> FastMap2D<char, W, H> {
    pub fn from_starting(input: &str, start: char) -> (Self, (usize, usize)) {
        unsafe { 
            let mut begin = (0, 0);
            #[allow(invalid_value)]
            let mut data = [[MaybeUninit::<char>::uninit().assume_init(); W]; H];
            let mut row = 0;
            for ln in input.lines() {
                for (col, c) in ln.char_indices() {
                    data[row][col] = c;
                    if c == start { begin = (row, col) }
                }
                row += 1;
            }
            (Self { data, curr: (0, 0), next: (0, 1) }, begin)
        }
    }
}
