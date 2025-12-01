use std::collections::{HashMap, HashSet};
#[derive(Debug, Clone, Copy)]
enum OpType {
    And,
    Or,
    Xor,
}
#[derive(Debug, Clone)]
struct Operation {
    op_type: OpType,
    input1: String,
    input2: String,
    output: String,
}
fn topological_sort(operations: &[Operation], variables: &HashMap<String, u8>) -> Vec<Operation> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for op in operations {
        graph.entry(op.input1.clone()).or_default();
        graph.entry(op.input2.clone()).or_default();
        graph.entry(op.output.clone()).or_default();
        graph.get_mut(&op.input1).unwrap().insert(op.output.clone());
        graph.get_mut(&op.input2).unwrap().insert(op.output.clone());
        *in_degree.entry(op.output.clone()).or_insert(0) += 2;
    }
    for var in variables.keys() {
        if !in_degree.contains_key(var) {
            in_degree.insert(var.clone(), 0);
        }
    }
    let mut queue: Vec<String> = in_degree
        .iter()
        .filter_map(|(k, &v)| if v == 0 { Some(k.clone()) } else { None })
        .collect();
    let mut sorted = Vec::new();
    while let Some(node) = queue.pop() {
        for op in operations.iter().filter(|op| op.output == node) {
            sorted.push(op.clone());
        }
        for neighbor in graph.get(&node).unwrap_or(&HashSet::new()).clone() {
            if let Some(count) = in_degree.get_mut(&neighbor) {
                *count -= 1;
                if *count == 0 {
                    queue.push(neighbor);
                }
            }
        }
    }
    sorted
}

fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let (variables, operations) = input_file.split_once("\n\n").unwrap();
    let mut variables = variables
        .lines()
        .map(|line| {
            let (var, val) = line.split_once(": ").unwrap();
            (var.to_string(), val.parse().unwrap())
        })
        .collect();
    let operations = operations
        .lines()
        .map(|line| {
            let (expr, output) = line.split_once(" -> ").unwrap();
            let parts = expr.split(' ').collect::<Vec<_>>();
            let (input1, op_str, input2) = (parts[0], parts[1], parts[2]);
            let op_type = match op_str {
                "AND" => OpType::And,
                "OR" => OpType::Or,
                "XOR" => OpType::Xor,
                _ => unreachable!(),
            };
            Operation {
                op_type,
                input1: input1.to_string(),
                input2: input2.to_string(),
                output: output.to_string(),
            }
        })
        .collect::<Vec<_>>();
    let sorted_ops = topological_sort(&operations, &variables);
    for op in sorted_ops {
        let val1 = variables.get(&op.input1).unwrap();
        let val2 = variables.get(&op.input2).unwrap();
        let result = match op.op_type {
            OpType::And => val1 & val2,
            OpType::Or => val1 | val2,
            OpType::Xor => val1 ^ val2,
        };
        variables.insert(op.output.clone(), result);
    }
    let mut zs = variables
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .collect::<Vec<_>>();
    zs.sort_by_key(|(k, _)| &k[1..]);
    let res = zs
        .iter()
        .rev()
        .fold(0u64, |acc, (_, v)| acc * 2 + **v as u64);
    println!("{}", res);
}
