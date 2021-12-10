use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day2", about = "Solver for Advent of Code 2021 day 2")]
struct Opt {
    /// Filename
    #[structopt(name = "FILE")]
    filename: String,
}

struct Position {
    x: isize,
    y: isize,
}

fn main() {
    let opt = Opt::from_args();
    let filename = opt.filename;

    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let data = split_data(&data);
    let moves: Vec<Move> = data.iter().map(|i| parse_instruction(i)).collect();

    let mut position = Position { x: 0, y: 0 };

    move_robot(&mut position, &moves);

    println!(
        "Position x:{}, y:{} - multiple:{}",
        position.x,
        position.y,
        position.x * position.y
    );
}

fn move_robot(position: &mut Position, moves: &[Move]) {
    for m in moves {
        match m {
            Move::Forward(n) => {
                position.x += n;
            }
            Move::Down(n) => {
                position.y += n;
            }
            Move::Up(n) => {
                position.y -= n;
            }
        }
    }
}

fn split_data(data: &str) -> Vec<String> {
    data.split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect()
}
fn parse_instruction(instruction: &str) -> Move {
    let (direction, steps) = instruction.split_at(instruction.find(' ').unwrap());
    let steps = steps.trim().parse().unwrap();
    match direction {
        "forward" => Move::Forward(steps),
        "down" => Move::Down(steps),
        "up" => Move::Up(steps),
        _ => panic!("Unknown direction {}", direction),
    }
}

#[derive(Debug, PartialEq)]
enum Move {
    Forward(isize),
    Down(isize),
    Up(isize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split() {
        assert_eq!(
            split_data("forward 5\ndown 5\n"),
            vec!["forward 5", "down 5"]
        );
    }

    #[test]
    fn parse() {
        assert_eq!(parse_instruction("forward 5"), Move::Forward(5));
        assert_eq!(parse_instruction("down 5"), Move::Down(5));
        assert_eq!(parse_instruction("up 5"), Move::Up(5));
    }

    #[test]
    fn moving() {
        let mut position = Position { x: 0, y: 0 };
        move_robot(
            &mut position,
            &vec![Move::Forward(5), Move::Down(5), Move::Up(1)],
        );
        assert_eq!(position.x, 5);
        assert_eq!(position.y, 4);
    }
}
