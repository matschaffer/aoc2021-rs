use std::{fs, mem};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day3", about = "Solver for Advent of Code 2021 day 3")]
struct Opt {
    /// Filename
    #[structopt(name = "FILE")]
    filename: String,
}

struct ParseResult {
    bit_depth: usize,
    data: Vec<usize>,
}

fn main() {
    let opt = Opt::from_args();
    let filename = opt.filename;
    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let result = parse_lines(&data);
    let gamma = find_common_bits(&result.data);
    let epsilon = reverse_bits(gamma, result.bit_depth);
    let power_consumption = gamma * epsilon;

    print!(
        "Gamma: {}, Epsilon: {}, Power consumption: {}",
        gamma, epsilon, power_consumption
    );
}

fn parse_lines(input: &str) -> ParseResult {
    let mut bit_depth = 0;
    let data = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|s| {
            if s.len() > bit_depth {
                bit_depth = s.len();
            }

            usize::from_str_radix(s, 2).unwrap()
        })
        .collect();
    ParseResult { bit_depth, data }
}

fn find_common_bits(data: &[usize]) -> usize {
    let mut bit: usize = 1;
    let mut result: usize = 0;
    for _i in 1..15 {
        let mut zeros = 0;
        let mut ones = 0;

        for datum in data {
            if datum & bit == bit {
                ones += 1;
            } else {
                zeros += 1;
            }
        }

        if zeros < ones {
            result |= bit;
        }

        bit <<= 1;
    }
    result
}

fn reverse_bits(data: usize, bit_depth: usize) -> usize {
    let omitted = (mem::size_of::<usize>() * 8) - bit_depth;
    let mut result = !(data);
    result <<= omitted;
    result >>= omitted;
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        let gamma = 0b10110;
        let result = reverse_bits(gamma, 5);
        assert_eq!(result, 0b01001);
    }

    #[test]
    fn test_find_common_bits() {
        let data = include_str!("../data/intro.txt");
        let result = parse_lines(data);
        let bits = find_common_bits(&result.data);
        assert_eq!(bits, 0b10110);
    }

    #[test]
    fn test_parse_lines() {
        let data = "00100\n11110\n";
        let result = parse_lines(data);
        assert_eq!(result.bit_depth, 5);
        assert_eq!(result.data[0], 4);
        assert_eq!(result.data[1], 30);
    }
}
