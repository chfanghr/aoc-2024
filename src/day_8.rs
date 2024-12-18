use anyhow::anyhow;
use nom::Parser;

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub struct Answer {
    pub part_1: usize,
    pub part_2: usize,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let input = parser::input()
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;

    Ok(Answer {
        part_1: solution::count_of_antinodes_p_1(&input),
        part_2: solution::count_of_antinodes_p_2(&input),
    })
}

#[derive(Debug, PartialEq, Eq)]
struct Input {
    grid_size: (usize, usize),
    antennas_for_frequencies: BTreeMap<char, BTreeSet<(usize, usize)>>,
}

mod parser {
    use std::{
        collections::{BTreeMap, BTreeSet},
        ops::Not,
    };

    use super::Input;

    pub type ParserInput<'a> = &'a str;
    pub type Error<'a> = nom::error::Error<ParserInput<'a>>;
    pub trait Parser<'a, T> = nom::Parser<ParserInput<'a>, T, Error<'a>>;

    pub fn input<'a>() -> impl Parser<'a, Input> {
        nom::combinator::map_res(grid(), grid_to_input)
    }

    fn grid_to_input(grid: Vec<Vec<Option<char>>>) -> Result<Input, String> {
        let row_size = grid.len();
        let col_size = grid.first().ok_or("empty grid".to_string())?.len();
        let grid_size = (row_size, col_size);

        let antennas_for_frequencies = grid
            .into_iter()
            .enumerate()
            .map(
                |(row_index, col)| -> Result<BTreeMap<char, BTreeSet<(usize, usize)>>, String> {
                    if col.len() != col_size {
                        return Err("ambiguous col size".to_string());
                    }

                    Ok(col.into_iter().enumerate().fold(
                        BTreeMap::new(),
                        |mut acc, (col_index, ch)| {
                            if let Some(ch) = ch {
                                acc.entry(ch).or_default().insert((row_index, col_index));
                            }
                            acc
                        },
                    ))
                },
            )
            .collect::<Result<Vec<_>, String>>()?
            .into_iter()
            .fold(
                BTreeMap::<char, BTreeSet<(usize, usize)>>::new(),
                |acc, m| {
                    m.into_iter().fold(acc, |mut acc, (ch, positions)| {
                        acc.entry(ch).or_default().extend(positions.into_iter());
                        acc
                    })
                },
            );

        Ok(Input {
            grid_size,
            antennas_for_frequencies,
        })
    }

    fn grid<'a>() -> impl Parser<'a, Vec<Vec<Option<char>>>> {
        nom::multi::separated_list1(nom::character::complete::newline, col())
    }

    fn col<'a>() -> impl Parser<'a, Vec<Option<char>>> {
        nom::multi::many1(
            nom::character::complete::satisfy(|ch| ch.is_alphanumeric() || ch == '.')
                .map(|ch| (ch == '.').not().then_some(ch)),
        )
    }

    #[test]
    fn example() {
        assert_eq!(
            Ok(("", super::example::intermediate())),
            input().parse(super::example::input())
        );
    }
}

mod solution {
    use std::collections::BTreeSet;

    use itertools::Itertools;

    use super::Input;

    pub fn count_of_antinodes_p_1(input: &Input) -> usize {
        discover_antinodes_of_all_frequencies_p_1(input).len()
    }

    pub fn count_of_antinodes_p_2(input: &Input) -> usize {
        discover_antinodes_of_all_frequencies_p_2(input).len()
    }

    fn discover_antinodes_of_all_frequencies_p_1(input: &Input) -> BTreeSet<(usize, usize)> {
        input
            .antennas_for_frequencies
            .iter()
            .map(|(_, antennas)| {
                discover_antinodes_of_certain_frequency_p1(input.grid_size, antennas)
            })
            .flatten()
            .collect()
    }

    fn discover_antinodes_of_all_frequencies_p_2(input: &Input) -> BTreeSet<(usize, usize)> {
        input
            .antennas_for_frequencies
            .iter()
            .map(|(_, antennas)| {
                discover_antinodes_of_certain_frequency_p_2(input.grid_size, antennas)
            })
            .flatten()
            .collect()
    }

