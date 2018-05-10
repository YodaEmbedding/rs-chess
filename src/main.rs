extern crate arrayvec;

mod bitboard;
mod game;
mod movegen;
mod moves;
mod pieces;
mod position;
mod square;

use std::fmt;
use std::slice::Iter;

use game::Game;
use moves::Move;
use pieces::PieceName;
use position::Position;
use square::Square;

fn iterable_join<I, T>(v: I, sep: &str) -> String
    where I: Iterator<Item=T>,
          T: fmt::Display {
    v
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(sep)
}

fn main() {
    let game = Game::new();
    let bbs = game.move_generator.bishop_attack_map.iter()
        .map(|x| format!("{}", x))
        .take(2)
        .collect::<Vec<_>>();
    println!("\n{}", bbs.join("\n\n"));

    let bb = game.move_generator.bishop_attack_map[2];
    let sbb = iterable_join(bb.iter(), " ");
    println!("\n{}\n{}", sbb, bb);

    let position = Position::new_default();
    println!("\n{}", position);
    println!("\n{}", position.bitboard_piece[PieceName::Pawn as usize]);

    let mut moves = game.get_moves().iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    moves.sort();
    println!("\n[{}]", moves.join(" "));
    // println!("\n[{}]", iterable_join(game.get_moves().iter(), " "));

    let move_ = Move::new(Square(0x0C), Square(0x1C), 0x00);
    println!("\n{}", move_);
}

// TODO
// Magics
// Evaluation
// a-b

// Experiment: Delta static evaluation (positional evaluation remains largely same between most positions)

