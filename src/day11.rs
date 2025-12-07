use advent_of_code_2024::*;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    timer!("total");
    let mut stones;

    {
        timer!("prepare");
        stones = String::from_utf8_lossy(include_bytes!("../inputs/day11.txt"))
            .trim()
            .split_ascii_whitespace()
            .filter_map(|n| n.parse().ok().map(|n| (n, 1)))
            .collect();
    }

    let blink = |stones: &mut HashMap<usize, _>| {
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
    };

    // part 1
    {
        timer!("part 1");
        (0..25).for_each(|_| blink(&mut stones));
        println!("part 1 : {}", stones.values().sum::<usize>());
    }

    // part 2
    {
        timer!("part 2");
        (0..50).for_each(|_| blink(&mut stones));
        println!("part 2 : {}", stones.values().sum::<usize>());
    }
}
