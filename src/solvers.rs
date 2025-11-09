use crate::board::*;

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

    // initialise vector of moves
    let mut moves = Vec::new();

    // make vectors of all the moves
    let nakedsingles = find_naked_singles(board);
    let mut hiddensingles = find_hidden_singles(board);
    
    // removes hidden singles which are also naked singles
    hiddensingles.retain(|h| { !nakedsingles.iter().any(|n| n.row() == h.row() && n.col() == h.col())});

    // add all move types to move vector
    moves.extend(nakedsingles);
    moves.extend(hiddensingles);

    moves
}

// applying moves from a vector
pub fn apply_moves(board: &mut Board, moves: &Vec<Move>) {
    for mv in moves {
        board.grid[mv.row()][mv.col()].value = mv.value();
    }
}

// find naked singles (in your area!) ie. cells with only 1 candidate
pub fn find_naked_singles(board: &Board) -> Vec<Move> {

    // set up vector of moves
    let mut moves = Vec::new();

    // iterate through all cells 
    for row in 0..9 {
        for col in 0..9 {
            let cell = board.grid[row][col];
            if cell.value == 0 && cell.is_solvable() {
                let val = cell.find_candidate();
                moves.push(Move::NakedSingle { row, col, value: val as u8 });
            }
        }
    }
    moves // no duplicates and currently works perfectly
}

// finding cells which have a unique candidate in their row/column/subgrid
fn find_hidden_singles(board: &Board) -> Vec<Move> {

    let mut moves = Vec::new();

    for row in 0..9 {
        for col in 0..9 {

            // skip solved cells
            if board.grid[row][col].value != 0 { continue; }

            // For each candidate digit d in this cell, if it's unique it goes in possible
            let mut possible = Vec::new();

            // looping over every possible candidate to check if they're unique
            for d in 1..=9 {

                // skip if d not a candidate
                if !board.grid[row][col].candidates[(d-1) as usize] { continue; }

                // counter variable 
                let mut cnt = 0;

                // testing uniqueness in all categories, if any category filled -> candidate saved and move on

                // unique in row?
                for i in 0..9 {
                    if board.grid[row][i].candidates[(d-1) as usize] { 
                        cnt += 1; 
                    }
                }
                if cnt == 1 { possible.push(d); continue; } // add to possible and skip

                // unique in col?
                cnt = 0; // reset between tests
                for j in 0..9 {
                    if board.grid[j][col].candidates[(d-1) as usize] { 
                        cnt += 1;
                    }
                }
                if cnt == 1 { possible.push(d); continue; } // add to possible and skip

                // unique in box?
                let br = (row/3)*3; 
                let bc = (col/3)*3;
                cnt = 0;
                for rr in br..br+3 {
                    for cc in bc..bc+3 {
                        if board.grid[rr][cc].candidates[(d-1) as usize] { cnt += 1; }
                    }
                }
                if cnt == 1 { possible.push(d); }
            }

            // we have a list of candidates that fill one of the uniqueness conditions, but we only KNOW
            // it to be the solution if every other candidate fails all the uniqueness tests 
            if possible.len() == 1 {
                moves.push(Move::HiddenSingle { row: row, col: col, value: possible[0] });
            }
        }
    }

    moves
}
