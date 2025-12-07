fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let operators = input_file
        .lines()
        .last()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .unwrap();
    let res = transpose(
        &input_file
            .lines()
            .take(input_file.lines().count() - 1)
            .map(|l| l.split("").collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
    .iter()
    .map(|l| l.iter().map(|s| s.to_string()).collect::<String>())
    .map(|l| l.trim().to_string())
    .reduce(|a, b| format!("{}\n{}", a, b))
    .unwrap()
    .split("\n\n")
    .map(|hw| {
        hw.split_whitespace()
            .map(|n| n.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    })
    .enumerate()
    .map(|(i, hw)| match operators[i] {
        "+" => hw.iter().sum::<u64>(),
        "*" => hw.iter().product::<u64>(),
        _ => panic!("Unknown operator"),
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
