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

    println!(
        "prepare : {}.{:0>3}ms",
        now.elapsed().as_millis(),
        now.elapsed().subsec_millis()
    );

    // part 1
    let now = Instant::now();
    println!(
        "part 1 : {} ({}.{:0>3}ms)",
        left.iter()
            .zip(&right)
            .map(|(l, r)| l.abs_diff(*r))
            .sum::<usize>(),
        now.elapsed().as_millis(),
        now.elapsed().subsec_millis()
    );

    // part 2
    let now = Instant::now();
    println!(
        "part 2 : {} ({}.{:0>3}ms)",
        left.iter()
            .filter_map(|l| {
                right
                    .iter()
                    .position(|r| r == l)
                    .map(|i| l * right.iter().skip(i).take_while(|r| *r == l).count())
            })
            .sum::<usize>(),
        now.elapsed().as_millis(),
        now.elapsed().subsec_millis()
    )
}
