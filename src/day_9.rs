use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: u64,
    pub part_2: u64,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let part_1_input = parser::part1::input()
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;
    let part_2_input = parser::part2::input()
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;

    Ok(Answer {
        part_1: solution::part_1::compact_disk_and_calculate_checksum(&part_1_input),
        part_2: solution::part_2::compact_disk_and_calculate_checksum(&part_2_input),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Free,
    File { id: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fragment {
    Free { size: usize },
    File { id: usize, size: usize },
}

mod parser {
    use itertools::Itertools;

    pub type ParserInput<'a> = &'a str;
    pub type Error<'a> = nom::error::Error<ParserInput<'a>>;
    pub trait Parser<'a, T> = nom::Parser<ParserInput<'a>, T, Error<'a>>;

    struct FragmentState {
        is_file: bool,
        file_id: usize,
    }

    impl FragmentState {
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

    fn input_from_digits<'a, T, F: Fn(Vec<usize>) -> T>(f: F) -> impl Parser<'a, T> {
        const RADIX: u32 = 10;
        nom::multi::many1(nom::character::complete::satisfy(|ch| ch.is_digit(RADIX)))
            .map(|v: Vec<char>| {
                v.into_iter()
                    .map(|ch: char| ch.to_digit(RADIX).unwrap() as usize)
                    .collect_vec()
            })
            .map(f)
    }

    pub mod part1 {
        use std::iter::repeat;

        use itertools::Itertools;

        use super::{super::Block, input_from_digits, FragmentState, Parser};

        pub fn input<'a>() -> impl Parser<'a, Vec<Block>> {
            input_from_digits(digits_to_blocks)
        }

        fn digits_to_blocks(counts: Vec<usize>) -> Vec<Block> {
            counts
                .into_iter()
                .scan(FragmentState::initial_state(), |state, count| {
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

    pub mod part2 {
        use itertools::Itertools;

        use super::{super::Fragment, input_from_digits, FragmentState, Parser};

        pub fn input<'a>() -> impl Parser<'a, Vec<Fragment>> {
            input_from_digits(digits_to_fragments)
        }

        pub fn digits_to_fragments(counts: Vec<usize>) -> Vec<Fragment> {
            counts
                .into_iter()
                .scan(FragmentState::initial_state(), |state, count| {
                    let fragment = if state.is_file {
                        Fragment::File {
                            id: state.file_id,
                            size: count,
                        }
                    } else {
                        Fragment::Free { size: count }
                    };
                    state.advance();
                    Some(fragment)
                })
                .collect_vec()
        }

        #[test]
        fn example() {
            assert_eq!(
                Ok(("", super::super::example::part_2::intermediate())),
                input().parse(super::super::example::input())
            )
        }
    }
}

mod solution {
    use super::Block;

    fn calculate_disk_checksum(blocks: &[Block]) -> u64 {
        blocks
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, block)| match block {
                Block::File { id } => acc + idx as u64 * *id as u64,
                Block::Free => acc,
            })
    }

    pub mod part_1 {
        use super::{super::Block, calculate_disk_checksum};

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

    pub mod part_2 {
        use std::{collections::BTreeSet, iter::repeat, mem::replace};

        use itertools::Itertools;

        use super::{
            super::{Block, Fragment},
            calculate_disk_checksum,
        };

        fn compact_disk(fragments: &[Fragment]) -> Vec<Block> {
            let mut output_fragments = fragments.to_vec();

            let mut file_ids_to_move = fragments
                .iter()
                .filter_map(|frag| match frag {
                    Fragment::Free { .. } => None,
                    Fragment::File { id, .. } => Some(*id),
                })
                .collect::<BTreeSet<_>>();

            let mut r_neg_offset = 0usize;

            while r_neg_offset < fragments.len() {
                let idx = fragments.len() - 1 - r_neg_offset;
                let is_fragment_moved = match output_fragments[idx] {
                    Fragment::Free { .. } => false,
                    Fragment::File { id, size: count } => {
                        file_ids_to_move.remove(&id)
                            && move_file_fragment(&mut output_fragments, idx, id, count)
                    }
                };
                if !is_fragment_moved {
                    r_neg_offset += 1
                }
            }

            output_fragments
                .into_iter()
                .map(|fragment| match fragment {
                    Fragment::Free { size } => repeat(Block::Free).take(size).collect_vec(),
                    Fragment::File { id, size } => {
                        repeat(Block::File { id }).take(size).collect_vec()
                    }
                })
                .flatten()
                .collect_vec()
        }

        fn move_file_fragment(
            fragments: &mut Vec<Fragment>,
            file_idx: usize,
            file_id: usize,
            file_size: usize,
        ) -> bool {
            if let Some((move_to_index, fragment)) = fragments
                .iter_mut()
                .find_position(|fragment| match fragment {
                    Fragment::Free { size: count } => *count >= file_size,
                    Fragment::File { .. } => false,
                })
                .filter(|(move_to_index, _)| *move_to_index < file_idx)
            {
                let empty_fragment = replace(
                    fragment,
                    Fragment::File {
                        id: file_id,
                        size: file_size,
                    },
                );
                fragments[file_idx] = Fragment::Free { size: file_size };

                match empty_fragment {
                    Fragment::Free { size } => {
                        if size > file_size {
                            fragments.insert(
                                move_to_index + 1,
                                Fragment::Free {
                                    size: size - file_size,
                                },
                            );
                        }
                    }
                    _ => panic!(),
                }

                return true;
            }

            return false;
        }

        pub fn compact_disk_and_calculate_checksum(fragments: &[Fragment]) -> u64 {
            calculate_disk_checksum(&compact_disk(fragments))
        }

        #[test]
        fn example() {
            assert_eq!(
                super::super::example::part_2::output(),
                compact_disk_and_calculate_checksum(&super::super::example::part_2::intermediate())
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
            include!("./examples/day9/intermediate.1.in").to_vec()
        }

        pub fn output() -> u64 {
            1928
        }
    }

    pub mod part_2 {
        use super::super::Fragment::{self, *};

        pub fn intermediate() -> Vec<Fragment> {
            include!("./examples/day9/intermediate.2.in").to_vec()
        }

        pub fn output() -> u64 {
            2858
        }
    }
}
