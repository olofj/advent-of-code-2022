use std::collections::VecDeque;

const MOVES: [(isize, isize, char); 4] = [(-1, 0, 'v'), (1, 0, '^'), (0, -1, '>'), (0, 1, '<')];

fn main() {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let input: Vec<Vec<u8>> = include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.as_bytes()
                .iter()
                .enumerate()
                .map(|(col, c)| {
                    let c: u8 = match *c {
                        b'S' => {
                            start = (row, col);
                            b'a'
                        }
                        b'E' => {
                            end = (row, col);
                            b'z'
                        }
                        _ => *c,
                    };
                    c - b'a'
                })
                .collect::<Vec<u8>>()
        })
        .collect();

    let xmax = input.len() as isize;
    let ymax = input[0].len() as isize;
    let mut moves: VecDeque<(usize, usize)> = VecDeque::new();

    let mut steps: Vec<Vec<isize>> = vec![vec![-1; ymax as usize]; xmax as usize];
    steps[start.0][start.1] = 0;

    moves.push_back(start);
    while let Some((x, y)) = moves.pop_front() {
        let cur = steps[x][y];
        let curlevel = input[x][y];
        for m in MOVES {
            let mx = (x as isize + m.0) as usize;
            let my = (y as isize + m.1) as usize;
            let Some(&gridlevel) = input.get(mx).and_then(|l| l.get(my)) else { continue };
            let Some(&gridsteps) = steps.get(mx).and_then(|l| l.get(my)) else { continue };
            if gridlevel <= curlevel + 1 && (gridsteps > cur + 1 || gridsteps == -1) {
                steps[mx][my] = cur + 1;
                moves.push_back((mx, my));
            }
        }
    }
    println!("xmax {} ymax {}", xmax, ymax);
    println!("start {:?} end {:?}", start, end);
    println!("steps at end: {}", steps[end.0][end.1]);

    let mut path: Vec<Vec<char>> = vec![vec!['.'; ymax as usize]; xmax as usize];
    path[end.0][end.1] = 'E';
    path[start.0][start.1] = 'S';

    let mut cur = end;
    while steps[cur.0][cur.1] != 0 {
        for m in MOVES {
            let mx = (cur.0 as isize + m.0) as usize;
            let my = (cur.1 as isize + m.1) as usize;
            let Some(&s) = steps.get(mx).and_then(|l| l.get(my)) else { continue };
            if s == steps[cur.0][cur.1] - 1 {
                path[mx][my] = m.2;
                cur = (mx, my);
                break;
            }
        }
    }
    println!("Moves for the shorterst path:");
    for l in path {
        println!("{}", l.iter().collect::<String>());
    }
}
