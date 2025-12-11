use std::collections::HashMap;

fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let res: HashMap<String, Vec<String>> = input_file
        .lines()
        .map(|line| {
            let (src, ts) = line.split_once(": ").unwrap();
            let key = src.to_string();
            let values = ts.split(" ").map(|s| s.to_string()).collect::<Vec<_>>();
            (key, values)
        })
        .collect();
    let mut memo = HashMap::new();
    let res = count_paths(&res, "you", "out", &mut memo);
    println!("{}", res);
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if current == target {
        return 1;
    }

    if let Some(&count) = memo.get(current) {
        return count;
    }

    let mut total_paths = 0;

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            total_paths += count_paths(graph, neighbor, target, memo);
        }
    }

    memo.insert(current.to_string(), total_paths);

    total_paths
}
