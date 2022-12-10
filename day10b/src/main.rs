use itertools::Itertools;

fn main() {
    let mut X: isize = 1;
    let mut Xtrace: Vec<isize> = Vec::new();
    let mut clk: usize = 1;

    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    input.iter().for_each(|cmd| match cmd[0] {
        "noop" => {
            Xtrace.push(X);
            clk += 1;
        }
        "addx" => {
            Xtrace.push(X);
            clk += 1;
            Xtrace.push(X);
            clk += 1;
            X += cmd[1].parse::<isize>().unwrap();
        }
        _ => panic!("unknown command {}", cmd[0]),
    });

    let out = Xtrace
        .iter()
        .chunks(40)
        .into_iter()
        .map(|c| {
            c.enumerate()
                .map(|(i, &x)| {
                    let i = i as isize;
                    if x + 1 == i || x == i || x - 1 == i {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    for l in out {
        println!("{}", l);
    }
}
