use std::collections::VecDeque;

const MOVES: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let mut end: (usize, usize) = (0, 0);
    let mut starts: Vec<(usize, usize)> = Vec::new();
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
                            starts.push((row, col));
                            b'a'
                        }
                        b'E' => {
                            end = (row, col);
                            b'z'
                        }
                        b'a' => {
                            starts.push((row, col));
                            b'a'
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

    let allstarts = starts
        .iter()
        .map(|&start| {
            let mut moves: VecDeque<(usize, usize)> = VecDeque::new();
            let mut steps: Vec<Vec<usize>> = vec![vec![0; ymax as usize]; xmax as usize];

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
                    if gridlevel <= curlevel + 1 && (gridsteps > cur + 1 || gridsteps == 0) {
                        steps[mx][my] = cur + 1;
                        moves.push_back((mx, my));
                    }
                }
            }
            steps[end.0][end.1]
        })
        .filter(|x| *x > 0)
        .collect::<Vec<usize>>();

    println!("allstarts {:?}", allstarts);
    println!("min: {}", allstarts.iter().min().unwrap());
}
