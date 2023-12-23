use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(3);
    let (part_numbers, symbols) = parse_schematic(&input);
    advent_of_code::answer(1, Some(538046), part1(&part_numbers, &symbols));
    advent_of_code::answer(2, Some(81709807), part2(&part_numbers, &symbols));
}

fn part1(part_numbers: &Vec<PartNumber>, symbols: &Vec<Symbol>) -> i32 {
    part_numbers.iter().filter(|part_number| {
        let min_x = part_number.position.0 - 1;
        let min_y = part_number.position.1 - 1;
        let max_x = part_number.position.0 + part_number.length;
        let max_y = part_number.position.1 + 1;
        symbols.iter().any(|symbol| symbol.position.0 >= min_x && symbol.position.0 <= max_x && symbol.position.1 >= min_y && symbol.position.1 <= max_y)
    }).map(|part_number| part_number.value).sum()
}

fn part2(part_numbers: &Vec<PartNumber>, symbols: &Vec<Symbol>) -> i32 {
    let mut sum = 0;

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
            let item = row.next().unwrap();
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

struct PartNumber {
    value: i32,
    position: (i32, i32),
    length: i32,
}

struct Symbol {
    value: char,
    position: (i32, i32),
}
