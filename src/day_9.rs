use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: u64,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let part_1_input = parser::part1::input()
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;

    Ok(Answer {
        part_1: solution::part_1::compact_disk_and_calculate_checksum(&part_1_input),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Free,
    File { id: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fragment {
    Free { count: usize },
    File { id: usize, count: usize },
}

mod parser {

    pub type ParserInput<'a> = &'a str;
    pub type Error<'a> = nom::error::Error<ParserInput<'a>>;
    pub trait Parser<'a, T> = nom::Parser<ParserInput<'a>, T, Error<'a>>;

    pub mod part1 {
        use std::iter::repeat;

        use itertools::Itertools;

        use super::{super::Block, Parser};

        pub fn input<'a>() -> impl Parser<'a, Vec<Block>> {
            nom::multi::many1(nom::character::complete::satisfy(|ch| ch.is_digit(10)))
                .map(digits_to_blocks)
        }

        fn digits_to_blocks(digits: Vec<char>) -> Vec<Block> {
            struct State {
                is_file: bool,
                file_id: usize,
            }

            impl State {
                fn advance(&mut self) {
                    if self.is_file {
                        self.file_id += 1
                    }
                    self.is_file = !self.is_file
                }

                fn initial_state() -> Self {
                    Self {
                        is_file: true,
                        file_id: 0,
                    }
                }
            }

            digits
                .into_iter()
                .map(|digit| -> usize { digit.to_digit(10).unwrap() as usize })
                .scan(State::initial_state(), |state, count| {
                    let block = if state.is_file {
                        Block::File { id: state.file_id }
                    } else {
                        Block::Free
                    };
                    let blocks = repeat(block).take(count).collect_vec();
                    state.advance();
                    Some(blocks)
                })
                .flatten()
                .collect_vec()
        }

        #[test]
        fn example() {
            assert_eq!(
                Ok(("", super::super::example::part_1::intermediate())),
                input().parse(super::super::example::input())
            )
        }
    }
}

mod solution {
    pub mod part_1 {
        use super::super::Block;

        fn compact_disk(blocks: &[Block]) -> Vec<Block> {
            if blocks.is_empty() {
                return vec![];
            }

            let mut blocks = blocks.to_vec();
            let indices = 0..blocks.len();

            let mut l_iter = indices.clone().into_iter();
            let mut r_iter = indices.rev();

            loop {
                let l_idx = l_iter.find(|idx| blocks[*idx] == Block::Free);
                let r_idx = r_iter.find(|idx| blocks[*idx] != Block::Free);

                match l_idx.zip(r_idx).filter(|(l_idx, r_idx)| l_idx < r_idx) {
                    Some((l_idx, r_idx)) => blocks.swap(l_idx, r_idx),
                    None => break,
                }
            }

            blocks
        }

        fn calculate_disk_checksum(blocks: &[Block]) -> u64 {
            blocks
                .iter()
                .enumerate()
                .fold(0, |acc, (idx, block)| match block {
                    Block::File { id } => acc + idx as u64 * *id as u64,
                    Block::Free => acc,
                })
        }

        pub fn compact_disk_and_calculate_checksum(blocks: &[Block]) -> u64 {
            calculate_disk_checksum(&compact_disk(blocks))
        }

        #[test]
        fn example() {
            assert_eq!(
                super::super::example::part_1::output(),
                compact_disk_and_calculate_checksum(&super::super::example::part_1::intermediate())
            )
        }
    }
}

#[cfg(test)]
mod example {

    pub fn input() -> &'static str {
        include_str!("./examples/day9/example.txt")
    }

    pub mod part_1 {
        use super::super::Block::{self, *};

        pub fn intermediate() -> Vec<Block> {
            include!("./examples/day9/intermediate.in").to_vec()
        }

        pub fn output() -> u64 {
            1928
        }
    }
}
