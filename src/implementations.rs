use std::ops::*;

pub const fn to_offset(value: [[i64; 2]; 9]) -> [Offset; 9] {
    let mut offset = [Offset { x: 0, y: 0 }; 9];
    let mut i = 0;

    while i < value.len() {
        offset[i] = Offset {
            x: value[i][0],
            y: value[i][1],
        };
        i += 1;
    }

    offset
}
#[derive(Debug, Clone, Copy)]
pub struct Offset {
    x: i64,
    y: i64,
}
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Position {
    pub x: Range<usize>,
    pub y: usize,
}
impl Add<&Offset> for &Position {
    type Output = Position;

    fn add(self, rhs: &Offset) -> Self::Output {
        let x_offset = usize::try_from(rhs.x).unwrap();
        let x = (self.x.clone().min().unwrap() + x_offset)..(self.x.clone().max().unwrap() + x_offset);
        let y = TryInto::<i64>
            ::try_into(self.y)
            .unwrap()
            .checked_add(rhs.y)
            .unwrap()
            .try_into()
            .unwrap();

        Position { x, y }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum CharClass {
    Digits,
    Symbols,
    Dots,
}

pub fn classify_char(c: &char) -> CharClass {
    match *c {
        '.' => CharClass::Dots,
        '0'..='9' => CharClass::Digits,
        _ => CharClass::Symbols,
    }
}

#[derive(Debug)]
pub enum Element {
    Number {
        digits: usize,
        len: usize,
    },
    Symbol {
        str_repr: String,
        len: usize,
    },
    Points {
        len: usize,
    },
}

pub const ADJACENT_MAT: [Offset; 9] = to_offset([
    [-1, 1],
    [0, 1],
    [1, 1],
    [-1, 0],
    [0, 0],
    [1, 0],
    [-1, -1],
    [0, -1],
    [1, -1],
]);
