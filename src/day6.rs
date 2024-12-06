use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::{HashMap, HashSet};

fn main() {
    let map = String::from_utf8_lossy(include_bytes!("../inputs/day6.txt"))
        .trim()
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let (visited, (x, y)): (HashMap<(isize, isize), HashSet<char>>, _) = (
        (0..map.len() as isize)
            .flat_map(|x| (0..map[0].len() as isize).map(move |y| ((x, y), HashSet::new())))
            .collect(),
        (map.iter()
            .enumerate()
            .find_map(|(x, l)| {
                l.iter()
                    .position(|&c| c == '^')
                    .map(|y| (x as isize, y as isize))
            })
            .expect("could not find start position")),
    );

    let solve = |mut map: Vec<Vec<_>>,
                 mut dx,
                 mut dy,
                 mut d,
                 mut x,
                 mut y,
                 mut visited: HashMap<_, HashSet<_>>| {
        while let Some(&c) = map
            .get((x + dx) as usize)
            .and_then(|l| l.get((y + dy) as usize))
        {
            map[x as usize][y as usize] = 'X';
            if !visited.get_mut(&(x, y)).unwrap().insert(d) {
                return 0;
            }

            if c == '#' {
                (dx, dy, d) = match (dx, dy) {
                    (-1, 0) => (0, 1, '>'),  // ^ => >
                    (0, 1) => (1, 0, 'v'),   // > => v
                    (1, 0) => (0, -1, '<'),  // v => <
                    (0, -1) => (-1, 0, '^'), // < => ^
                    _ => panic!(),
                };
            }

            (x, y) = (x + dx, y + dy);
        }

        map[x as usize][y as usize] = 'X';
        map.iter()
            .map(|l| l.iter().filter(|&c| *c == 'X').count())
            .sum::<usize>()
    };

    // part 1
    println!(
        "part 1 : {}",
        solve(map.clone(), -1, 0, '^', x, y, visited.clone())
    );

    // part 2
    println!(
        "part 2 : {}",
        (0..map.len())
            .flat_map(|x| (0..map[0].len()).map(move |y| (x, y)))
            .filter(|&(x, y)| map[x][y] == '.')
            .collect_vec()
            .into_par_iter()
            .filter_map(|(rx, ry)| {
                let mut new = map.clone();
                new[rx][ry] = '#';
                (solve(new, -1, 0, '^', x, y, visited.clone()) == 0).then_some(1)
            })
            .sum::<usize>()
    )
}
