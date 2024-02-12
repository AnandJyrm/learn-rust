use regex::Regex;
use std::fs::read_to_string;

const MAX_SIZE: usize = 1000;
type Matrix = [[u32; MAX_SIZE]; MAX_SIZE];

fn count(cell: Box<Matrix>) -> u32 {
    let mut c: u32 = 0;
    for i in 0..MAX_SIZE {
        for j in 0..MAX_SIZE {
            c += cell[i][j];
        }
    }
    c
}

fn turn_on(cell: &mut Box<Matrix>, x_a: usize, y_a: usize, x_b: usize, y_b: usize) {
    for i in x_a..=x_b {
        for j in y_a..=y_b {
            cell[i][j] += 1;
        }
    }
}

fn turn_off(cell: &mut Box<Matrix>, x_a: usize, y_a: usize, x_b: usize, y_b: usize) {
    for i in x_a..=x_b {
        for j in y_a..=y_b {
            if cell[i][j] >= 1 {
                cell[i][j] -= 1;
            }
        }
    }
}

fn toggle(cell: &mut Box<Matrix>, x_a: usize, y_a: usize, x_b: usize, y_b: usize) {
    for i in x_a..=x_b {
        for j in y_a..=y_b {
            cell[i][j] += 2;
        }
    }
}

fn main() {
    let mut cell: Box<Matrix> = Box::new([[0; MAX_SIZE]; MAX_SIZE]);
    let input = read_to_string("src/input").unwrap();
    let re = Regex::new(r"(?<cmd>(turn on|turn off|toggle))\s(?<x_a>\d+),(?<y_a>\d+)\sthrough\s(?<x_b>\d+),(?<y_b>\d+)").unwrap();
    for entry in input.split("\n") {
        if re.is_match(entry) {
            let result = re.captures_iter(entry).next().unwrap();
            if &result["cmd"] == "turn on" {
                turn_on(
                    &mut cell,
                    result["x_a"].parse::<usize>().unwrap(),
                    result["y_a"].parse::<usize>().unwrap(),
                    result["x_b"].parse::<usize>().unwrap(),
                    result["y_b"].parse::<usize>().unwrap(),
                );
            } else if &result["cmd"] == "turn off" {
                turn_off(
                    &mut cell,
                    result["x_a"].parse::<usize>().unwrap(),
                    result["y_a"].parse::<usize>().unwrap(),
                    result["x_b"].parse::<usize>().unwrap(),
                    result["y_b"].parse::<usize>().unwrap(),
                );
            } else if &result["cmd"] == "toggle" {
                toggle(
                    &mut cell,
                    result["x_a"].parse::<usize>().unwrap(),
                    result["y_a"].parse::<usize>().unwrap(),
                    result["x_b"].parse::<usize>().unwrap(),
                    result["y_b"].parse::<usize>().unwrap(),
                );
            } else {
                continue;
            }
        }
    }
    println!("count: {}", count(cell));
}
