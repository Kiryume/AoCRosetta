fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let grid = input_file
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut res = 0;
    for (x, y, c) in grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, item)| (x, y, item)))
    {
        if *c != '@' {
            continue;
        }
        let mut neigh = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0
                    && ny >= 0
                    && (ny as usize) < grid.len()
                    && (nx as usize) < grid[ny as usize].len()
                    && grid[ny as usize][nx as usize] == '@'
                {
                    neigh += 1;
                }
            }
        }
        if neigh < 4 {
            res += 1;
        }
    }
    println!("{}", res);
}
