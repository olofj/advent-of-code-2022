fn main() {
    let mut X: isize = 1;
    let mut Xtrace: Vec<usize> = vec![1; 1];
    let mut clk: usize = 1;

    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    input.iter().for_each(|cmd| {
        match cmd[0] {
            "noop" => {
                Xtrace.push(X as usize); clk += 1; },
            "addx" => {
                Xtrace.push(X as usize); clk += 1;
                Xtrace.push(X as usize); clk += 1;
                X += cmd[1].parse::<isize>().unwrap();
            },
            _ => panic!("unknown command {}", cmd[0]),
        }
    });

    println!("input: {:?}", input);
    println!("clk: {}", clk);
    let mut total: usize = 0;
    for i in vec![ 20, 60, 100, 140, 180, 220 ]  {
        println!("x[{}]: {}", i, Xtrace[i]);
        total += i * Xtrace[i];
    }
    println!("total: {}", total);
}
