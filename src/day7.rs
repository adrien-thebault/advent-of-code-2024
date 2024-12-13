use itertools::Itertools;
use std::time::Instant;

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
    let now = Instant::now();
    let input = String::from_utf8_lossy(include_bytes!("../inputs/day7.txt"))
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

    println!(
        "prepare : {}.{:0>3}ms",
        now.elapsed().as_millis(),
        now.elapsed().subsec_millis()
    );

    // part 1
    let now = Instant::now();
    println!(
        "part 1 : {} ({}.{:0>3}ms)",
        input
            .iter()
            .filter_map(|(r, v)| compute(v, &[|a, b| a + b, |a, b| a * b,])
                .iter()
                .any(|x| x == r)
                .then_some(r))
            .sum::<isize>(),
        now.elapsed().as_millis(),
        now.elapsed().subsec_millis()
    );

    // part 2
    let now = Instant::now();
    println!(
        "part 2 : {} ({}.{:0>3}ms)",
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
            .sum::<isize>(),
        now.elapsed().as_millis(),
        now.elapsed().subsec_millis()
    );
}
