extern crate arrayvec;

use std::fmt;
use std::io::Write;

use self::arrayvec::ArrayVec;  // err... self::?

// enum Direction {
//   North =  8,
//   East  =  1,
//   South = -8,
//   West  = -1,
//   NorthEast = North + East,
//   SouthEast = South + East,
//   SouthWest = South + West,
//   NorthWest = North + West
// };

pub const COLOR_SIZE: usize = 2;
pub const PIECE_NAME_SIZE: usize = 6;

#[derive(Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug)]
pub enum PieceName {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    //All,
    //Empty,
}

// Consider making this a tuple struct?
// Or a uchar for space savings; with accessor methods for extracting pieceName/color
#[derive(Debug)]
pub struct Piece {
    pub piece_name: PieceName,
    pub color: Color,
}

#[derive(Debug)]
pub struct PieceBoard(pub ArrayVec<[Option<Piece>; 64]>);

impl Piece {
    pub fn new(piece_name: PieceName, color: Color) -> Piece {
        Piece { piece_name: piece_name, color: color }
    }
}


impl PieceBoard {
    pub fn new_default() -> PieceBoard {
        //macro_rules! piece {
        //    ($p:expr, $c:expr) => (Some(Piece::new(PieceName::$p, Color::$c)))
        //}
        //macro_rules! wp { () => (piece!(Pawn, White)) }

        macro_rules! wp { () => (Some(Piece::new(PieceName::Pawn, Color::White))) }
        macro_rules! bp { () => (Some(Piece::new(PieceName::Pawn, Color::Black))) }

        // TODO incomplete table
        PieceBoard(ArrayVec::from([
            None, None, None, None, None, None, None, None,
            wp!(), wp!(), wp!(), wp!(), wp!(), wp!(), wp!(), wp!(),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            bp!(), bp!(), bp!(), bp!(), bp!(), bp!(), bp!(), bp!(),
            None, None, None, None, None, None, None, None,
        ]))
    }
}

impl PieceName {
    fn to_letter(&self) -> String {
        match self {
            Pawn   => "p",
            Knight => "n",
            Bishop => "b",
            Rook   => "r",
            Queen  => "q",
            King   => "k"
        }
    }
}

impl fmt::Display for PieceName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_letter())
    }
}

impl fmt::Display for PieceBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

