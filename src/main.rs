mod solver;
use solver::*;
mod board; 
use board::*;

// easy test setup with naked single
const TEST: &str = "800000000003600000070090200050007000000045702000100030001000068008500010090000400";

// test setup with naked singles 
const TEST2: &str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";

fn main() {
    let mut board = Board::new();
    //board.print();
    board.setup(TEST2);
    board.evaluate();
    board.print();
    let moves = solver::find_moves(&board);
    
    for mv in &moves {
        println!("{:?}", mv);
    }

    println!("{:?}", &board.grid[7][7].candidates);
    solver::apply_moves(&mut board, &moves);

    board.print()

}
