use fxhash::FxHashSet;

use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(11);
    advent_of_code::answer(1, Some(10231178), part1(&input));
    advent_of_code::answer(2, Some(622120986954), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let galaxies = find_galaxies(input);
    let (empty_rows, empty_columns) = find_empty_rows_and_columns(input);

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            if i == j {
                continue;
            }

            let a = galaxies[i];
            let b = galaxies[j];

            let additional_rows = empty_rows.iter().filter(|&r| (r > &a.1 && r < &b.1) || (r > &b.1 && r < &a.1)).count() as i32;
            let additional_columns = empty_columns.iter().filter(|&c| (c > &a.0 && c < &b.0) || (c > &b.0 && c < &a.0)).count() as i32;

            sum += i32::abs(a.0 - b.0) + additional_columns + i32::abs(a.1 - b.1) + additional_rows;
        }
    }

    sum
}

fn part2(input: &[String]) -> i64 {
    let galaxies = find_galaxies(input);
    let (empty_rows, empty_columns) = find_empty_rows_and_columns(input);

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            if i == j {
                continue;
            }

            let a = galaxies[i];
            let b = galaxies[j];

            let additional_rows = empty_rows.iter().filter(|&r| (r > &a.1 && r < &b.1) || (r > &b.1 && r < &a.1)).count() as i64;
            let additional_columns = empty_columns.iter().filter(|&c| (c > &a.0 && c < &b.0) || (c > &b.0 && c < &a.0)).count() as i64;

            sum += i64::abs((a.0 - b.0).into()) + additional_columns * 999999  + i64::abs((a.1 - b.1).into()) + additional_rows * 999999;
        }
    }

    sum
}

fn find_galaxies(input: &[String]) -> Vec<(i32, i32)> {
    let mut galaxies = vec![];
    for (y, line) in input.iter().enumerate() {
        galaxies.extend(line.chars().enumerate().filter(|(_, ch)| ch == &'#').map(|(x, _)| (x as i32, y as i32)));
    }
    
    galaxies
}

fn find_empty_rows_and_columns(input: &[String]) -> (Vec<i32>, Vec<i32>) {
    let mut rows = vec![];
    let mut columns = vec![FxHashSet::default(); input.len()];
    for (y, line) in input.iter().enumerate() {
        let mut row = FxHashSet::default();
        for (x, j) in line.chars().enumerate() {
            row.insert(j);
            columns[x].insert(j);
        }

        if row.len() == 1 {
            rows.push(y as i32);
        }
    }

    let columns = columns.iter().enumerate().filter(|(_, h)| h.len() == 1).map(|(x, _)| x as i32).collect::<Vec<_>>();
    (rows, columns)
}
