use std::collections::HashSet;
use rand::prelude::*;

const VALUES: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn print_board(board: &Vec<Vec<char>>) {
    println!("---------------------------------------------");
    for i in 0..9 {
        println!("{:?}", board[i]);
    }
    println!("---------------------------------------------");
}

fn generate_sudoku(number_of_entries: usize) -> Vec<Vec<char>> {
    let mut rng = thread_rng();
    let mut start = vec![vec!['.'; 9]; 9];

    for shift in 0..3 {
        let mut values = VALUES.clone();
        values.shuffle(&mut rng);

        let mut index = 0;
        for i in 0..3 {
            for j in 0..3 {
                start[i + shift*3][j + shift*3] = values[index];
                index += 1;
            }
        }
    }

    solve_sudoku(&mut start, 0, 0);

    let mut removed = 0;
    while removed < 9 * 9 - number_of_entries {
        let (i, j) = (rng.gen_range(0..9), rng.gen_range(0..9));
        if start[i][j] == '.' {
            continue;
        } else {
            start[i][j] = '.';
            removed += 1;
        }
    }
    return start;
}

pub fn solve_sudoku(board: &mut Vec<Vec<char>>, mut row: usize, mut col: usize) -> bool {
    if row == 9 - 1 && col == 9 {
        return true;
    }
    if col == 9 {
        row += 1;
        col = 0;
    }
    if board[row][col] != '.' {
        return solve_sudoku(board, row, col + 1)
    }
    for value in VALUES {
        if is_safe(&board, row, col, value) {
            board[row][col] = value;
            if solve_sudoku(board, row, col+1) {
                return true;
            }
        }
        board[row][col] = '.';
    }
    return false;
}

fn is_safe(board: &Vec<Vec<char>>, row: usize, col: usize, value: char) -> bool {
    for i in 0..9 {
        if board[i][col] == value {
            return false;
        }
        if board[row][i] == value {
            return false;
        }
    }
    let row_val = (row as f32 / 3.).floor() as usize;
    let col_val = (col as f32 / 3.).floor() as usize;
    if get_box_set(board, row_val *3, col_val *3).contains(&value) {
        return false;
    }
    return true;
}

fn get_box_set(board: &Vec<Vec<char>>, row_shift: usize, col_shift: usize) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();
    for i in 0..3 {
        for j in 0..3 {
            set.insert(board[i + row_shift][j + col_shift]);
        }
    };
    return set;
}

fn is_solved(board: &Vec<Vec<char>>) -> bool {
    return check_row_col(board) && check_boxes(board);
}


fn check_boxes(board: &Vec<Vec<char>>) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if get_box_set(board, i*3, j*3).len() != 9 {
                return false;
            }
        }
    }
    return true;
}

fn check_row_col(board: &Vec<Vec<char>>) -> bool {
    let mut col_set: HashSet<char> = HashSet::new();
    let mut row_set: HashSet<char> = HashSet::new();
    for i in 0..9 {
        for j in 0..9 {
            col_set.insert(board[i][j]);
            row_set.insert(board[i][j]);
        }
        col_set.remove(&'.');
        row_set.remove(&'.');
        if col_set.len() + row_set.len() < 18 {
            return false;
        }
        col_set.clear();
        row_set.clear();
    }
    return true;
}

fn array_to_vec(arr: [[char; 9]; 9]) -> Vec<Vec<char>> {
    let board: Vec<Vec<char>> = vec![Vec::from(arr[0]),
                                     Vec::from(arr[1]),
                                     Vec::from(arr[2]),
                                     Vec::from(arr[3]),
                                     Vec::from(arr[4]),
                                     Vec::from(arr[5]),
                                     Vec::from(arr[6]),
                                     Vec::from(arr[7]),
                                     Vec::from(arr[8])];
    return board;
}

fn main() {
    let generated_board = generate_sudoku(30);
    println!("Generated sudoku:");
    print_board(&generated_board);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_board_verification() {
        let correct: [[char; 9]; 9] =
            [['7', '9', '2', '1', '5', '4', '3', '8', '6'],
            ['6', '4', '3', '8', '2', '7', '1', '5', '9'],
            ['8', '5', '1', '3', '9', '6', '7', '2', '4'],
            ['2', '6', '5', '9', '7', '3', '8', '4', '1'],
            ['4', '8', '9', '5', '6', '1', '2', '7', '3'],
            ['3', '1', '7', '4', '8', '2', '9', '6', '5'],
            ['1', '3', '6', '7', '4', '8', '5', '9', '2'],
            ['9', '7', '4', '2', '1', '5', '6', '3', '8'],
            ['5', '2', '8', '6', '3', '9', '4', '1', '7']];
        let correct_board: Vec<Vec<char>> = array_to_vec(correct);
        assert!(is_solved(&correct_board));
    }

    #[test]
    fn test_solving() {
        let arr_board: [[char; 9]; 9] =
            [['.', '5', '.', '.', '1', '.', '.', '4', '.'],
            ['2', '.', '.', '.', '.', '.', '.', '3', '.'],
            ['.', '6', '.', '.', '.', '9', '1', '.', '2'],
            ['7', '.', '.', '.', '5', '.', '6', '.', '1'],
            ['.', '.', '5', '.', '.', '3', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '4', '.', '.'],
            ['.', '9', '.', '8', '.', '.', '.', '.', '.'],
            ['6', '.', '.', '.', '3', '.', '5', '.', '7'],
            ['.', '.', '.', '.', '.', '.', '.', '2', '.']];
        let mut board: Vec<Vec<char>> = array_to_vec(arr_board);
        solve_sudoku(&mut board, 0, 0);
        assert!(is_solved(&board));
    }
}
