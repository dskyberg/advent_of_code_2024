use anyhow::Result;

use day_4::*;

#[inline]
fn nw_se(row: usize, col: usize, matrix: &[Vec<u8>]) -> bool {
    matrix[row][col] == A
        && ((matrix[row - 1][col - 1] == M && matrix[row + 1][col + 1] == S)
            || (matrix[row - 1][col - 1] == S && matrix[row + 1][col + 1] == M))
}

#[inline]
fn ne_sw(row: usize, col: usize, matrix: &[Vec<u8>]) -> bool {
    matrix[row][col] == A
        && ((matrix[row - 1][col + 1] == M && matrix[row + 1][col - 1] == S)
            || (matrix[row - 1][col + 1] == S && matrix[row + 1][col - 1] == M))
}

fn is_xmas(row: usize, col: usize, matrix: &[Vec<u8>]) -> bool {
    nw_se(row, col, matrix) && ne_sw(row, col, matrix)
}

fn main() -> Result<()> {
    let mut result = 0;
    let matrix = read_data(DATA);
    let rows = matrix.len();
    let cols = matrix[0].len();
    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            if is_xmas(row, col, &matrix) {
                result += 1;
            }
        }
    }

    println!("Part 2: {}", result);
    Ok(())
}
