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
    let res = count_paths(&res, "svr", "out", (false, false), &mut memo);
    println!("{}", res);
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    (dac, fft): (bool, bool),
    memo: &mut HashMap<(String, (bool, bool)), u64>,
) -> u64 {
    if current == target {
        if fft && dac {
            return 1;
        } else {
            return 0;
        }
    }

    if let Some(&count) = memo.get(&(current.to_string(), (dac, fft))) {
        return count;
    }

    let mut total_paths = 0;

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            let mut dac = dac;
            let mut fft = fft;
            if neighbor == "dac" {
                dac = true;
            }
            if neighbor == "fft" {
                fft = true;
            }
            total_paths += count_paths(graph, neighbor, target, (dac, fft), memo);
        }
    }

    memo.insert((current.to_string(), (dac, fft)), total_paths);

    total_paths
}
