fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let res = input_file
        .lines()
        .skip_while(|line| !line.contains('x'))
        .map(|base| {
            base.split_once(": ")
                .map(|(size, presents)| {
                    let area = size
                        .split('x')
                        .map(|d| d.parse::<u16>().unwrap())
                        .product::<u16>();
                    let garea = presents
                        .split(' ')
                        .map(|p| p.parse::<u16>().unwrap() * 9)
                        .sum::<u16>();
                    (area, garea)
                })
                .unwrap()
        })
        .filter(|&(area, garea)| garea <= area)
        .count();

    println!("{}", res);
}
