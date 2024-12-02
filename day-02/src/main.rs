use nom::{
    character::complete::{digit1, space1},
    combinator::{map, map_res},
    multi::separated_list0,
    IResult,
};
use std::fs::File;
use std::io::{self, BufRead};

type Level = i8;
struct Report(Vec<Level>);

impl Report {
    fn is_safe(&self) -> bool {
        let levels = &self.0;
        if levels.len() < 2 {
            return false;
        }

        let is_increasing = match levels[..] {
            [first, second, ..] if first < second => true,
            [first, second, ..] if first > second => false,
            _ => return false,
        };

        levels.windows(2).all(|l| {
            let [a, b] = l else { panic!() };
            let diff = (a - b).abs();
            let is_valid = if is_increasing { a < b } else { a > b };
            is_valid && diff <= 3 && diff > 0
        })
    }
}

fn parse_report(input: &str) -> IResult<&str, Report> {
    map(separated_list0(space1, map_res(digit1, str::parse)), Report)(input)
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        if let Ok((_, report)) = parse_report(&line) {
            if report.is_safe() {
                count += 1;
            }
        }
    }

    println!("Number of valid reports: {}", count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strictly_increasing() {
        let report = Report(vec![1, 2, 3, 4, 5]);
        assert!(report.is_safe());
    }

    #[test]
    fn test_strictly_decreasing() {
        let report = Report(vec![5, 4, 3, 2, 1]);
        assert!(report.is_safe());
    }

    #[test]
    fn test_exceeding_max_diff() {
        let report = Report(vec![1, 2, 7, 8, 9]);
        assert!(!report.is_safe());
    }

    #[test]
    fn test_mixed_mode_decreasing_to_increasing() {
        let report = Report(vec![9, 8, 7, 2, 1]);
        assert!(!report.is_safe());
    }

    #[test]
    fn test_mixed_mode_increasing_to_decreasing() {
        let report = Report(vec![1, 2, 3, 2, 1]);
        assert!(!report.is_safe());
    }

    #[test]
    fn test_duplicate_values() {
        let report = Report(vec![1, 2, 3, 3, 4]);
        assert!(!report.is_safe());
    }
}
