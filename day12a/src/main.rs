use std::collections::VecDeque;

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
    let mut steps: Vec<Vec<usize>> = vec![vec![0; ymax as usize]; xmax as usize];

    steps[start.0][start.1] = 1;

    const MOVES: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    moves.push_back(start);
    let mut visited = 1;
    while let Some((x, y)) = moves.pop_front() {
        let cur = steps[x][y];
        let curlevel = input[x][y];
        visited += 1;
        for (mx, my) in MOVES.iter().filter_map(|(mx, my)| {
            let x = x as isize + mx;
            let y = y as isize + my;
            if x < 0 || y < 0 || x >= xmax || y >= ymax {
                None
            } else {
                Some((x as usize, y as usize))
            }
        }) {
            if input[mx][my] <= curlevel + 1 && (steps[mx][my] > cur + 1 || steps[mx][my] == 0) {
                steps[mx][my] = cur + 1;
                moves.push_back((mx, my));
            }
        }
    }
    println!("xmax {} ymax {}", xmax, ymax);
    println!("start {:?} end {:?}", start, end);
    println!("steps at end: {}", steps[end.0][end.1] - 1);
}
