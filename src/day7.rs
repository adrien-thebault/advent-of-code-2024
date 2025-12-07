use advent_of_code_2024::*;
use itertools::Itertools;

fn compute(v: &[isize], ops: &[fn(isize, isize) -> isize]) -> Vec<isize> {
    match v.len() {
        0 | 1 => v.to_vec(),
        _ => compute(&v[1..], ops)
            .into_iter()
            .flat_map(|r| ops.iter().map(move |op| op(v[0], r)))
            .collect_vec(),
    }
}

fn main() {
    timer!("total");
    let input;

    {
        timer!("prepare");
        input = String::from_utf8_lossy(include_bytes!("../inputs/day7.txt"))
            .lines()
            .filter_map(|l| {
                l.split_once(": ").and_then(|(r, v)| {
                    r.parse()
                        .map(|r| {
                            (
                                r,
                                v.split(' ')
                                    .rev()
                                    .filter_map(|x| x.parse().ok())
                                    .collect_vec(),
                            )
                        })
                        .ok()
                })
            })
            .collect_vec();
    }

    // part 1

    {
        timer!("part 1");
        println!(
            "part 1 : {}",
            input
                .iter()
                .filter_map(|(r, v)| compute(v, &[|a, b| a + b, |a, b| a * b,])
                    .iter()
                    .any(|x| x == r)
                    .then_some(r))
                .sum::<isize>()
        );
    }

    // part 2

    {
        timer!("part 2");
        println!(
            "part 2 : {}",
            input
                .iter()
                .filter_map(|(r, v)| compute(
                    v,
                    &[
                        |a, b| a + b,
                        |a, b| a * b,
                        |a, b| b * 10isize.pow(a.ilog10() + 1) + a,
                    ]
                )
                .iter()
                .any(|x| x == r)
                .then_some(r))
                .sum::<isize>()
        );
    }
}
