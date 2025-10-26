pub struct Cell {

    pub value: u8, // what value is it

    pub row: u8,
    pub col: u8, // are these really needed?

    pub candidates: [bool; 9], // if candidates[0] == true then 1 is a candidate 
    // subtle reminder to start at 0, Julia has scarred me with 1 indexing

}

pub impl Cell {

    // got to be able to set the value
    pub fn set(&mut self, value: u8) {
        self.value = value
    }

    // don't think i need read as can just .value

    // check if there's a candidate
    pub fn has_candidate(&self, n: u8) -> bool {
        self.candidates[(n - 1) as usize]
    }

    // set candidates 
    pub fn set_candidate(&mut self, n: u8, present: bool) {
        self.candidates[(n - 1) as usize] = present
    }
    
    // count so then can query whether solvable ie count == 1
    pub fn count_candidates(&self) -> usize {
        self.candidates.iter().filter(|&&b| b).count()
        // find the candidates (reads &[bool; 9] from &self) 
        // -> iterate over each element (reads &bool from &[bool; 9])
        // -> filter the borrowed (from &self) borrowed (from iter) for true values 
        // -> count the number of true values (from &bool to bool then +1)
        // rust is a beautiful language
    }

    pub fn is_solvable(&self) -> bool {
        self.count_candidates() == 1
    }
}

pub struct Board {
    pub grid: [[Cell; 9]; 9],
}

pub impl Board {

    // make a new board
    fn new() -> Self {
        Self { grid: [[Cell; 9]; 9] }
    }

    // find a cell's value 
    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.grid[row][col].value
    }

    // set a cell's value
    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        self.grid[row][col].value = value;
    }

    // setup a board from a string of 81 integers
    pub fn setup(&mut self, setup: str) { 
    }
}
