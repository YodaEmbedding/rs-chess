use movegen::MoveGenerator;

pub struct Game {
    // TODO why are these public?
    pub move_generator: MoveGenerator
}

impl Game {
    pub fn new() -> Self {
        Self {
            move_generator: MoveGenerator::new()
        }
    }
}

