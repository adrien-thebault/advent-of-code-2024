use itertools::Itertools;
use std::{iter, str::FromStr};

fn main() {
    let (mut left, mut right): (Vec<_>, Vec<_>) =
        String::from_utf8_lossy(include_bytes!("../inputs/day1.txt"))
            .lines()
            .filter_map(|l| {
                l.split_whitespace()
                    .collect_tuple()
                    .and_then(|(l, r)| Some(((usize::from_str(l).ok()?), usize::from_str(r).ok()?)))
            })
            .unzip();

    left.sort();
    right.sort();

    // part 1

    println!(
        "part 1 : {}",
        iter::zip(left.iter(), right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum::<usize>()
    );

    // part 2
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
    )
}
