use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn main() {
    let now = Instant::now();
    let input = String::from_utf8_lossy(include_bytes!("../inputs/day4.txt"))
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect();

    let search_fn = |input: &HashMap<_, _>, search, dirs: &[Vec<_>], word| {
        input
            .iter()
            .filter(|&(_, &c)| c == search)
            .map(|((x, y), _)| {
                dirs.iter()
                    .filter(|offsets| {
                        offsets
                            .iter()
                            .filter_map(|(dx, dy)| input.get(&(x + dx, y + dy)))
                            .join("")
                            == word
                    })
                    .count()
            })
            .collect_vec()
    };

    println!(
        "prepare : {}.{:0>3}ms",
        now.elapsed().as_millis(),
        now.elapsed().subsec_millis()
    );

    // part 1
    let now = Instant::now();
    println!(
        "part 1 : {} ({}.{:0>3}ms)",
        search_fn(
            &input,
            'X',
            &[
                (0, -1),  // ↑
                (0, 1),   // ↓
                (-1, 0),  // ←
                (1, 0),   // →
                (-1, -1), // ↑ + ←
                (-1, 1),  // ↑ + →
                (1, -1),  // ↓ + ←
                (1, 1),   // ↓ + →
            ]
            .into_iter()
            .map(|(dx, dy)| (0..4).map(|i| (i * dx, i * dy)).collect())
            .collect_vec(),
            "XMAS",
        )
        .into_iter()
        .sum::<usize>(),
        now.elapsed().as_millis(),
        now.elapsed().subsec_millis()
    );

    // part 2
    let now = Instant::now();
    println!(
        "part 2 : {} ({}.{:0>3}ms)",
        search_fn(
            &input,
            'A',
            &[
                vec![(-1, -1), (0, 0), (1, 1)], // ↓ + →
                vec![(1, 1), (0, 0), (-1, -1)], // ↑ + ←
                vec![(-1, 1), (0, 0), (1, -1)], // ↑ + →
                vec![(1, -1), (0, 0), (-1, 1)], // ↓ + ←
            ],
            "MAS",
        )
        .into_iter()
        .filter(|r| *r == 2)
        .count(),
        now.elapsed().as_millis(),
        now.elapsed().subsec_millis()
    )
}
