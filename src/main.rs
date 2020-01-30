#![feature(plugin, custom_attribute)]

extern crate arrayvec;
extern crate flame;

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
use std::fs::File;

use crate::evaluate::normalize_evaluation;
use crate::game::Game;

fn iterable_join<I, T>(v: I, sep: &str) -> String
where
    I: Iterator<Item = T>,
    T: fmt::Display,
{
    v.map(|x| x.to_string()).collect::<Vec<_>>().join(sep)
}

fn iterator_join_sorted<I, T>(v: I, sep: &str) -> String
where
    I: Iterator<Item = T>,
    T: fmt::Display,
{
    let mut v_ = v.map(|x| x.to_string()).collect::<Vec<_>>();
    v_.sort();
    v_.join(sep)
}

fn main() {
    let mut game = Game::new();

    println!("\n");

    loop {
        let res = game.get_best_move();

        let (best_move, score) = match res {
            Ok(t) => t,
            Err(e) => { println!("{:?}", e); break },
        };

        println!("[{}]", iterator_join_sorted(game.get_moves().iter(), " "));
        println!("\n{}", game.position);
        println!(
            "\nStatic evaluation: {:.2}",
            normalize_evaluation(game.position.evaluate())
        );
        println!("Depth evaluation:  {:.2}", normalize_evaluation(score));
        println!("Best move: {}", best_move);
        println!("\nStatic breakdown:");
        println!("{}", game.position.evaluate_breakdown());
        println!("\n");

        game.make_move(best_move);
    }

    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}

// TODO
// Magics

// Experiment: Delta static evaluation (positional evaluation remains largely same between most positions)
