use itertools::Itertools;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = String::from_utf8_lossy(include_bytes!("../inputs/day2.txt"))
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect_vec()
        })
        .collect_vec();

    let is_safe = |l: &[i32]| {
        let ord = (l[0] - l[1]).is_positive();
        l.iter()
            .tuple_windows()
            .map(|(x, y)| x - y)
            .all(|d| d != 0 && d.abs() <= 3 && d.is_positive() == ord)
    };

    println!("prepare : {}µs", now.elapsed().as_micros());

    // part 1
    let now = Instant::now();
    println!(
        "part 1 : {} ({}µs)",
        input.iter().filter(|l| is_safe(l)).count(),
        now.elapsed().as_micros()
    );

    // part 2
    let now = Instant::now();
    println!(
        "part 2 : {} ({}µs)",
        input
            .iter()
            .filter(|l| {
                is_safe(l)
                    || (0..l.len()).any(|i| {
                        is_safe(
                            &l.iter()
                                .enumerate()
                                .filter_map(|(j, x)| (i != j).then_some(*x))
                                .collect_vec(),
                        )
                    })
            })
            .count(),
        now.elapsed().as_micros()
    )
}
