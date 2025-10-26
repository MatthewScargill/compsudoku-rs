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

    // set .value of the cell
    pub fn set(&mut self, value: u8) {
        self.value = value
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

    // find cell value
    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.grid[row][col].value
    }

    // set cell value
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
