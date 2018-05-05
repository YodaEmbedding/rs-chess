use std::fmt;

#[derive(Debug)]
pub struct Bitboard(pub u64);

impl Bitboard {
    // fn func(&self) -> Bitboard { self.0 }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

