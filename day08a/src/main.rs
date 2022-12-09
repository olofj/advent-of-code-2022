fn main() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|l| l.as_bytes().iter().map(|c| (c-b'0') as usize).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    let rows = grid.len();
    let cols = grid[0].len();

    let visible = (0..rows).map(|row| {
        (0..cols).map(|col| {
            col == 0 || row == 0 || col == cols-1 || row == rows-1 ||
            (0..row).all(|r| grid[r][col] < grid[row][col]) ||
            (row+1..rows).all(|r| grid[r][col] < grid[row][col]) ||
            (0..col).all(|c| grid[row][c] < grid[row][col]) ||
            (col+1..cols).all(|c| grid[row][c] < grid[row][col])
        })
//        .map(|v| if v { 1 } else { 0 } )
//        .sum::<usize>()
        .collect::<Vec<bool>>()
    })
//    .sum();
    .collect::<Vec<Vec<bool>>>();
    println!("Array: {:?}", visible);
    let count = visible.iter().map(|r| r.iter().map(|c| if *c { 1 } else { 0 } ).sum::<usize>()).sum::<usize>();
    println!("Count: {:?}", count);
}
