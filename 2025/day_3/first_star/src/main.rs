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
            let mut deca = 0;
            let mut inx = 0;
            for (i, &bat) in bank.iter().enumerate().take(bank.len() - 1) {
                if bat > deca {
                    deca = bat;
                    inx = i;
                }
            }
            let mut single = 0;
            for &bat in bank.iter().skip(inx + 1) {
                if bat > single {
                    single = bat;
                }
            }
            deca * 10 + single
        })
        .sum::<u32>();
    println!("{}", res);
}
