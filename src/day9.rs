use itertools::Itertools;
use std::{iter, time::Instant};

fn main() {
    let mut fid = -1;
    let mut input = String::from_utf8_lossy(include_bytes!("../inputs/day9.txt"))
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

    let checksum = |vec: Vec<i32>| {
        vec.into_iter()
            .enumerate()
            .filter(|&(_, n)| n != -1)
            .map(|(i, n)| i * n as usize)
            .sum::<usize>()
    };

    // part 1
    let now = Instant::now();
    let mut p1 = input.clone();
    input
        .iter()
        .positions(|&n| n == -1)
        .zip(input.iter().positions(|&n| n != -1).rev())
        .take_while(|&(f, u)| f < u)
        .for_each(|(f, u)| p1.swap(f, u));

    println!(
        "part 1 : {} ({}ms)",
        checksum(p1),
        now.elapsed().as_millis()
    );

    // part 2
    let now = Instant::now();
    let (free, mut used): (Vec<_>, Vec<_>) = input
        .iter()
        .enumerate()
        .chunk_by(|(_, x)| *x)
        .into_iter()
        .map(|(_, chunk)| {
            let chunk = chunk.collect_vec();
            (chunk[0].0, chunk.len())
        })
        .partition(|&(p, _)| input[p] == -1);

    free.into_iter().for_each(|(f_start, f_len)| {
        let mut free = f_len;
        while let Some(p) = used
            .iter()
            .rev()
            .position(|&(u_start, u_len)| free > 0 && u_len <= free && u_start > f_start)
        {
            let (u_start, u_len) = used.remove(used.len() - 1 - p);
            (0..u_len).for_each(|i| input.swap(f_start + (f_len - free) + i, u_start + i));
            free -= u_len;
        }
    });

    println!(
        "part 2 : {} ({}ms)",
        checksum(input),
        now.elapsed().as_millis()
    );
}
