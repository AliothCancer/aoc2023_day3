use std::ops::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_offset() {
        assert_eq!(
            Position { x: 3..=6, y: 0 },
            &(Position { x: 4..=7, y: 1 }) + &Offset { x: -1, y: -1 }
        );
    }
}

pub const fn to_offset(value: [[isize; 2]; 9]) -> [Offset; 9] {
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
    pub x: isize,
    pub y: isize,
}
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Position {
    pub x: RangeInclusive<usize>,
    pub y: usize,
}
impl Position {
    pub fn min_distances(self, mut other: Position) -> f64 {
        self.x
            .map(|x| {
                let dx = other.x.next().unwrap().saturating_sub(x);
                let dy = other.y.saturating_sub(self.y);
                ((dx.pow(2) + dy.pow(2)) as f64).sqrt().abs()
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }
}

impl Add<&Offset> for &Position {
    type Output = Position;

    fn add(self, rhs: &Offset) -> Self::Output {
        let x_min = self.x.start();
        let x_max = self.x.end();
        let x_min = x_min.checked_add_signed(rhs.x).unwrap();
        let x_max = x_max.checked_add_signed(rhs.x).unwrap();

        let x = x_min..=x_max;
        let y = self.y.checked_add_signed(rhs.y).unwrap();
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Element {
    Number {
        digits: usize,
        x: RangeInclusive<usize>,
    },
    Symbol {
        str_repr: String,
        x: RangeInclusive<usize>,
    },
    Points {
        x: RangeInclusive<usize>,
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
