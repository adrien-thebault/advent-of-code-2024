use itertools::Itertools;
use std::{cmp::Ordering, str::FromStr};

fn main() {
    if let Some((rules, updates)) = String::from_utf8_lossy(include_bytes!("../inputs/day5.txt"))
        .trim()
        .split_once("\n\n")
    {
        let rules = rules
            .lines()
            .filter_map(|l| {
                l.split('|')
                    .filter_map(|n| usize::from_str(n).ok())
                    .next_tuple()
            })
            .collect_vec();

        let (ordered, unordered): (Vec<_>, Vec<_>) = updates
            .lines()
            .map(|l| {
                l.split(',')
                    .filter_map(|n| usize::from_str(n).ok())
                    .collect_vec()
            })
            .partition(|u| u.is_sorted_by(|a, b| !rules.contains(&(*b, *a))));

        // part 1
        println!(
            "part 1 : {}",
            ordered.iter().map(|u| u[u.len() / 2]).sum::<usize>()
        );

        // part 2
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