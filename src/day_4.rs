use anyhow::anyhow;
use nom::Parser;

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
        part_1: solution::part_1::count_xmas(&input),
        part_2: solution::part_2::count_xmas(&input),
    })
}

mod parser {
    pub type Error<'a> = nom::error::Error<&'a str>;
    pub trait Parser<'a, T> = nom::Parser<&'a str, T, Error<'a>>;

    pub fn input<'a>() -> impl Parser<'a, Vec<Vec<char>>> {
        nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::multi::many1(nom::character::complete::one_of("XMAS")),
        )
    }

    #[test]
    fn example() {
        assert_eq!(
            input().parse(super::example::input()),
            Ok(("", super::example::intermediate()))
        )
    }
}

mod solution {
    use guard::guard;

    fn check_position(grid: &Vec<Vec<char>>, position: (i32, i32), expected_char: char) -> bool {
        let (row_idx, col_idx) = position;
        guard! {
            let Some(row) = grid.get(row_idx as usize) else {
                return false
            }
        };
        guard! {
            let Some(this_char) = row.get(col_idx as usize) else {
                return false
            }
        }

        expected_char == *this_char
    }

    fn make_positions<const C: usize>(
        current_position: (usize, usize),
        offset: [(i32 /* row */, i32 /* col */); C],
    ) -> [(i32 /* row */, i32 /* col */); C] {
        let (current_row_index, current_col_index) = current_position;
        offset.map(|(row_offset, col_offset)| {
            (
                current_row_index as i32 + row_offset,
                current_col_index as i32 + col_offset,
            )
        })
    }

    fn for_each_position<F>(grid: &Vec<Vec<char>>, mut f: F)
    where
        F: FnMut((usize, usize)),
    {
        grid.iter().enumerate().for_each(|(row_index, row)| {
            row.iter()
                .enumerate()
                .for_each(|(col_index, _)| f((row_index, col_index)))
        });
    }

    pub mod part_1 {
        use super::{check_position, make_positions};

        fn check_xmas_sequence(
            grid: &Vec<Vec<char>>,
            positions: [(i32 /* row */, i32 /* col */); 4],
        ) -> bool {
            let [x_pos, m_pos, a_pos, s_pos] = positions;

            check_position(grid, x_pos, 'X')
                && check_position(grid, m_pos, 'M')
                && check_position(grid, a_pos, 'A')
                && check_position(grid, s_pos, 'S')
        }

        pub fn count_xmas(grid: &Vec<Vec<char>>) -> usize {
            let mut count = 0;

            let offsets: [[(i32 /* row */, i32 /* col */); 4]; 8] = [
                [(0, 0), (0, -1), (0, -2), (0, -3)],    // left
                [(0, 0), (0, 1), (0, 2), (0, 3)],       // right
                [(0, 0), (-1, 0), (-2, 0), (-3, 0)],    // up
                [(0, 0), (1, 0), (2, 0), (3, 0)],       // down
                [(0, 0), (-1, -1), (-2, -2), (-3, -3)], // upper left
                [(0, 0), (-1, 1), (-2, 2), (-3, 3)],    // upper right
                [(0, 0), (1, -1), (2, -2), (3, -3)],    // lower left
                [(0, 0), (1, 1), (2, 2), (3, 3)],       // lower right
            ];

            super::for_each_position(grid, |position| {
                for offset in offsets {
                    let positions = make_positions(position, offset);
                    if check_xmas_sequence(&grid, positions) {
                        count += 1
                    }
                }
            });

            count
        }

        #[test]
        fn example() {
            assert_eq!(
                count_xmas(&super::super::example::intermediate()),
                super::super::example::output_p_1()
            );
        }
    }

    pub mod part_2 {
        use super::{check_position, make_positions};

        fn check_combination(
            grid: &Vec<Vec<char>>,
            positions: [(i32 /* row */, i32 /* col */); 5],
            combinatation: [char; 5],
        ) -> bool {
            positions
                .into_iter()
                .zip(combinatation.into_iter())
                .all(|(position, expected_char)| check_position(grid, position, expected_char))
        }

        pub fn count_xmas(grid: &Vec<Vec<char>>) -> usize {
            let mut count: usize = 0;

            let offset: [(i32 /* row */, i32 /* col */); 5] =
                [(0, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)];

            let combinations: [[char; 5]; 4] = [
                ['A', 'M', 'M', 'S', 'S'],
                ['A', 'M', 'S', 'M', 'S'],
                ['A', 'S', 'S', 'M', 'M'],
                ['A', 'S', 'M', 'S', 'M'],
            ];

            super::for_each_position(grid, |position| {
                for combination in combinations {
                    let positions = make_positions(position, offset);
                    if check_combination(grid, positions, combination) {
                        count += 1;
                        break;
                    }
                }
            });

            return count;
        }

        #[test]
        fn example() {
            assert_eq!(
                count_xmas(&super::super::example::intermediate()),
                super::super::example::output_p_2()
            );
        }
    }
}

#[cfg(test)]
mod example {
    pub fn input() -> &'static str {
        include_str!("./examples/day4/input.txt")
    }

    pub fn intermediate() -> Vec<Vec<char>> {
        include!("./examples/day4/intermediate.in")
            .into_iter()
            .map(Vec::from)
            .collect()
    }

    pub fn output_p_1() -> usize {
        18
    }

    pub fn output_p_2() -> usize {
        9
    }
}
