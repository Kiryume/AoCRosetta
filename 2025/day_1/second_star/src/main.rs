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
        .fold((0, 50), |(mut p, v), c| {
            let nc = c % 100;
            let nv = v + nc;
            if (nv < 0 && v != 0) || (nv == 0 && c % 100 != 0) || nv >= 100 {
                p += 1
            }
            (p + c.abs() / 100, (v + nc + 100) % 100)
        })
        .0;
    println!("{}", res);
}
