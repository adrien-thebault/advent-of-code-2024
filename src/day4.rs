use itertools::Itertools;
use std::collections::HashMap;

fn main() {
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

    // part 1
    println!(
        "part 1 : {}",
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
        .sum::<usize>()
    );

    // part 2
    println!(
        "part 2 : {}",
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
        .count()
    )
}
