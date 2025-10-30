use crate::solver;

#[derive(Clone, Copy)]
pub struct Cell {
    pub value: u8, // value of the cell
    pub candidates: [bool; 9], // if candidates[0] == true then 1 is a candidate 
    // subtle reminder to start at 0, Julia has scarred me with 1 indexing
}

impl Cell {

    // initialise a new cell
    pub fn new() -> Self {
        Self {value: 0, candidates: [true; 9],
        }
    }

    // check if there's a candidate
    pub fn has_candidate(&self, n: u8) -> bool {
        self.candidates[(n - 1) as usize]
    }

    // edit nth .candidates entries
    pub fn set_candidate(&mut self, n: u8, present: bool) {
        self.candidates[(n - 1) as usize] = present
    }
    
    // count number of True .candidates 
    pub fn count_candidates(&self) -> usize {
        self.candidates.iter().filter(|&&b| b).count()
        // find the candidates (reads &[bool; 9] from &self) 
        // -> iterate over each element (reads &bool from &[bool; 9])
        // -> filter the borrowed (&bool from &self) borrowed (from iter so &&bool) values
        // -> count the number of unfiltered (True) values 
        // rust is a beautiful language
    }

    // check if cell is solvable (number of candidates = 1)
    pub fn is_solvable(&self) -> bool {
        self.count_candidates() == 1
    }

    // find the value of the solvable cell 
    pub fn find_candidate(&self) -> usize {

        let mut index = 0;

        for (i, &candidate) in self.candidates.iter().enumerate() {
            if candidate {
                index = i
            }
        }
        index + 1
    }
}

pub struct Board {
    // 9 length array (rows) of 9 length arrays (columns) of cells 
    pub grid: [[Cell; 9]; 9],
}

impl Board {
    // make a new board
    pub fn new() -> Self {
        Self { grid: [[Cell::new(); 9]; 9] }
    }

    // set cell value. this is also on the chopping block
    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        self.grid[row][col].value = value;
    }

    // setup a board from a string of 81 integers
    pub fn setup(&mut self, setup: &str) {
        // length check
        assert!(setup.len() == 81, "Setup string must be exactly 81 characters"); 
        // for index i, iterate over every character ch in setup string
        for (i, ch) in setup.chars().enumerate() {
            let row = i / 9; 
            let col = i % 9;
            
            // unwrap char and break if invalid string
            let value = match ch {
                '1'..='9' => ch.to_digit(10).unwrap() as u8, 
                '0' | '.' => 0, 
                _ => panic!("Invalid character in setup string"), 
            };

            self.grid[row][col].value = value;
        }
    }

    // update the candidates of every cell
    pub fn evaluate(&mut self) {
        for row in 0..9 {
            for col in 0..9 {

                // temp variable because the borrow checker is a fickle mistress
                let mut used = [false; 9]; // reverse candidates array

                // if used up or down the colum -> add true in to used vector 
                for i in 0..9 {
                    let num = self.grid[row][i].value;
                    if num != 0 {
                        used[(num - 1) as usize] = true;
                    }
                }

                for j in 0..9 {
                    let num = self.grid[j][col].value;
                    if num != 0 {
                        used[(num - 1) as usize] = true;
                    }
                }

                let box_row = (row / 3) * 3; // floor div by 3 (0,1,2) * 3 to get beginning of each box row
                let box_col = (col / 3) * 3; // same for column
                for i in box_row..box_row + 3 {
                    for j in box_col..box_col + 3 {
                        let val = self.grid[i][j].value;
                        if val != 0 {
                            used[(val - 1) as usize] = true;
                        }
                    }
                }

                // now we can call the cell and replace with anti temp values
                let cell = &mut self.grid[row][col];

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

    // solve the board automatically
    pub fn solve(&mut self) {

        // initial state of the board
        self.print();

        loop {

            // update candidates list
            self.evaluate();

            // find possible moves
            let moves = solver::find_moves(&*self);

            // break once the board is complete (no more moves)
            if moves.is_empty() {
                break;
            }

            // if they exist, apply moves
            solver::apply_moves(self, &moves);

            // print moves for debugging
            for mv in moves {
                println!("{:?}", mv);
            }
            
            // board with above moves applied (again for debugging)
            self.print();
        }    
    }


    // terminal output of the current board
    pub fn print(&self) {
        for i in 0..9 {
            if i % 3 == 0 && i != 0 {
                println!("------+-------+------");
            }

            for j in 0..9 {
                if j % 3 == 0 && j != 0 {
                    print!("| ");
                }

                let val = self.grid[i][j].value;
                if val == 0 {
                    print!(". ");
                } else {
                    print!("{} ", val);
                }
            }

            println!();
        }
    }

}
