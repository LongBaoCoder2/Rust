#[allow(dead_code)]
struct Sudoku {
    board: [[u8; 9]; 9]
}


impl Sudoku {
    pub fn new(board: [[u8; 9]; 9]) -> Self {
        Self { 
            board 
        }
    }

    pub fn is_safe(&self, row: usize, col: usize, num: u8) -> bool {
        for i in 0..9 { 
            if (self.board[row][i] == num) || (self.board[i][col] == num) {
                return false;
            }
        }

        let chunk_x : usize = row / 3;
        let chunk_y : usize = col / 3;

        for i in 0..3 {
            for j in 0..3 {
                if self.board[chunk_x * 3 + i][chunk_y * 3 + j] == num {
                    return false;
                }
            }
        }

        true
    }

    fn fine_the_next_cell(&self, (row, col): (usize, usize)) -> Option<(usize, usize)> {
        for i in col..9 {
            if self.board[row][i] == 0 {
                return Some((row, i));
            }
        }

        for i in row + 1..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    return Some((i, j));
                }
            }
        }

        None
    }

    pub fn try_the_next_state(&mut self, (row, col): (usize, usize)) -> bool {
        let next_cell = self.fine_the_next_cell((row, col));

        if let Some((next_row, next_col)) = next_cell {
            for i in 1..=9 {
                if self.is_safe(next_row, next_col, i) {
                    self.board[next_row][next_col] = i;
                    if self.try_the_next_state((next_row, next_col)) {
                        return true;
                    }

                    self.board[next_row][next_col] = 0;
                }
            }
        } else {
            return true;
        }

        false
    }


    pub fn solve(&mut self) -> bool {
        self.try_the_next_state((0, 0))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku_correct() {
        let board: [[u8; 9]; 9] = [
            [3, 0, 6, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ];

        let board_result = [
            [3, 1, 6, 5, 7, 8, 4, 9, 2],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 9, 2, 6, 4, 3],
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 9],
        ];

        let mut sudoku = Sudoku::new(board);
        let is_solved = sudoku.solve();

        assert!(is_solved);
        assert_eq!(sudoku.board, board_result);
    }

    #[test]
    fn test_sudoku_incorrect() {
        let board: [[u8; 9]; 9] = [
            [6, 0, 3, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ];

        let mut sudoku = Sudoku::new(board);
        let is_solved = sudoku.solve();

        assert!(!is_solved);
    }
}
