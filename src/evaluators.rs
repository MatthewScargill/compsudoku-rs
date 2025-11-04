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
pub fn medium(board: &mut Board) {
    // in each box, row, and column, look for n cells with the same n candidates
    // remove these candidates from the other cells in the box, row, or column
    // does the row and column logic automatically take care of pointing pairs and triples?
    // i dont think it matters if in the same box, as behaves the same whether or not they're pointing
    // just if in the same box then can get rid of them from the the same box

    // plan: row and column logic, and if they share a box, edit out of other cells in the box + means no repeats

    // do we go for every combination? that's 9C2 + 9C3 = 36 + 84 = 120 combinations 
    // also need to check the candidates are unique to those 2 
    // candidate sort 


    // reusable variables
    let mut count = 0;
    let mut cells: Vec<usize> = Vec::new();

    //row per row
    for row in 0..9 {
        for i in 1..10 {
            let mut count = 0;
            let mut indices: Vec<usize> = Vec::new();
            // checks to see if i is a candidate, if it is 
            if board.grid[row][i].has_candidate(i as u8) { count += 1;}
            if count > 2 {continue;}
            if count == 2 {cells.push(i); }
        }
        
        if board.grid[row][cells[0]].has_candidate(cells[0] as u8) 
        && board.grid[row][cells[0]].has_candidate(cells[0] as u8) {

        }

    }

    // clear between types of moves
    count = 0;
    cells.clear();




}

pub fn test(board: &mut Board) {
    // closure to process any unit (row, column, or box)
    let mut process_unit = |coords: &[(usize, usize)], unit_name: &str, unit_idx: usize| {
        // For each digit 1–9, record which cells it appears in
        let mut digit_cells: [Vec<(usize, usize)>; 9] = Default::default();

        for &(r, c) in coords {
            let cell = &board.grid[r][c];
            if cell.value != 0 {
                continue; // skip solved cells
            }
            for digit in 1..=9 {
                if cell.candidates[digit - 1] {
                    digit_cells[digit - 1].push((r, c));
                }
            }
        }

        // Find all combinations of 2 or 3 digits that share exactly 2 or 3 identical cells
        for n in [2, 3] {
            for d1 in 1..=(9 - (n - 1)) {
                for d2 in (d1 + 1)..=9 {
                    if n == 2 {
                        let cells1 = &digit_cells[d1 - 1];
                        let cells2 = &digit_cells[d2 - 1];
                        if cells1.len() == 2 && cells2.len() == 2 && cells1 == cells2 {
                            apply_hidden_set(board, coords, cells1, &[d1, d2]);
                            println!(
                                "Hidden pair in {} {}: digits ({}, {}) at {:?}",
                                unit_name, unit_idx + 1, d1, d2, cells1
                            );
                        }
                    } else {
                        for d3 in (d2 + 1)..=9 {
                            let cells1 = &digit_cells[d1 - 1];
                            let cells2 = &digit_cells[d2 - 1];
                            let cells3 = &digit_cells[d3 - 1];

                            // all must have exactly 3 candidate cells, and all must be identical
                            if cells1.len() == 3
                                && cells2.len() == 3
                                && cells3.len() == 3
                                && cells1 == cells2
                                && cells2 == cells3
                            {
                                apply_hidden_set(board, coords, cells1, &[d1, d2, d3]);
                                println!(
                                    "Hidden triple in {} {}: digits ({}, {}, {}) at {:?}",
                                    unit_name, unit_idx + 1, d1, d2, d3, cells1
                                );
                            }
                        }
                    }
                }
            }
        }
    };

    // rows
    for row in 0..9 {
        let coords: Vec<(usize, usize)> = (0..9).map(|col| (row, col)).collect();
        process_unit(&coords, "row", row);
    }

    // columns
    for col in 0..9 {
        let coords: Vec<(usize, usize)> = (0..9).map(|row| (row, col)).collect();
        process_unit(&coords, "column", col);
    }

    // boxes
    for box_idx in 0..9 {
        let br = (box_idx / 3) * 3;
        let bc = (box_idx % 3) * 3;
        let coords: Vec<(usize, usize)> =
            (0..3).flat_map(|r| (0..3).map(move |c| (br + r, bc + c))).collect();
        process_unit(&coords, "box", box_idx);
    }

    // inline helper (hidden sets elimination)
    fn apply_hidden_set(
        board: &mut Board,
        coords: &[(usize, usize)],
        target_cells: &[(usize, usize)],
        digits: &[usize],
    ) {
        // only these digits in the target cells
        for &(r, c) in target_cells {
            let cell = &mut board.grid[r][c];
            for d in 1..=9 {
                cell.candidates[d - 1] = digits.contains(&d);
            }
        }

        // remove these digits from all other cells in the same unit
        for &(r, c) in coords {
            if target_cells.contains(&(r, c)) || board.grid[r][c].value != 0 {
                continue;
            }
            let cell = &mut board.grid[r][c];
            for &d in digits {
                cell.candidates[d - 1] = false;
            }
        }
    }
}