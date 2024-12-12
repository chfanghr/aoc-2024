use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: usize,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let input = parser::input
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;

    Ok(Answer {
        part_1: solution::calculate_total_price(&input),
    })
}

mod parser {
    use crate::grid::Grid;

    use itertools::Itertools;
    use nom::Parser;

    pub fn input(input: &str) -> nom::IResult<&str, Grid<char>> {
        nom::combinator::map_res(grid, |grid| {
            let cols = grid.first().unwrap().len();

            grid.iter()
                .all(|row| row.len() == cols)
                .then_some(Grid(grid))
                .ok_or("ambiguous column length".to_string())
        })
        .parse(input)
    }

    fn grid(input: &str) -> nom::IResult<&str, Vec<Vec<char>>> {
        nom::multi::separated_list1(nom::character::complete::line_ending, col)(input)
    }

    fn col(input: &str) -> nom::IResult<&str, Vec<char>> {
        nom::character::complete::alpha1
            .map(|str: &str| str.chars().collect_vec())
            .parse(input)
    }

    #[test]
    fn example() {
        assert_eq!(
            Ok(("", super::example::intermediate_1())),
            input.parse(&super::example::input_1())
        );
        assert_eq!(
            Ok(("", super::example::intermediate_2())),
            input.parse(&super::example::input_2())
        );
        assert_eq!(
            Ok(("", super::example::intermediate_3())),
            input.parse(&super::example::input_3())
        );
    }
}

mod solution {
    use itertools::Itertools;

    use crate::grid::{Grid, Offset};

    pub fn calculate_total_price(grid: &Grid<char>) -> usize {
        let grid_size = grid.size();
        let mut visited = Grid::fill_with(false, grid_size);
        let mut total_price = 0;

        for position in grid.positions() {
            let region_identifier = grid.must_get_cell(position);
            let offsets = [Offset(-1, 0), Offset(1, 0), Offset(0, 1), Offset(0, -1)];

            let mut area = 0usize;
            let mut perimeter = 0usize;

            let mut next_positions = vec![position];

            while let Some(position) = next_positions.pop() {
                if *visited.must_get_cell(position) {
                    continue;
                }

                let neighbor_positions = offsets
                    .into_iter()
                    .filter_map(|offset| {
                        position
                            .checked_add_offset(offset, grid_size)
                            .filter(|position| grid.must_get_cell(*position) == region_identifier)
                    })
                    .collect_vec();

                area += 1;
                perimeter += 4 - neighbor_positions.len();

                next_positions.extend(neighbor_positions);

                *visited.must_get_mut_cell(position) = true;
            }

            total_price += area * perimeter;
        }

        total_price
    }

    #[test]
    fn example() {
        assert_eq!(
            super::example::output_1(),
            calculate_total_price(&super::example::intermediate_1())
        );
        assert_eq!(
            super::example::output_2(),
            calculate_total_price(&super::example::intermediate_2())
        );
        assert_eq!(
            super::example::output_3(),
            calculate_total_price(&super::example::intermediate_3())
        );
    }
}

#[cfg(test)]
mod example {
    use crate::grid::Grid;
    use itertools::Itertools;

    pub fn input_1() -> &'static str {
        include_str!("./examples/day12/example.1.txt")
    }

    pub fn input_2() -> &'static str {
        include_str!("./examples/day12/example.2.txt")
    }

    pub fn input_3() -> &'static str {
        include_str!("./examples/day12/example.3.txt")
    }

    fn make_intermediate<const COLS: usize, const ROWS: usize>(
        a: [[char; COLS]; ROWS],
    ) -> Grid<char> {
        Grid(a.into_iter().map(|a| a.to_vec()).collect_vec())
    }

    pub fn intermediate_1() -> Grid<char> {
        make_intermediate(include!("./examples/day12/intermediate.1.in"))
    }

    pub fn intermediate_2() -> Grid<char> {
        make_intermediate(include!("./examples/day12/intermediate.2.in"))
    }

    pub fn intermediate_3() -> Grid<char> {
        make_intermediate(include!("./examples/day12/intermediate.3.in"))
    }

    pub fn output_1() -> usize {
        140
    }

    pub fn output_2() -> usize {
        772
    }

    pub fn output_3() -> usize {
        1930
    }
}
