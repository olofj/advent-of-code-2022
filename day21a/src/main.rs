use std::collections::HashMap;
/*
name: <number> or <operation>
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
*/

#[derive(Debug, Clone)]
enum MonkeyJob {
    Number(usize),
    Operation(char, String, String),
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    job: MonkeyJob,
}

fn solve(str: String, monkeys: &HashMap<String,Monkey>) -> usize {
    let m = monkeys.get(&str).unwrap();
    match &m.job {
        MonkeyJob::Number(x) => *x,
        MonkeyJob::Operation(op, op1, op2) => {
            let op1: usize = solve(op1.clone(), monkeys);
            let op2: usize = solve(op2.clone(), monkeys);
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
    let mut monkeys = include_str!("input.txt")
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(name, job)| (name.to_string(), Monkey {
            name: name.to_string(),
            job: if let Some(num) = job.parse::<usize>().ok() {
                MonkeyJob::Number(num)
            } else {
                let op: Vec<&str> = job.split(" ").collect::<Vec<&str>>();
                MonkeyJob::Operation(op[1].chars().next().unwrap(), op[0].to_string(), op[2].to_string())
            }
        }))
        .collect::<HashMap<String,Monkey>>();

    println!("root: {:?}", monkeys.get("root").unwrap());
//    println!("input: {:#?}", monkeys);
    let res = solve("root".to_string(), &monkeys);
    println!("root = {}", res);
}
