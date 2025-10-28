mod solver;
use solver::*;
mod board; 
use board::*;

// easy test setup
const TEST: &str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";

// test setup with naked singles 
const TEST2: &str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";

fn main() {
    let mut board = Board::new();
    //board.print();
    board.setup(TEST2);
    board.evaluate();
    board.print();
    let moves = solver::find_naked_singles(&board);
    
    for mv in &moves {
        println!("{:?}", mv);
    }

    solver::apply_moves(&mut board, &moves);

    board.print()

}
