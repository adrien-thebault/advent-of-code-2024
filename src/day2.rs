use advent_of_code_2024::*;
use itertools::Itertools;

fn main() {
    timer!("total");
    let input;

    {
        timer!("prepare");
        input = String::from_utf8_lossy(include_bytes!("../inputs/day2.txt"))
            .trim()
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .filter_map(|x| x.parse().ok())
                    .collect_vec()
            })
            .collect_vec();
    }

    let is_safe = |l: &[i32]| {
        let ord = (l[0] - l[1]).is_positive();
        l.iter()
            .tuple_windows()
            .map(|(x, y)| x - y)
            .all(|d| d != 0 && d.abs() <= 3 && d.is_positive() == ord)
    };

    // part 1
    {
        timer!("part 1");
        println!("part 1 : {}", input.iter().filter(|l| is_safe(l)).count());
    }

    // part 2
    {
        timer!("part 2");
        println!(
            "part 2 : {}",
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
                .count()
        );
    }
}
