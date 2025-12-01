fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let res = input_file
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .map(|x| (x[0], x.iter().skip(1).collect::<String>()))
        .map(|(d, v)| {
            if d == 'L' {
                -v.parse::<i32>().unwrap()
            } else {
                v.parse::<i32>().unwrap()
            }
        })
        .fold((0, 50), |(p, v), c| {
            if (v + c) % 100 == 0 {
                return (p + 1, (v + c) % 100);
            }
            (p, (v + c) % 100)
        })
        .0;
    println!("{}", res);
}
