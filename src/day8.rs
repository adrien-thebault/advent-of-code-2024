use itertools::Itertools;
use std::time::Instant;

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../inputs/day8.txt"));
    let (h, w, combinations) = (
        input.lines().count(),
        input.lines().next().unwrap().len(),
        input
            .lines()
            .enumerate()
            .flat_map(|(x, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(move |(y, c)| (c != '.').then_some((c, (x, y))))
            })
            .into_group_map()
            .into_values()
            .flat_map(|pos| pos.into_iter().combinations(2))
            .collect_vec(),
    );

    // part 1
    let now = Instant::now();
    println!(
        "part 1 : {} ({}ms)",
        combinations
            .iter()
            .flat_map(|c| {
                let ((x1, y1), (x2, y2)) = (c[0], c[1]);
                let (dx, dy) = (x2 - x1, y2 - y1);
                [(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)]
            })
            .filter(|&(x, y)| x < h && y < w)
            .unique()
            .count(),
        now.elapsed().as_millis()
    );

    // part 2
    let now = Instant::now();
    println!(
        "part 2 : {} ({}ms)",
        combinations
            .iter()
            .flat_map(|c| {
                let mut antinodes = c.to_owned();
                let ((mut x1, mut y1), (mut x2, mut y2)) = (c[0], c[1]);
                let (dx, dy) = (x2 - x1, y2 - y1);

                while (x1 - dx) < h && (y1 - dy) < w {
                    antinodes.push((x1 - dx, y1 - dy));
                    x1 -= dx;
                    y1 -= dy;
                }

                while (x2 + dx) < h && (y2 + dy) < w {
                    antinodes.push((x2 + dx, y2 + dy));
                    x2 += dx;
                    y2 += dy;
                }

                antinodes
            })
            .unique()
            .count(),
        now.elapsed().as_millis()
    );
}
