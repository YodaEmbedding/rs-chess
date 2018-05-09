use position::Position;

pub type Square = u32;
pub struct Move(pub u32);

// TODO why are some of these needed?
enum MoveFlags {
    CastleKing,
    CastleQueen,
    Capture,
    DoublePawn,
    Enpassant,
}

impl Move {
    pub fn new(from: Square, to: Square, flags: u32) -> Self {
        Move(from | to << 6 | flags << 12)
    }
}

impl Position {
    const INITIAL_MOVELIST_CAPACITY: usize = 64;

    pub fn get_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::with_capacity(Self::INITIAL_MOVELIST_CAPACITY);

        // Player, Opponent?

        // get_valid_attacks() // different for sliding?
        // if bish,queen,rook, do get_moves_sliding()

        moves
    }

    // fn get_pawn_attacks(&self) -> [Moves] {
    //
    // }

    // fn get_attacks(&self) -> Vec<Move> {
    // }
}

// Magics?

