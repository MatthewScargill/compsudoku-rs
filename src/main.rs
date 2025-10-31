mod board; // mod imports functions
use board::*; // use imports structures
mod solvers;
mod evaluators;



// easy test puzzles (solvable by singles only)
const TEST1: &str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
const TEST2: &str = "040000005568010400107054600000008000730162098685470030400800250000706300026305001";

// no initial singles
const TEST3: &str = "800000000003600000070090200050007000000045700000100030001000068008500010090000400";

// First Times super-fiendish from my book (singles gets it as far as i got)
const TEST4: &str = "000800407070000002008000031420630000800040003000057024650000900700000040103005000";

//First Times fiendish from my book
const TEST5: &str = "010600034300800600004070000001000028000905000730000500000080900009004007270006050";
// seems like fiendish is a good level to aim for as super fiendish sometimes requires guessing
// which we can work on but for now let's keep it to logically solvable puzzles


fn main() {
    //initialise and setup board
    let mut board = Board::new();
    board.setup(TEST2);
    
    // look at solution
    board.solve();
}