    fn discover_antinodes_of_certain_frequency_p1(
        grid_size: (usize, usize),
        antennas: &BTreeSet<(usize, usize)>,
    ) -> BTreeSet<(usize, usize)> {
        antennas
            .iter()
            .map(|pos_l| {
                antennas
                    .iter()
                    .map(|pos_r| {
                        let offset = pos_offset(*pos_l, *pos_r);
                        vec![
                            pos_checked_add(grid_size, *pos_l, offset),
                            pos_checked_add(grid_size, *pos_r, offset),
                            pos_checked_sub(grid_size, *pos_l, offset),
                            pos_checked_sub(grid_size, *pos_r, offset),
                        ]
                    })
                    .flatten()
                    .flatten()
                    .collect_vec()
            })
            .flatten()
            .collect::<BTreeSet<(usize, usize)>>()
            .difference(antennas)
            .copied()
            .collect()
    }

    fn discover_antinodes_of_certain_frequency_p_2(
        grid_size: (usize, usize),
        antennas: &BTreeSet<(usize, usize)>,
    ) -> BTreeSet<(usize, usize)> {
        antennas
            .iter()
            .map(|pos_l| {
                antennas
                    .iter()
                    .map(|pos_r| {
                        if pos_l == pos_r {
                            return vec![];
                        }

                        let offset = pos_offset(*pos_l, *pos_r);
                        let mut all_possible_positions = vec![];
                        let mut add_possible_positions =
                            |make_pos: fn(
                                (usize, usize),
                                (usize, usize),
                                (i64, i64),
                            )
                                -> Option<(usize, usize)>,
                             pos: (usize, usize)| {
                                let mut x = 1i64;
                                while let Some(pos) =
                                    make_pos(grid_size, pos, scale_offset(offset, x))
                                {
                                    all_possible_positions.push(pos);
                                    x += 1
                                }
                            };

                        add_possible_positions(pos_checked_add, *pos_l);
                        add_possible_positions(pos_checked_add, *pos_r);
                        add_possible_positions(pos_checked_sub, *pos_l);
                        add_possible_positions(pos_checked_sub, *pos_r);

                        all_possible_positions
                    })
                    .flatten()
                    .collect_vec()
            })
            .flatten()
            .collect::<BTreeSet<(usize, usize)>>()
    }

    fn scale_offset(offset: (i64, i64), x: i64) -> (i64, i64) {
        (offset.0 * x, offset.1 * x)
    }

    fn pos_offset(pos_l: (usize, usize), pos_r: (usize, usize)) -> (i64, i64) {
        (
            pos_l.0 as i64 - pos_r.0 as i64,
            pos_l.1 as i64 - pos_r.1 as i64,
        )
    }

    fn pos_checked_add(
        grid_size: (usize, usize),
        pos: (usize, usize),
        offset: (i64, i64),
    ) -> Option<(usize, usize)> {
        let x = pos.0 as i64 + offset.0;
        let y = pos.1 as i64 + offset.1;

        ((0..grid_size.0 as i64).contains(&x) && (0..grid_size.1 as i64).contains(&y))
            .then_some((x as usize, y as usize))
    }

    fn pos_checked_sub(
        grid_size: (usize, usize),
        pos: (usize, usize),
        offset: (i64, i64),
    ) -> Option<(usize, usize)> {
        pos_checked_add(grid_size, pos, (-offset.0, -offset.1))
    }

    #[test]
    fn example() {
        assert_eq!(
            super::example::output_p_1(),
            count_of_antinodes_p_1(&super::example::intermediate())
        );
        assert_eq!(
            super::example::output_p_2(),
            count_of_antinodes_p_2(&super::example::intermediate())
        );
    }
}

#[cfg(test)]
mod example {
    use super::Input;

    pub fn input() -> &'static str {
        include_str!("./examples/day8/example.txt")
    }

    pub fn intermediate() -> Input {
        include!("./examples/day8/intermediate.in")
    }

    pub fn output_p_1() -> usize {
        14
    }

    pub fn output_p_2() -> usize {
        34
    }
}
