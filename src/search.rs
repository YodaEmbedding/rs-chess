use std;
use std::cmp;

use crate::game::Game;
use crate::moves::Move;
use crate::position::Position;

#[derive(Debug, Clone, Copy)]
pub enum GameError {
    Checkmate,
    Stalemate,
}

impl Game {
    #[flame]
    /// Returns tuple of best move and evaluation score
    pub fn get_best_move(&self) -> Result<(Move, i32), GameError> {
        let color = self.position.turn.to_int();  // 1 or -1 for white/black

        // TODO No legal moves exception: stalemate, checkmate
        let moves = self.position.get_moves(&self.move_generator);

        // TODO always returns stalemate?
        if moves.len() == 0 { return Err(GameError::Stalemate); }

        // TODO should we really consider all moves?
        //      should this loop be inside alpha beta instead?
        let best = moves.into_iter()
            .map(|m| (m, self.position.make_move(m)))
            .map(|(m, p)| (m, self.negamax(&p, 5, -color, std::i32::MIN + 1, std::i32::MAX)))
            .max_by_key(|(m, v)| color * v)
            .unwrap();

        Ok(best)
    }

    fn negamax(&self, position: &Position, depth: u32, color: i32,
        alpha: i32, beta: i32) -> i32 {

        if depth == 0 { return position.evaluate(); }

        let mut a = alpha;
        let mut best = std::i32::MIN + 1;
        let positions = position.get_moves(&self.move_generator).into_iter()
            .map(|m| position.make_move(m));

        for p in positions {
            let v = color * self.negamax(&p, depth - 1, -color, -beta, -a);
            best = cmp::max(v, best);
            a    = cmp::max(v, a);
            if a >= beta { break; }
        }

        color * best
    }

    // TODO transposition tables
}

