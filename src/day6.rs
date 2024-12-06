use itertools::Itertools;

fn main() {
    let mut input = String::from_utf8_lossy(include_bytes!("../inputs/day6.txt"))
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let ((mut dx, mut dy), (mut x, mut y)) = (
        (-1, 0),
        input
            .iter()
            .enumerate()
            .find_map(|(x, l)| {
                l.iter()
                    .position(|&c| c == '^')
                    .map(|y| (x as isize, y as isize))
            })
            .expect("could not find start position"),
    );

    while let Some(&c) = input
        .get((x + dx) as usize)
        .and_then(|l| l.get((y + dy) as usize))
    {
        input[x as usize][y as usize] = 'X';
        if c == '#' {
            (dx, dy) = match (dx, dy) {
                (-1, 0) => (0, 1),  // ^ => >
                (0, 1) => (1, 0),   // > => v
                (1, 0) => (0, -1),  // v => <
                (0, -1) => (-1, 0), // < => ^
                _ => panic!(),
            };
        }
        (x, y) = (x + dx, y + dy);
    }

    input[x as usize][y as usize] = 'X';
    // println!("{}", input.iter().map(|l| l.iter().join("")).join("\n"));

    // part 1
    println!(
        "part 1 : {}",
        input
            .iter()
            .map(|l| l.iter().filter(|&c| *c == 'X').count())
            .sum::<usize>()
    );
}
