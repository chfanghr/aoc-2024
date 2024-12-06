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
        part_1: solution::move_guard_until_out_of_bound(&input),
        part_2: solution::number_of_obstructions_that_causes_looping(&input),
    })
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Input {
    guard_initial_direction: Direction,
    guard_initial_position: Position,
    map: Vec<Vec<Cell>>,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    row_index: i64,
    col_index: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Obstruction,
    Empty,
}

mod parser {
    use itertools::Itertools;

    use super::{Cell, Direction, Input, Position};

    pub type ParserInput<'a> = &'a str;
    pub type Error<'a> = nom::error::Error<ParserInput<'a>>;
    pub trait Parser<'a, T> = nom::Parser<ParserInput<'a>, T, Error<'a>>;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum IntermediateCell {
        Obstruction,
        Empty,
        Guard(Direction),
    }

    pub fn input<'a>() -> impl Parser<'a, Input> {
        nom::combinator::map_res(intermediate_map(), intermediate_map_to_input)
    }

    fn intermediate_map_to_input(map: Vec<Vec<IntermediateCell>>) -> Result<Input, String> {
        let (guard_initial_position, guard_initial_direction) = map
            .iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(col_index, cell)| match cell {
                        IntermediateCell::Guard(direction) => Some((
                            Position {
                                row_index: row_index.try_into().unwrap(),
                                col_index: col_index.try_into().unwrap(),
                            },
                            *direction,
                        )),
                        _ => None,
                    })
            })
            .exactly_one()
            .map_err(|_| "more than one guard found".to_string())?;

        let map = map
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|cell| match cell {
                        IntermediateCell::Empty | IntermediateCell::Guard(_) => Cell::Empty,
                        IntermediateCell::Obstruction => Cell::Obstruction,
                    })
                    .collect_vec()
            })
            .collect_vec();

        Ok(Input {
            guard_initial_direction,
            guard_initial_position,
            map,
        })
    }

    fn intermediate_map<'a>() -> impl Parser<'a, Vec<Vec<IntermediateCell>>> {
        nom::multi::separated_list1(nom::character::complete::newline, col())
    }

    fn col<'a>() -> impl Parser<'a, Vec<IntermediateCell>> {
        nom::multi::many1(intermediate_cell())
    }

    fn intermediate_cell<'a>() -> impl Parser<'a, IntermediateCell> {
        nom::combinator::map_res(
            nom::character::complete::anychar,
            |ch: char| -> Result<IntermediateCell, String> {
                match ch {
                    '.' => Ok(IntermediateCell::Empty),
                    '#' => Ok(IntermediateCell::Obstruction),
                    ch => {
                        let direction = match ch {
                            '^' => Ok(Direction::Up),
                            '>' => Ok(Direction::Right),
                            'v' => Ok(Direction::Down),
                            '<' => Ok(Direction::Left),
                            ch => Err(format!("invalid character on map: {ch}")),
                        }?;
                        Ok(IntermediateCell::Guard(direction))
                    }
                }
            },
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
    use std::collections::HashSet;

    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    use super::{Cell, Direction, Input, Position};

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum PositionValidity {
        Valid,
        InObstruction,
        OutOfBound,
    }

    impl Direction {
        fn next(&self) -> Self {
            match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        }
    }

    impl Position {
        fn advance(&self, at_direction: Direction) -> Self {
            match at_direction {
                Direction::Up => Self {
                    row_index: self.row_index - 1,
                    col_index: self.col_index,
                },
                Direction::Right => Self {
                    row_index: self.row_index,
                    col_index: self.col_index + 1,
                },
                Direction::Down => Self {
                    row_index: self.row_index + 1,
                    col_index: self.col_index,
                },
                Direction::Left => Self {
                    row_index: self.row_index,
                    col_index: self.col_index - 1,
                },
            }
        }

        fn grab_cell(&self, map: &Vec<Vec<Cell>>) -> Option<Cell> {
            let row_index = usize::try_from(self.row_index).ok()?;
            let col_index = usize::try_from(self.col_index).ok()?;
            let col = map.get(row_index)?;
            col.get(col_index).copied()
        }

        fn check_validity(&self, map: &Vec<Vec<Cell>>) -> PositionValidity {
            match self.grab_cell(map) {
                Some(cell) => match cell {
                    Cell::Obstruction => PositionValidity::InObstruction,
                    Cell::Empty => PositionValidity::Valid,
                },
                None => PositionValidity::OutOfBound,
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    struct GuardState {
        direction: Direction,
        current_position: Position,
    }

    impl GuardState {
        fn advance(&self, map: &Vec<Vec<Cell>>) -> Option<GuardState> {
            let next_position = self.current_position.advance(self.direction);
            match next_position.check_validity(map) {
                PositionValidity::Valid => Some(GuardState {
                    direction: self.direction,
                    current_position: next_position,
                }),
                PositionValidity::InObstruction => Some(GuardState {
                    direction: self.direction.next(),
                    current_position: self.current_position,
                }),
                PositionValidity::OutOfBound => None,
            }
        }
    }

    fn move_guard_until_out_of_bound_state_sequence(input: &Input) -> Vec<GuardState> {
        let mut guard_states = Vec::<GuardState>::new();
        let mut guard_state = GuardState {
            direction: input.guard_initial_direction,
            current_position: input.guard_initial_position,
        };

        loop {
            guard_states.push(guard_state);
            match guard_state.advance(&input.map) {
                Some(next_guard_state) => guard_state = next_guard_state,
                None => break,
            }
        }

        guard_states
    }

    pub fn move_guard_until_out_of_bound(input: &Input) -> usize {
        move_guard_until_out_of_bound_state_sequence(input)
            .into_iter()
            .map(|guard_state| guard_state.current_position)
            .collect::<HashSet<_>>()
            .len()
    }

    pub fn move_guard_while_detecting_looping(input: &Input) -> bool {
        let mut unique_guard_states = HashSet::<GuardState>::new();
        let mut guard_state = GuardState {
            direction: input.guard_initial_direction,
            current_position: input.guard_initial_position,
        };
        loop {
            if unique_guard_states.contains(&guard_state) {
                return true;
            }
            unique_guard_states.insert(guard_state);
            match guard_state.advance(&input.map) {
                Some(next_guard_state) => guard_state = next_guard_state,
                None => break,
            }
        }
        return false;
    }

    fn potential_additional_obstruction_positions(
        guard_states_without_addition_obstruction: Vec<GuardState>,
    ) -> HashSet<Position> {
        guard_states_without_addition_obstruction
            .into_iter()
            .skip(1)
            .map(|state| state.current_position)
            .collect()
    }

    pub fn number_of_obstructions_that_causes_looping(input: &Input) -> usize {
        let original_state_sequence = move_guard_until_out_of_bound_state_sequence(input);
        let potential_positions =
            potential_additional_obstruction_positions(original_state_sequence);

        potential_positions
            .into_par_iter()
            .filter(|position| {
                let mut input = input.clone();
                input.map[usize::try_from(position.row_index).unwrap()]
                    [usize::try_from(position.col_index).unwrap()] = Cell::Obstruction;
                move_guard_while_detecting_looping(&input)
            })
            .count()
    }

    #[test]
    fn example() {
        assert_eq!(
            super::example::output_p_1(),
            move_guard_until_out_of_bound(&super::example::intermediate())
        );
        assert_eq!(
            super::example::output_p_2(),
            number_of_obstructions_that_causes_looping(&super::example::intermediate())
        );
    }
}

#[cfg(test)]
mod example {
    use super::{Cell, Direction, Input, Position};

    pub fn input() -> &'static str {
        include_str!("./examples/day6/example.txt")
    }

    pub fn intermediate() -> Input {
        include!("./examples/day6/intermediate.in")
    }

    pub fn output_p_1() -> usize {
        41
    }

    pub fn output_p_2() -> usize {
        6
    }
}
