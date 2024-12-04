use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: i64,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let multiplications = parser::input()
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;
    Ok(Answer {
        part_1: solution::sum_of_results_of_the_multiplications(&multiplications),
    })
}

mod parser {
    pub type Error<'a> = nom::error::Error<&'a str>;
    pub trait Parser<'a, T> = nom::Parser<&'a str, T, Error<'a>>;

    pub fn input<'a>() -> impl Parser<'a, Vec<(i64, i64)>> {
        nom::multi::many1(nom::branch::alt((
            mul().map(Some),
            nom::character::complete::anychar.map(|_| None),
        )))
        .map(|v: Vec<Option<(i64, i64)>>| {
            v.into_iter().filter_map(|x| x).collect::<Vec<(i64, i64)>>()
        })
    }

    fn mul<'a>() -> impl Parser<'a, (i64, i64)> {
        nom::sequence::preceded(
            nom::bytes::complete::tag("mul"),
            nom::sequence::delimited(
                nom::character::complete::char('('),
                nom::sequence::separated_pair(
                    nom::character::complete::i64,
                    nom::character::complete::char(','),
                    nom::character::complete::i64,
                ),
                nom::character::complete::char(')'),
            ),
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
    pub fn sum_of_results_of_the_multiplications(multiplications: &[(i64, i64)]) -> i64 {
        multiplications.iter().map(|(l, r)| l * r).sum()
    }

    #[test]
    fn example() {
        assert_eq!(
            sum_of_results_of_the_multiplications(&super::example::intermediate()),
            super::example::output()
        )
    }
}

#[cfg(test)]
mod example {
    pub fn input() -> &'static str {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    }

    pub fn intermediate() -> Vec<(i64, i64)> {
        vec![(2, 4), (5, 5), (11, 8), (8, 5)]
    }

    pub fn output() -> i64 {
        161
    }
}
