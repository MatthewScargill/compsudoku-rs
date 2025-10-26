use crate::board::*;

// move type

#[derive(Debug)]
pub enum Move {
    NakedSingle { row: usize, col: usize, value: u8 },
}

// board evaluator



// may have written this before even making a board evaluator woops
// find naked singles (in your area!) ie. cells with only 1 candidate
pub fn find_naked_singles(board: &Board) -> Vec<Move> {

    // set up vector of moves
    let mut moves = Vec::new();

    // iterate through all cells 
    for row in 0..9 {
        for col in 0..9 {
            let cell = &board.grid[row][col];
            if cell.value == 0 && cell.is_solvable() {
                let val = cell.find_candidate();
                moves.push(Move::NakedSingle { row, col, value: val as u8 });
            }
        }
    }

    moves
}