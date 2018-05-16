use game::Game;
use moves::Move;
use position::Position;

impl Game {
    /// Returns tuple of best move and evaluation score
    pub fn get_best_move(&self) -> (Move, i32) {
        let color = self.position.turn.to_int();  // 1 or -1 for white/black

        self.position.get_moves(&self.move_generator).into_iter()
            .map(|m| (m, self.position.make_move(m)))
            .map(|(m, p)| (m, self.negamax(&p, 3, -color)))
            .max_by_key(|(m, e)| color * e)
            .unwrap()
    }

    fn negamax(&self, position: &Position, depth: u32, color: i32) -> i32 {
        // evaluate() is + if white is winning
        if depth == 0 { return position.evaluate(); }

        color *
        position.get_moves(&self.move_generator).into_iter()
            .map(|m| position.make_move(m))
            .map(|p| color * self.negamax(&p, depth - 1, -color))
            .max()
            .unwrap()
    }
}

