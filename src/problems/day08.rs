use num_integer::lcm;

use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(8);
    advent_of_code::answer(1, Some(18727), part1(&input));
    advent_of_code::answer(2, Some(18024643846273), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let instructions = input[0].chars().collect::<Vec<_>>();
    let nodes = input[2..].to_owned();
    let mut current_node = "AAA";
    let mut steps = 0;
    let mut ip = 0;
    while current_node != "ZZZ" {
        if ip >= instructions.len() {
            ip = 0;
        }

        let instruction = instructions[ip];
        
        let blah = nodes.iter().find(|&n| n[..3].to_string() == current_node).unwrap();
        let blah2 = blah.split([',', '(', ')']).collect::<Vec<_>>();

        if instruction == 'L' {
            current_node = blah2[1];
        } else {
            current_node = blah2[2].trim();
        }

        ip += 1;
        steps += 1;
    }

    steps
}

fn part2(input: &[String]) -> i64 {
    let instructions = input[0].chars().collect::<Vec<_>>();
    let nodes = input[2..].to_owned();
    let initial_nodes = nodes.iter().filter(|&n| n.chars().nth(2) == Some('A')).map(|n| &n[..3]).collect::<Vec<_>>(); 
    let mut step_list = vec![];

    for node in initial_nodes {
        let mut steps = 0;
        let mut ip = 0;
        let mut current_node = node;
        while current_node.chars().nth(2) != Some('Z') {
            if ip >= instructions.len() {
                ip = 0;
            }

            let instruction = instructions[ip];
        
            let blah = nodes.iter().find(|&n| n[..3].to_string() == current_node).unwrap();
            let blah2 = blah.split([',', '(', ')']).collect::<Vec<_>>();

            if instruction == 'L' {
                current_node = blah2[1];
            } else {
                current_node = blah2[2].trim();
            }

            ip += 1;
            steps += 1;
        }

        step_list.push(steps)
    }

    step_list.into_iter().reduce(|a, b| lcm(a, b)).unwrap()
}
