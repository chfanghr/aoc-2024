use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: i64,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let input = parser::input()
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;
    Ok(Answer {
        part_1: solution::sum_of_middle_page_numbers_of_valid_updates(&input),
    })
}

#[derive(Debug, PartialEq, Eq)]
struct Input {
    page_ordering_rules: Vec<(i64, i64)>,
    updates: Vec<Vec<i64>>,
}

mod parser {
    use super::Input;

    pub type Error<'a> = nom::error::Error<&'a str>;
    pub trait Parser<'a, T> = nom::Parser<&'a str, T, Error<'a>>;

    pub fn input<'a>() -> impl Parser<'a, Input> {
        nom::sequence::separated_pair(
            page_ordering_rules(),
            nom::multi::many1(nom::character::complete::newline),
            updates(),
        )
        .map(|(page_ordering_rules, updates)| Input {
            page_ordering_rules,
            updates,
        })
    }

    fn page_ordering_rules<'a>() -> impl Parser<'a, Vec<(i64, i64)>> {
        nom::multi::separated_list1(nom::character::complete::newline, page_ordering_rule())
    }

    fn page_ordering_rule<'a>() -> impl Parser<'a, (i64, i64)> {
        nom::sequence::separated_pair(
            nom::character::complete::i64,
            nom::character::complete::char('|'),
            nom::character::complete::i64,
        )
    }

    fn updates<'a>() -> impl Parser<'a, Vec<Vec<i64>>> {
        nom::multi::separated_list1(nom::character::complete::newline, update())
    }

    fn update<'a>() -> impl Parser<'a, Vec<i64>> {
        nom::multi::separated_list1(
            nom::character::complete::char(','),
            nom::character::complete::i64,
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
    use std::collections::{BTreeMap, BTreeSet};

    use super::Input;

    fn make_disallowed_in_suffix_map(
        page_ordering_rules: &[(i64, i64)],
    ) -> BTreeMap<i64, BTreeSet<i64>> {
        page_ordering_rules
            .iter()
            .fold(BTreeMap::<i64, BTreeSet<i64>>::new(), |mut acc, (l, r)| {
                acc.entry(*r).or_default().insert(*l);
                acc
            })
    }

    fn is_valid_update(
        disallowed_in_suffix_map: &BTreeMap<i64, BTreeSet<i64>>,
        update: &[i64],
    ) -> bool {
        let mut all_disallowed = BTreeSet::<i64>::new();

        for page in update {
            if all_disallowed.contains(page) {
                return false;
            }
            if let Some(disallowed) = disallowed_in_suffix_map.get(page) {
                all_disallowed.append(&mut disallowed.clone());
            }
        }

        return true;
    }

    fn middle_page_number(update: &[i64]) -> i64 {
        update[update.len() / 2]
    }

    pub fn sum_of_middle_page_numbers_of_valid_updates(input: &Input) -> i64 {
        let disallowed_in_suffix_map = make_disallowed_in_suffix_map(&input.page_ordering_rules);
        input
            .updates
            .iter()
            .filter_map(|update| {
                is_valid_update(&disallowed_in_suffix_map, update)
                    .then_some(middle_page_number(update))
            })
            .sum()
    }

    #[test]
    fn example_is_valid_update() {
        let input = super::example::intermediate();
        let disallowed_in_suffix_map = make_disallowed_in_suffix_map(&input.page_ordering_rules);
        let check_update = |idx: usize, expect_valid: bool| {
            let is_valid = is_valid_update(&disallowed_in_suffix_map, &input.updates[idx]);
            assert_eq!(is_valid, expect_valid, "idx = {idx}")
        };
        check_update(0, true);
        check_update(1, true);
        check_update(2, true);
        check_update(3, false);
        check_update(4, false);
        check_update(5, false);
    }

    #[test]
    fn example() {
        assert_eq!(
            super::example::output(),
            sum_of_middle_page_numbers_of_valid_updates(&super::example::intermediate())
        )
    }
}

#[cfg(test)]
mod example {
    use super::Input;

    pub fn input() -> &'static str {
        include_str!("./examples/day5/example.txt")
    }

    pub fn intermediate() -> Input {
        include!("./examples/day5/intermediate.in")
    }

    pub fn output() -> i64 {
        143
    }
}
