extern crate arrayvec;

mod bitboard;
mod evaluate;
mod game;
mod movegen;
mod moves;
mod pieces;
mod position;
mod search;
mod square;

use std::fmt;

use evaluate::normalize_evaluation;
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

fn iterator_join_sorted<I, T>(v: I, sep: &str) -> String
    where I: Iterator<Item=T>,
          T: fmt::Display {
    let mut v_ = v
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    v_.sort();
    v_.join(sep)
}

fn main() {
    let mut game = Game::new();

    // let bbs = game.move_generator.knight_attack_map.iter();
    // println!("\n{}", iterator_join_sorted(bbs.take(2), "\n\n"));

    // let bb = game.move_generator.knight_attack_map[2];
    // println!("\n{}\n{}", iterable_join(bb.iter(), " "), bb);

    // let move_ = Move::new(Square(0x0C), Square(0x1C), 0x00);
    // println!("\n{}", move_);

    loop {
        println!("\n{}", game.position);
        println!("\n[{}]", iterator_join_sorted(game.get_moves().iter(), " "));
        println!("\nStatic evaluation: {:.2}", normalize_evaluation(game.position.evaluate()));

        let (best_move, score) = game.get_best_move();
        println!("Depth evaluation: {:.2}", normalize_evaluation(score));
        println!("Best move: {}", best_move);

        game.make_move(best_move);
    }
}

// TODO
// Magics
// Evaluation
// a-b

// Experiment: Delta static evaluation (positional evaluation remains largely same between most positions)

