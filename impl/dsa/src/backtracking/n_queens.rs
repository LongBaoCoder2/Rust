#[allow(dead_code)]
pub fn n_queens_solver(size: usize) -> Vec<Vec<String>> {
    let mut queens_board = QueensBoard::new(size);
    queens_board.solve(0);

    queens_board.convert_to_string_result()
}

struct QueensBoard {
    result: Vec<Vec<usize>>,
    board: Vec<usize>,
    columns: Vec<bool>,
    main_diag: Vec<bool>,
    sub_diag: Vec<bool>,
}

impl QueensBoard {
    pub fn new(size: usize) -> Self {
        Self {
            result: vec![],
            board: vec![],
            columns: vec![false; size],
            main_diag: vec![false; 2 * size - 1],
            sub_diag: vec![false; 2 * size - 1],
        }
    }

    pub fn get_size(&self) -> usize {
        self.columns.len()
    }

    pub fn is_safe(&self, col: usize, row: usize) -> bool {
        let size = self.get_size();

        let main_index = (col as isize) - (row as isize) + (size as isize) - 1;
        !self.columns[col] && !self.main_diag[main_index as usize] && !self.sub_diag[col + row]
    }

    pub fn update(&mut self, col: usize, row: usize, value: bool) {
        let size = self.get_size();
        let main_index = (col as isize) - (row as isize) + (size as isize) - 1;

        self.columns[col] = value;
        self.main_diag[main_index as usize] = value;
        self.sub_diag[col + row] = value;
    }

    pub fn solve(&mut self, row: usize) {
        if row == self.get_size() {
            self.result.push(self.board.clone());
            return;
        }

        for col in 0..=self.get_size() - 1 {
            if self.is_safe(col, row) {
                // Mark state
                self.board.push(col);
                self.update(col, row, true);
                // Recursive
                self.solve(row + 1);
                // Backtracking
                self.update(col, row, false);
                self.board.pop();
            }
        }
    }

    pub fn convert_to_string_result(&self) -> Vec<Vec<String>> {
        let mut str_result: Vec<Vec<String>> = vec![];
        let size = self.get_size();

        for board in &self.result {
            let mut base_res = vec![];
            for col in board {
                let mut base_string = ".".repeat(size).to_string();
                base_string.replace_range(col..&(col + 1), "Q");
                base_res.push(base_string.clone());
            }
            str_result.push(base_res.clone());
        }

        str_result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_queens_solver() {
        // Test case: Solve the 4-Queens problem
        let solutions = n_queens_solver(4);

        assert_eq!(solutions.len(), 2); // There are two solutions for the 4-Queens problem

        // Verify the first solution
        assert_eq!(solutions[0], vec![".Q..", "...Q", "Q...", "..Q."]);

        // Verify the second solution
        assert_eq!(solutions[1], vec!["..Q.", "Q...", "...Q", ".Q.."]);
    }
}
