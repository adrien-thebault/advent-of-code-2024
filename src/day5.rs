use advent_of_code_2024::*;
use itertools::Itertools;
use std::{cmp::Ordering, str::FromStr};

fn main() {
    timer!("total");
    let (rules, ordered, unordered): (_, Vec<_>, Vec<_>);

    {
        timer!("prepare");

        let input = String::from_utf8_lossy(include_bytes!("../inputs/day5.txt"));
        let (parsed, updates) = input.trim().split_once("\n\n").unwrap();

        rules = parsed
            .lines()
            .filter_map(|l| {
                l.split('|')
                    .filter_map(|n| usize::from_str(n).ok())
                    .next_tuple()
            })
            .collect_vec();

        (ordered, unordered) = updates
            .lines()
            .map(|l| {
                l.split(',')
                    .filter_map(|n| usize::from_str(n).ok())
                    .collect_vec()
            })
            .partition(|u| u.is_sorted_by(|a, b| !rules.contains(&(*b, *a))));
    }

    // part 1
    {
        timer!("part 1");
        println!(
            "part 1 : {}",
            ordered.iter().map(|u| u[u.len() / 2]).sum::<usize>()
        );
    }

    // part 2
    {
        timer!("part 2");
        println!(
            "part 2 : {}",
            unordered
                .into_iter()
                .map(|u| {
                    let sorted = u
                        .into_iter()
                        .sorted_by(|&a, &b| {
                            if rules.contains(&(a, b)) {
                                Ordering::Less
                            } else if rules.contains(&(b, a)) {
                                Ordering::Greater
                            } else {
                                Ordering::Equal
                            }
                        })
                        .collect_vec();
                    sorted[sorted.len() / 2]
                })
                .sum::<usize>()
        );
    }
}
