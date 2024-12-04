#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    let map = FastMap2D::<char, W, H>::from(input);
    let mut xmas = 0;

    for row in 0..H {
        for col in 0..W {
            if map[(row, col)] == 'X' {
                if row+3 < H {
                    if map[(row+1, col)] == 'M' && map[(row+2, col)] == 'A' && map[(row+3, col)] == 'S' { xmas += 1}

                    if col+3 < H && map[(row+1, col+1)] == 'M' && map[(row+2, col+2)] == 'A' && map[(row+3, col+3)] == 'S' { xmas += 1}
                    if col >= 3 && map[(row+1, col-1)] == 'M' && map[(row+2, col-2)] == 'A' && map[(row+3, col-3)] == 'S' { xmas += 1}
                }
                if row >= 3 {
                    if map[(row-1, col)] == 'M' && map[(row-2, col)] == 'A' && map[(row-3, col)] == 'S' { xmas += 1}

                    if col+3 < H && map[(row-1, col+1)] == 'M' && map[(row-2, col+2)] == 'A' && map[(row-3, col+3)] == 'S' { xmas += 1}
                    if col >= 3 && map[(row-1, col-1)] == 'M' && map[(row-2, col-2)] == 'A' && map[(row-3, col-3)] == 'S' { xmas += 1}
                }
                
                if col+3 < H && map[(row, col+1)] == 'M' && map[(row, col+2)] == 'A' && map[(row, col+3)] == 'S' { xmas += 1}
                if col >= 3 && map[(row, col-1)] == 'M' && map[(row, col-2)] == 'A' && map[(row, col-3)] == 'S' { xmas += 1}
            }
        }
    }
    
    xmas
}
#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    let map = FastMap2D::<char, W, H>::from(input);
    let mut mas = 0;
    for row in 1..H-1 {
        for col in 1..W-1 {
            if map[(row, col)] == 'A' {
                let mut x = 0;
                match (map[(row-1, col-1)], map[(row+1, col+1)]) {
                    ('M', 'S') => x += 1,
                    ('S', 'M') => x += 1,
                    _ => continue
                }
                match (map[(row-1, col+1)], map[(row+1, col-1)]) {
                    ('M', 'S') => x += 1,
                    ('S', 'M') => x += 1,
                    _ => continue
                }
                mas += x >> 1;
            }
        }
    }
    mas
}

const W: usize = 140;
const H: usize = 140;

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
