use std::fmt;

use crate::square::Square;

// TODO why are some of these needed?
enum MoveFlags {
    CastleKing,
    CastleQueen,
    Capture,
    DoublePawn,
    Enpassant,
}

#[derive(Debug, Clone, Copy)]
pub struct Move(pub u32);

impl Move {
    pub fn new(from: Square, to: Square, flags: u32) -> Self {
        Move(from.0 | to.0 << 6 | flags << 12)
    }

    #[inline] pub fn get_from(&self) -> Square { Square(self.0      & 0x3F) }
    #[inline] pub fn get_to  (&self) -> Square { Square(self.0 >> 6 & 0x3F) }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.get_from(), self.get_to())
    }
}

