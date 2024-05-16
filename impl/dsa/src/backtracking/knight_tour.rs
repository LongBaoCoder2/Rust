#[allow(dead_code)]
pub fn find_knight_tour(size_x: usize, size_y: usize, start_x: usize, start_y: usize)
    -> Option<Vec<Vec<usize>>>
{
    let mut knight_board = KnightTour::new(size_x, size_y);

    if !knight_board.is_valid(start_x as isize, start_y as isize) {
        return None;
    }

    knight_board.board[start_x][start_y] = 1;
    if knight_board.solve_tour(start_x, start_y, 1) {
        return Some(knight_board.board);
    }
    None

}



#[allow(dead_code)]
struct KnightTour {
    board: Vec<Vec<usize>>
}


#[allow(dead_code)]
impl KnightTour {

    const MOVE: [(isize, isize); 8] = [
        (2, 1),
        (1, 2),
        (-1, 2),
        (-2, 1),
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
    ];

    pub fn new(size_x: usize, size_y : usize) -> Self {
        let board = vec![vec![0; size_x]; size_y];
        Self {
            board
        }
    }

    pub fn size_x(&self) -> usize { 
        self.board.len()
    }


    pub fn size_y(&self) -> usize { 
        self.board[0].len()
    }

    pub fn is_valid(&self, x: isize, y: isize) -> bool {
        x >= 0 &&
        y >= 0 &&
        x < self.size_x() as isize &&
        y < self.size_y() as isize &&
        self.board[x as usize][y as usize] == 0
    }


    pub fn solve_tour(&mut self, x: usize, y: usize, move_counts: usize) -> bool {
        if move_counts == self.size_x() * self.size_y() {
            return true;
        }

        for &(dx, dy) in &Self::MOVE {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if self.is_valid(new_x, new_y) {
                self.board[new_x as usize][new_y as usize] = move_counts + 1;
                if self.solve_tour(new_x as usize, new_y as usize, move_counts + 1) {
                    return true;
                }

                self.board[new_x as usize][new_y as usize] = 0;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_knight_tour {
        ($($name: ident: $tc: expr, )*) => {
            $(
                #[test]
                pub fn $name() {
                    let (size_x, size_y, start_x, start_y, expected) = $tc;

                    if expected.is_some() {
                        assert_eq!(expected.clone().unwrap()[start_x][start_y], 1);
                    }
                    assert_eq!(expected, find_knight_tour(size_x, size_y, start_x, start_y));
                }
            )*
        }
    }
    test_knight_tour! {
        test_knight_tour_5x5: (5, 5, 0, 0, Some(vec![
            vec![1, 6, 15, 10, 21],
            vec![14, 9, 20, 5, 16],
            vec![19, 2, 7, 22, 11],
            vec![8, 13, 24, 17, 4],
            vec![25, 18, 3, 12, 23],
        ])),
        test_knight_tour_6x6: (6, 6, 0, 0, Some(vec![
            vec![1, 16, 7, 26, 11, 14],
            vec![34, 25, 12, 15, 6, 27],
            vec![17, 2, 33, 8, 13, 10],
            vec![32, 35, 24, 21, 28, 5],
            vec![23, 18, 3, 30, 9, 20],
            vec![36, 31, 22, 19, 4, 29],
        ])),
        test_knight_tour_8x8: (8, 8, 0, 0, Some(vec![
            vec![1, 60, 39, 34, 31, 18, 9, 64],
            vec![38, 35, 32, 61, 10, 63, 30, 17],
            vec![59, 2, 37, 40, 33, 28, 19, 8],
            vec![36, 49, 42, 27, 62, 11, 16, 29],
            vec![43, 58, 3, 50, 41, 24, 7, 20],
            vec![48, 51, 46, 55, 26, 21, 12, 15],
            vec![57, 44, 53, 4, 23, 14, 25, 6],
            vec![52, 47, 56, 45, 54, 5, 22, 13],
        ])),
        test_no_solution: (5, 5, 2, 1, None::<Vec<Vec<usize>>>),
        test_invalid_start_position: (8, 8, 10, 10, None::<Vec<Vec<usize>>>),
    }
}