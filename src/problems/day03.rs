use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(3);
    advent_of_code::answer(1, Some(538046), part1(&input));
    advent_of_code::answer(2, Some(81709807), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let mut sum = 0;

    let (part_numbers, symbols) = parse_schematic(input);
    for part_number in part_numbers {
        let min_x = part_number.position.0 - 1;
        let min_y = part_number.position.1 - 1;
        let max_x = part_number.position.0 + part_number.length;
        let max_y = part_number.position.1 + 1;
        if symbols.iter().any(|symbol| symbol.position.0 >= min_x && symbol.position.0 <= max_x && symbol.position.1 >= min_y && symbol.position.1 <= max_y) {
            sum += part_number.value;
        }
    }

    sum
}

fn part2(input: &[String]) -> i32 {
    let mut sum = 0;

    let (part_numbers, symbols) = parse_schematic(input);
    for symbol in symbols.iter().filter(|symbol| symbol.value == '*') {
        let min_x = symbol.position.0 - 1;
        let min_y = symbol.position.1 - 1;
        let max_x = symbol.position.0 + 1;
        let max_y = symbol.position.1 + 1;
        let adjacent = part_numbers.iter().filter(|num| num.position.0 <= max_x && num.position.0 + num.length - 1 >= min_x && num.position.1 <= max_y && num.position.1 >= min_y).collect::<Vec<_>>();
        if adjacent.len() == 2 {
            sum += adjacent[0].value * adjacent[1].value;
        }
    }

    sum
}

fn parse_schematic(input: &[String]) -> (Vec<PartNumber>, Vec<Symbol>) {
    let mut numbers = vec![];
    let mut symbols = vec![];
    let mut builder = String::new();

    for y in 0..input.len() {
        let mut row = input[y].chars();
        for x in 0..input[y].len() {
            let item = row.nth(0).unwrap();
            if item.is_digit(10) {
                builder.push(item);
            } else {
                if builder.len() != 0 {
                    let length = builder.len() as i32;
                    numbers.push(PartNumber{value: builder.parse::<i32>().unwrap(), position: (x as i32 - length, y as i32), length});
                    builder.clear();
                }

                if item != '.' {
                    symbols.push(Symbol{value: item, position: (x as i32, y  as i32)});
                }
            }
        }

        if builder.len() != 0 {
            let length = builder.len() as i32;
            numbers.push(PartNumber{value: builder.parse::<i32>().unwrap(), position: (input[y].len() as i32 - length, y as i32), length});
            builder.clear();
        }
    }

    (numbers, symbols)
}

#[derive(Debug)]
struct PartNumber {
    value: i32,
    position: (i32, i32),
    length: i32,
}

#[derive(Debug)]
struct Symbol {
    value: char,
    position: (i32, i32),
}
