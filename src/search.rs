use game::Game;
use moves::Move;

// Alpha beta search?

impl Game {
    pub fn get_best_move(&self) -> Move {
        // self.minimax(self.position, 3)

        // first try a single depth search
        // for move_ in position.get_moves() {
        // }
        // TODO consider unmake_move instead of zipping moves?
        self.position.get_moves(&self.move_generator).into_iter()
            .map(|x| (x, self.position.make_move(x)))
            .max_by_key(|x| x.1.evaluate())
            .unwrap()
            .0
    }

    // fn minimax(&self, position: &Position, depth: u32) -> (Move, u32) {
    //     // TODO ply?

    //     if depth == 0 {  }
    // }
}

