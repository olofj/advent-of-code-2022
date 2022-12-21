use std::collections::HashMap;

#[derive(Debug)]
enum Monkey {
    Number(isize),
    Operation(char, String, String),
}

fn solve(str: String, monkeys: &HashMap<String, Monkey>) -> isize {
    let m = monkeys.get(&str).unwrap();
    match &m {
        Monkey::Number(x) => *x,
        Monkey::Operation(op, op1, op2) => {
            let op1: isize = solve(op1.clone(), monkeys);
            let op2: isize = solve(op2.clone(), monkeys);
            match op {
                '+' => op1 + op2,
                '*' => op1 * op2,
                '-' => op1 - op2,
                '/' => op1 / op2,
                _ => panic!("Unknown operator {}", op),
            }
        }
    }
}

fn main() {
    let monkeys = include_str!("input.txt")
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(name, job)| {
            (
                name.to_string(),
                if let Some(num) = job.parse::<isize>().ok() {
                    Monkey::Number(num)
                } else {
                    let op: Vec<&str> = job.split(" ").collect();
                    Monkey::Operation(
                        op[1].chars().next().unwrap(),
                        op[0].to_string(),
                        op[2].to_string(),
                    )
                },
            )
        })
        .collect::<HashMap<String, Monkey>>();

    let res = solve("root".to_string(), &monkeys);
    println!("root = {}", res);
}
