use std::collections::VecDeque;

fn main() {
    let (start,moves) = include_str!("input.txt")
        .split_once("\n\n").unwrap();

    let mut start = start.lines().rev();
    let numstacks = start.next().unwrap().split_whitespace().count();
//    let mut stacks:Vec<Vec<u8>> = Vec::new();
    let map:Vec<Vec<char>> = start.map(|l| l
        .chars()
        .skip(1)
        .enumerate()
        .filter_map(|(i, c)| if i % 4 == 0 { Some(c) } else { None } )
        .collect::<Vec<char>>()
    )
    .collect();

    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(numstacks);
    for s in 0..numstacks { 
        stacks.push(VecDeque::new());
    }
    for l in map {
        for (i, &c) in l.iter().enumerate() {
            if c != ' ' {
                stacks[i].push_front(c);
            }
        }
    }
    println!("{:?}", stacks);

    let moves: Vec<Vec<usize>> = moves
        .lines()
        .map(|l| l
            .split_whitespace()
            .map(|c| c.parse::<usize>())
            .filter(|c| c.is_ok())
            .map(|c| c.unwrap())
            .collect::<Vec<usize>>()
        )
        .collect();
    for m in moves {
        println!("move {} from {} to {}", m[0], m[1], m[2]);
        for c in 0..m[0] {
            let p = stacks[m[1]-1].pop_front().unwrap();
            stacks[m[2]-1].push_front(p);
        }
    }
    for i in 0..numstacks {
        println!("{}", stacks[i].pop_front().unwrap());
    }
}
