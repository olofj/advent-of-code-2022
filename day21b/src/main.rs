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
    job: MonkeyJob,
}

fn solve(str: &String, monkeys: &HashMap<String,Monkey>, humn: usize) -> Result<usize,usize> {
    if str == "humn" {
        return Ok(humn);
    }
    let m = monkeys.get(str).unwrap();
    match &m.job {
        MonkeyJob::Number(x) => Ok(*x),
        MonkeyJob::Operation(op, op1, op2) => {
            let op1: usize = solve(&op1, monkeys, humn)?;
            let op2: usize = solve(&op2, monkeys, humn)?;
            match op {
                '+' => Ok(op1 + op2),
                '*' => Ok(op1 * op2),
                '-' => op1.checked_sub(op2).ok_or(1),
                '/' => op1.checked_div(op2).ok_or(0),
                '=' => Ok((op1 == op2) as usize),
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
            job: if let Some(num) = job.parse::<usize>().ok() {
                MonkeyJob::Number(num)
            } else {
                let op: Vec<&str> = job.split(" ").collect::<Vec<&str>>();
                MonkeyJob::Operation(op[1].chars().next().unwrap(), op[0].to_string(), op[2].to_string())
            }
        }))
        .collect::<HashMap<String,Monkey>>();

    let r = monkeys.remove("root").unwrap();
    let (root_op1, root_op2) = if let MonkeyJob::Operation(_,root_op1,root_op2) = r.job {
        (root_op1, root_op2)
    } else {
        ("FOO1".to_string(), "FOO2".to_string())
    };
    monkeys.insert("root".to_string(), Monkey { job: MonkeyJob::Operation( '=', root_op1.clone(), root_op2.clone()) });
    println!("root: {:?}", monkeys.get("root").unwrap());
    
    let try1 = solve(&root_op1, &monkeys, 1000).unwrap();
    let try2 = solve(&root_op1, &monkeys, 2000).unwrap();
    let positive = if try1 != try2 {
        try2 > try1
    } else {
        let try1 = solve(&root_op2, &monkeys, 1000).unwrap();
        let try2 = solve(&root_op2, &monkeys, 2000).unwrap();
        try2 > try1
    };

    let mut min = 0;
    let mut max = std::usize::MAX>>21;
    let mut med = (max + min) / 2;
    while min < max-1 {
        let op1 = solve(&root_op1, &monkeys, med);
        let op2 = solve(&root_op2, &monkeys, med);
        if op1.is_err() || op2.is_err() {
            let newmed = max-2;
            if solve(&root_op1, &monkeys, newmed).is_ok() && 
               solve(&root_op2, &monkeys, newmed).is_ok() {
                min = med;
                med = (max+med)/2;
                continue;
            }
            let newmed = min+2;
            if solve(&root_op1, &monkeys, newmed).is_ok() && 
               solve(&root_op2, &monkeys, newmed).is_ok() {
                max = med;
                med = (min+med)/2;
                continue;
            } 
        }
        let op1 = op1.unwrap();
        let op2 = op2.unwrap();
        println!("bisecting {:14} - {:14}, med {:14} (delta: {})", min, max, med, op2 as isize - op1 as isize);
        if op1 == op2 {
            println!("Found it: {}", med);
            break;
        }
        if (positive && op2 > op1) || (!positive && op1 > op2) {
            min = med;
        } else if (!positive && op2 > op1) || (positive && op1 > op2) {
            max = med;
        }
        med = (max + min) / 2;
    }


    if solve(&"root".to_string(), &monkeys, med).unwrap() != 1 {
        println!("No solution in the range");
    } else {
        println!("For humn = {}, root: {}", med, solve(&"root".to_string(), &monkeys, med).unwrap());
    }

//    println!("input: {:#?}", monkeys);
}
