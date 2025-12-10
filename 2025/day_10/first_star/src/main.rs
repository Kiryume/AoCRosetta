#![feature(strip_circumfix)]

use std::collections::{HashSet, VecDeque};
fn main() {
    let input = "../input.txt";
    let input_file = std::fs::read_to_string(input).expect("Error reading input file");
    let res = input_file
        .lines()
        .map(Machine::new)
        .map(|m| m.solve())
        .sum::<usize>();
    println!("{}", res);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Machine {
    lght: Vec<bool>,
    swts: Vec<Vec<usize>>,
    jlts: Vec<usize>,
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
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect::<Vec<_>>()
        };
        Machine { lght, swts, jlts }
    }

    fn solve(&self) -> usize {
        let mut visited: HashSet<Vec<bool>> = HashSet::new();
        // (index, state, steps)
        let mut queue: VecDeque<(usize, Vec<bool>, usize)> = VecDeque::new();
        self.swts.iter().enumerate().for_each(|(i, _)| {
            queue.push_back((i, vec![false; self.lght.len()], 0));
        });
        while let Some((idx, state, steps)) = queue.pop_front() {
            let mut new_state = state.clone();
            Self::toggle(&mut new_state, &self.swts[idx]);
            if new_state == self.lght {
                return steps + 1;
            }
            if visited.insert(new_state.clone()) {
                self.swts.iter().enumerate().for_each(|(i, _)| {
                    queue.push_back((i, new_state.clone(), steps + 1));
                });
            }
        }
        panic!("No solution found");
    }

    fn toggle(state: &mut [bool], switches: &[usize]) {
        for &sw in switches.iter() {
            state[sw] = !state[sw];
        }
    }
}
