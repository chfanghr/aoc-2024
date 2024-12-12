use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: u64,
    pub part_2: u64,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let input = parser::input()
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;

    Ok(Answer {
        part_1: solution::total_score_of_topographic_map(&input),
        part_2: solution::total_rating_of_topographic_map(&input),
    })
}

mod parser {
    use itertools::Itertools;

    use crate::grid::Grid;

    pub type ParserInput<'a> = &'a str;
    pub type Error<'a> = nom::error::Error<ParserInput<'a>>;
    pub trait Parser<'a, T> = nom::Parser<ParserInput<'a>, T, Error<'a>>;

    impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
        type Error = String;

        fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
            value
                .iter()
                .map(|v| v.len())
                .all_equal()
                .then_some(Grid(value))
                .ok_or("ambiguous column length".to_string())
        }
    }

    pub fn input<'a>() -> impl Parser<'a, Grid<u8>> {
        nom::combinator::map_res(grid(), Grid::<u8>::try_from)
    }

    fn grid<'a>() -> impl Parser<'a, Vec<Vec<u8>>> {
        nom::multi::separated_list1(nom::character::complete::newline, col())
    }

    fn col<'a>() -> impl Parser<'a, Vec<u8>> {
        const RADIX: u32 = 10;
        nom::multi::many1(nom::character::complete::satisfy(|ch| ch.is_digit(RADIX))).map(
            |v: Vec<char>| {
                v.into_iter()
                    .map(|ch: char| ch.to_digit(RADIX).unwrap().try_into().unwrap())
                    .collect_vec()
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
    use itertools::Itertools;

    use crate::grid::{Grid, Offset, Position};

    #[derive(Debug, Clone)]
    #[repr(transparent)]
    struct HeightMap(Grid<(u8, Vec<Position>)>);

    impl HeightMap {
        fn new(grid: &Grid<u8>) -> Self {
            let grid_size = grid.size();
            let offsets: [Offset; 4] = [Offset::DOWN, Offset::UP, Offset::RIGHT, Offset::LEFT];

            let height_and_neighbors = grid.positions().fold(
                Grid::fill_with((0, vec![]), grid_size),
                |mut neighbors, current_position| {
                    let current_height = *grid.must_get_cell(current_position);
                    *neighbors.must_get_mut_cell(current_position) = (
                        current_height,
                        offsets
                            .into_iter()
                            .filter_map(|offset| -> Option<Position> {
                                current_position
                                    .checked_add_offset(offset, grid_size.into())
                                    .filter(|position| {
                                        *grid.must_get_cell(*position) == current_height + 1
                                    })
                            })
                            .collect_vec(),
                    );
                    neighbors
                },
            );

            HeightMap(height_and_neighbors)
        }

        fn calculate_score_of_trailhead(
            &self,
            trailhead_position: Position,
            unique_trail_ends: bool,
        ) -> u64 {
            let mut visited = Grid::fill_with(false, self.0.size());
            let mut score = 0u64;
            let mut next_positions = vec![trailhead_position];

            while let Some(current_position) = next_positions.pop() {
                if !unique_trail_ends && *visited.must_get_cell(current_position) {
                    continue;
                }

                let (current_height, current_neighbors) = self.0.must_get_cell(current_position);

                if *current_height == 9 {
                    score += 1
                } else {
                    next_positions.extend(current_neighbors.into_iter())
                }

                *visited.must_get_mut_cell(current_position) = true;
            }

            score
        }

        fn discover_trailheads<'a>(&'a self) -> impl 'a + Iterator<Item = Position> {
            self.0
                .positions()
                .filter(|position| self.0.must_get_cell(*position).0 == 0)
        }

        fn calculate_total_score(&self, unique_trail_ends: bool) -> u64 {
            self.discover_trailheads()
                .map(|trailhead| self.calculate_score_of_trailhead(trailhead, unique_trail_ends))
                .sum()
        }
    }

    pub fn total_score_of_topographic_map(grid: &Grid<u8>) -> u64 {
        HeightMap::new(grid).calculate_total_score(false)
    }

    pub fn total_rating_of_topographic_map(grid: &Grid<u8>) -> u64 {
        HeightMap::new(grid).calculate_total_score(true)
    }

    #[test]
    fn example() {
        assert_eq!(
            super::example::output_p_1(),
            total_score_of_topographic_map(&super::example::intermediate())
        );
        assert_eq!(
            super::example::output_p_2(),
            total_rating_of_topographic_map(&super::example::intermediate())
        );
    }
}

#[cfg(test)]
mod example {
    use itertools::Itertools;

    use crate::grid::Grid;

    pub fn input() -> &'static str {
        include_str!("./examples/day10/example.txt")
    }

    pub fn intermediate() -> Grid<u8> {
        Grid(
            include!("./examples/day10/intermediate.in")
                .into_iter()
                .map(|a| a.to_vec())
                .collect_vec(),
        )
    }

    pub fn output_p_1() -> u64 {
        36
    }

    pub fn output_p_2() -> u64 {
        81
    }
}
