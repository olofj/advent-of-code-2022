use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;

#[derive(Debug)]
enum MonkeyJob {
    Number(isize),
    Operation(char, String, String),
}

#[derive(Debug)]
struct Monkey {
    job: MonkeyJob,
}

fn solve(str: &String, monkeys: &HashMap<String, Monkey>, humn: isize) -> Option<isize> {
    // override for humn instead of trying to modify the hashmap before every call
    if str == "humn" {
        return Some(humn);
    }
    match &monkeys.get(str)?.job {
        MonkeyJob::Number(x) => Some(*x),
        MonkeyJob::Operation(op, op1, op2) => {
            let op1: isize = solve(&op1, monkeys, humn)?;
            let op2: isize = solve(&op2, monkeys, humn)?;
            match op {
                '+' => op1.checked_add(op2),
                '*' => op1.checked_mul(op2),
                '-' => op1.checked_sub(op2),
                '/' => op1.checked_div(op2),
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
                Monkey {
                    job: if let Some(num) = job.parse::<isize>().ok() {
                        MonkeyJob::Number(num)
                    } else {
                        let op: Vec<&str> = job.split(" ").collect::<Vec<&str>>();
                        MonkeyJob::Operation(
                            op[1].chars().next().unwrap(),
                            op[0].to_string(),
                            op[2].to_string(),
                        )
                    },
                },
            )
        })
        .collect::<HashMap<String, Monkey>>();

    // Get the two operands for root so we can iterate on them
    let r = monkeys.get("root").unwrap();
    let (mut root_op1, mut root_op2, _): (String, String, MonkeyJob) = match &r.job {
        MonkeyJob::Operation(_, op1, op2) => (
            op1.clone(),
            op2.clone(),
            MonkeyJob::Operation('=', op1.to_string(), op2.to_string()),
        ),
        MonkeyJob::Number(_) => panic!("root can't be a number"),
    };
//    println!("root: {:?}", r);

    // Possibly reorder op1/op2 to make positive corellation between
    // delta(humn) and delta(op2-op1) for easier binary search.
    
    // FIXME: If either side doesn't evaluate for humn=1000 or humn=2000 we'd
    // need to search for useful values to try around.

    let op1_1000 = solve(&root_op1, &monkeys, 1000).unwrap();
    let op1_2000 = solve(&root_op1, &monkeys, 2000).unwrap();
    let op2_1000 = solve(&root_op2, &monkeys, 1000).unwrap();
    let op2_2000 = solve(&root_op2, &monkeys, 2000).unwrap();
    (root_op1, root_op2) = match (op1_1000.cmp(&op1_2000), op2_1000.cmp(&op2_2000)) {
        (Equal, Less) => (root_op2, root_op1),
        (Equal, Greater) => (root_op1, root_op2),
        (Less, Equal) => (root_op1, root_op2),
        (Greater, Equal) => (root_op2, root_op1),
        _ => panic!("Neither side of root depends on humn?!"),
    };

    // Now all we need to do is binary search for the right humn value!
    
    let mut min = 0;
    let mut max = std::isize::MAX;
    let res = loop {
        let med = (max + min) / 2;
        if min >= max - 1 {
            break None;
        }
        let op1 = solve(&root_op1, &monkeys, med);
        let op2 = solve(&root_op2, &monkeys, med);
        if op1.is_none() || op2.is_none() {
    	    // Whoops, bad result on either side. Let's just move
	        // away from the half side that has bad values at either
            // end of the range.

            let newmed = max - 2;
            if solve(&root_op1, &monkeys, newmed).is_some()
                && solve(&root_op2, &monkeys, newmed).is_some()
            {
                min = med;
            } else {
                max = med;
            }
            continue;
        }
//        println!("bisecting {:14} - {:14}, med {:14}: {:?} {:?}: delta {}", min, max, med, op1, op2, op2.unwrap() - op1.unwrap());
        match op1.cmp(&op2) {
            Equal => break Some(med),
            Less => min = med,
            Greater => max = med,
        }
    };

    if res == None {
        println!("Can't find a solution in the range 0..{}", std::isize::MAX);
    } else {
        let med = res.unwrap();

        let answers = (med-20..med+20).filter(|i| {
            solve(&root_op1, &monkeys, *i) == solve(&root_op2, &monkeys, *i)
        }).collect::<Vec<isize>>();
        println!("These are valid answers: {:?}", answers);
    }
}
