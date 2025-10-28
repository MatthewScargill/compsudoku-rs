use crate::board::*;

// move type

#[derive(Debug)]
pub enum Move {
    NakedSingle { row: usize, col: usize, value: u8 },
}

impl Move {
    pub fn row(&self) -> usize {
        match *self {
            Move::NakedSingle { row, .. } => row,
        }
    }

    pub fn col(&self) -> usize {
        match *self {
            Move::NakedSingle { col, .. } => col,
        }
    }

    pub fn value(&self) -> u8 {
        match *self {
            Move::NakedSingle { value, .. } => value,
        }
    }
}

// find moves
pub fn find_moves(board: &Board) -> Vec<Move> {

    // initialise vecotr of moves
    let mut moves = Vec::new();

    // make vectors of all the moves
    let mut nakedmoves = find_naked_singles(board);


    // add every element 
    moves = std::mem::take(&mut nakedmoves);


    // return vector of moves
    moves
}

pub fn apply_moves(board: &mut Board, moves: &Vec<Move>) {
    for mv in moves {
        board.grid[mv.row()][mv.col()].value = mv.value();
    }
}

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