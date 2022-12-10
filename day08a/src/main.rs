fn main() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|c| (c - b'0') as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let rows = grid.len();
    let cols = grid[0].len();

    let visible = (0..rows)
        .map(|row| {
            (0..cols)
                .map(|col| {
                    let g = grid[row][col];
                    (col == 0
                        || row == 0
                        || col == cols - 1
                        || row == rows - 1
                        || (0..row).all(|r| grid[r][col] < g)
                        || (row + 1..rows).all(|r| grid[r][col] < g)
                        || (0..col).all(|c| grid[row][c] < g)
                        || (col + 1..cols).all(|c| grid[row][c] < g)) as usize
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("visible: {:?}", visible);
}
