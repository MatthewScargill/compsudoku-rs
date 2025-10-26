mod board;
use board::*;

const TEST: &str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";

fn main() {
    let mut board = Board::new();
    board.setup(TEST);
    board.print();
}   
