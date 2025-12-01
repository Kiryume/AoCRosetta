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
            let nv = v + (c % 100);
            let over = nv <= 0 && v != 0 || nv >= 100;
            (p + (c.abs() / 100) + (over as i32), (nv + 100) % 100)
        })
        .0;
    println!("{}", res);
}
