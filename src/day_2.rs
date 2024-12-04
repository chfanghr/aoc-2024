use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: usize,
    pub part_2: usize,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let reports = parser::input()
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;
    Ok(Answer {
        part_1: solution::number_of_safe_reports_p1(&reports),
        part_2: solution::number_of_safe_reports_p2(&reports),
    })
}
mod parser {
    pub type Error<'a> = nom::error::Error<&'a str>;
    pub trait Parser<'a, T> = nom::Parser<&'a str, T, Error<'a>>;

    pub fn input<'a>() -> impl Parser<'a, Vec<Vec<i64>>> {
        nom::multi::separated_list1(nom::character::complete::newline, line::<'a>())
    }

    fn line<'a>() -> impl Parser<'a, Vec<i64>> {
        nom::multi::separated_list0(
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
    fn is_safe_1(report: &Vec<i64>) -> bool {
        (report.iter().is_sorted() || report.iter().rev().is_sorted())
            && report.iter().zip(report.iter().skip(1)).all(|(l, r)| {
                let diff = (l - r).abs();
                (1..=3i64).contains(&diff)
            })
    }

    fn is_safe_2(report: &Vec<i64>) -> bool {
        for idx in 0..report.len() {
            let mut report = report.clone();
            report.remove(idx);
            if is_safe_1(&report) {
                return true;
            }
        }
        false
    }

    pub fn number_of_safe_reports_p1(reports: &Vec<Vec<i64>>) -> usize {
        reports.iter().filter(|report| is_safe_1(*report)).count()
    }

    pub fn number_of_safe_reports_p2(reports: &Vec<Vec<i64>>) -> usize {
        reports.iter().filter(|report| is_safe_2(*report)).count()
    }

    #[test]
    fn example() {
        assert_eq!(
            number_of_safe_reports_p1(&super::example::intermediate()),
            super::example::output_number_of_safe_reports_p1()
        );
        assert_eq!(
            number_of_safe_reports_p2(&super::example::intermediate()),
            super::example::output_number_of_safe_reports_p2()
        );
    }

    #[test]
    fn edge_cases_p2() {
        assert!(is_safe_2(&vec![11, 9, 6, 2, 5]));
        assert!(is_safe_2(&vec![86, 86, 89, 91, 94, 96, 98]));
        assert!(is_safe_2(&vec![41, 45, 48, 50, 52, 55, 58]));
    }
}

#[cfg(test)]
mod example {
    pub fn input() -> &'static str {
        "7 6 4 2 1\n\
         1 2 7 8 9\n\
         9 7 6 2 1\n\
         1 3 2 4 5\n\
         8 6 4 4 1\n\
         1 3 6 7 9"
    }

    pub fn intermediate() -> Vec<Vec<i64>> {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }

    pub fn output_number_of_safe_reports_p1() -> usize {
        2
    }

    pub fn output_number_of_safe_reports_p2() -> usize {
        4
    }
}
