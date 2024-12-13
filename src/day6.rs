use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashSet, time::Instant};

fn main() {
    let map = String::from_utf8_lossy(include_bytes!("../inputs/day6.txt"))
        .trim()
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let (start_x, start_y) = (0..map.len())
        .cartesian_product(0..map[0].len())
        .find_map(|(x, y)| (map[x][y] == '^').then_some((x as isize, y as isize)))
        .expect("could not find start position");

    let solve = |map: &[Vec<_>], mut dx, mut dy, mut d, mut x, mut y| {
        let mut visited = HashSet::new();

        while let Some(&c) = map.get(x as usize).and_then(|l| l.get(y as usize)) {
            if c == '#' {
                (x, y) = (x - dx, y - dy);
                (dx, dy, d) = match (dx, dy) {
                    (-1, 0) => (0, 1, '>'),  // ^ => >
                    (0, 1) => (1, 0, 'v'),   // > => v
                    (1, 0) => (0, -1, '<'),  // v => <
                    (0, -1) => (-1, 0, '^'), // < => ^
                    _ => panic!(),
                };
            } else if !visited.insert((x, y, d)) {
                return HashSet::with_capacity(0);
            }

            (x, y) = (x + dx, y + dy);
        }

        visited
    };

    // part 1
    let now = Instant::now();
    let p1 = solve(&map, -1, 0, '^', start_x, start_y)
        .into_iter()
        .filter(|&(x, y, _)| !(x == start_x && y == start_y))
        .unique_by(|&(x, y, _)| (x, y))
        .collect_vec();
    println!(
        "part 1 : {} ({}ms)",
        p1.len() + 1,
        now.elapsed().as_millis()
    );

    // part 2
    let now = Instant::now();
    println!(
        "part 2 : {} ({}ms)",
        p1.into_par_iter()
            .filter(|&(x, y, _)| {
                let mut map = map.clone();
                map[x as usize][y as usize] = '#';
                solve(&map, -1, 0, '^', start_x, start_y).is_empty()
            })
            .count(),
        now.elapsed().as_millis()
    )
}
