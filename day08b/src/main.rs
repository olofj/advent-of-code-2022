use std::cmp::min;

fn main() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|l| l.as_bytes().iter().map(|c| (c-b'0') as usize).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    let rows = grid.len();
    let cols = grid[0].len();

    let visible = (0..rows).map(|row| {
        (0..cols).map(|col| {
            let g = grid[row][col];
            let mut up = 1+(0..row).rev().map_while(|r| if grid[r][col] < g { Some(1) } else { None }).sum::<usize>();
            up = min(up, row);
            let mut down = 1+(row+1..rows).map_while(|r| if grid[r][col] < g { Some(1) } else { None }).sum::<usize>();
            down = min(down, rows-row-1);
            let mut left = (0..col).rev().map_while(|c| if grid[row][c] < g { Some(1) } else { None }).sum::<usize>();
            left = min(left, col);
            let mut right = (col+1..cols).map_while(|c| if grid[row][c] < g { Some(1) } else { None }).sum::<usize>();
            right = min(right, cols-col-1);
            up * down * left * right
        })
        .collect::<Vec<usize>>()
    })
    .collect::<Vec<Vec<usize>>>();

    let max = visible.iter().map(|r| r.iter().max().unwrap()).max().unwrap();
    println!("Max: {}", max);
}
