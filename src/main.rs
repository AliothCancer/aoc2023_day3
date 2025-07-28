#![allow(unused, clippy::uninlined_format_args)]

mod implementations;

use core::panic;
use implementations::*;
use itertools::Itertools;
use std::{
    collections::{BTreeMap, HashMap, btree_map::Range},
    fmt::Debug,
    ops::{Add, RangeInclusive, Sub},
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
...$.*....
.664.598..";

fn find_element(
    row: &[Element],
    other_pos: Position,
    x: RangeInclusive<usize>,
) -> Option<&Element> {
    row.iter().find(|el| {
        {
            matches!(el, Element::Number {x: inc_x, ..} if {
                inc_x.clone()
                .any(|inc_x| x.clone().any(|x| inc_x == x+1 || inc_x == x-1))
            })
        }
    })
}
fn main() {
    let parsed_input = load_input()
        .lines()
        .map(parse_line_p1)
        .enumerate() // will be y
        .map(parse_line_p2)
        //.inspect(|x| println!("{x:?}"))
        .collect::<Vec<_>>();
    let row_len = parsed_input[0].len();
    let col_len = parsed_input.len();

    let mut total_gear_ratio: u64 = 0;
    parsed_input
        .iter()
        .flatten()
        .enumerate()
        .for_each(|(nth, element)| {
            if let Element::Symbol { str_repr, x } = element
                && str_repr.contains('*')
            {
                let mut row_index = nth / row_len;
                
                let adj_mat = if row_len == row_index{
                    ADJACENT_MAT.iter().filter(|offset| offset.y != 1).collect_vec()
                } else{
                    ADJACENT_MAT.iter().collect_vec()
                };
                //println!("row_index = {nth} / {row_len} = {row_index}");
                //dbg!(&str_repr, col_number);
                let sym_len = x.try_len().unwrap();
                match sym_len {
                    1 => {
                        let res = adj_mat
                            .into_iter()
                            .filter_map(|offset| {
                                let mut other_pos = (&Position {
                                    x: x.clone(),
                                    y: row_index,
                                } + &offset);
                                if other_pos.y > 139{
                                    other_pos.y = 139;
                                }
                                let sym_pos = Position {
                                    x: x.clone(),
                                    y: row_index,
                                };
                                match find_element(
                                    &parsed_input[other_pos.y],
                                    other_pos.clone(),
                                    x.clone(),
                                ) {
                                    Some(number) => Some((
                                        number,
                                        sym_pos.clone().min_distances(other_pos),
                                        str_repr,
                                        sym_pos.y,
                                    )),
                                    None => None,
                                }
                            })
                            .filter(|tup| tup.1 <= 1.5)
                            .unique_by(|(num, distance, sym_str, sym_y)|{
                                (*num, *sym_y)
                            })
                            .inspect(|(num, distance, sym_str, sym_y)| {
                                println!(
                                    "{num:?} distance {sym_str} at ({x:?}, {sym_y}): {distance} "
                                )
                            })
                            .map(|tup| match tup.0{
                                Element::Number { digits, .. } => u64::try_from(*digits).unwrap(),
                                _ => unimplemented!()
                            }).collect_vec();
                        if res.len() == 2{
                            //println!("{res:?}");
                            total_gear_ratio += res.into_iter().product::<u64>()
                        }
                    }
                    _ => {
                        unimplemented!("adjacent Multi symbol not handled")
                    }
                };
            };
        });
        println!("Solution: {}", total_gear_ratio);
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
fn parse_line_p2(first_parse: (usize, FirstParse)) -> Vec<Element> {
    let y = first_parse.0;
    let mut x_counter = 0;
    first_parse
        .1
        .into_iter()
        .map(|(el_num, str_repr, group_type)| match group_type {
            CharClass::Digits => {
                let len = str_repr.len();
                let digits = str_repr.parse::<usize>().unwrap_or_else(|_| {
                    panic!("Expected group type: Digits for str_repr: {}", str_repr)
                });
                let len = str_repr.len();
                let x_min = x_counter;
                x_counter += len;
                let x_max = x_counter;
                Element::Number {
                    digits,
                    x: x_min..=(x_max - 1),
                }
            }
            CharClass::Symbols => {
                let len = str_repr.len();
                let x_min = x_counter;
                x_counter += len;
                let x_max = x_counter;

                Element::Symbol {
                    x: x_min..=(x_max - 1),
                    str_repr,
                }
            }
            CharClass::Dots => {
                let len = str_repr.len();
                let x_min = x_counter;
                x_counter += len;
                let x_max = x_counter;

                Element::Points {
                    x: x_min..=(x_max - 1),
                }
            }
        })
        .collect::<Vec<_>>()
}
