use std::{cmp::max, collections::HashSet};

#[derive(Debug)]
struct Range {
    begin: usize,
    end: usize,
}

fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let res = input_file
        .split(",")
        .map(|s| s.trim().split_once("-").unwrap())
        .map(|(f, s)| Range {
            begin: f.parse().unwrap(),
            end: s.parse().unwrap(),
        })
        .flat_map(|r| {
            let len = r.begin.to_string().len() as u32;
            let ulen = r.end.to_string().len() as u32;
            let mut res = HashSet::new();
            for len in len..=ulen {
                for k in 1..len {
                    let maxrep = len / k;
                    for rep in 2..=maxrep {
                        if !len.is_multiple_of(rep) {
                            continue;
                        }
                        let mult = (10_usize.pow(k * rep) - 1) / (10_usize.pow(k) - 1);
                        for x in (10_usize.pow(k - 1))..=(10_usize.pow(k) - 1) {
                            if x * mult >= r.begin && x * mult <= r.end {
                                res.insert(x * mult);
                            }
                        }
                    }
                }
            }
            res
        })
        .sum::<usize>();
    println!("{}", res)
}
