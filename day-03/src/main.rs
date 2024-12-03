use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{map_res, value},
    multi::{many1, many_till},
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Clone)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

fn parse_int(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(input)
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul(")(input)?;
    let (input, (a, b)) = separated_pair(parse_int, tag(","), parse_int)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Instruction::Mul(a, b)))
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Do, tag("do()"))(input)
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Dont, tag("don't()"))(input)
}

fn parse_all_instruction(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, alt((parse_do, parse_dont, parse_mul))).map(|(_, i)| i))(
        input,
    )
}

fn main() {
    let file_content =
        std::fs::read_to_string("src/input.txt").expect("expected file at src/input.txt");

    let (_, instructions) = parse_all_instruction(&file_content).unwrap();
    let part_a = instructions.clone().into_iter().fold(0, |acc, m| match m {
        Instruction::Mul(a, b) => acc + (a * b),
        _ => acc,
    });

    let (_, part_b) =
        instructions
            .into_iter()
            .fold((true, 0), |(enabled, acc), i| match i {
                Instruction::Mul(a, b) => {
                    if enabled {
                        (enabled, acc + (a * b))
                    } else {
                        (enabled, acc)
                    }
                }
                Instruction::Do => (true, acc),
                Instruction::Dont => (false, acc),
            });

    println!("Part a: {}", part_a);
    println!("Part b: {}", part_b);
}
