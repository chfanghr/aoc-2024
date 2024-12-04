mod parser {
    pub type Error<'a> = nom::error::Error<&'a str>;
    pub trait Parser<'a, T> = nom::Parser<&'a str, T, Error<'a>>;

    static ACCEPTED_CHARS: [char; 4] = ['X', 'M', 'A', 'S'];

    pub fn input<'a>() -> impl Parser<'a, Vec<Vec<char>>> {
        nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::multi::many1(nom::character::complete::one_of(ACCEPTED_CHARS.as_slice())),
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

#[cfg(test)]
mod example {
    pub fn input() -> &'static str {
        include_str!("./examples/day4/input.txt")
    }

    pub fn intermediate() -> Vec<Vec<char>> {
        include!("./examples/day4/intermediate.in")
            .into_iter()
            .map(|l| -> Vec<_> { l.into() })
            .collect()
    }
}
