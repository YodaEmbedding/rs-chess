extern crate arrayvec;

mod bitboard;
mod game;
mod moves;
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
    let bbs = game.move_generator.bishop_attack_map.iter()
        .map(|x| format!("{}", x))
        .take(32)
        .collect::<Vec<_>>();
    println!("\n{}", bbs.join("\n\n"));

    let bb = game.move_generator.bishop_attack_map[0];
    let vbb = bb.iter().collect::<Vec<_>>();
    println!("\n{:?}\n{}", vbb, bb);
}

// TODO
// Magics
// Evaluation
// a-b

// Experiment: Delta static evaluation (positional evaluation remains largely same between most positions)

