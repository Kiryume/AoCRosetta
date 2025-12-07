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
        .map(|c| c == 'S')
        .collect::<Vec<_>>();
    let mut splits = 0;
    for row in splitters {
        let mut new_beams = beams.clone();
        for splitter in row {
            if beams[splitter] {
                new_beams[splitter] = false;
                new_beams[splitter - 1] = true;
                new_beams[splitter + 1] = true;
                splits += 1;
            }
        }
        beams = new_beams;
    }
    println!("{}", splits);
}
