use advent_of_code_2024::*;
use itertools::Itertools;

fn main() {
    timer!("total");
    let (mut left, mut right): (Vec<_>, Vec<_>);

    {
        timer!("prepare");
        (left, right) = String::from_utf8_lossy(include_bytes!("../inputs/day1.txt"))
            .lines()
            .filter_map(|l| {
                l.split_whitespace()
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect_tuple()
            })
            .unzip();

        left.sort();
        right.sort();
    }

    // part 1
    {
        timer!("part 1");
        println!(
            "part 1 : {}",
            left.iter()
                .zip(&right)
                .map(|(l, r)| l.abs_diff(*r))
                .sum::<usize>()
        );
    }

    // part 2
    {
        timer!("part 2");
        println!(
            "part 2 : {}",
            left.iter()
                .filter_map(|l| {
                    right
                        .iter()
                        .position(|r| r == l)
                        .map(|i| l * right.iter().skip(i).take_while(|r| *r == l).count())
                })
                .sum::<usize>()
        );
    }
}
