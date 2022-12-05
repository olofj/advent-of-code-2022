use std::collections::VecDeque;

fn main() {
    let (start, moves) = include_str!("input.txt").split_once("\n\n").unwrap();

    let mut start = start.lines().rev();
    let mut stacks: Vec<VecDeque<char>> = start
        .next()
        .unwrap()
        .split_whitespace()
        .map(|_| VecDeque::<char>::new())
        .collect();

    start
        .map(|l| {
            l.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .collect::<Vec<(usize, char)>>()
        })
        .flatten()
        .for_each(|(i, c)| stacks[i].push_front(c));

    moves
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|c| c.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .map(|m| (m[0], m[1] - 1, m[2] - 1))
        .for_each(|(num, from, to)| {
            let m: Vec<char> = (0..num)
                .map(|_| stacks[from].pop_front().unwrap())
                .collect();
            m.iter().rev().for_each(|c| stacks[to].push_front(*c));
        });

    let out: String = stacks.iter().map(|s| s[0]).collect();

    println!("{}", out);
}
