use nom::{
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

type Pair = (i64, i64);

fn parse_int(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |s: &str| s.parse::<i64>())(input)
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    let (input, (left, right)) = separated_pair(parse_int, space1, parse_int)(input)?;
    Ok((input, (left, right)))
}

fn parse_file(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list0(line_ending, parse_pair)(input)
}

fn main() {
    let file_content = std::fs::read_to_string("src/input.txt").expect("Failed to read file");

    match parse_file(&file_content) {
        Ok((_, pairs)) => {
            let mut left_list: Vec<i64> = vec![];
            let mut right_list: Vec<i64> = vec![];
            for (left, right) in pairs {
                left_list.push(left);
                right_list.push(right);
            }
            left_list.sort_unstable();
            right_list.sort_unstable();

            let result = left_list
                .iter()
                .zip(right_list.iter())
                .fold(0, |acc, (l, r)| acc + (l - r).abs());
            println!("{}", result)
        }
        Err(e) => {
            eprintln!("Failed to parse file: {:?}", e);
        }
    }
}
