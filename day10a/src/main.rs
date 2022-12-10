fn main() {
    let mut x: isize = 1;

    let mut xtrace: Vec<isize> = include_str!("input.txt")
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
        
    // make it 1-indexed by adding an initial value at cycle 0
    xtrace.insert(0, 1);

    let mut total: isize = 0;
    for i in (20..=220).step_by(40) {
        println!("x[{}]: {}", i, xtrace[i]);
        total += i as isize * xtrace[i];
    }
    println!("total: {}", total);
}
