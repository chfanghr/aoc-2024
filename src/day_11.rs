use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: usize,
    pub part_2: usize,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let input = parser::input
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;

    Ok(Answer {
        part_1: solution::blink_n_times(&input, 25),
        part_2: solution::blink_n_times(&input, 75),
    })
}
mod parser {
    pub fn input(input: &str) -> nom::IResult<&str, Vec<u64>> {
        nom::multi::separated_list1(
            nom::character::complete::space1,
            nom::character::complete::u64,
        )(input)
    }

    #[test]
    fn example() {
        use nom::Parser;

        assert_eq!(
            Ok(("", super::example::intermediate())),
            input.parse(super::example::input()),
        );
    }
}

mod solution {
    use std::collections::HashMap;

    fn next_nums(num: u64) -> Vec<u64> {
        let mut digits = 1;

        while num / 10u64.pow(digits) > 0 {
            digits += 1
        }

        if num == 0 {
            vec![1]
        } else if digits % 2 == 0 {
            let d = 10u64.pow(digits / 2);
            vec![num / d, num % d]
        } else {
            vec![num * 2024]
        }
    }

    fn blink_num_n_times(
        num: u64,
        memo: &mut HashMap<usize, HashMap<u64, usize>>,
        depth: usize,
    ) -> usize {
        if depth == 0 {
            return 1;
        }

        if let Some(count) = memo
            .get(&depth)
            .and_then(|memo_at_depth| memo_at_depth.get(&num))
        {
            return *count;
        }

        let count = next_nums(num)
            .into_iter()
            .map(|num| blink_num_n_times(num, memo, depth - 1))
            .sum();

        memo.entry(depth)
            .or_default()
            .entry(num)
            .insert_entry(count);

        count
    }

    pub fn blink_n_times(nums: &[u64], n: usize) -> usize {
        let mut memo = HashMap::new();
        nums.iter()
            .map(|num| blink_num_n_times(*num, &mut memo, n))
            .sum()
    }

    #[test]
    fn example() {
        assert_eq!(
            super::example::output(),
            blink_n_times(&super::example::intermediate(), 25)
        )
    }
}

#[cfg(test)]
mod example {
    pub fn input() -> &'static str {
        include_str!("./examples/day11/example.txt")
    }

    pub fn intermediate() -> Vec<u64> {
        include!("./examples/day11/intermediate.in")
    }

    pub fn output() -> usize {
        55312
    }
}
