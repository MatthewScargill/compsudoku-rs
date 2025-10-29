mod solver;
use solver::*;
mod board; 
use board::*;

// no naked or hidden singles 
const TEST: &str = "800000000003600000070090200050007000000045700000100030001000068008500010090000400";

// naked and hidden singles 
const TEST2: &str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";

fn main() {
    let mut board = Board::new();
    //board.print();
    board.setup(TEST);
    board.evaluate();
    board.print();

    let mut i = 0;

    while i < 0 {
        let moves = solver::find_moves(&board);
    
        for mv in &moves {
            println!("{:?}", mv);
        }

        solver::apply_moves(&mut board, &moves);

        board.print();
        i += 1
    }

}
