use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.split(",").map(|e| e.parse::<usize>().unwrap()+1).collect_tuple::<(usize,usize,usize)>().unwrap()).collect::<HashSet<(usize,usize,usize)>>();

    let moves: Vec<(isize,isize,isize)> = vec![
        (-1, 0, 0),
        ( 1, 0, 0),
        ( 0,-1, 0),
        ( 0, 1, 0),
        ( 0, 0,-1),
        ( 0, 0, 1),
    ];

    let xmax = input.iter().map(|(x,_,_)| x).max().unwrap()+1;
    let ymax = input.iter().map(|(_,y,_)| y).max().unwrap()+1;
    let zmax = input.iter().map(|(_,_,z)| z).max().unwrap()+1;
    let mut air: HashSet<(usize,usize,usize)> = HashSet::new();

    let mut q: VecDeque<(usize,usize,usize)> = VecDeque::new();

    air.insert((0,0,0));
    q.push_back((0,0,0));

    while let Some((x,y,z)) = q.pop_front() {
        for (dx,dy,dz) in moves.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            let nz = z as isize + dz;
            if nx < 0 || ny < 0 || nz < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            let nz = nz as usize;
            if nx > xmax || ny > ymax || nz > zmax {
                continue;
            }
            if ! input.contains(&(nx,ny,nz)) &&
                ! air.contains(&(nx,ny,nz)) {
                air.insert((nx,ny,nz));
                q.push_back((nx,ny,nz));
            }
        }
    }

    println!("xmax {} ymax {} zmax {}", xmax, ymax, zmax);

    let mut surfaces = 0;
    for (x,y,z) in input.iter() {
        for (dx,dy,dz) in moves.iter() {
            let nx = (*x as isize + dx) as usize;
            let ny = (*y as isize + dy) as usize;
            let nz = (*z as isize + dz) as usize;
            if air.contains(&(nx,ny,nz)) {
                surfaces += 1;
            }
        }
        println!("({},{},{}): {}", x,y,z,surfaces);
    }

//    println!("input: {:#?}", input);
    println!("surfaces: {}", surfaces);
}
