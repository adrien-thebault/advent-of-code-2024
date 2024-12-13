use itertools::Itertools;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let (mut left, mut right): (Vec<_>, Vec<_>) =
        String::from_utf8_lossy(include_bytes!("../inputs/day1.txt"))
            .lines()
            .filter_map(|l| {
                l.split_whitespace()
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect_tuple()
            })
            .unzip();

    left.sort();
    right.sort();

    println!("prepare : {}µs", now.elapsed().as_micros());

    // part 1
    let now = Instant::now();
    println!(
        "part 1 : {} ({}µs)",
        left.iter()
            .zip(&right)
            .map(|(l, r)| l.abs_diff(*r))
            .sum::<usize>(),
        now.elapsed().as_micros()
    );

    // part 2
    let now = Instant::now();
    println!(
        "part 2 : {} ({}µs)",
        left.iter()
            .filter_map(|l| {
                right
                    .iter()
                    .position(|r| r == l)
                    .map(|i| l * right.iter().skip(i).take_while(|r| *r == l).count())
            })
            .sum::<usize>(),
        now.elapsed().as_micros()
    )
}
