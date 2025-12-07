fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let hws = input_file
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let hws = transpose(&hws);
    let hws = hws.into_iter();
    let res = hws
        .map(|hw| {
            let mut hw = hw.into_iter().rev();
            match hw.next().unwrap() {
                "+" => hw.map(|n| n.parse::<u64>().unwrap()).sum::<u64>(),
                "*" => hw.map(|n| n.parse::<u64>().unwrap()).product::<u64>(),
                _ => panic!("Unknown operator"),
            }
        })
        .sum::<u64>();
    println!("{}", res);
}

fn transpose<'a>(matrix: &'a Vec<Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    let mut transposed = vec![vec![""; matrix.len()]; matrix[0].len()];
    for i in 0..matrix.len() {
        (0..matrix[0].len()).for_each(|j| {
            transposed[j][i] = matrix[i][j];
        });
    }
    transposed
}
