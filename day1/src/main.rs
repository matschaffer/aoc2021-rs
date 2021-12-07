use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day1", about = "Solver for Advent of Code 2021 day 1")]
struct Opt {
    /// Windowed mode
    #[structopt(short, long)]
    windowed: bool,

    /// Filename
    #[structopt(name = "FILE")]
    filename: String,
}

fn main() {
    let opt = Opt::from_args();
    let filename = opt.filename;

    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let data = split_data(&data);

    if opt.windowed {
        println!("Windowed Increases: {}", find_windowed_increases(3, &data));
    } else {
        println!("Increases: {}", count_increases(&data));
    }
}

fn split_data(data: &str) -> Vec<isize> {
    data.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn count_increases(data: &Vec<isize>) -> isize {
    let mut prev = data[0];
    let mut count = 0;
    for i in 1..data.len() {
        if data[i] > prev {
            count += 1;
        }
        prev = data[i];
    }
    count
}

fn find_windowed_increases(size: usize, data: &Vec<isize>) -> isize {
    let sums = data
        .windows(size)
        .map(|window| window.iter().sum())
        .collect();
    count_increases(&sums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split() {
        assert_eq!(split_data("1\n2\n3"), vec![1, 2, 3]);
    }

    #[test]
    fn intro() {
        let data = include_str!("../data/intro.txt");
        let data = split_data(data);
        assert_eq!(count_increases(&data), 7);
    }

    #[test]
    fn puzzle() {
        let data = include_str!("../data/input.txt");
        let data = split_data(data);
        assert_eq!(count_increases(&data), 1301);
    }

    #[test]
    fn windowed_intro() {
        let data = include_str!("../data/intro-windowed.txt");
        let data = split_data(data);
        assert_eq!(find_windowed_increases(3, &data), 5);
    }

    #[test]
    fn windowed_puzzle() {
        let data = include_str!("../data/input.txt");
        let data = split_data(data);
        assert_eq!(find_windowed_increases(3, &data), 1346);
    }
}
