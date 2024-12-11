use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn blink(stones: &mut HashMap<usize, usize>) {
    stones
        .drain()
        .flat_map(|(n, count)| match n {
            0 => vec![(1, count)],
            n if (n.ilog10() + 1) % 2 == 0 => {
                let div = 10usize.pow(((n).ilog10() + 1) / 2);
                vec![(n / div, count), (n % div, count)]
            }
            n => vec![(n * 2024, count)],
        })
        .into_grouping_map_by(|(n, _)| *n)
        .fold(0, |acc, _, (_, count)| acc + count)
        .into_iter()
        .for_each(|(n, count)| {
            stones.insert(n, count);
        });
}

fn main() {
    let mut stones: HashMap<usize, usize> =
        String::from_utf8_lossy(include_bytes!("../inputs/day11.txt"))
            .trim()
            .split_ascii_whitespace()
            .filter_map(|n| n.parse().ok().map(|n| (n, 1)))
            .collect();

    // part 1
    let now = Instant::now();
    (0..25).for_each(|_| blink(&mut stones));
    println!(
        "part 1 : {} ({}ms)",
        stones.values().sum::<usize>(),
        now.elapsed().as_millis()
    );

    // part 2
    let now = Instant::now();
    (0..50).for_each(|_| blink(&mut stones));
    println!(
        "part 2 : {} ({}ms)",
        stones.values().sum::<usize>(),
        now.elapsed().as_millis()
    );
}
