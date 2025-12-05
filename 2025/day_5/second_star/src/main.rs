#![feature(get_disjoint_mut_helpers)]
use core::slice::GetDisjointMutIndex;
use std::cmp::max;

#[derive(Debug)]
struct Ranges {
    len: usize,
    current_range: Option<(usize, usize)>,
}

fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let (ranges, _) = input_file.split_once("\n\n").unwrap();
    let mut ranges = ranges
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(s, e)| (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();
    ranges.sort_by_key(|&(s, _)| s);
    let res = ranges.iter().fold(
        Ranges {
            len: 0,
            current_range: None,
        },
        |acc, r| match acc.current_range {
            Some((sc, ec)) => {
                let range = sc..=ec;
                if range.is_overlapping(&(r.0..=r.1)) {
                    Ranges {
                        len: acc.len,
                        current_range: Some((sc, max(ec, r.1))),
                    }
                } else {
                    Ranges {
                        len: acc.len + range.count(),
                        current_range: Some(*r),
                    }
                }
            }
            None => Ranges {
                len: acc.len,
                current_range: Some(*r),
            },
        },
    );
    let res = res.len + (res.current_range.unwrap().0..=res.current_range.unwrap().1).count();

    println!("{}", res);
}
