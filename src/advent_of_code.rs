use std::fs;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn read_input(day: i32) -> String {
    fs::read_to_string(format!("src/input/day{:02}_input.txt", day))
        .expect("input file should be a valid file")
}

pub fn read_input_lines(day: i32) -> Vec<String> {
    let contents = fs::read_to_string(format!("src/input/day{:02}_input.txt", day))
        .expect("input file should be a valid file");

    contents.lines().map(str::to_string).collect()
}

pub fn answer<T>(part: i32, correct: Option<T>, calculated: T)
where
    T: std::cmp::PartialEq,
    T: std::fmt::Display,
{
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    if correct == None {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
            .ok();
    } else if calculated == correct.unwrap() {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
            .ok();
    } else {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
            .ok();
    }

    println!("Part {} answer {}", part, calculated);

    stdout.reset().ok();
}
