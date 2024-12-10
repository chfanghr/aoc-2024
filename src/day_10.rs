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

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
struct Grid<T>(Vec<Vec<T>>);

mod parser {
    use itertools::Itertools;
    use nom::multi::many1;

    use super::Grid;

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
        many1(nom::character::complete::satisfy(|ch| ch.is_digit(RADIX))).map(|v: Vec<char>| {
            v.into_iter()
                .map(|ch: char| ch.to_digit(RADIX).unwrap().try_into().unwrap())
                .collect_vec()
        })
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
    use std::iter::repeat;

    use itertools::Itertools;

    use super::Grid;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct Position(usize, usize);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct Offset(isize, isize);

    impl Position {
        #[inline]
        fn checked_add_offset<T>(&self, offset: Offset, grid: &Grid<T>) -> Option<Self> {
            let Position(row_index, col_index) = self;
            let Offset(row_offset, col_offset) = offset;
            let GridSize(rows, cols) = grid.size();
            let row_index = row_index
                .checked_add_signed(row_offset)
                .filter(|row_index| *row_index < rows)?;
            let col_index = col_index
                .checked_add_signed(col_offset)
                .filter(|col_index| *col_index < cols)?;
            Some(Position(row_index, col_index))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct GridSize(pub(crate) usize, pub(crate) usize);

    impl<T> Grid<T> {
        fn new_fill_with(elm: T, grid_size: GridSize) -> Self
        where
            T: Clone,
        {
            let GridSize(cols, rows) = grid_size;
            Grid(
                repeat(repeat(elm).take(cols).collect_vec())
                    .take(rows)
                    .collect_vec(),
            )
        }

        #[inline]
        fn size(&self) -> GridSize {
            let rows = self.0.len();
            let cols = self.0.get(0).map(|row| row.len()).unwrap_or(0);
            GridSize(rows, cols)
        }

        #[inline]
        fn must_get_cell<'a>(&'a self, position: Position) -> &'a T {
            let Position(row_index, col_index) = position;
            self.0.get(row_index).unwrap().get(col_index).unwrap()
        }

        #[inline]
        fn must_get_mut_cell<'a>(&'a mut self, position: Position) -> &'a mut T {
            let Position(row_index, col_index) = position;
            self.0
                .get_mut(row_index)
                .unwrap()
                .get_mut(col_index)
                .unwrap()
        }
    }

    #[derive(Debug, Clone)]
    #[repr(transparent)]
    struct HeightMap<'a>(&'a Grid<u8>);

    impl<'a> HeightMap<'a> {
        fn new(grid: &'a Grid<u8>) -> Self {
            HeightMap(grid)
        }

        fn calculate_score_of_trailhead(
            &self,
            trailhead_position: Position,
            unique_trail_ends: bool,
        ) -> u64 {
            let offsets: [Offset; 4] = [
                Offset(1, 0),  // Down
                Offset(-1, 0), // Up
                Offset(0, 1),  // Right
                Offset(0, -1), // Left
            ];

            let mut visited = Grid::new_fill_with(false, self.0.size());
            let mut score = 0u64;
            let mut next_positions = vec![trailhead_position];

            while let Some(current_position) = next_positions.pop() {
                if !unique_trail_ends && *visited.must_get_cell(current_position) {
                    continue;
                }

                let current_height = *self.0.must_get_cell(current_position);

                if current_height == 9 {
                    score += 1;
                } else {
                    next_positions.extend(offsets.into_iter().filter_map(
                        |offset| -> Option<Position> {
                            current_position.checked_add_offset(offset, &self.0).filter(
                                |position| *self.0.must_get_cell(*position) == current_height + 1,
                            )
                        },
                    ));
                }

                *visited.must_get_mut_cell(current_position) = true;
            }

            score
        }

        fn discover_trailheads(&self) -> Vec<Position> {
            let GridSize(rows, cols) = self.0.size();

            (0..rows)
                .into_iter()
                .map(|row_index| {
                    (0..cols)
                        .into_iter()
                        .filter_map(|col_index| {
                            let pos = Position(row_index, col_index);
                            (*self.0.must_get_cell(pos) == 0).then_some(pos)
                        })
                        .collect_vec()
                })
                .flatten()
                .collect_vec()
        }

        fn calculate_total_score(&self, unique_trail_ends: bool) -> u64 {
            self.discover_trailheads()
                .into_iter()
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

    use super::Grid;

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
