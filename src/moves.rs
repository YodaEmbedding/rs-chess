
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

