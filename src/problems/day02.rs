use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(2);
    advent_of_code::answer(1, Some(2449), part1(&input));
    advent_of_code::answer(2, Some(63981), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    input.iter().map(|line| {
        let split = line.split(':').filter(|&x| !x.is_empty()).collect::<Vec<_>>();
        if is_game_possible(&split[1].to_string()) {
            let game = split[0].split(' ').collect::<Vec<_>>();
            return game[1].parse::<i32>().unwrap();
        }

        0
    }).sum()
}

fn part2(input: &[String]) -> i32 {
    input.iter().map(|line| {
        let split = line.split(':').filter(|&x| !x.is_empty()).collect::<Vec<_>>();
        game_power(&split[1].to_string())
    }).sum()
}

fn is_game_possible(line: &String) -> bool {
    let draws = line.split(';').collect::<Vec<_>>();
    for draw in draws {
        let colors = draw.split(',').collect::<Vec<_>>();
        for color in colors {
            let info = color.split(' ').filter(|&x| !x.is_empty()).collect::<Vec<_>>();
            let amt = info[0].parse::<i32>().unwrap();
            if info[1] == "red" && amt > 12 || info[1] == "green" && amt > 13 || info[1] == "blue" && amt > 14 {
                return false;
            }
        }
    }

    true
}

fn game_power(line: &String) -> i32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

    let draws = line.split(';').collect::<Vec<_>>();
    for draw in draws {
        let colors = draw.split(',').collect::<Vec<_>>();
        for color in colors {
            let info = color.split(' ').filter(|&x| !x.is_empty()).collect::<Vec<_>>();
            let amt = info[0].parse::<i32>().unwrap();
            if info[1] == "red" && amt > min_red {
                min_red = amt;
            }
            if info[1] == "green" && amt > min_green {
                min_green = amt;
            }
            if info[1] == "blue" && amt > min_blue {
                min_blue = amt;
            }
        }
    }

    min_red * min_green * min_blue
}
