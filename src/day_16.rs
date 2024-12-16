use crate::grid::{Grid, Position};

use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: u64,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let input = parser::input
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;

    Ok(Answer {
        part_1: solution::calaculate_lowest_score(&input)
            .ok_or(anyhow!("unable to reach the ending cell"))?,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    starting_position: Position,
    ending_position: Position,
    grid: Grid<Cell>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Air,
    Wall,
}

mod parser {
    use closure::closure;
    use itertools::Itertools;
    use nom::Parser;

    use crate::grid::{Grid, Position};

    use super::{Cell, Input};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum IntermediateCell {
        Start,
        End,
        Wall,
        Air,
    }

    fn find_position<T, F: Fn(&T) -> bool>(
        tag: &str,
        f: F,
        vec: &Vec<Vec<T>>,
    ) -> Result<Position, String> {
        vec.iter()
            .enumerate()
            .flat_map(|(row_index, col)| {
                col.iter().enumerate().filter_map(
                    closure!(move row_index, ref f, |(col_index, cell)| {
                        f(cell).then_some(Position {
                            row_index, col_index
                        })
                    }),
                )
            })
            .exactly_one()
            .map_err(|err| format!("expect exactly one {tag} position found, err: {err}"))
    }

    impl TryFrom<Vec<Vec<IntermediateCell>>> for Input {
        type Error = String;

        fn try_from(vec: Vec<Vec<IntermediateCell>>) -> Result<Self, Self::Error> {
            let cols = vec.first().ok_or("empty grid".to_owned())?.len();
            let starting_position =
                find_position("starting", |cell| *cell == IntermediateCell::Start, &vec)?;
            let ending_position =
                find_position("ending", |cell| *cell == IntermediateCell::End, &vec)?;
            let grid = Grid(
                vec.into_iter()
                    .map(|col| {
                        if col.len() != cols {
                            Err("ambiguous col len".to_owned())
                        } else {
                            Ok(col
                                .into_iter()
                                .map(|cell| match cell {
                                    IntermediateCell::Wall => Cell::Wall,
                                    _ => Cell::Air,
                                })
                                .collect_vec())
                        }
                    })
                    .try_collect::<_, Vec<_>, _>()?,
            );

            Ok(Input {
                starting_position,
                ending_position,
                grid,
            })
        }
    }

    pub fn input(input: &str) -> nom::IResult<&str, Input> {
        nom::combinator::map_res(grid, Input::try_from).parse(input)
    }

    fn grid(input: &str) -> nom::IResult<&str, Vec<Vec<IntermediateCell>>> {
        nom::multi::separated_list1(nom::character::complete::newline, col).parse(input)
    }

    fn col(input: &str) -> nom::IResult<&str, Vec<IntermediateCell>> {
        nom::multi::many1(
            nom::character::complete::one_of("SE#.").map(|value| match value {
                'S' => IntermediateCell::Start,
                'E' => IntermediateCell::End,
                '#' => IntermediateCell::Wall,
                '.' => IntermediateCell::Air,
                _ => panic!(),
            }),
        )
        .parse(input)
    }

    #[test]
    fn example() {
        assert_eq!(
            Ok(("", super::example::intermediate_1())),
            input.parse(super::example::input_1())
        );
        assert_eq!(
            Ok(("", super::example::intermediate_2())),
            input.parse(super::example::input_2())
        );
    }
}

mod solution {
    use std::collections::HashMap;

    use crate::grid::{Offset, Position};

    use super::{Cell, Input};

    fn turning_penalty(current_direction: Offset, next_direction: Offset) -> u64 {
        match current_direction.dot(next_direction) {
            0 => 1000,
            -1 => 2000,
            1 => 0,
            _ => panic!(),
        }
    }

    pub fn calaculate_lowest_score(input: &Input) -> Option<u64> {
        let grid_size = input.grid.size();

        let offsets = [Offset::UP, Offset::DOWN, Offset::LEFT, Offset::RIGHT];

        let mut next_positions: Vec<(Position, Offset, u64)> = offsets
            .into_iter()
            .map(|offset| (input.starting_position, offset, 0))
            .collect();
        let mut visited: HashMap<Position, u64> = HashMap::new();

        while let Some((position, current_direction, score)) = next_positions.pop() {
            if let Some(last_known_score) = visited.get(&position) {
                if *last_known_score < score {
                    continue;
                }
            }

            visited.insert(position, score);

            next_positions.extend(offsets.into_iter().filter_map(
                |offset| -> Option<(Position, Offset, u64)> {
                    let next_position = position.checked_add_offset(offset, grid_size.into())?;
                    (input.grid.must_get_cell(next_position) == &Cell::Air).then_some((
                        next_position,
                        offset,
                        score + 1 + turning_penalty(current_direction, offset),
                    ))
                },
            ));
        }

        visited.get(&input.ending_position).copied()
    }

    #[test]
    fn example() {
        assert_eq!(
            Some(super::example::output_1()),
            calaculate_lowest_score(&super::example::intermediate_1())
        );
        assert_eq!(
            Some(super::example::output_2()),
            calaculate_lowest_score(&super::example::intermediate_2())
        );
    }
}

#[cfg(test)]
mod example {
    use super::{Cell::*, Input};
    use crate::grid::{Grid, Position};

    pub fn input_1() -> &'static str {
        include_str!("./examples/day16/example.1.txt")
    }

    pub fn input_2() -> &'static str {
        include_str!("./examples/day16/example.2.txt")
    }

    pub fn intermediate_1() -> Input {
        include!("./examples/day16/intermediate.1.in")
    }

    pub fn intermediate_2() -> Input {
        include!("./examples/day16/intermediate.2.in")
    }

    pub fn output_1() -> u64 {
        6036
    }

    pub fn output_2() -> u64 {
        10048
    }
}
