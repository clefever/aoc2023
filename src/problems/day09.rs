use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(9);
    advent_of_code::answer(1, Some(1834108701), part1(&input));
    advent_of_code::answer(2, Some(993), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let lines = input.iter().map(|l| l.split(' ').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut sum = 0;
    for line in lines {
        let mut steps = vec![line.clone()];
        while !steps.last().unwrap().iter().all(|&s| s == 0) {
            let curr_step = steps.last().unwrap();
            let mut new_step = vec![];
            for i in 0..curr_step.len()-1 {
                new_step.push(curr_step[i+1] - curr_step[i]);
            }
            steps.push(new_step);
        }

        let mut new_val = 0;
        for i in (0..steps.len()).rev() {
            steps[i].push(new_val);
            if i > 0 {
                new_val += steps[i-1].last().unwrap();
            }
        }

        sum += steps.first().unwrap().last().unwrap();
    }
    
    sum
}

fn part2(input: &[String]) -> i32 {
    let lines = input.iter().map(|l| l.split(' ').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut sum = 0;
    for line in lines {
        let mut steps = vec![line.clone()];
        while !steps.last().unwrap().iter().all(|&s| s == 0) {
            let curr_step = steps.last().unwrap();
            let mut new_step = vec![];
            for i in 0..curr_step.len()-1 {
                new_step.push(curr_step[i+1] - curr_step[i]);
            }
            steps.push(new_step);
        }

        let mut new_val = 0;
        for i in (0..steps.len()).rev() {
            steps[i].insert(0, new_val);
            if i > 0 {
                new_val = steps[i-1].first().unwrap() - new_val;
            }
        }

        sum += steps.first().unwrap().first().unwrap();
    }
    
    sum
}
