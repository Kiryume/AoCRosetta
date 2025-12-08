fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let coords = input_file
        .lines()
        .map(|s| s.split(",").map(|n| n.parse::<f64>().unwrap()))
        .map(|mut l| (l.next().unwrap(), l.next().unwrap(), l.next().unwrap()))
        .collect::<Vec<_>>();
    let mut dists = coords
        .iter()
        .enumerate()
        .flat_map(|(i, &p1)| {
            coords.iter().enumerate().filter_map(move |(j, &p2)| {
                if i < j {
                    Some(Dist::new(p1, p2, i, j))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();
    dists.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut pars = (0..coords.len()).collect::<Vec<_>>();
    let mut sizes = vec![1; coords.len()];
    for d in dists.iter().take(1000) {
        let mut p1 = d.p1;
        while pars[p1] != p1 {
            p1 = pars[p1];
        }
        let mut p2 = d.p2;
        while pars[p2] != p2 {
            p2 = pars[p2];
        }
        if p1 != p2 {
            if sizes[p1] < sizes[p2] {
                pars[p1] = p2;
                sizes[p2] += sizes[p1];
            } else {
                pars[p2] = p1;
                sizes[p1] += sizes[p2];
            }
        }
    }
    let mut sizes = sizes
        .into_iter()
        .enumerate()
        .filter_map(|(i, s)| if pars[i] == i { Some(s) } else { None })
        .collect::<Vec<_>>();
    sizes.sort();
    sizes.reverse();
    println!("{}", sizes.iter().take(3).product::<usize>());
}

#[derive(PartialEq)]
struct Dist {
    p1: usize,
    p2: usize,
    dist: f64,
}

impl Dist {
    fn new(p1: (f64, f64, f64), p2: (f64, f64, f64), idx1: usize, idx2: usize) -> Self {
        let dist = ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2) + (p1.2 - p2.2).powi(2)).sqrt();
        Dist {
            p1: idx1,
            p2: idx2,
            dist,
        }
    }
}

impl PartialOrd for Dist {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}
