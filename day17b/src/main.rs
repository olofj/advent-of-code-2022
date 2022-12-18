use std::time::{Duration, Instant};


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
    rocks: u64,
}

fn fits(p: &Piece, h: usize, s: usize, board: &Vec<u64>) -> bool {
    let boardidx = h / 8;
    let boardshift = (7 - (h % 8)) * 8;
    let nextidx = boardidx - 1;
    let nextshift = ((h % 8) + 1) * 8;
    /*
    println!(
        "h {} boardidx {} boardshift {} nextidx {} nextshift{}",
        h, boardidx, boardshift, nextidx, nextshift
    );
    */
    let b: u64 = board[boardidx].checked_shl(boardshift as u32).unwrap_or(0)
        | board[nextidx].checked_shr(nextshift as u32).unwrap_or(0);
    let ret = (p.rocks >> s) & b == 0;
    /*
    let pp = p.rocks >> s;
    let pprint = (0..8)
        .map(|w| format!("{:#010b}", (pp >> ((7 - w) * 8)) & 0xff))
        .collect::<Vec<String>>()
        .join("\n");
    let print = (0..8)
        .map(|w| format!("{:#010b}", (b >> ((7 - w) * 8)) & 0xff))
        .collect::<Vec<String>>()
        .join("\n");
    println!(
        "fit h {} s {} ret {}p:\n{}\nboard:\n{}",
        h, s, ret, pprint, print
    );
    */
    ret
}

fn fits2(pc: (u64, u64), bc: (u64, u64)) -> bool {
    let ret = (pc.0 & bc.0) | (pc.1 & bc.1) == 0;
    /*

            let pc0 = (0..8) .map(|w| {
                        format!(
                            "{:<3} {:#010b}",
                            7-w as usize,
                            (pc.0 >> ((7 - w) * 8)) & 0xff
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                
            let pc1 = (0..8) .map(|w| {
                        format!(
                            "{:<3} {:#010b}",
                            7-w as usize,
                            (pc.1 >> ((7 - w) * 8)) & 0xff
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

            let bc0 = (0..8) .map(|w| {
                        format!(
                            "{:<3} {:#010b}",
                            7-w as usize,
                            (bc.0 >> ((7 - w) * 8)) & 0xff
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                
            let bc1 = (0..8) .map(|w| {
                        format!(
                            "{:<3} {:#010b}",
                            7-w as usize,
                            (bc.1 >> ((7 - w) * 8)) & 0xff
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

    println!("pc.0:\n{}\npc.1:\n{}", pc0, pc1);
    println!("bc.0:\n{}\nbc.1:\n{}", bc0, bc1);
    */
    ret
                
}

