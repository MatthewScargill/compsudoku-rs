use crate::solvers;
use crate::solvers::*;
use crate::evaluators;

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


    // solve the board automatically
    pub fn solve(&mut self) {

        // initial state of the board
        self.print();

        loop {

            // update candidates list with simple moves
            evaluators::basic(self);

            // find possible moves
            let mut moves = solvers::find_moves(&*self);
            let oldmoves = moves.clone(); // for comparison with new moves later


            // print moves for debugging
            for mv in &moves {
                println!("{:?}", mv);
            }

            evaluators::test(self);

            // find possible moves
            let mut newmoves = solvers::find_moves(&*self);
            
            newmoves.retain(|h| { !oldmoves.iter().any(|n| n.row() == h.row() && n.col() == h.col())});

            for mv in &newmoves {
                println!("{:?}", mv);
            }

            moves.extend(&newmoves);
            // break once the board is complete (no more moves)
            //if moves.is_empty() {
                //break;
            //}

            //evaluators::test(self);

            // update evaluators and resulting moves above here

            // if they exist, apply moves
            solvers::apply_moves(self, &newmoves);

            let mut allmoves: Vec<Move> = Vec::new();
            allmoves.extend(oldmoves);
            allmoves.extend(newmoves);

            if allmoves.is_empty() {break;}


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
