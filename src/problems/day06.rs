use itertools::Itertools;

use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(6);
    advent_of_code::answer(1, Some(2344708), part1(&input));
    advent_of_code::answer(2, Some(30125202), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let (times, distances) = input.iter().map(|line| line.split(' ').filter(|&x| !x.is_empty()).skip(1).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect_tuple().unwrap();
    (0..times.len()).map(|time| (0..times[time]).filter(|&t| (times[time] - t) * t > distances[time]).count() as i32).reduce(|a, b| a * b).unwrap()
}

fn part2(input: &[String]) -> i64 {
    let (time, distance) = input.iter().map(|line| line.split(' ').filter(|&x| !x.is_empty()).skip(1).collect::<Vec<_>>().join("").parse::<i64>().unwrap()).collect_tuple().unwrap();
    (0..time).filter(|&t| (time - t) * t > distance).count() as i64
}
