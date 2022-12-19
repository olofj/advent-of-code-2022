use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.split(",").map(|e| e.parse::<usize>().unwrap()).collect_tuple::<(usize,usize,usize)>().unwrap()).collect::<HashSet<(usize,usize,usize)>>();

    let moves: Vec<(isize,isize,isize)> = vec![
        (-1, 0, 0),
        ( 1, 0, 0),
        ( 0,-1, 0),
        ( 0, 1, 0),
        ( 0, 0,-1),
        ( 0, 0, 1),
    ];
    let mut surfaces = 0;
    for (x,y,z) in input.iter() {
        for (dx,dy,dz) in moves.iter() {
            let nx = (*x as isize + dx) as usize;
            let ny = (*y as isize + dy) as usize;
            let nz = (*z as isize + dz) as usize;
            if !input.contains(&(nx,ny,nz)) {
                surfaces += 1;
            }
        }
        println!("({},{},{}): {}", x,y,z,surfaces);
    }

//    println!("input: {:#?}", input);
    println!("surfaces: {}", surfaces);
}
