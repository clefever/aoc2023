use crate::advent_of_code;

const NUMBERS: &[&str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(1);
    advent_of_code::answer(1, Some(55029), part1(&input));
    advent_of_code::answer(2, Some(55686), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    input.iter().map(|line| {
        let nums = line.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
        format!("{}{}", nums.first().unwrap(), nums.last().unwrap()).parse::<i32>().unwrap()
    }).sum()
}

fn part2(input: &[String]) -> i32 {
    input.iter().map(|line| {
        let nums = insert_digits(line).chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
        format!("{}{}", nums.first().unwrap(), nums.last().unwrap()).parse::<i32>().unwrap()
    }).sum()
}

fn insert_digits(line: &String) -> String {
    let mut line = line.to_owned();

    NUMBERS.iter().enumerate().for_each(|(index, num)| {
        line.clone().rmatch_indices(num).for_each(|(idx, _)| {
            line.insert(idx+1, (index + 49) as u8 as char);
        })
    });

    line
}
