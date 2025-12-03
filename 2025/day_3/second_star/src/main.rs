fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let res = input_file
        .lines()
        .map(|bank| {
            bank.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .map(|bank| {
            let mut res: Vec<(usize, u32)> = vec![(0, 0)];
            for i in 0..12 {
                let max = bank
                    .iter()
                    .cloned()
                    .enumerate()
                    .take(bank.len() - (12 - i - 1))
                    .skip(res[res.len() - 1].0 + 1)
                    .reduce(|a, b| if a.1 >= b.1 { a } else { b })
                    .unwrap();
                res.push(max);
            }
            res.iter().fold(0_u64, |acc, &x| acc * 10 + x.1 as u64)
        })
        .sum::<u64>();
    println!("{}", res);
}
