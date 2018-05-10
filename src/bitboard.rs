use std::fmt;

use square::Square;

// Consider using pleco library?
// Or even better: bit_collection... looks quite complicated but probably fast too
// #[derive(BitCollection)]
#[derive(Debug, Copy, Clone)]
pub struct Bitboard(pub u64);

pub struct BitboardIterator {
    bitboard: Bitboard
}

pub const Empty: Bitboard = Bitboard(0x0000000000000000u64);
pub const FileA: Bitboard = Bitboard(0x0101010101010101u64);
pub const FileH: Bitboard = Bitboard(0x8080808080808080u64);
pub const Rank1: Bitboard = Bitboard(0x00000000000000FFu64);
pub const Rank8: Bitboard = Bitboard(0xFF00000000000000u64);
pub const A1H8:  Bitboard = Bitboard(0x8040201008040201u64);
pub const A8H1:  Bitboard = Bitboard(0x0102040810204080u64);

pub const A1: Bitboard = Bitboard(1 << 7);
pub const A8: Bitboard = Bitboard(1 << 7);

pub const HorzShiftMask: [Bitboard; 9] = [
    Bitboard(0x0000000000000000u64),
    Bitboard(0x0101010101010101u64),
    Bitboard(0x0303030303030303u64),
    Bitboard(0x0707070707070707u64),
    Bitboard(0x0F0F0F0F0F0F0F0Fu64),
    Bitboard(0x1F1F1F1F1F1F1F1Fu64),
    Bitboard(0x3F3F3F3F3F3F3F3Fu64),
    Bitboard(0x7F7F7F7F7F7F7F7Fu64),
    Bitboard(0xFFFFFFFFFFFFFFFFu64),
];

pub const VertShiftMask: [Bitboard; 9] = [
    Bitboard(0x0000000000000000u64),
    Bitboard(0x00000000000000FFu64),
    Bitboard(0x000000000000FFFFu64),
    Bitboard(0x0000000000FFFFFFu64),
    Bitboard(0x00000000FFFFFFFFu64),
    Bitboard(0x000000FFFFFFFFFFu64),
    Bitboard(0x0000FFFFFFFFFFFFu64),
    Bitboard(0x00FFFFFFFFFFFFFFu64),
    Bitboard(0xFFFFFFFFFFFFFFFFu64),
];

impl Bitboard {
    pub fn from(square: Square) -> Bitboard {
        Bitboard(1 << square.0)
    }

    pub fn iter(&self) -> BitboardIterator {
        BitboardIterator { bitboard: *self }
    }

    #[inline] pub fn shift_left (&self) -> Self { Bitboard((self.0 & !FileA.0) >> 1) }
    #[inline] pub fn shift_right(&self) -> Self { Bitboard((self.0 & !FileH.0) << 1) }
    #[inline] pub fn shift_up   (&self) -> Self { Bitboard((self.0 & !Rank8.0) << 8) }
    #[inline] pub fn shift_down (&self) -> Self { Bitboard((self.0 & !Rank1.0) >> 8) }

    pub fn shift_left_n (&self, n: usize) -> Self { Bitboard((self.0 & !HorzShiftMask[n].0)   >> n) }
    pub fn shift_right_n(&self, n: usize) -> Self { Bitboard((self.0 &  HorzShiftMask[8-n].0) << n) }
    pub fn shift_up_n   (&self, n: usize) -> Self { Bitboard((self.0 &  VertShiftMask[8-n].0) << 8*n) }
    pub fn shift_down_n (&self, n: usize) -> Self { Bitboard((self.0 & !VertShiftMask[n].0)   >> 8*n) }
}

impl Iterator for BitboardIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let tz = self.bitboard.0.trailing_zeros();
        if tz >= 64 { return None }
        self.bitboard.0 -= 1 << tz;
        Some(Square(tz))
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bin = format!("{:064b}", self.0);

        let xs = bin.chars()
            .map(|x| if x == '0' {'.'} else {x})
            .rev()
            .collect::<Vec<_>>();

        let rows = xs.chunks(8)
            .map(|x| x.into_iter().collect::<String>())
            .rev()
            .collect::<Vec<_>>();

        let grid = rows.join("\n");
        write!(f, "{}", grid)
    }
}

