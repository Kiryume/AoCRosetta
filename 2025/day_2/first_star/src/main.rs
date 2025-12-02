use std::cmp::max;

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
            let k = max((r.begin.to_string().len() / 2) as u32, 1);
            let uk = max((r.end.to_string().len() / 2) as u32, 1);
            let mut res = Vec::new();
            for i in (10_usize.pow(k - 1))..(10_usize.pow(uk)) {
                let k = i.to_string().len() as u32;
                let rep = i * (10_usize.pow(k) + 1);
                if rep >= r.begin && rep <= r.end {
                    res.push(rep);
                }
            }
            res
        })
        .sum::<usize>();
    println!("{}", res)
}
