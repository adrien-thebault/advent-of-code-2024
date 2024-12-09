use itertools::Itertools;
use std::iter;

fn main() {
    let mut fid = -1;
    let input = String::from_utf8_lossy(include_bytes!("../inputs/day9.txt"))
        .chars()
        .filter_map(|c| c.to_digit(10).and_then(|d| d.try_into().ok()))
        .enumerate()
        .flat_map(|(i, n)| {
            iter::repeat_n(
                if i % 2 == 0 {
                    fid += 1;
                    fid
                } else {
                    -1
                },
                n,
            )
        })
        .collect_vec();

    // part 1
    let mut p1 = input.clone();
    input
        .iter()
        .positions(|&n| n == -1)
        .zip(input.iter().positions(|&n| n != -1).rev())
        .take_while(|&(f, u)| f < u)
        .for_each(|(f, u)| p1.swap(f, u));

    println!(
        "part 1 : {}",
        p1.iter()
            .take_while(|&&n| n != -1)
            .enumerate()
            .map(|(i, &n)| i * n as usize)
            .sum::<usize>()
    );

    // part 2
}
