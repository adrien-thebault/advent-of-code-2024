use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::time::Instant;

fn trail(map: &[Vec<u32>], (w, h): (usize, usize), (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    if map[x][y] == 9 {
        vec![(x, y)]
    } else {
        [
            (x + 1, y), // up
            (x - 1, y), // down
            (x, y + 1), // right
            (x, y - 1), // left
        ]
        .into_iter()
        .filter(|&(x2, y2)| x2 < h && y2 < w && map[x2][y2] == map[x][y] + 1)
        .flat_map(|pos| trail(map, (w, h), pos))
        .collect_vec()
    }
}

fn main() {
    let map = String::from_utf8_lossy(include_bytes!("../inputs/day10.txt"))
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect_vec())
        .collect_vec();

    let (d, starts) = (
        (map.len(), map[0].len()),
        map.iter()
            .enumerate()
            .flat_map(|(x, l)| l.iter().positions(|&h| h == 0).map(move |y| (x, y)))
            .collect_vec(),
    );

    // part 1
    let now = Instant::now();
    println!(
        "part 1 : {} ({}ms)",
        starts
            .par_iter()
            .map(|&pos| trail(&map, d, pos).iter().unique().count())
            .sum::<usize>(),
        now.elapsed().as_millis()
    );

    // part 2
    let now = Instant::now();
    println!(
        "part 2 : {} ({}ms)",
        starts
            .par_iter()
            .map(|&pos| trail(&map, d, pos).len())
            .sum::<usize>(),
        now.elapsed().as_millis()
    );
}
