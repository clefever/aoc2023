use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(4);
    advent_of_code::answer(1, Some(21568), part1(&input));
    advent_of_code::answer(2, Some(11827296), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let mut sum = 0;
    for line in input {
        let (winning, numbers) = parse_numbers(line);
        let matching = numbers.iter().filter(|&x| winning.contains(x)).count() as u32;
        sum += match matching {
            0 => 0,
            _ => i32::pow(2, matching-1)
        }
    }

    sum
}

fn part2(input: &[String]) -> i32 {
    let mut cards = vec![1; input.len()];
    let mut current_card = 1;
    for line in input {
        let (winning, numbers) = parse_numbers(line);
        let result = numbers.iter().filter(|&x| winning.contains(x)).count();
        for i in current_card..current_card+result {
            cards[i] += cards[current_card-1];
        }

        current_card += 1;
    }

    cards.iter().sum()
}

fn parse_numbers(line: &String) -> (Vec<i32>, Vec<i32>) {
    let split = line.split('|').collect::<Vec<_>>();
    let left = split[0].split(':').collect::<Vec<_>>();
    let winning = left[1].split(' ').filter(|&x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let numbers = split[1].split(' ').filter(|&x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    (winning, numbers)
}
