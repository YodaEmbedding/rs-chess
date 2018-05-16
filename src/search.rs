use game::Game;
use moves::Move;
use position::Position;

// Alpha beta search?

impl Game {
    /// Returns tuple of best move and evaluation
    pub fn get_best_move(&self) -> (Move, i32) {
        // TODO consider unmake_move instead of zipping moves?
        // self.position.get_moves(&self.move_generator).into_iter()
        //     .map(|m| (m, self.position.make_move(m)))
        //     .max_by_key(|(m, p)| p.evaluate())
        //     .unwrap()
        //     .0

        // TODO movegen should probably handle colors itself...?
        let color = self.position.turn.to_int();

        self.position.get_moves(&self.move_generator).into_iter()
            .map(|m| (m, self.position.make_move(m)))
            .map(|(m, p)| (m, self.minimax(&p, 2, -color)))
            .max_by_key(|(m, e)| color * e)
            .unwrap()
    }

    fn minimax(&self, position: &Position, depth: u32, color: i32) -> i32 {
        if depth == 0 { return position.evaluate(); }

        color *
        self.position.get_moves(&self.move_generator).into_iter()
            .map(|m| self.position.make_move(m))
            .map(|p| color * self.minimax(&p, depth - 1, -color))
            .max()
            .unwrap()
    }

    // // NOTE In order for negaMax to work, your Static Evaluation function must return a score relative to the side to being evaluated
    // fn negamax(&self, position: &Position, depth: u32, color: i32) -> i32 {
    //     if depth == 0 { color * position.evaluate() }

    //     -self.position.get_moves(&self.move_generator).into_iter()
    //         .map(|m| self.position.make_move(m))
    //         .map(|p| negamax(p, depth - 1, -color))
    //         .min()
    //         .unwrap()
    // }
}

