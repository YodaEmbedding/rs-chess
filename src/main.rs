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
    let bbs = game.queen_attack_map.iter()
        .map(|x| format!("{}", x))
        .take(16)
        .collect::<Vec<_>>();
    println!("\n{}", bbs.join("\n\n"));
}

// TODO
// Magics
// Evaluation
// a-b

// Experiment: Delta static evaluation (positional evaluation remains largely same between most positions)

