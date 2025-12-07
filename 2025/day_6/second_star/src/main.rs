fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let operators = input_file
        .lines()
        .last()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .unwrap();
    let res = input_file
        .lines()
        .take(input_file.lines().count() - 1)
        .map(|l| l.split("").collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .transpose()
        .iter()
        .map(|l| l.iter().map(|s| s.to_string()).collect::<String>())
        .map(|l| l.trim().to_string())
        .reduce(|a, b| format!("{}\n{}", a, b))
        .unwrap()
        .split("\n\n")
        .map(|hw| hw.split_whitespace().map(|n| n.parse::<u64>().unwrap()))
        .enumerate()
        .map(|(i, hw)| match operators[i] {
            "+" => hw.sum::<u64>(),
            "*" => hw.product::<u64>(),
            _ => panic!("Unknown operator"),
        })
        .sum::<u64>();
    println!("{}", res);
}

trait Transpose<T> {
    fn transpose(self) -> Vec<Vec<T>>;
}

impl<T> Transpose<T> for Vec<Vec<T>> {
    fn transpose(self) -> Vec<Vec<T>> {
        if self.is_empty() {
            return vec![];
        }
        let row_len = self[0].len();
        let mut transposed = Vec::with_capacity(row_len);
        for _ in 0..row_len {
            transposed.push(Vec::with_capacity(self.len()));
        }
        for row in self {
            for (j, val) in row.into_iter().enumerate() {
                transposed[j].push(val);
            }
        }
        transposed
    }
}
