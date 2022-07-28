fn main() {
    let initial_grid: [[i8; 9]; 9] = [
        [0, 4, 3, 0, 0, 0, 0, 0, 9], 
        [0, 0, 0, 6, 0, 0, 0, 0, 5], 
        [0, 0, 0, 0, 0, 4, 1, 0, 0], 
        [9, 0, 1, 0, 5, 0, 0, 0, 0], 
        [0, 0, 0, 7, 2, 6, 0, 0, 0], 
        [0, 0, 8, 0, 1, 0, 0, 0, 0], 
        [0, 1, 0, 0, 0, 0, 7, 2, 0], 
        [7, 0, 0, 0, 0, 0, 0, 0, 0], 
        [2, 0, 0, 0, 0, 5, 0, 6, 0], 
        ];
        solve_sudoku(initial_grid);
}

fn find_empty(grid: [[i8; 9]; 9]) -> (usize, usize){
    for row in 0..9{
        for col in 0..9 {
            if grid[row][col] == 0 {
                return (row, col)
            }
        }
    }
    print!("Done");
    return (9,9)
}

fn solve_sudoku(mut grid: [[i8; 9]; 9]) -> bool{
    let l: (usize, usize) = find_empty(grid);
    if l == (9, 9) {
        print_grid(grid);
        return true
    }
    for i in 1..10 {
        if is_location_safe(grid, l.0, l.1, i) {
            grid[l.0][l.1] = i;
            if solve_sudoku(grid) {
                return true;
            }
            grid[l.0][l.1] = 0;
        }
    }
    return false
}

fn is_location_safe(grid: [[i8; 9]; 9], row: usize, col: usize, num: i8) -> bool {
    return !used_in_col(grid, col, num) & !used_in_row(grid, row, num) & !used_in_box(grid, row, col, num)
}

fn used_in_box(grid: [[i8; 9]; 9], row: usize, col: usize, num: i8) -> bool {
    let first_cell_row = row - (row%3);
    let first_cell_column = col - (col%3);
    for i in 0..3 {
        for j in 0..3{
            if grid[i+first_cell_row][j+first_cell_column] == num {
                return true
            }
        }
    }
    return false
}

fn used_in_col(grid: [[i8; 9]; 9], col: usize, num: i8) -> bool {
    for i in 0..8 {
        if grid[i][col] == num {
            return true;
        }
    }
    return false;
}

fn used_in_row(grid: [[i8; 9]; 9], row: usize, num: i8) -> bool{
    for i in grid[row] {
        if i == num {
            return true;
        }
    }
    return false
}

fn print_grid(grid: [[i8; 9]; 9]) {
    println!();
    for row in grid {
        for item in row{
            print!("{:?} ", item);
        }
        println!();
    }
}
