type Update = fn(usize, Option<usize>) -> usize;

struct Monkey {
    items: Vec<usize>,
    update: Update,
    arg: Option<usize>,
    divisible: usize,
    next: [usize; 2],
}

fn main() {
    let input = include_str!("input.txt")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let mut monkey: Vec<Monkey> = input
        .iter()
        .map(|m| m.lines().collect::<Vec<&str>>())
        .map(|l| {
            let mut new = Monkey {
                items: Vec::new(),
                update: |_, _| -> usize { 0 },
                arg: None,
                divisible: 0,
                next: [0, 0],
            };
            l.iter().enumerate().for_each(|(i, l)| match i {
                1 => {
                    new.items = l
                        .split_once(": ")
                        .unwrap()
                        .1
                        .split(", ")
                        .filter_map(|i| i.parse::<usize>().ok())
                        .collect::<Vec<_>>();
                }
                2 => {
                    let op = l
                        .split_once("new = old ")
                        .unwrap()
                        .1
                        .split_whitespace()
                        .collect::<Vec<&str>>();
                    new.arg = op[1].parse::<usize>().ok();
                    new.update = match (op[0], new.arg) {
                        ("+", None) => |old, _| -> usize { old + old },
                        ("+", Some(_)) => |old, arg| -> usize { old + arg.unwrap() },
                        ("*", None) => |old, _| -> usize { old * old },
                        ("*", Some(_)) => |old, arg| -> usize { old * arg.unwrap() },
                        _ => panic!("Unkown operator {} on line: {}", op[0], l),
                    };
                }
                3 => {
                    new.divisible = l.split_once(" by ").unwrap().1.parse::<usize>().unwrap();
                }
                4 => {
                    new.next[1] = l
                        .split_once("throw to monkey ")
                        .unwrap()
                        .1
                        .parse::<usize>()
                        .unwrap();
                }
                5 => {
                    new.next[0] = l
                        .split_once("throw to monkey ")
                        .unwrap()
                        .1
                        .parse::<usize>()
                        .unwrap();
                }
                _ => {}
            });
            new
        })
        .collect();

    let mut insp = vec![0; monkey.len()];

    let wrap: usize = monkey.iter().map(|m| m.divisible).product();
    println!("wrap is {}", wrap);

    for _ in 0..10000 {
        for m in 0..monkey.len() {
            let moves = monkey[m]
                .items
                .iter()
                .map(|item| {
                    let new = (monkey[m].update)(*item, monkey[m].arg) % wrap;
                    let check: bool = new % monkey[m].divisible == 0;
                    let next = monkey[m].next[check as usize];
                    (next, new)
                })
                .collect::<Vec<(usize, usize)>>();
            monkey[m].items = Vec::new();
            insp[m] += moves.len();
            moves
                .iter()
                .for_each(|(next, new)| monkey[*next].items.push(*new));
        }
    }
    println!("Inspections {:?}", insp);
    insp.sort();
    let insp = insp.into_iter().rev().take(2).collect::<Vec<usize>>();
    println!("Monkey Business: {}", insp[0] * insp[1]);
}
