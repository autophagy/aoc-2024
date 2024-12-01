use nom::{
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

type Pair<T> = (T, T);

fn parse_int(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |s: &str| s.parse::<i64>())(input)
}

fn parse_pair(input: &str) -> IResult<&str, Pair<i64>> {
    let (input, (left, right)) = separated_pair(parse_int, space1, parse_int)(input)?;
    Ok((input, (left, right)))
}

fn parse_file(input: &str) -> IResult<&str, Pair<Vec<i64>>> {
    let (input, pairs) = separated_list0(line_ending, parse_pair)(input)?;
    Ok((input, pairs.into_iter().unzip()))
}

fn main() {
    let file_content = std::fs::read_to_string("src/input.txt").expect("Failed to read file");

    match parse_file(&file_content) {
        Ok((_, (mut left, mut right))) => {
            left.sort_unstable();
            right.sort_unstable();

            let result = left
                .iter()
                .zip(right.iter())
                .fold(0, |acc, (l, r)| acc + (l - r).abs());
            println!("{}", result)
        }
        Err(e) => {
            eprintln!("Failed to parse file: {:?}", e);
        }
    }
}
