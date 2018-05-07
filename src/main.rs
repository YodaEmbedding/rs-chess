#![feature(const_fn)]

extern crate arrayvec;

mod bitboard;
mod game;
mod movegen;
mod pieces;
mod position;

use game::Game;
use pieces::PieceName;
use position::Position;

fn main() {
    let position = Position::new_default();
    println!("\n{}", position);
    println!("\n{}", position.bitboard_piece[PieceName::Pawn as usize]);

    let game = Game::new();
    let bbs = game.pawn_attack_map.iter()
        .map(|x| format!("{}", x))
        .take(16)
        .collect::<Vec<_>>();
    println!("\n{}", bbs.join("\n\n"));
}

// Design some data structures first?
// Also interfaceslet s: String = v.iter().collect();
// And function signatures

// Display piece_board and bitboards?

// Experiment: Delta static evaluation (positional evaluation remains largely same between most positions)

