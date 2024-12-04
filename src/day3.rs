use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../inputs/day3.txt"));
    let (mul_regex, cond_regex) = (
        Regex::new("mul\\((\\d+),(\\d+)\\)").expect("invalid regex"),
        Regex::new("(do|don't)\\(\\)").expect("invalid regex"),
    );

    let fn_mul = |i: &str| {
        mul_regex
            .captures_iter(i)
            .filter_map(|c| {
                c.iter()
                    .skip(1)
                    .filter_map(|opt| opt.and_then(|x| x.as_str().parse::<i32>().ok()))
                    .next_tuple()
                    .map(|(l, r)| l * r)
            })
            .sum::<i32>()
    };

    // part 1
    println!("part 1 : {}", fn_mul(&input));

    // part 2
    let mut op = true;
    println!(
        "part 2 : {}",
        cond_regex
            .replace_all(&input, "\n$1\n")
            .lines()
            .map(|l| match l {
                "do" if !op => {
                    op = true;
                    0
                }
                "don't" if op => {
                    op = false;
                    0
                }
                l if op => fn_mul(l),
                _ => 0,
            })
            .sum::<i32>()
    );
}
