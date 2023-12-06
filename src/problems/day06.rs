use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(6);
    advent_of_code::answer(1, Some(2344708), part1(&input));
    advent_of_code::answer(2, Some(30125202), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let mut product = 1;
    let times = input[0].split(' ').filter(|&x| !x.is_empty()).skip(1).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let distances = input[1].split(' ').filter(|&x| !x.is_empty()).skip(1).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    for i in 0..times.len() {
        let mut wins = 0;
        for time in 0..times[i] {
            if (times[i] - time) * time > distances[i] {
                wins += 1;
            }
        }

        product *= wins;
    }

    product
}

fn part2(input: &[String]) -> i64 {
    let time = input[0].split(' ').filter(|&x| !x.is_empty()).skip(1).collect::<Vec<_>>().join("").parse::<i64>().unwrap();
    let distance = input[1].split(' ').filter(|&x| !x.is_empty()).skip(1).collect::<Vec<_>>().join("").parse::<i64>().unwrap();

    let mut wins = 0;
    for t in 0..time {
        if (time - t) * t > distance {
            wins += 1;
        }
    }

    wins
}
