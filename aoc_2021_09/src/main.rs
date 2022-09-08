fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let height = &input.lines().count();
    let width = &input.lines().next().unwrap().len();

    let grid = vec![vec![0; *width]; *height];
    let grid = populate_grid(grid, input);

    let point_values: Vec<(u32, u32, u32)> = Vec::new();

    grid.iter()
}

fn populate_grid(mut grid: Vec<Vec<u32>>, input: String) -> Vec<Vec<u32>> {
    for (i, line) in input.lines().enumerate() {
        for (j, val) in line.chars().enumerate() {
            grid[i][j] = val.to_digit(10).unwrap();
        }
    }
    grid
}

