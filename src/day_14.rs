use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: u64,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let robots = parser::input
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;

    Ok(Answer {
        part_1: solution::calculate_safety_factors(&robots, GridSize { x: 101, y: 103 }, 100),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Offset {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GridSize {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Robot {
    current_position: Position,
    velocity: Offset,
}

mod parser {
    use nom::Parser;

    use super::{Offset, Position, Robot};

    pub fn input(input: &str) -> nom::IResult<&str, Vec<Robot>> {
        nom::multi::separated_list1(nom::character::complete::newline, robot).parse(input)
    }

    fn robot(input: &str) -> nom::IResult<&str, Robot> {
        nom::sequence::separated_pair(position, nom::character::complete::space1, velocity)
            .map(|(current_position, velocity)| Robot {
                current_position,
                velocity,
            })
            .parse(input)
    }

    fn tagged_pair<'a, T>(
        tag: char,
        p: impl 'a + Copy + Parser<&'a str, T, nom::error::Error<&'a str>>,
    ) -> impl Fn(&'a str) -> nom::IResult<&'a str, (T, T)> {
        move |input| {
            nom::sequence::preceded(
                nom::bytes::complete::tag(format!("{tag}=").as_str()),
                nom::sequence::separated_pair(p, nom::character::complete::char(','), p),
            )
            .parse(input)
        }
    }

    fn position(input: &str) -> nom::IResult<&str, Position> {
        tagged_pair('p', nom::character::complete::u64)
            .map(|(x, y)| Position {
                x: usize::try_from(x).unwrap(),
                y: usize::try_from(y).unwrap(),
            })
            .parse(input)
    }

    fn velocity(input: &str) -> nom::IResult<&str, Offset> {
        tagged_pair('v', nom::character::complete::i64)
            .map(|(x, y)| Offset {
                x: isize::try_from(x).unwrap(),
                y: isize::try_from(y).unwrap(),
            })
            .parse(input)
    }

    #[test]
    fn example() {
        assert_eq!(
            Ok(("", super::example::intermediate())),
            input.parse(super::example::input())
        );
    }
}

mod solution {
    use super::{GridSize, Offset, Position, Robot};

    #[inline]
    fn wrapping_add_usize_and_isize_between_zero_and_upper_bound(
        l: usize,
        r: isize,
        upper_bound: usize,
    ) -> usize {
        let l = i128::try_from(l).unwrap();
        let r = i128::try_from(r).unwrap();

        let upper_bound = u64::try_from(upper_bound).unwrap();

        let sum = wrap_i128_between_zero_and_upper_bound(l + r, upper_bound);

        usize::try_from(sum).unwrap()
    }

    #[inline]
    fn wrap_i128_between_zero_and_upper_bound(x: i128, upper_bound_not_included: u64) -> i128 {
        assert!(upper_bound_not_included > 0);

        let upper_bound_not_included = i128::from(upper_bound_not_included);
        let m = x % upper_bound_not_included;

        if m < 0 {
            upper_bound_not_included + m
        } else {
            m
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Quadrant {
        UL,
        UR,
        DL,
        DR,
    }

    impl Position {
        fn wrapping_add_offset(&self, offset: Offset, grid_size: GridSize) -> Self {
            Self {
                x: wrapping_add_usize_and_isize_between_zero_and_upper_bound(
                    self.x,
                    offset.x,
                    grid_size.x,
                ),
                y: wrapping_add_usize_and_isize_between_zero_and_upper_bound(
                    self.y,
                    offset.y,
                    grid_size.y,
                ),
            }
        }

        fn quadrant(&self, grid_size: GridSize) -> Option<Quadrant> {
            let mid_x = grid_size.x / 2;
            let mid_y = grid_size.y / 2;

            let is_l = self.x < mid_x;
            let is_r = self.x > mid_x;

            let is_u = self.y < mid_y;
            let is_d = self.y > mid_y;

            match (is_u, is_d, is_l, is_r) {
                (true, false, true, false) => Some(Quadrant::UL),
                (true, false, false, true) => Some(Quadrant::UR),
                (false, true, true, false) => Some(Quadrant::DL),
                (false, true, false, true) => Some(Quadrant::DR),
                _ => None,
            }
        }
    }

    impl Robot {
        fn advance(&self, grid_size: GridSize) -> Self {
            Robot {
                current_position: self
                    .current_position
                    .wrapping_add_offset(self.velocity, grid_size),
                velocity: self.velocity,
            }
        }
    }

    pub fn calculate_safety_factors(robots: &[Robot], grid_size: GridSize, secs: usize) -> u64 {
        robots
            .iter()
            .cloned()
            .filter_map(|robot| {
                (0..secs)
                    .into_iter()
                    .fold(robot, |robot, _| robot.advance(grid_size))
                    .current_position
                    .quadrant(grid_size)
            })
            .fold([0u64, 0, 0, 0], |mut counts: [u64; 4], q| {
                match q {
                    Quadrant::UL => counts[0] += 1,
                    Quadrant::UR => counts[1] += 1,
                    Quadrant::DL => counts[2] += 1,
                    Quadrant::DR => counts[3] += 1,
                };

                counts
            })
            .into_iter()
            .product()
    }

    #[test]
    fn example() {
        assert_eq!(
            super::example::output(),
            calculate_safety_factors(
                &super::example::intermediate(),
                GridSize { x: 11, y: 7 },
                100
            )
        );
    }
}

#[cfg(test)]
mod example {
    use super::{Offset, Position, Robot};

    pub fn input() -> &'static str {
        include_str!("./examples/day14/example.txt")
    }

    pub fn intermediate() -> Vec<Robot> {
        include!("./examples/day14/intermediate.in")
    }

    pub fn output() -> u64 {
        12
    }
}
