#![feature(strip_circumfix)]

use z3::ast::Int;
use z3::{Optimize, SatResult};

fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let res = input_file
        .lines()
        .map(Machine::new)
        .map(|m| m.solve())
        .sum::<i64>();
    println!("{}", res);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Machine {
    lght: Vec<bool>,
    swts: Vec<Vec<usize>>,
    jlts: Vec<i64>,
}

impl Machine {
    fn new(base: &str) -> Self {
        let mut parts = base.split(" ");
        let count = parts.clone().count();
        let lght = {
            parts
                .next()
                .unwrap()
                .strip_circumfix("[", "]")
                .unwrap()
                .chars()
                .map(|c| c == '#')
                .collect::<Vec<_>>()
        };
        let swts = {
            parts
                .clone()
                .take(count - 2)
                .map(|sw| sw.strip_circumfix("(", ")").unwrap().split(","))
                .map(|sw| sw.map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        };
        let jlts = {
            parts
                .nth(count - 2)
                .unwrap()
                .strip_circumfix("{", "}")
                .unwrap()
                .split(",")
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .collect::<Vec<_>>()
        };
        Machine { lght, swts, jlts }
    }

    fn solve(&self) -> i64 {
        let opt = Optimize::new();

        let swt_vars = (0..self.swts.len())
            .map(|i| Int::new_const(format!("btn_{}", i)))
            .collect::<Vec<_>>();

        let zero = Int::from_i64(0);
        for swt in &swt_vars {
            opt.assert(&swt.ge(&zero));
        }

        for (idx, &tval) in self.jlts.iter().enumerate() {
            let expr = self
                .swts
                .iter()
                .enumerate()
                .filter(|(_, affected_counters)| affected_counters.contains(&idx))
                .map(|(btn_idx, _)| &swt_vars[btn_idx])
                .cloned()
                .reduce(|acc, x| acc + x)
                .unwrap();

            let target = Int::from_i64(tval);
            opt.assert(&expr.eq(&target));
        }

        let total_presses = swt_vars.iter().cloned().reduce(|acc, x| acc + x).unwrap();

        opt.minimize(&total_presses);

        if opt.check(&[]) == SatResult::Sat {
            let model = opt.get_model().unwrap();
            let result = model.eval(&total_presses, true).unwrap();
            return result.as_i64().unwrap();
        }

        panic!("No solution found")
    }
}
