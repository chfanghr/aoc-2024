use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: i64,
    pub part_2: i64,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let input = parser::input()
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;

    Ok(Answer {
        part_1: solution::sum_of_possible_calibration_results::<false>(&input),
        part_2: solution::sum_of_possible_calibration_results::<true>(&input),
    })
}
mod parser {
    pub type ParserInput<'a> = &'a str;
    pub type Error<'a> = nom::error::Error<ParserInput<'a>>;
    pub trait Parser<'a, T> = nom::Parser<ParserInput<'a>, T, Error<'a>>;

    pub fn input<'a>() -> impl Parser<'a, Vec<(i64, Vec<i64>)>> {
        nom::multi::separated_list1(nom::character::complete::newline, equation())
    }

    fn equation<'a>() -> impl Parser<'a, (i64, Vec<i64>)> {
        nom::sequence::separated_pair(
            nom::character::complete::i64,
            nom::character::complete::char(':').and(nom::character::complete::space1),
            nom::multi::separated_list1(
                nom::character::complete::space1,
                nom::character::complete::i64,
            ),
        )
    }

    #[test]
    fn example() {
        assert_eq!(
            Ok(("", super::example::intermediate())),
            input().parse(super::example::input())
        )
    }
}

mod solution {
    use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

    fn all_expr_results<const DO_CONCAT: bool>(nums: &[i64]) -> Vec<i64> {
        let mut stack: Vec<(&[i64], Option<i64>)> = vec![(nums, None)];

        let mut results = Vec::<i64>::new();

        while let Some((remaining, current)) = stack.pop() {
            if let Some(x) = remaining.get(0) {
                let remaining = &remaining[1..];
                if let Some(current) = current {
                    stack.push((remaining, Some(current + x)));
                    stack.push((remaining, Some(current * x)));
                    if DO_CONCAT {
                        stack.push((remaining, Some(concat(current, *x))));
                    }
                } else {
                    stack.push((remaining, Some(*x)));
                }
            } else {
                if let Some(current) = current {
                    results.push(current);
                }
            }
        }

        results
    }

    fn is_equation_possible<const DO_CONCAT: bool>(target: i64, nums: &[i64]) -> bool {
        all_expr_results::<DO_CONCAT>(nums)
            .into_iter()
            .any(|result| result == target)
    }

    pub fn sum_of_possible_calibration_results<const DO_CONCAT: bool>(
        input: &Vec<(i64, Vec<i64>)>,
    ) -> i64 {
        input
            .par_iter()
            .filter_map(|(target, nums)| {
                is_equation_possible::<DO_CONCAT>(*target, nums).then_some(target)
            })
            .sum()
    }

    fn concat(l: i64, r: i64) -> i64 {
        let mut exp = 1;

        while r / 10i64.pow(exp) > 0 {
            exp += 1
        }

        return l * 10i64.pow(exp) + r;
    }

    #[test]
    fn example() {
        let examples = super::example::intermediate();
        assert_eq!(
            super::example::output_p_1(),
            sum_of_possible_calibration_results::<false>(&examples)
        );
        assert_eq!(
            super::example::output_p_2(),
            sum_of_possible_calibration_results::<true>(&examples)
        );
    }

    proptest::proptest! {
        #[test]
        fn prop_concat(x: u16, y:u16) {
            let using_format_parse: i64 = format!("{x}{y}").parse().unwrap();
            proptest::prop_assert_eq!(using_format_parse, concat(x as i64, y as i64))
        }
    }
}

#[cfg(test)]
mod example {
    pub fn input() -> &'static str {
        include_str!("./examples/day7/example.txt")
    }

    pub fn intermediate() -> Vec<(i64, Vec<i64>)> {
        include!("./examples/day7/intermediate.in")
    }

    pub fn output_p_1() -> i64 {
        3749
    }

    pub fn output_p_2() -> i64 {
        11387
    }
}
