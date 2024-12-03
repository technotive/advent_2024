#[aoc(day3, part1)]
pub fn one(input: &str) -> u32 {
    let mut a = 0;
    let mut b = 0;
    let mut expect = Expect::M;
    let mut sum = 0;
    for c in input.chars() {
        expect = match (c, expect) {
            ('m', Expect::M) => Expect::U,
            ('u', Expect::U) => Expect::L,
            ('l', Expect::L) => Expect::Open,
            ('(', Expect::Open) => Expect::AOrComma,
            (',', Expect::AOrComma) => Expect::BOrClose,
            (c, Expect::AOrComma) => {
                if c.is_numeric() {
                    a = 10*a + ((c as u32)-48);
                    if a > 999 { Expect::M } else { Expect::AOrComma }
                } else {
                    a = 0;
                    Expect::M
                }
            },
            (')', Expect::BOrClose) => {
                sum += a*b;
                a = 0;
                b = 0;
                Expect::M
            },
            (c, Expect::BOrClose) => {
                if c.is_numeric() {
                    b = 10*b + ((c as u32)-48);
                    if b > 999 { Expect::M } else { Expect::BOrClose }
                } else {
                    a = 0;
                    b = 0;
                    Expect::M
                }
            },
            _ => Expect::M
        }
    }
    sum
}

#[derive(PartialEq, Eq)]
enum Expect {
    M, U, L, Open, AOrComma, BOrClose
}

#[derive(PartialEq, Eq)]
enum Expect2 {
    MOrD, U, L, Open, AOrComma, BOrClose, O, NOrOpen, CloseDo, Apostrophe, T, OpenDont, CloseDont
}

#[aoc(day3, part2)]
pub fn two(input: &str) -> u32 {
    let mut enabled = true;
    let mut a = 0;
    let mut b = 0;
    let mut expect = Expect2::MOrD;
    let mut sum = 0;
    for c in input.chars() {
        expect = match (c, expect, enabled) {
            ('m', Expect2::MOrD, true) => Expect2::U,
            ('u', Expect2::U, true) => Expect2::L,
            ('l', Expect2::L, true) => Expect2::Open,
            ('(', Expect2::Open, true) => Expect2::AOrComma,
            (',', Expect2::AOrComma, true) => Expect2::BOrClose,
            (c, Expect2::AOrComma, true) => {
                if c.is_numeric() {
                    a = 10*a + ((c as u32)-48);
                    if a > 999 { Expect2::MOrD } else { Expect2::AOrComma }
                } else {
                    a = 0;
                    Expect2::MOrD
                }
            },
            (')', Expect2::BOrClose, true) => {
                sum += a*b;
                a = 0;
                b = 0;
                Expect2::MOrD
            },
            (c, Expect2::BOrClose, true) => {
                if c.is_numeric() {
                    b = 10*b + ((c as u32)-48);
                    if b > 999 { Expect2::MOrD } else { Expect2::BOrClose }
                } else {
                    a = 0;
                    b = 0;
                    Expect2::MOrD
                }
            },

            ('d', Expect2::MOrD, _) => Expect2::O,
            ('o', Expect2::O, _) => Expect2:: NOrOpen,
            ('(', Expect2::NOrOpen, _) => Expect2::CloseDo,
            (')', Expect2::CloseDo, _) => {
                enabled = true;
                Expect2::MOrD
            }

            ('n', Expect2::NOrOpen, _) => Expect2::Apostrophe,
            ('\'', Expect2::Apostrophe, _) => Expect2::T,
            ('t', Expect2::T, _) => Expect2::OpenDont,
            ('(', Expect2::OpenDont, _) => Expect2::CloseDont,
            (')', Expect2::CloseDont, _) => {
                enabled = false;
                Expect2::MOrD
            }
            _ => Expect2::MOrD
        }
    }
    sum
}
