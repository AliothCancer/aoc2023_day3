#![allow(unused, clippy::uninlined_format_args)]

mod implementations;

use implementations::*;
use itertools::Itertools;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    ops::{Add, Sub},
    string::{self, String},
};

pub const EXAMPLE: &str = // esempio modificato
    "\
467..114..  
...*......
..35..633.
......#&..
617*......
.....+.58.
..592.....
......755.
...$.*#@..
.664.598..";

fn main() {
    let parsed_input = EXAMPLE
        .lines()
        .map(parse_line_p1)
        //.inspect(|x| println!("{x:?}"))
        .enumerate() // will be y
        .flat_map(parse_line_p2)
        .collect::<BTreeMap<_, _>>();

    let symbols = parsed_input
        .iter()
        .filter(|(_, el)| matches!(el, Element::Symbol { .. }));

    let mut total = 0;

    symbols.for_each(|(sym_pos, sym)| {
        let Element::Symbol { str_repr, len } = sym else {
            panic!()
        };

        let adj_prod = ADJACENT_MAT
            .iter()
            .map(|offset| {
                let adj_pos = sym_pos + offset;
                if let Some(Element::Number { digits, len: num_len }) = parsed_input.get(&adj_pos)
                    && let Element::Symbol { str_repr, len: sym_len } = sym
                    && str_repr == "*"
                {
                    println!("symbol: {str_repr:?} adj to num: {digits}");
                    digits
                } else {
                    &1
                }
            })
            .product::<usize>();
        total += adj_prod;
    });
    dbg!(total);
}

type FirstParse = Vec<(usize, String, CharClass)>;
fn parse_line_p1(s: &str) -> Vec<(usize, String, CharClass)> {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .chunk_by(classify_char)
        .into_iter()
        .enumerate()
        .map(|(x_coor, (group_type, group))| (x_coor, group.collect::<String>(), group_type))
        .collect_vec()
}
fn parse_line_p2(first_parse: (usize, FirstParse)) -> Vec<(Position, Element)> {
    let y = first_parse.0;

    first_parse
        .1
        .into_iter()
        .map(|(el_num, str_repr, group_type)| {
            let (x, element) = match group_type {
                CharClass::Digits => {
                    let len = str_repr.len();
                    let digits = str_repr.parse::<usize>().unwrap_or_else(|_| {
                        panic!("Expected group type: Digits for str_repr: {}", str_repr)
                    });
                    (el_num, Element::Number { digits, len })
                }
                CharClass::Symbols => {
                    let len = str_repr.len();
                    (el_num, Element::Symbol { len, str_repr })
                }
                CharClass::Dots => {
                    let len = str_repr.len();
                    (el_num, Element::Points { len })
                }
            };

            (Position { x, y }, element)
        })
        .collect::<Vec<_>>()
}

fn test() {
    dbg!(&(Position { x: 1, y: 1 }) + &ADJACENT_MAT[6]);
}
