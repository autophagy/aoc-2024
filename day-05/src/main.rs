use nom::{
    character::complete::{char, digit1, line_ending, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Rule {
    before: i32,
    after: i32,
}

type Update = Vec<i32>;

fn parse_int(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (before, after)) = separated_pair(parse_int, char('|'), parse_int)(input)?;
    Ok((input, Rule { before, after }))
}

fn parse_update(input: &str) -> IResult<&str, Update> {
    separated_list1(char(','), parse_int)(input)
}

fn parse_file(input: &str) -> IResult<&str, (Vec<Rule>, Vec<Update>)> {
    separated_pair(
        separated_list1(line_ending, parse_rule),
        pair(newline, newline),
        separated_list1(line_ending, parse_update),
    )(input)
}

fn main() {
    let file_content = std::fs::read_to_string("src/input.txt").expect("Failed to read file");
    let (_, (rules, updates)) = parse_file(&file_content).unwrap();

    let predecessors: HashMap<i32, HashSet<i32>> =
        rules.iter().fold(HashMap::new(), |mut map, rule| {
            map.entry(rule.after).or_default().insert(rule.before);
            map
        });

    let mut a = 0;
    let mut b = 0;
    for mut update in updates {
        let idx = update.len() / 2;
        if update.is_sorted_by(|a, b| predecessors[b].contains(a)) {
            a += update[idx];
        } else {
            update.sort_by(|a, b| predecessors[b].contains(a).cmp(&true));
            b += update[idx];
        }
    }

    println!("Part a: {}", a);
    println!("Part b: {}", b);
}
