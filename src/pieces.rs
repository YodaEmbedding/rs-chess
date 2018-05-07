use std::fmt;
use std::option::Option;
use std::string::String;

use arrayvec::ArrayVec;

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
    pub fn new(piece_name: PieceName, color: Color) -> Self {
        Piece { piece_name: piece_name, color: color }
    }
}

impl PieceBoard {
    pub fn new_default() -> Self {
        macro_rules! opt_piece {
            ($p:expr, $c:expr) => (Some(Piece::new($p, $c)))
        }

        macro_rules! wp { () => (opt_piece!(PieceName::Pawn,   Color::White)) }
        macro_rules! wn { () => (opt_piece!(PieceName::Knight, Color::White)) }
        macro_rules! wb { () => (opt_piece!(PieceName::Bishop, Color::White)) }
        macro_rules! wr { () => (opt_piece!(PieceName::Rook,   Color::White)) }
        macro_rules! wq { () => (opt_piece!(PieceName::Queen,  Color::White)) }
        macro_rules! wk { () => (opt_piece!(PieceName::King,   Color::White)) }
        macro_rules! bp { () => (opt_piece!(PieceName::Pawn,   Color::Black)) }
        macro_rules! bn { () => (opt_piece!(PieceName::Knight, Color::Black)) }
        macro_rules! bb { () => (opt_piece!(PieceName::Bishop, Color::Black)) }
        macro_rules! br { () => (opt_piece!(PieceName::Rook,   Color::Black)) }
        macro_rules! bq { () => (opt_piece!(PieceName::Queen,  Color::Black)) }
        macro_rules! bk { () => (opt_piece!(PieceName::King,   Color::Black)) }

        // TODO incomplete table
        PieceBoard(ArrayVec::from([
            wr!(), wn!(), wb!(), wq!(), wk!(), wb!(), wn!(), wr!(),
            wp!(), wp!(), wp!(), wp!(), wp!(), wp!(), wp!(), wp!(),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            bp!(), bp!(), bp!(), bp!(), bp!(), bp!(), bp!(), bp!(),
            br!(), bn!(), bb!(), bq!(), bk!(), bb!(), bn!(), br!(),
        ]))
    }
}

impl PieceName {
    fn to_char(&self) -> char {
        match *self {
            PieceName::Pawn   => 'p',
            PieceName::Knight => 'n',
            PieceName::Bishop => 'b',
            PieceName::Rook   => 'r',
            PieceName::Queen  => 'q',
            PieceName::King   => 'k'
        }
    }
}

impl fmt::Display for PieceName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{}", self.piece_name);

        write!(f, "{}", match self.color {
            Color::White => s.to_uppercase(),
            Color::Black => s
        })
    }
}

impl fmt::Display for PieceBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn opt_piece_to_str(piece: &Option<Piece>) -> String {
            match *piece {
                Some(ref p) => format!("{}", p),
                None        => String::from(".")
            }
        }

        let xs = self.0.iter().map(opt_piece_to_str).collect::<Vec<_>>();
        let rows = xs.chunks(8).map(|x| x.join("")).rev().collect::<Vec<_>>();
        let grid = rows.join("\n");

        write!(f, "{}", grid)
    }
}

