use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day2", about = "Solver for Advent of Code 2021 day 2")]
struct Opt {
    /// Aimed mode
    #[structopt(short, long)]
    aimed: bool,

    /// Filename
    #[structopt(name = "FILE")]
    filename: String,
}

struct Position {
    x: isize,
    y: isize,
}

struct Sub {
    x: isize,
    depth: isize,
    aim: isize,
}

impl Sub {
    fn multiple(&self) -> isize {
        self.x * self.depth
    }
}

fn main() {
    let opt = Opt::from_args();
    let filename = opt.filename;

    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let moves = parse_moves(&data);

    if opt.aimed {
        let mut sub = Sub {
            x: 0,
            depth: 0,
            aim: 0,
        };
        sub_move(&mut sub, &moves);

        println!("Aimed final position: {}", sub.multiple());
    } else {
        let mut position = Position { x: 0, y: 0 };
        direct_move(&mut position, &moves);

        println!(
            "Position x:{}, y:{} - multiple:{}",
            position.x,
            position.y,
            position.x * position.y
        );
    }
}

fn parse_moves(data: &str) -> Vec<Move> {
    let data = split_data(data);
    let moves: Vec<Move> = data.iter().map(|i| parse_instruction(i)).collect();
    moves
}

fn sub_move(sub: &mut Sub, moves: &[Move]) {
    for m in moves {
        match m {
            Move::Up(n) => {
                sub.aim -= n;
            }
            Move::Down(n) => {
                sub.aim += n;
            }
            Move::Forward(n) => {
                sub.x += n;
                sub.depth += sub.aim * n;
            }
        }
    }
}

fn direct_move(position: &mut Position, moves: &[Move]) {
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
    fn direct_motion() {
        let mut position = Position { x: 0, y: 0 };
        direct_move(
            &mut position,
            &[Move::Forward(5), Move::Down(5), Move::Up(1)],
        );
        assert_eq!(position.x, 5);
        assert_eq!(position.y, 4);
    }

    #[test]
    fn aimed_motion() {
        let data = include_str!("../data/intro.txt");
        let moves = parse_moves(data);
        let mut sub = Sub {
            x: 0,
            depth: 0,
            aim: 0,
        };
        sub_move(&mut sub, &moves);
        assert_eq!(sub.multiple(), 900);
    }
}
