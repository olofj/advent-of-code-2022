fn main() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|l| l.as_bytes().iter().map(|c| (c-b'0') as usize).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    let rows = grid.len();
    let cols = grid[0].len();

    let visible = (0..rows).map(|row| {
        (0..cols).map(|col| {
            let mut up = (0..row).rev().map_while(|r| if grid[r][col] < grid[row][col] { Some(1) } else { None }).sum::<usize>();
            if up < row { up += 1 } 
            let mut down = (row+1..rows).map_while(|r| if grid[r][col] < grid[row][col] { Some(1) } else { None }).sum::<usize>();
            if down < rows-row-1 { down += 1 }
            let mut left = (0..col).rev().map_while(|c| if grid[row][c] < grid[row][col] { Some(1) } else { None }).sum::<usize>();
            if left < col { left += 1 }
            let mut right = (col+1..cols).map_while(|c| if grid[row][c] < grid[row][col] { Some(1) } else { None }).sum::<usize>();
            if right < cols - col - 1 { right += 1 }
            println!("{} {}: {} {} {} {}", row, col, up, down, left, right);
            up * down * left * right
        })
        .collect::<Vec<usize>>()
    })
    .collect::<Vec<Vec<usize>>>();
    println!("Array: {:?}", visible);
    let max = visible.iter().map(|r| r.iter().max().unwrap()).max().unwrap();
    println!("Max: {}", max);
}
