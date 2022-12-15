use std::cmp::{max, min};
use std::{thread, time};
use std::io::Write;
use std::io::stdout;

#[derive(PartialEq)]
enum Spot {
    Air,
    Rock,
    Sand,
    EmptyBelow,
}

fn range(from: (usize, usize), to: (usize, usize)) -> Vec<(usize, usize)> {
    let a = (min(from.0, to.0), min(from.1, to.1));
    let b = (max(from.0, to.0), max(from.1, to.1));
    if a.0 == b.0 {
        (a.1..=b.1)
            .map(|v| (a.0, v))
            .collect::<Vec<(usize, usize)>>()
    } else {
        (a.0..=b.0)
            .map(|v| (v, a.1))
            .collect::<Vec<(usize, usize)>>()
    }
}

fn main() {
    let mut input = include_str!("sample.txt")
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|coord| coord.split_once(",").unwrap())
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<Vec<(usize, usize)>>>();

    let minrow = 0;
    let maxrow = input
        .iter()
        .map(|l| l.iter().map(|(_, b)| *b).max().unwrap())
        .max()
        .unwrap()
        + 3;

    let mincol = input
        .iter()
        .map(|l| l.iter().map(|(a, _)| *a).min().unwrap())
        .min()
        .unwrap()
        - (maxrow + 1);
    let maxcol = input
        .iter()
        .map(|l| l.iter().map(|(a, _)| *a).max().unwrap())
        .max()
        .unwrap()
        + (maxrow + 2);

    //    println!("Floor: {:?} {:?}", (0,maxrow-1),(maxcol,maxrow-1));
    input.push(vec![(mincol, maxrow-1), (maxcol - 1, maxrow-1)]);

    let mut cave: Vec<Vec<Spot>> = Vec::with_capacity(maxrow + 1);
    cave.resize_with(maxrow + 1, || Vec::new());
    for row in minrow..=maxrow {
        cave[row].resize_with(maxcol + 2, || Spot::Air);
    }

    println!("X {}-{} Y {}-{}", mincol, maxcol, minrow, maxrow);

    input.iter().for_each(|l| {
        l.windows(2).for_each(|pair| {
            let from = pair[0];
            let to = pair[1];
            println!("Rocks: {:?} - {:?}", from, to);
            for (col, row) in range(from, to).into_iter() {
                cave[row][col] = Spot::Rock;
            }
        })
    });

    for col in mincol..maxcol {
        for row in (minrow..=maxrow).rev() {
            if row == 0 || cave[row-1][col] != Spot::Air {
                (row..=maxrow).for_each(|row| cave[row][col] = Spot::EmptyBelow);
                break;
            }
        }
    }

    let board = (minrow..=maxrow)
        .map(|row| {
            (mincol..maxcol)
                .map(|col| match cave[row][col] {
                    Spot::Air => '.',
                    Spot::Rock => '#',
                    Spot::Sand => 'o',
                    _ => '?',
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>();
    println!("Cave:\n{}", board.join("\n"));

    let mut sand: usize = 0;
    loop {
        let mut col = 500;
        let mut landed: bool = false;
        for row in 0..maxrow {
            if cave[row][col] == Spot::EmptyBelow {
                println!("EmptyBelow {} {}", row, col);
                break;
            }

            match cave[row+1][col] {
                Spot::Air => {}
                Spot::EmptyBelow => {
                    println!("EmptyBelow {} {}", row, col+1);
                    break;
                }
                _ => {
                    if cave[row + 1][col - 1] == Spot::Air || cave[row + 1][col - 1] == Spot::EmptyBelow {
                        col = col - 1;
                        println!("MoveLeft {} {}", row, col+1);
                        continue;
                    } else if cave[row + 1][col + 1] == Spot::Air
                        || cave[row + 1][col + 1] == Spot::EmptyBelow
                    {
                        println!("MoveRight {} {}", row, col+1);
                        col = col + 1;
                        continue;
                    } else {
                        if cave[row][col] != Spot::Sand {
                            println!("Landed {} {}", row, col+1);
                            landed = true;
                            cave[row][col] = Spot::Sand;
                        }
                        println!("Not landed {} {}", row, col+1);
                        break;
                    }
                }
            }
        }
        if !landed {
            break;
        } else {
            sand += 1;
        }
        let board = (minrow..=maxrow)
            .map(|row| {
                (mincol..maxcol)
                    .map(|col| match cave[row][col] {
                        Spot::Air => '.',
                        Spot::Rock => '#',
                        Spot::Sand => 'o',
                        _ => '?',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        println!("\x1B[2J");
        println!("Sand {}::\n{}", sand, board);
        stdout().flush().unwrap();
        let ten_millis = time::Duration::from_millis(20);

        thread::sleep(ten_millis);
    }

    println!("sand: {}", sand);
}
