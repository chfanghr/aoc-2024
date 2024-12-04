use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: i64,
    pub part_2: i64,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let (left_list, right_list) = parser::input()
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;
    Ok(Answer {
        part_1: solution::total_distance(&left_list, &right_list),
        part_2: solution::similarity_score(&left_list, &right_list),
    })
}

mod parser {
    pub type Error<'a> = nom::error::Error<&'a str>;
    pub trait Parser<'a, T> = nom::Parser<&'a str, T, Error<'a>>;

    pub fn input<'a>() -> impl Parser<'a, (Vec<i64>, Vec<i64>)> {
        nom::multi::separated_list1(nom::character::complete::newline, line::<'a>())
            .map(|v: Vec<(i64, i64)>| v.into_iter().unzip())
    }

    fn line<'a>() -> impl Parser<'a, (i64, i64)> {
        nom::sequence::separated_pair(
            nom::character::complete::i64,
            nom::character::complete::space1,
            nom::character::complete::i64,
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
    use std::collections::BTreeMap;

    use itertools::Itertools;

    pub fn total_distance(left_list: &[i64], right_list: &[i64]) -> i64 {
        left_list
            .iter()
            .sorted()
            .rev()
            .zip(right_list.iter().sorted().rev())
            .map(|(l, r)| (l - r).abs())
            .sum()
    }

    pub fn similarity_score(left_list: &[i64], right_list: &[i64]) -> i64 {
        let freqs = right_list
            .into_iter()
            .fold(BTreeMap::<i64, i64>::new(), |mut acc, num| {
                acc.entry(*num).and_modify(|x| *x += 1).or_insert(1);
                acc
            });
        left_list
            .into_iter()
            .map(|num| num * freqs.get(&num).unwrap_or(&0))
            .sum()
    }

    #[test]
    fn example() {
        let (left_list, right_list) = super::example::intermediate();
        assert_eq!(
            total_distance(&left_list, &right_list),
            super::example::output_total_distance()
        );
        assert_eq!(
            similarity_score(&left_list, &right_list),
            super::example::output_similarity_score()
        );
    }
}

#[cfg(test)]
mod example {
    pub fn input() -> &'static str {
        "3   4\n\
         4   3\n\
         2   5\n\
         1   3\n\
         3   9\n\
         3   3"
    }

    pub fn intermediate() -> (Vec<i64>, Vec<i64>) {
        (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
    }

    pub fn output_total_distance() -> i64 {
        11
    }

    pub fn output_similarity_score() -> i64 {
        31
    }
}
