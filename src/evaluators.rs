use crate::board::*;

// basic candidate evaluation
pub fn basic(board: &mut Board) {
    for row in 0..9 {
        for col in 0..9 {

            // temp variable because the borrow checker is a fickle mistress
            let mut used = [false; 9]; // reverse candidates array

            // if used up or down the colum -> add true in to used vector 
            for i in 0..9 {
                let num = board.grid[row][i].value;
                if num != 0 {
                    used[(num - 1) as usize] = true;
                }
            }

            for j in 0..9 {
                let num = board.grid[j][col].value;
                if num != 0 {
                    used[(num - 1) as usize] = true;
                }
            }

            let box_row = (row / 3) * 3; // floor div by 3 (0,1,2) * 3 to get beginning of each box row
            let box_col = (col / 3) * 3; // same for column
            for i in box_row..box_row + 3 {
                for j in box_col..box_col + 3 {
                    let val = board.grid[i][j].value;
                    if val != 0 {
                        used[(val - 1) as usize] = true;
                    }
                }
            }

            // now we can call the cell and replace with anti temp values
            let cell = &mut board.grid[row][col];

            if cell.value != 0 {
                cell.candidates = [false; 9]; // no candidates if it already has a value (sad this is at the end)
            } else {
                cell.candidates = [true; 9]; // reset between moves
                for i in 0..9 {
                    cell.candidates[i] = !used[i];
                }
            }
            // this was a bit convoluted but it works yay
        }
    }
}

// second tier candidate evaluation, considering (pointing) pairs, triples, ...
pub fn evaluate2(board: &mut Board) {
    // in each box, row, and column, look for n cells with the same n candidates
    // remove these candidates from the other cells in the box, row, or column
}