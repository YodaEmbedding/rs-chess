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

    println!("\n");

    loop {
        let (best_move, score) = game.get_best_move();

        println!("[{}]", iterator_join_sorted(game.get_moves().iter(), " "));
        println!("\n{}", game.position);
        println!("\nStatic evaluation: {:.2}", normalize_evaluation(game.position.evaluate()));
        println!(   "Depth evaluation: {:.2}", normalize_evaluation(score));
        println!("Best move: {}", best_move);
        println!("\n");

        game.make_move(best_move);
    }
}

// TODO
// Magics

// Experiment: Delta static evaluation (positional evaluation remains largely same between most positions)

