use fxhash::FxHashMap;

use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(10);
    let tiles = get_loop_tiles(&input);
    advent_of_code::answer(1, Some(6923), part1(&tiles));
    advent_of_code::answer(2, Some(529), part2(&tiles, &input));
}

fn part1(tiles: &Vec<(i32, i32)>) -> i32 {
    (tiles.len() / 2) as i32
}

fn part2(tiles: &Vec<(i32, i32)>, lines: &[String]) -> i32 {
    let mut count = 0;
    for y in 0..lines.len() {
        let row = lines[y].chars().collect::<Vec<_>>();
        for x in 0..row.len() {
            if tiles.contains(&(x as i32, y as i32)) {
                continue;
            }

            let mut intersections = 0;
            for j in x..row.len() {
                if tiles.contains(&(j as i32, y as i32)) && (row[j] == '|' || row[j] == 'J' || row[j] == 'L' || row[j] == 'S')  {
                    intersections += 1;
                }
            }

            if intersections % 2 == 1 {
                count += 1;
            }
        }
    }
    
    count
}

fn get_loop_tiles(lines: &[String]) -> Vec<(i32, i32)> {
    let mut tiles: FxHashMap<(i32, i32), char> = FxHashMap::default();
    let mut start = (0, 0);
    for y in 0..lines.len() {
        let row = lines[y].chars().collect::<Vec<_>>();
        for x in 0..row.len() {
            tiles.insert((x as i32, y as i32), row[x]);
            if row[x] == 'S' {
                start = (x as i32, y as i32);
            }
        }
    }

    let mut curr = start;
    let mut visited = vec![start];

    loop {
        if let Some(next) = find_next_tiles(&tiles, &visited, curr) {
            curr = next;
            visited.push(curr);
        } else {
            break;
        }
    }

    visited
}

fn find_next_tiles(tiles: &FxHashMap<(i32, i32), char>, visited: &Vec<(i32, i32)>, position: (i32, i32)) -> Option<(i32, i32)> {
    let up = tiles.get(&(position.0, position.1 - 1));
    let right = tiles.get(&(position.0 + 1, position.1));
    let down = tiles.get(&(position.0, position.1 + 1));
    let left = tiles.get(&(position.0 - 1, position.1));

    let curr = tiles.get(&position);

    if !visited.contains(&(position.0, position.1 - 1)) && (curr == Some(&'S') || curr == Some(&'|') || curr == Some(&'L') || curr == Some(&'J')) && (up == Some(&'F') || up == Some(&'|') || up == Some(&'7')) {
        return Some((position.0, position.1 - 1));
    }

    if !visited.contains(&(position.0 + 1, position.1)) && (curr == Some(&'S') || curr == Some(&'-') || curr == Some(&'L') || curr == Some(&'F')) && (right == Some(&'7') || right == Some(&'-') || right == Some(&'J')) {
        return Some((position.0 + 1, position.1));
    }

    if !visited.contains(&(position.0, position.1 + 1)) && (curr == Some(&'S') || curr == Some(&'|') || curr == Some(&'F') || curr == Some(&'7')) && (down == Some(&'L') || down == Some(&'|') || down == Some(&'J')) {
        return Some((position.0, position.1 + 1));
    }

    if !visited.contains(&(position.0 - 1, position.1)) && (curr == Some(&'S') || curr == Some(&'-') || curr == Some(&'7') || curr == Some(&'J')) && (left == Some(&'F') || left == Some(&'-') || left == Some(&'L')) {
        return Some((position.0 - 1, position.1));
    }

    None
}
