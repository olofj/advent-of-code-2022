fn main() {
    let mut x: isize = 1;

    let xtrace: Vec<isize> = include_str!("input.txt")
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .map(|cmd| match cmd[0] {
            "noop" => {
                vec![x]
            }
            "addx" => {
                let prev = x;
                x += cmd[1].parse::<isize>().unwrap();
                vec![prev, prev]
            }
            _ => panic!("unknown command {}", cmd[0]),
        })
        .flatten()
        .collect::<Vec<isize>>();

    let out: Vec<String> = xtrace
        .chunks(40)
        .into_iter()
        .map(|c| {
            c.iter()
                .enumerate()
                .map(|(i, &x)| match i as isize - x {
                    -1 | 0 | 1 => 'X',
                    _ => '.',
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    println!("{}", out.join("\n"));
}
