// ####
//
// .#.
// ###
// .#.
//
// ..#
// ..#
// ###
//
// #
// #
// #
// #
//
// ##
// ##

struct Piece {
    width: usize,
    height: usize,
    rocks: Vec<(usize, usize)>,
}

fn fits(p: &Piece, h: usize, v: usize, b: &Vec<Vec<bool>>) -> bool {
    for (r,c) in p.rocks.iter() {
        if *r > h || b[h-r][v+c] {
            return false;
        }
    }
    true
}

fn main() {
    let mut height: usize = 0;
    let mut board: Vec<Vec<bool>> = Vec::new();

    let pieces = vec![
        Piece { width: 4, height: 1, rocks: vec![(0, 0), (0, 1), (0, 2), (0, 3)] },
        Piece { width: 3, height: 3, rocks: vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)] },
        Piece { width: 3, height: 3, rocks: vec![(0, 2), (1, 2), (2, 0), (2, 1), (2, 2)] },
        Piece { width: 1, height: 4, rocks: vec![(0, 0), (1, 0), (2, 0), (3, 0)] },
        Piece { width: 2, height: 2, rocks: vec![(0, 0), (0, 1), (1, 0), (1, 1)] },
    ];
    let input = include_str!("input.txt").trim().chars().collect::<Vec<char>>();

    let mut iter = input.iter();

    println!("input {:?}", input);

    for rock in 0..2022 {
        let mut vert: usize = 2;
        let piece = &pieces[rock % 5];

        for _ in board.len()..height+piece.height+3 {
            board.push(vec![false; 7]);
        }
        let mut row: usize = height+piece.height+2;

        loop {
            let mut jet = iter.next();
            if jet == None {
                iter = input.iter();
                jet = iter.next();
            }
            let oldvert = vert;
            vert = match jet {
                Some('<') => if vert > 0 { vert - 1 } else { vert },
                Some('>') => if (vert+piece.width) < 7 { vert + 1 } else { vert },
                _ => panic!("should never happen"),
            };
            if ! fits(piece, row, vert, &board) {
                vert = oldvert;
            }
//            println!("move: {} ({} -> {})", jet.unwrap(), oldvert, vert);
            if row > 0 && fits(piece, row - 1, vert, &board) {
//                println!("move down");
                row -= 1;
            } else {
//                println!("can't move down from {}", row);
                break;
            }
        }

        for (r,c) in piece.rocks.iter() {
            board[row-r][vert+c] = true;
        }

        height = 0;
        while board[height].iter().any(|r| *r) {
            height += 1;
        }
        let print = board.iter().rev().map(|row| row.iter().map(|r| if *r { '#' } else { '.' }).collect::<String>()).collect::<Vec<String>>().join("\n");
//        println!("rock {} board height {}:\n{}", rock, height, print);
    }
    println!("height: {}", height);
}
