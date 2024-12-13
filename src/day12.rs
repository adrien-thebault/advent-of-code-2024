use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

struct Region {
    _plant: char,
    points: Vec<(usize, usize)>,
}

impl Region {
    fn area(&self) -> usize {
        self.points.len()
    }

    fn fences(&self) -> Vec<(isize, isize)> {
        self.points
            .iter()
            .map(|&(x, y)| ((x as isize) * 10, (y as isize) * 10))
            .flat_map(|(x, y)| {
                [
                    (x - 5, y), // top side
                    (x, y - 5), // left side
                    (x, y + 5), // right side
                    (x + 5, y), // bottom side
                ]
            })
            .counts()
            .into_iter()
            .filter(|&(_, count)| count == 1)
            .map(|(fence, _)| fence)
            .collect()
    }

    fn part1(&self) -> usize {
        self.fences().len()
    }

    fn part2(&self) -> usize {
        // TODO
        0
    }
}

#[allow(clippy::filter_map_bool_then)]
fn main() {
    let now = Instant::now();
    let regions = String::from_utf8_lossy(include_bytes!("../inputs/day12.txt"))
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(i, l)| l.char_indices().map(move |(j, c)| (c, (i, j))))
        .into_group_map_by(|(p, _)| *p)
        .into_iter()
        .flat_map(|(plant, points)| {
            let mut remaining = points.iter().map(|(_, p)| *p).collect::<HashSet<_>>();
            points.into_iter().filter_map(move |(_, start)| {
                remaining.contains(&start).then(|| {
                    let mut stack = vec![start];
                    let mut region = vec![];

                    while let Some((i, j)) = stack.pop() {
                        if remaining.remove(&(i, j)) {
                            region.push((i, j));

                            stack.extend_from_slice(&[
                                (i + 1, j),
                                (i - 1, j),
                                (i, j + 1),
                                (i, j - 1),
                            ])
                        }
                    }

                    Region {
                        _plant: plant,
                        points: region,
                    }
                })
            })
        })
        .collect_vec();

    println!(
        "prepare : {}.{:0>3}ms",
        now.elapsed().as_millis(),
        now.elapsed().subsec_millis()
    );

    // part 1
    let now = Instant::now();

    println!(
        "part 1 : {} ({}.{:0>3}ms)",
        regions.iter().map(|r| r.area() * r.part1()).sum::<usize>(),
        now.elapsed().as_millis(),
        now.elapsed().subsec_millis()
    );

    // part 2
    let now = Instant::now();

    println!(
        "part 2 : {} ({}.{:0>3}ms)",
        regions.iter().map(|r| r.area() * r.part2()).sum::<usize>(),
        now.elapsed().as_millis(),
        now.elapsed().subsec_millis()
    );
}
