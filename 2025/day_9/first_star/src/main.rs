fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let tiles = input_file
        .lines()
        .map(|l| l.split(",").map(|n| n.parse::<u64>().unwrap()))
        .map(|mut i| (i.next().unwrap(), i.next().unwrap()))
        .collect::<Vec<_>>();
    let mut largest = 0;
    for (t1, t2) in tiles
        .iter()
        .enumerate()
        .flat_map(|(i, &t)| tiles.iter().skip(i + 1).map(move |&o| (t, o)))
    {
        let area = (t1.0.abs_diff(t2.0) + 1) * (t1.1.abs_diff(t2.1) + 1);
        if area > largest {
            largest = area;
        }
    }
    println!("{}", largest);
}
