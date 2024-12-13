use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: u64,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let input = parser::input
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;

    Ok(Answer {
        part_1: solution::total_tokens_needed(&input),
    })
}

#[derive(Debug, PartialEq, Eq)]
struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

#[derive(Debug, PartialEq, Eq)]
struct Button {
    x_offset: i128,
    y_offset: i128,
}

#[derive(Debug, PartialEq, Eq)]
struct Prize {
    x: i128,
    y: i128,
}

mod parser {
    use nom::Parser;

    use super::{Button, ClawMachine, Prize};

    pub fn input(input: &str) -> nom::IResult<&str, Vec<ClawMachine>> {
        nom::multi::separated_list1(
            nom::multi::many1(nom::character::complete::newline),
            claw_machine,
        )
        .parse(input)
    }

    fn claw_machine(input: &str) -> nom::IResult<&str, ClawMachine> {
        let (input, button_a) = labeled_button('A').parse(input)?;
        let (input, _) = nom::character::complete::newline(input)?;
        let (input, button_b) = labeled_button('B').parse(input)?;
        let (input, _) = nom::character::complete::newline(input)?;
        let (input, prize) = prize.parse(input)?;
        Ok((
            input,
            ClawMachine {
                button_a,
                button_b,
                prize,
            },
        ))
    }

    fn prize(input: &str) -> nom::IResult<&str, Prize> {
        nom::sequence::preceded(
            nom::bytes::complete::tag("Prize: "),
            nom::sequence::separated_pair(
                labeled_i128("X="),
                nom::bytes::complete::tag(", "),
                labeled_i128("Y="),
            ),
        )
        .map(|(x, y)| Prize { x, y })
        .parse(input)
    }

    fn labeled_button(label: char) -> impl for<'a> Fn(&'a str) -> nom::IResult<&'a str, Button> {
        move |input: &str| {
            nom::sequence::preceded(
                nom::bytes::complete::tag(AsRef::<str>::as_ref(&format!("Button {label}: "))),
                nom::sequence::separated_pair(
                    labeled_i128("X+"),
                    nom::bytes::complete::tag(", "),
                    labeled_i128("Y+"),
                ),
            )
            .map(|(x_offset, y_offset)| Button { x_offset, y_offset })
            .parse(input)
        }
    }

    fn labeled_i128(label: &str) -> impl for<'a> Fn(&'a str) -> nom::IResult<&'a str, i128> {
        let label = label.to_owned();
        move |input: &str| {
            nom::sequence::preceded(
                nom::bytes::complete::tag(AsRef::<str>::as_ref(&format!("{label}"))),
                nom::character::complete::i128,
            )
            .parse(input)
        }
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
    use rational::Rational;

    use super::ClawMachine;

    fn full_div(n: i128, d: i128) -> Option<i128> {
        let r = Rational::new(n, d);
        (r.denominator() == 1).then_some(r.numerator())
    }

    fn check_and_convert(t: i128) -> Option<u8> {
        (0..=100).contains(&t).then(|| u8::try_from(t).unwrap())
    }

    fn press_buttons(m: &ClawMachine) -> Option<(u8, u8)> {
        let ClawMachine {
            button_a,
            button_b,
            prize,
        } = m;

        // b = (Y_A * T_X - X_A * T_Y) / (Y_A * X_B - X_A * Y_B)
        let b = full_div(
            button_a.y_offset * prize.x - button_a.x_offset * prize.y,
            button_a.y_offset * button_b.x_offset - button_a.x_offset * button_b.y_offset,
        )?;

        // a = (T_X - X_B * b) / X_A
        let a = full_div(prize.x - button_b.x_offset * b, button_a.x_offset)?;

        check_and_convert(a).zip(check_and_convert(b))
    }

    fn tokens_needed(m: &ClawMachine) -> Option<u64> {
        let (a, b) = press_buttons(m)?;
        Some(u64::from(a) * 3 + u64::from(b) * 1)
    }

    pub fn total_tokens_needed(ms: &[ClawMachine]) -> u64 {
        ms.iter().filter_map(tokens_needed).sum()
    }

    #[test]
    fn example() {
        assert_eq!(
            super::example::output(),
            total_tokens_needed(&super::example::intermediate())
        );
    }
}

#[cfg(test)]
mod example {
    use super::{Button, ClawMachine, Prize};

    pub fn input() -> &'static str {
        include_str!("./examples/day13/example.txt")
    }

    pub fn intermediate() -> Vec<ClawMachine> {
        include!("./examples/day13/intermediate.in")
    }

    pub fn output() -> u64 {
        480
    }
}
