extern crate arrayvec;

mod bitboard;
mod pieces;
mod position;

use position::Position;

fn main() {
    let position = Position::new_default();
    println!("{}", position);
}

// Design some data structures first?
// Also interfaces
// And function signatures

// Display piece_board and bitboards?

// Experiment: Delta static evaluation (positional evaluation remains largely same between most positions)

