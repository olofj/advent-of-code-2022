struct Monkey<'a> {
    items: Vec<usize>,
    op: &'a str,
    arg: Option<usize>,
    divisible: usize,
    if_true: usize,
    if_false: usize,
}
fn main() {
    let input = include_str!("input.txt")
        .split("\n\n")
        .collect::<Vec<&str>>();
    
    let mut monkey: Vec<Monkey> = Vec::new();

    for (i,m) in input.iter().enumerate() {
        let m = m.lines().collect::<Vec<&str>>();
        let items: Vec<_> = m[1].split_once(":").unwrap().1.split(&[',',' '][..]).filter_map(|i| i.parse::<usize>().ok()).collect::<Vec<_>>();
        println!("items: {:?}", items);
        let div = m[3].split_once(" by ").unwrap().1.parse::<usize>().unwrap();
        println!("div: {}", div);
        let op = m[2].split_once("new = old ").unwrap().1.split_whitespace().collect::<Vec<&str>>();
        let arg = op[1].parse::<usize>().ok();
        let op = match op[0] {
            "+" => "+",
            "*" => "*",
            _ => panic!("Unkown operator {}", op[0]),
        };
        println!("op {:?} arg {:?}", op, arg);
        let if_true = m[4].split_once("throw to monkey ").unwrap().1.parse::<usize>().unwrap();
        let if_false = m[5].split_once("throw to monkey ").unwrap().1.parse::<usize>().unwrap();
        println!("if_true {} if_false {}", if_true, if_false);
        monkey.insert(i, Monkey{ items: items, op: op, arg: arg, divisible: div, if_true: if_true, if_false: if_false});
    }

    let mut insp: Vec<usize> = monkey.iter().map(|_| 0).collect::<Vec<usize>>();

    for i in 0..20 {
        for m in 0..monkey.len() {
            let moves = monkey[m].items.iter().map(|item| {
                let arg = match &monkey[m].arg {
                    None => item,
                    Some(x) => x,
                };
                let new = match monkey[m].op {
                    "*" => item * arg,
                    "+" => item + arg,
                    _ => panic!("Unknown op {}", monkey[m].op),
                } / 3;
                let check: bool = new % monkey[m].divisible == 0;
                let next = match check {
                    true => monkey[m].if_true,
                    false => monkey[m].if_false,
                };
                (next, new)
            }).collect::<Vec<(usize, usize)>>();
            monkey[m].items = Vec::new();
            insp[m] += moves.len();
            moves.iter().for_each(|(next, new)| monkey[*next].items.push(*new));
        }
        for m in 0..monkey.len() {
            println!("Monkey {} items: {:?}", m, monkey[m].items);
        }
    }
    insp.sort();
    let insp = insp.into_iter().rev().collect::<Vec<usize>>();
    println!("Inspections {:?}", insp);
    println!("Monkey Business: {}", insp[0] * insp[1]);
}