fn main() {
    let mut height: usize = 9;
    let mut board: Vec<u64> = vec![0, 0x00000000_000000fe];
    let mut pruned: usize = 0;

    board.reserve(2048);

    let pieces = vec![
        Piece {
            width: 4,
            height: 1,
            rocks: 0b11110000_00000000_00000000_00000000 << 32,
        },
        Piece {
            width: 3,
            height: 3,
            rocks: 0b01000000_11100000_01000000_00000000 << 32,
        },
        Piece {
            width: 3,
            height: 3,
            rocks: 0b00100000_00100000_11100000_00000000 << 32,
        },
        Piece {
            width: 1,
            height: 4,
            rocks: 0b10000000_10000000_10000000_10000000 << 32,
        },
        Piece {
            width: 2,
            height: 2,
            rocks: 0b11000000_11000000_00000000_00000000 << 32,
        },
    ];
    let input = include_str!("input.txt")
        .trim()
        .chars()
        .map(|c| if c == '<' { -1 } else { 1 })
        .collect::<Vec<isize>>();

    let mut iter: usize = 0;
    let itermod: usize = input.len();
    let mut len = board.len();

    println!("input[{}]: {:?}", input.len(), input);
    let mut maxfall = 3;

    let start = Instant::now();

    let mut lastrock = 0;
    let mut lastheight = 0;

    // repeat rock 16062333 (6780)  height 24960596 (10536)  3120073
    // repeat rock 8616124 (280)  height 13047291 (424)  1630910
    for rock in 0..(1000000000000 % 6780) {
//    for rock in 0..100000000 {
//    for rock in 0..2022 {
        let mut shift: usize = 2;
        let piece = &pieces[rock % 5];
        let maxshift = 7-piece.width;

        if (rock+1) % 10000000 == 0 {
            let progress = rock as f64 / 1000000000000.0;
            let est = Duration::from_secs_f64(start.elapsed().as_secs_f64() / progress);
            println!(
                "rock {} height {} maxfall {} {}% {:?} / {:?}",
                rock,
                height,
                maxfall,
                rock as f64 / 1000000000000.0 * 100.0,
                start.elapsed(),
                est
            );
        }

        /*
        if height > 1024 {
            pruned += 512;
            height -= 512;
            len -= 512/8;
            board.drain(0..(512/8));
        }
        */

        for _ in len..(height + piece.height + 10) / 8 {
            board.push(0);
            len += 1;
        }
        let mut row: usize = height + piece.height + 2;

        let fromtop = 1 + (row % 8);
        let firstshift = (8-fromtop) * 8 + shift;
        let nextshift = (fromtop) * 8 - shift;

        let mut pc: (u64, u64) = (piece.rocks >> firstshift, piece.rocks << nextshift);
        let mut boardindex = row / 8;
        let mut bc: (u64, u64) = (board[boardindex], board[boardindex-1]);

        loop {
            let jet = input[iter];
            iter = (iter + 1) % itermod;
            let oldshift = shift;
            let oldc = pc;
            match jet {
                -1 => if shift > 0 {
                    shift -= 1;
                    pc.0 <<= 1;
                    pc.1 <<= 1;
                },
                1 => if shift < maxshift {
                    shift += 1;
                    pc.0 >>= 1;
                    pc.1 >>= 1;
                },
                _ => {},
            }
//            println!("move: {} ({} -> {})", jet, oldshift, shift);
            if oldshift != shift && !fits2(pc, bc) {
//                println!("didn't fit, undoing shift ({})", oldshift);
                shift = oldshift;
                pc = oldc;
            }
//            println!("trying move down {} -> {}", row, row-1);
            let nc = (pc.0 >> 8, (pc.1 >> 8) | (pc.0 & 0xff) << 56);
            if fits2(nc, bc) {
//                println!("move down");
                row -= 1;
                pc = nc;
                if row / 8 < boardindex {
//                    println!("shuffling");
                    pc.0 = pc.1;
                    pc.1 = 0;
                    boardindex -= 1;
                    bc.0 = bc.1;
                    bc.1 = board[boardindex-1];
                }
            } else {
//                println!("can't move down from {}", row);
                break;
            }
        }

        board[boardindex] |= pc.0;
        board[boardindex-1] |= pc.1;

        if rock > 1000 && board[boardindex] == board[100] {
            println!("repeat rock {} ({})  height {} ({})  {}", rock, rock-lastrock, height, height-lastheight, boardindex);
            lastrock = rock;
            lastheight = height;
        }

        maxfall = maxfall.max(height + piece.height + 2 - row);
        height = height.max(row + 1);
        /*
        let print = board
            .iter()
            .enumerate()
            .rev()
            .map(|(i, r)| {
                (0..8)
                    .map(|w| {
                        format!(
                            "{:<5} {:#010b}",
                            i * 8 + 7-w as usize,
                            (r >> ((7 - w) * 8)) & 0xff
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n- - - - -\n");
            println!("rock {} board height {}:\n{}", rock, height, print);
            */
    }
//        let print = board.iter().rev().map(|r| format!("{:#010x}", r)).collect::<Vec<String>>().join("\n");
//        println!("board:\n{}", print);
    println!("height: {} {}", height - 9 + 1553982297000, board.len());
}
