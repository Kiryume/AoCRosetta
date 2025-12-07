fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let splitters = input_file
        .lines()
        .map(|l| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '^' => Some(x),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut beams = input_file
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| (c == 'S', (c == 'S') as usize))
        .collect::<Vec<_>>();
    for row in splitters {
        let mut new_beams = beams.clone();
        for splitter in row {
            if let (true, paths) = beams[splitter] {
                new_beams[splitter] = (false, 0);
                new_beams[splitter - 1].0 = true;
                new_beams[splitter - 1].1 += paths;
                new_beams[splitter + 1].0 = true;
                new_beams[splitter + 1].1 += paths;
            }
        }
        beams = new_beams;
    }
    println!("{}", beams.iter().map(|(_, paths)| paths).sum::<usize>());
}
