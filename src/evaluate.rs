use crate::pieces::{Color, PieceName};
use crate::position::Position;

/// Standardized measurements in pawn units
pub fn normalize_evaluation(val: i32) -> f64 {
    0.01 * val as f64
}

impl Position {
    pub fn evaluate(&self) -> i32 {
        let game_phase = 0u8;  // 0 = early game, 50 = midgame, 100 = endgame
        // TODO make this an enum...? Or maybe do linear interp?

        // Also add in king value to avoid captures...? But legal move gen does that...
        (
            self.evaluate_center  (game_phase) +
            self.evaluate_material(game_phase)
        )
    }

    pub fn evaluate_breakdown(&self) -> String {
        let game_phase = 0u8;  // 0 = early game, 50 = midgame, 100 = endgame

        macro_rules! f { () => { "{: <12} {}" }; }
        [
            format!(f!(), "Center",   self.evaluate_center  (game_phase)),
            format!(f!(), "Material", self.evaluate_material(game_phase)),
        ].join("\n")
    }

    fn evaluate_material(&self, game_phase: u8) -> i32 {
        let w = self.get_bb_white();
        let b = self.get_bb_black();

        const MaterialValuesMg: [i32; 6] = [
            171,   // Pawn
            764,   // Knight
            826,   // Bishop
            1282,  // Rook
            2526,  // Queen
            65536, // King
        ];

        const MaterialValuesEg: [i32; 6] = [
            240,   // Pawn
            848,   // Knight
            891,   // Bishop
            1373,  // Rook
            2646,  // Queen
            65536, // King
        ];

        self.bitboard_piece.iter()
            .map(|x|
                (x.0 & w.0).count_ones() as i32 -
                (x.0 & b.0).count_ones() as i32)
            .zip(MaterialValuesMg.iter())
            .map(|(x, y)| x * y)
            .sum()
    }

    fn evaluate_center(&self, game_phase: u8) -> i32 {
        let w = self.get_bb_white();
        let b = self.get_bb_black();

        // From stockfish/psqt.cpp
        const PawnValuesMg: [i32; 64] = [
              0,   0,   0,   0,   0,   0,   0,   0,
              0,   0,   0,   0,   0,   0,   0,   0,
            -11,   6,   7,   3,   3,   7,   6, -11,
            -18,  -2,  19,  24,  24,  19,  -2, -18,
            -17,  -9,  20,  35,  35,  20,  -9, -17,
             -6,   5,   3,  21,  21,   3,   5,  -6,
             -6,  -8,  -6,  -2,  -2,  -6,  -8,  -6,
             -4,  20,  -8,  -4,  -4,  -8,  20,  -4,
        ];

        const PawnValuesEg: [i32; 64] = [
             0,  0,  0,  0,  0,  0,  0,  0,
             0,  0,  0,  0,  0,  0,  0,  0,
             7, -4,  8, -2, -2,  8, -4,  7,
            -4, -5,  5,  4,  4,  5, -5, -4,
             3,  3, -8, -3, -3, -8,  3,  3,
             8,  9,  7, -6, -6,  7,  9,  8,
             8, -5,  2,  4,  4,  2, -5,  8,
             3, -9,  1, 18, 18,  1, -9,  3,
        ];

        // TODO color should be +1, -1 to prevent branching...
        let x: i32 = self.bitboard_piece[PieceName::Pawn as usize].iter()
            .map(|sq|
                match self.piece_board.0[sq.0 as usize].unwrap().color {
                    Color::White =>  1 * PawnValuesMg[     sq.0 as usize],
                    Color::Black => -1 * PawnValuesMg[63 - sq.0 as usize],
                    // NOTE no need to rank/file index since matrix is symmetric
                })
            .sum();

        4 * x
    }
}

