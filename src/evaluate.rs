use arrayvec::ArrayVec;

use position::Position;

impl Position {
    pub fn evaluate(&self) -> i32 {
        let game_phase = 0u8;  // 0 = early game, 50 = midgame, 100 = endgame

        // Also add in king value to avoid captures...? But legal move gen does that...
        self.evaluate_material(game_phase)
    }

    /// Standardized measurements in pawn units
    pub fn evaluate_normalized(&self) -> f64 {
        0.01 * self.evaluate() as f64
    }

    fn evaluate_material(&self, game_phase: u8) -> i32 {
        let w = self.get_bb_white();
        let b = self.get_bb_black();

        // const MaterialValuesMg: ArrayVec<[i32; 5]> = ArrayVec::from([
        const MaterialValuesMg: [i32; 5] = [
            171,   // Pawn
            764,   // Knight
            826,   // Bishop
            1282,  // Rook
            2526,  // Queen
            // 10000, // King
        ];

        const MaterialValuesEg: [i32; 5] = [
            240,   // Pawn
            848,   // Knight
            891,   // Bishop
            1373,  // Rook
            2646,  // Queen
            // 10000, // King
        ];

        self.bitboard_piece.iter()
            .map(|x|
                (x.0 & w.0).count_ones() as i32 -
                (x.0 & b.0).count_ones() as i32)
            .zip(MaterialValuesMg.iter())
            .map(|(x, y)| x * y)
            .sum()
    }
}

