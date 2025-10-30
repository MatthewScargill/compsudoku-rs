mod solver;
mod board; 
use board::*;

// no naked or hidden singles 
const TEST: &str = "800000000003600000070090200050007000000045700000100030001000068008500010090000400";

// naked and hidden singles 
const TEST2: &str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";

// easy puzzle to test singles detection -- currently broken
const TEST3: &str = "040000005568010400107054600000008000730162098685470030400800250000706300026305001";

fn main() {
    //initialise and setup board
    let mut board = Board::new();
    board.setup(TEST2);
    
    // look at solution
    board.solve(); // this doesn't work yet
}
