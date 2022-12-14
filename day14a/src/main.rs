use std::cmp::{max, min};

#[derive(PartialEq, PartialOrd)]
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
    let input = include_str!("input.txt")
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|coord| coord.split_once(",").unwrap())
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<Vec<(usize, usize)>>>();

    let xmin = input
        .iter()
        .map(|l| l.iter().map(|(a, _)| *a).min().unwrap())
        .min()
        .unwrap()
        - 1;
    let xmax = input
        .iter()
        .map(|l| l.iter().map(|(a, _)| *a).max().unwrap())
        .max()
        .unwrap()
        + 1;
    let ymin = 0;
    let ymax = input
        .iter()
        .map(|l| l.iter().map(|(_, b)| *b).max().unwrap())
        .max()
        .unwrap()
        + 1;

    let mut cave: Vec<Vec<Spot>> = Vec::with_capacity(xmax + 1);
    cave.resize_with(xmax + 1, || Vec::new());
    for x in xmin..=xmax {
        cave[x].resize_with(ymax + 1, || Spot::Air);
    }

    println!("X {}-{} Y {}-{}", xmin, xmax, ymin, ymax);

    input.iter().for_each(|l| {
        l.windows(2).for_each(|pair| {
            let from = pair[0];
            let to = pair[1];
            println!("Rocks: {:?} - {:?}", from, to);
            for (x, y) in range(from, to).into_iter() {
                cave[x][y] = Spot::Rock;
            }
        })
    });

    for x in xmin..=xmax {
        for y in (ymin..=ymax).rev() {
            if y == 0 || cave[x][y - 1] != Spot::Air {
                cave[x][y] = Spot::EmptyBelow;
                break;
            }
        }
    }

    let mut sand: usize = 0;
    loop {
        let mut x = 500;
        let mut landed: bool = false;
        for y in 0..ymax {
            if cave[x][y] == Spot::EmptyBelow {
                println!("EmptyBelow at {} {}", x, y);
                break;
            }

            match cave[x][y + 1] {
                Spot::Air => {}
                Spot::EmptyBelow => {
                    println!("EmptyBelow at {} {}", x, y);
                    break;
                }
                _ => {
                    if cave[x - 1][y + 1] == Spot::Air || cave[x - 1][y + 1] == Spot::EmptyBelow {
                        println!("Moveleft at {} {}", x, y);
                        x = x - 1;
                        continue;
                    } else {
                        if cave[x + 1][y + 1] == Spot::Air || cave[x + 1][y + 1] == Spot::EmptyBelow
                        {
                            println!("Moveright at {} {}", x, y);
                            x = x + 1;
                            continue;
                        } else {
                            println!("Sand at {} {}", x, y);
                            landed = true;
                            cave[x][y] = Spot::Sand;
                            break;
                        }
                    }
                }
            }
        }
            if !landed {
                println!("Done at {}", sand);
                break;
            } else {
                sand += 1;
            }
    }

    let board = (ymin..=ymax)
        .map(|y| {
            (xmin..=xmax)
                .map(|x| match cave[x][y] {
                    Spot::Air => '.',
                    Spot::Rock => '#',
                    Spot::Sand => 'o',
                    _ => '?',
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>();
    println!("Cave:\n{}", board.join("\n"));
}
