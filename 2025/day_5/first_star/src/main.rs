fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let (ranges, ids) = input_file.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(s, e)| (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap()))
        .map(|(s, e)| s..=e)
        .collect::<Vec<_>>();
    let res = ids
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count();
    println!("{}", res);
}
