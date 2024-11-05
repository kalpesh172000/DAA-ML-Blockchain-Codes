
#[derive(Clone)]
struct Nqueen {
    size: usize,
    board: Vec<Vec<bool>>,
}

impl Nqueen {
    fn new(size: usize, first_queen_position: (usize, usize)) -> Self {
        let mut board = vec![vec![false;size];size];
        board[first_queen_position.0][first_queen_position.1] = true;
        Nqueen { size, board }
    }

    fn is_safe(&self, row: usize, col: usize) -> bool {
        for i in 0..row {
            if self.board[i][col] {
                return false;
            }
        }

        //upper left diagonal
        let mut i = (row as isize) - 1;
        let mut j = (col as isize) - 1;
        while i >= 0 && j >= 0 {
            if self.board[i as usize][j as usize] {
                return false;
            }
            i -= 1;
            j -= 1;
        }

        //upper right diagonal
        let mut i = (row as isize) - 1;
        let mut j = (col as isize) + 1;
        while i >= 0 && j < (self.size as isize) {
            if self.board[i as usize][j as usize] {
                return false;
            }
            i -= 1;
            j += 1;
        }

        //below left diagonal
        let mut i = (row as isize) + 1;
        let mut j = (col as isize) - 1;
        while i < (self.size as isize) && j >= 0 {
            if self.board[i as usize][j as usize] {
                return false;
            }
            i += 1;
            j -= 1;
        }

        //below left diagonal
        let mut i = (row as isize) + 1;
        let mut j = (col as isize) + 1;
        while i < (self.size as isize) && j < (self.size as isize) {
            if self.board[i as usize][j as usize] {
                return false;
            }
            i += 1;
            j += 1;
        }


        true
    }

    fn solve_nqueens(&mut self, row: usize) -> bool {
        if row >= self.size {
            return true;
        }

        //skip the row for already placed queen
        if self.board[row].iter().any(|&cell| cell) {
            return self.solve_nqueens(row + 1);
        }

        for col in 0..self.size {
            if self.is_safe(row, col) {
                self.board[row][col] = true;

                if self.solve_nqueens(row + 1) {
                    return true;
                }

                self.board[row][col] = false;
            }
        }
        false
    }

    fn print_board(&self) {
        for row in &self.board {
            for &col in row {
                print!("{}", if col { "Q" } else { "_" });
            }
            println!();
        }
    }
}

fn main() {
    let size = 4;
    let first_queen_position = (1, 1);

    let mut nqueen = Nqueen::new(size, first_queen_position);

    if nqueen.solve_nqueens(0) {
        println!("solution exists");
        nqueen.print_board();
    } else {
        println!("solution doesn't exists");
    }
}
