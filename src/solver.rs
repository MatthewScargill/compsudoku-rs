use crate::board::*;
use std::collections::HashSet;

// move type

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Move {
    NakedSingle { row: usize, col: usize, value: u8 }, // only have one candidate possible 
    HiddenSingle { row: usize, col: usize, value: u8 } // have unique candidate in row/column/subgrid 
}

impl Move {
    pub fn row(&self) -> usize {
        match *self {
            Move::NakedSingle { row, .. } | Move::HiddenSingle { row, .. }=> row,
        }
    }

    pub fn col(&self) -> usize {
        match *self {
            Move::NakedSingle { col, .. } | Move::HiddenSingle { col, .. } => col,
        }
    }

    pub fn value(&self) -> u8 {
        match *self {
            Move::NakedSingle { value, .. } | Move::HiddenSingle { value, .. } => value,
        }
    }
}

// find moves
pub fn find_moves(board: &Board) -> Vec<Move> {

    // initialise vecotr of moves
    let mut moves = Vec::new();

    // make vectors of all the moves
    let mut nakedsingles = find_naked_singles(board);
    let mut hiddensingles = find_hidden_singles(board);

    // add all move types to move vector
    moves.extend(nakedsingles);
    moves.extend(hiddensingles);

    // filter for unique moves
    let unique_moves: Vec<Move> = HashSet::<_>::from_iter(moves).into_iter().collect();
    // this was and forever will be a stack overflow job


    // return vector of unique moves
    unique_moves
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

// finding cells which have a unique candidate in their row/column/subgrid
// i personally hate this function it made me have to figure out how to remove duplicates and it can't be very 
// fast but oh well, to be fixed soon (if it isn't already broken)
pub fn find_hidden_singles(board: &Board) -> Vec<Move> {

    let mut moves = Vec::new();

    for row in 0..9 {
        for digit in 1..10 {
            let mut count = 0;
            let mut col_pos = 0;

            for col in 0..9 {
                let cell = &board.grid[row][col];
                if cell.value == 0 && cell.candidates[(digit - 1) as usize] {
                    count += 1;
                    col_pos = col;
                }
            }

            if count == 1 {
                moves.push(Move::HiddenSingle {
                    row,
                    col: col_pos,
                    value: digit,
                });
            }
        }
    }
    
    for col in 0..9 {
        for digit in 1..10 {
            let mut count = 0;
            let mut row_pos = 0;

            for row in 0..9 {
                let cell = &board.grid[row][col];
                if cell.value == 0 && cell.candidates[(digit - 1) as usize] {
                    count += 1;
                    row_pos = row;
                }
            }

            if count == 1 {
                moves.push(Move::HiddenSingle {
                    row: row_pos,
                    col,
                    value: digit,
                });
            }
        }
    }

    for box_row in 0..3 {
        for box_col in 0..3 {
            for digit in 1..10 {
                let mut count = 0;
                let mut target = (0, 0);

                for r in 0..3 {
                    for c in 0..3 {
                        let row = box_row * 3 + r;
                        let col = box_col * 3 + c;
                        let cell = &board.grid[row][col];

                        if cell.value == 0 && cell.candidates[(digit - 1) as usize] {
                            count += 1;
                            target = (row, col);
                        }
                    }
                }

                if count == 1 {
                    moves.push(Move::HiddenSingle {
                        row: target.0,
                        col: target.1,
                        value: digit,
                    });
                }
            }
        }
    }
    moves // this produces duplicates in the moves but oh well still works for the solver for now
}