fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let tiles = input_file
        .lines()
        .map(|l| l.split(",").map(|n| n.parse::<u64>().unwrap()))
        .map(|mut i| (i.next().unwrap(), i.next().unwrap()))
        .collect::<Vec<_>>();
    let mut edges = tiles
        .windows(2)
        .map(|w| Edge { p1: w[0], p2: w[1] })
        .collect::<Vec<_>>();
    edges.push(Edge {
        p1: tiles[tiles.len() - 1],
        p2: tiles[0],
    });
    let mut largest = 0;
    for (t1, t2) in tiles
        .iter()
        .enumerate()
        .flat_map(|(i, &t)| tiles.iter().skip(i + 1).map(move |&o| (t, o)))
    {
        let area = (t1.0.abs_diff(t2.0) + 1) * (t1.1.abs_diff(t2.1) + 1);
        if area > largest && contained(t1, t2, &edges) {
            largest = area;
        }
    }
    println!("{}", largest);
}

fn contained(p1: (u64, u64), p2: (u64, u64), edges: &[Edge]) -> bool {
    let (minx, maxx) = (p1.0.min(p2.0), p1.0.max(p2.0));
    let (miny, maxy) = (p1.1.min(p2.1), p1.1.max(p2.1));
    for edge in edges {
        let ((eminx, eminy), (emaxx, emaxy)) = edge.sorted();
        if emaxx > minx && eminx < maxx && emaxy > miny && eminy < maxy {
            return false;
        }
    }
    true
}

struct Edge {
    p1: (u64, u64),
    p2: (u64, u64),
}

impl Edge {
    fn sorted(&self) -> ((u64, u64), (u64, u64)) {
        let (min_x, max_x) = (self.p1.0.min(self.p2.0), self.p1.0.max(self.p2.0));
        let (min_y, max_y) = (self.p1.1.min(self.p2.1), self.p1.1.max(self.p2.1));
        ((min_x, min_y), (max_x, max_y))
    }
}
