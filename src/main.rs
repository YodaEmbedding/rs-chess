#![feature(const_fn)]

extern crate arrayvec;

mod bitboard;
mod movegen;
mod pieces;
mod position;

use position::Position;
use pieces::PieceName;

fn main() {
    let position = Position::new_default();
    println!("\n{}", position);
    println!("\n{}", position.bitboard_piece[PieceName::Pawn as usize]);

    let bbs = movegen::init_rook_attacks().iter()
        .map(|x| format!("{}", x))
        .take(3)
        .collect::<Vec<_>>();
    println!("\n{}", bbs.join("\n\n"));
}

// Design some data structures first?
// Also interfaceslet s: String = v.iter().collect();
// And function signatures

// Display piece_board and bitboards?

// Experiment: Delta static evaluation (positional evaluation remains largely same between most positions)

