use movegen::MoveGenerator;
use moves::Move;
use position::Position;

pub struct Game {
    // TODO why are these public?
    pub move_generator: MoveGenerator,
    pub position: Position
}

impl Game {
    pub fn new() -> Self {
        Self {
            move_generator: MoveGenerator::new(),
            position: Position::new_default()
        }
    }

    pub fn get_moves(&self) -> Vec<Move> {
        self.position.get_moves(&self.move_generator)
    }

    pub fn make_move(&mut self, move_: Move) {
        self.position = self.position.make_move(move_);
    }
}

