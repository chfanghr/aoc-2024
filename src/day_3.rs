use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: i64,
    pub part_2: i64,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let instructions = parser::input()
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;
    Ok(Answer {
        part_1: solution::sum_of_results_of_the_multiplications_ignoring_do_dont(&instructions),
        part_2: solution::sum_of_results_of_the_multiplications(&instructions),
    })
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mul(i64, i64),
    Dont,
    Do,
    Nop,
}

mod parser {
    pub type Error<'a> = nom::error::Error<&'a str>;
    pub trait Parser<'a, T> = nom::Parser<&'a str, T, Error<'a>>;

    pub fn input<'a>() -> impl Parser<'a, Vec<super::Instruction>> {
        nom::multi::many1(nom::branch::alt((
            mul(),
            do_(),
            dont(),
            nom::character::complete::anychar.map(|_| super::Instruction::Nop),
        )))
    }

    fn do_<'a>() -> impl Parser<'a, super::Instruction> {
        nom::bytes::complete::tag("do()").map(|_| super::Instruction::Do)
    }

    fn dont<'a>() -> impl Parser<'a, super::Instruction> {
        nom::bytes::complete::tag("don't()").map(|_| super::Instruction::Dont)
    }

    fn mul<'a>() -> impl Parser<'a, super::Instruction> {
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
        .map(|(l, r)| super::Instruction::Mul(l, r))
    }

    #[test]
    fn example() {
        assert_eq!(
            input().parse(super::example::input_p_1()),
            Ok(("", super::example::intermediate_p_1()))
        );
        assert_eq!(
            input().parse(super::example::input_p_2()),
            Ok(("", super::example::intermediate_p_2()))
        );
    }
}

mod solution {
    use super::Instruction;

    pub fn sum_of_results_of_the_multiplications_ignoring_do_dont(
        instructions: &[Instruction],
    ) -> i64 {
        instructions
            .iter()
            .map(|instruction| match instruction {
                Instruction::Mul(l, r) => l * r,
                _ => 0,
            })
            .sum()
    }

    pub fn sum_of_results_of_the_multiplications(instructions: &[Instruction]) -> i64 {
        instructions
            .iter()
            .scan(
                true,
                |mul_enabled: &mut bool, instruction| match instruction {
                    Instruction::Mul(l, r) => {
                        if *mul_enabled {
                            Some(l * r)
                        } else {
                            Some(0)
                        }
                    }
                    Instruction::Do => {
                        *mul_enabled = true;
                        Some(0)
                    }
                    Instruction::Dont => {
                        *mul_enabled = false;
                        Some(0)
                    }
                    _ => Some(0),
                },
            )
            .sum()
    }

    #[test]
    fn example() {
        assert_eq!(
            sum_of_results_of_the_multiplications_ignoring_do_dont(
                &super::example::intermediate_p_1()
            ),
            super::example::output_p_1()
        );
        assert_eq!(
            sum_of_results_of_the_multiplications(&super::example::intermediate_p_2()),
            super::example::output_p_2()
        );
    }
}

#[cfg(test)]
mod example {
    use super::Instruction;
    pub fn input_p_1() -> &'static str {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    }

    pub fn intermediate_p_1() -> Vec<Instruction> {
        vec![
            Instruction::Nop,
            Instruction::Mul(2, 4),
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Mul(5, 5),
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Mul(11, 8),
            Instruction::Mul(8, 5),
            Instruction::Nop,
        ]
    }

    pub fn output_p_1() -> i64 {
        161
    }

    pub fn input_p_2() -> &'static str {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    }

    pub fn intermediate_p_2() -> Vec<Instruction> {
        vec![
            Instruction::Nop,
            Instruction::Mul(2, 4),
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Dont,
            Instruction::Nop,
            Instruction::Mul(5, 5),
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Mul(11, 8),
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Do,
            Instruction::Nop,
            Instruction::Mul(8, 5),
            Instruction::Nop,
        ]
    }

    pub fn output_p_2() -> i64 {
        48
    }
}
